#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SessionKind {
    Work,
    ShortBreak,
    LongBreak,
}
