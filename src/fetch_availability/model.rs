use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub timestamp: DateTime<Utc>,
    pub people: Vec<Person>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub available: bool,
}
