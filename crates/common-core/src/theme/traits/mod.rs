use super::ThemeTypes;

pub trait Themeable {
    fn change_theme(&mut self, next: ThemeTypes);
}
