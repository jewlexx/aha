use std::time::SystemTime;

use parking_lot::Mutex;
use tokio::task::spawn;

use inputbot::KeybdKey;

mod enabled;

use enabled::ENABLED;

/**
 * Problem:
 *  Upon pressing a key, the event will fire again, triggering the same issue until the program is closed.
 *
 * Idea to prevent infinite loops:
 *  Create a vector of keys that have been pressed as well as their timestamp in ms and remove them upon consumption assuming that the timestamp matches within 10ms.
*/

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

#[tokio::main]
async fn main() {
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
                    key.bind(move || {
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
                            } else {
                                pressed_keys.push((key, time));
                            }
                        }

                        let is_upper = rand::random::<bool>();

                        if is_upper {
                            KeybdKey::LShiftKey.press();
                        }

                        key.press();
                        key.release();
                        println!("pressed key {:?}", key);

                        if is_upper {
                            KeybdKey::LShiftKey.release();
                        }
                    })
                }
            } else {
                for key in KEYS {
                    key.unbind()
                }
            }
        }
    });

    // Begin listening for input events
    // Blocks current execution until the program is closed so
    // open in a thread in order to keep execution flow
    spawn(async { inputbot::handle_input_events() })
        .await
        .expect("failed to join thread");
}
