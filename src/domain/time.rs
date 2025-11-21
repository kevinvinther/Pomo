use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, Deserialize, Serialize)]
pub struct Seconds(pub u64);

impl Seconds {
    pub fn from_minutes<T>(minutes: T) -> Self
    where
        T: TryInto<u64>,
        T::Error: std::fmt::Debug,
    {
        Self(minutes.try_into().unwrap() * 60)
    }

    pub fn to_minutes(self) -> u64 {
        self.0 / 60
    }
}
