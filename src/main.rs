use std::thread;

use inputbot::KeybdKey;
use parking_lot::Mutex;

mod enabled;

use enabled::ENABLED;

/**
 * Problem:
 *  Upon pressing a key, the event will fire again, triggering the same issue until the program is closed.
 *
 * Idea to prevent infinite loops:
 *  Create a vector of keys that have been pressed as well as their timestamp in ms and remove them upon consumption assuming that the timestamp matches within 10ms.
*/

static JUST_TOGGLED: Mutex<bool> = Mutex::new(false);

fn main() {
    // BackquoteKey.bind(move || {
    //     if LControlKey.is_pressed() && LShiftKey.is_pressed() {
    //         ENABLED.toggle();
    //         *JUST_TOGGLED.lock() = true;
    //     }
    // });

    // thread::spawn(move || loop {
    //     if *JUST_TOGGLED.lock() {
    //         *JUST_TOGGLED.lock() = false;
    //         if ENABLED.get() {
    //             for key in KEYS {
    //                 key.bind(move || {
    //                     let is_upper = rand::random::<bool>();

    //                     if is_upper {
    //                         LShiftKey.press();
    //                     }

    //                     key.press();
    //                     key.release();
    //                     println!("pressed key {:?}", key);

    //                     if is_upper {
    //                         LShiftKey.release();
    //                     }
    //                     // BlockInput::Block
    //                 })
    //             }
    //         } else {
    //             for key in KEYS {
    //                 key.unbind()
    //             }
    //         }
    //     }
    // });

    // Begin listening for input events
    // Blocks current execution until the program is closed so
    // open in a thread in order to keep execution flow
    thread::spawn(inputbot::handle_input_events);
}
