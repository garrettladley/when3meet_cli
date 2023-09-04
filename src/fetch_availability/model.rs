use chrono::{DateTime, Utc};

pub struct Slot {
    pub timestamp: DateTime<Utc>,
    pub people: Vec<Person>,
}

pub struct Person {
    pub name: String,
    pub available: bool,
}
