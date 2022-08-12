use inputbot::{
    BlockInput, KeySequence,
    KeybdKey::{
        self, AKey, BKey, BackquoteKey, CKey, DKey, EKey, FKey, GKey, HKey, IKey, JKey, KKey,
        LControlKey, LKey, LShiftKey, MKey, NKey, OKey, PKey, QKey, RKey, SKey, TKey, UKey, VKey,
        WKey, XKey, YKey, ZKey,
    },
};
use parking_lot::{const_mutex, Mutex};
use std::thread;

mod enabled;

use enabled::ENABLED;

/**
 * Problem:
 *  Upon pressing a key, the event will fire again, triggering the same issue until the program is closed.
 *
 * Idea to prevent infinite loops:
 *  Create a vector of keys that have been pressed as well as their timestamp in ms and remove them upon consumption assuming that the timestamp matches within 10ms.
*/

const KEYS: [inputbot::KeybdKey; 26] = [
    AKey, BKey, CKey, DKey, EKey, FKey, GKey, HKey, IKey, JKey, KKey, LKey, MKey, NKey, OKey, PKey,
    QKey, RKey, SKey, TKey, UKey, VKey, WKey, XKey, YKey, ZKey,
];

static JUST_TOGGLED: Mutex<bool> = const_mutex(false);

fn main() {
    BackquoteKey.bind(move || {
        if LControlKey.is_pressed() && LShiftKey.is_pressed() {
            ENABLED.toggle();
            *JUST_TOGGLED.lock() = true;
        }
    });

    thread::spawn(move || loop {
        if *JUST_TOGGLED.lock() {
            *JUST_TOGGLED.lock() = false;
            if ENABLED.get() {
                for key in KEYS {
                    key.bind(move || {
                        let is_upper = rand::random::<bool>();

                        if is_upper {
                            LShiftKey.press();
                        }

                        key.press();
                        key.release();
                        println!("pressed key {:?}", key);

                        if is_upper {
                            LShiftKey.release();
                        }
                        // BlockInput::Block
                    })
                }
            } else {
                for key in KEYS {
                    key.unbind()
                }
            }
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}
