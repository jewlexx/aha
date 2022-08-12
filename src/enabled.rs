use parking_lot::Mutex;
use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum HotkeysError {
    #[error("Failed to toggle enabled state")]
    ToggleError,
}

pub struct HotkeysEnabled {
    enabled: Mutex<bool>,
}

impl HotkeysEnabled {
    pub const fn new() -> Self {
        Self {
            enabled: Mutex::new(false),
        }
    }

    pub fn toggle(&self) {
        let mut value = self.enabled.lock();

        *value = !*value;
    }

    pub fn try_toggle(&self) -> Result<(), HotkeysError> {
        let mut value = match self.enabled.try_lock() {
            Some(v) => v,
            None => return Err(HotkeysError::ToggleError),
        };

        *value = !*value;

        Ok(())
    }

    pub fn get(&self) -> bool {
        *self.enabled.lock()
    }

    pub fn try_get(&self) -> Result<bool, HotkeysError> {
        match self.enabled.try_lock() {
            Some(v) => Ok(*v),
            None => Err(HotkeysError::ToggleError),
        }
    }
}

pub static ENABLED: HotkeysEnabled = HotkeysEnabled::new();
