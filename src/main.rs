use std::time::SystemTime;

use parking_lot::Mutex;
use strum::IntoEnumIterator;
use tokio::task::spawn;

use inputbot::{BlockInput, KeybdKey};

#[macro_use]
extern crate log;

mod enabled;

use enabled::ENABLED;

static KEYS: [KeybdKey; 26] = [
    KeybdKey::AKey,
    KeybdKey::BKey,
    KeybdKey::CKey,
    KeybdKey::DKey,
    KeybdKey::EKey,
    KeybdKey::FKey,
    KeybdKey::GKey,
    KeybdKey::HKey,
    KeybdKey::IKey,
    KeybdKey::JKey,
    KeybdKey::KKey,
    KeybdKey::LKey,
    KeybdKey::MKey,
    KeybdKey::NKey,
    KeybdKey::OKey,
    KeybdKey::PKey,
    KeybdKey::QKey,
    KeybdKey::RKey,
    KeybdKey::SKey,
    KeybdKey::TKey,
    KeybdKey::UKey,
    KeybdKey::VKey,
    KeybdKey::WKey,
    KeybdKey::XKey,
    KeybdKey::YKey,
    KeybdKey::ZKey,
];

static PRESSED_KEYS: Mutex<Vec<(KeybdKey, u128)>> = Mutex::new(Vec::new());

const MAX_LEVEL: tracing::Level = {
    cfg_if::cfg_if! {
        if #[cfg(not(debug_assertions))] {
            tracing::Level::INFO
        } else {
            tracing::Level::TRACE
        }
    }
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(MAX_LEVEL)
        .init();

    ctrlc::set_handler(|| {
        for key in KeybdKey::iter() {
            if key.release().join().is_err() {
                error!("Failed to release key: {:?}", key);
            };
        }

        println!("Goodbye...");
        std::process::exit(0);
    })
    .expect("failed to set Ctrl-C handler");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);

    KeybdKey::BackquoteKey.bind(move || {
        if KeybdKey::LControlKey.is_pressed() && KeybdKey::LAlt.is_pressed() {
            ENABLED.toggle();
            tx.blocking_send(()).expect("failed to send message");
        }
    });

    spawn(async move {
        loop {
            rx.recv().await.expect("channel closed");

            if ENABLED.get() {
                for key in KEYS {
                    key.blockable_bind(move || {
                        // The following block attempts to prevent the program from triggering the event again if the key was pressed by the program itself.
                        {
                            let mut pressed_keys = PRESSED_KEYS.lock();
                            let time = SystemTime::now()
                                .duration_since(SystemTime::UNIX_EPOCH)
                                .expect("time went backwards")
                                .as_millis();

                            if let Some(pos) = pressed_keys
                                .iter()
                                .position(|(k, t)| k == &key && (time - t) < 10)
                            {
                                pressed_keys.remove(pos);
                                return BlockInput::DontBlock;
                            } else {
                                pressed_keys.push((key, time));
                            }
                        }

                        let is_upper = rand::random::<bool>();

                        std::thread::spawn(move || {
                            if is_upper {
                                KeybdKey::LShiftKey.press();
                            }

                            key.press();
                            key.release();

                            debug!("Pressed key {:?}", key);

                            if is_upper {
                                KeybdKey::LShiftKey.release();
                            }
                        });

                        BlockInput::Block
                    })
                }
            } else {
                for key in KEYS {
                    key.unbind()
                }
            }
        }
    });

    spawn(async {
        loop {
            if let Some(pressed_keys) = PRESSED_KEYS.try_lock() {
                if !pressed_keys.is_empty() {
                    debug!("{:?}", pressed_keys);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Begin listening for input events
    // Blocks current execution until the program is closed so
    // open in a thread in order to keep execution flow
    spawn(async { inputbot::handle_input_events() })
        .await
        .expect("failed to join thread");
}
