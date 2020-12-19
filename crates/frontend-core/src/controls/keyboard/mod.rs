use common_core::prelude::*;

pub enum KeyboardKeys {
    ToggleTheme,
    Cancel,
    Select,
}

impl IdProperty<KeyboardKeys> for KeyboardKeys {
    fn as_id(&self) -> &'static str {
        match self {
            KeyboardKeys::ToggleTheme => "T",
            KeyboardKeys::Cancel => "ECS",
            KeyboardKeys::Select => "S",
        }
    }
}
