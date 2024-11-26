use crate::screens::Screen;

pub enum Event {
    DoNothing,
    Quit,
    Switch(Screen),
}
