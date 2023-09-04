use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub struct Slot {
    pub timestamp: DateTime<Utc>,
    pub people: Vec<Person>,
}

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub available: bool,
}
