#[derive(Clone, Debug, Copy)]
pub struct Seconds(pub u64);

impl Seconds {
    pub fn from_minutes(minutes: u64) -> Self {
        Self(minutes * 60)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum SessionKind {
    Work,
    ShortBreak,
    LongBreak,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub work: Seconds,
    pub short_break: Seconds,
    pub long_break: Seconds,
    pub long_break_interval: u8,
    pub auto_start_next: bool,
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            work: Seconds::from_minutes(25),
            short_break: Seconds::from_minutes(5),
            long_break: Seconds::from_minutes(20),
            long_break_interval: 4,
            auto_start_next: false,
            path: "$HOME/.config/pomo".to_string(),
        }
    }
}
