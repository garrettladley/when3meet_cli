use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use iana_time_zone::get_timezone;

#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub people: Vec<Person>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub available: bool,
}

impl Slot {
    pub fn new(start_time: DateTime<Utc>, people: Vec<Person>) -> Self {
        Self {
            start_time,
            end_time: start_time + Duration::minutes(15),
            people,
        }
    }
}

pub fn fold(slots: Vec<Slot>) -> Vec<Slot> {
    slots
        .into_iter()
        .fold(Vec::new(), |mut folded_slots, slot| {
            if let Some(last_slot) = folded_slots.last_mut() {
                if last_slot.end_time == slot.start_time && last_slot.people == slot.people {
                    last_slot.end_time = slot.end_time;
                    return folded_slots;
                }
            }
            folded_slots.push(slot);
            folded_slots
        })
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tz_str = match get_timezone() {
            Ok(tz) => tz,
            Err(_) => return Err(std::fmt::Error),
        };

        let tz: Tz = match tz_str.parse() {
            Ok(tz) => tz,
            Err(_) => return Err(std::fmt::Error),
        };

        let offset = tz.offset_from_utc_date(&Utc::now().date_naive());
        let abbreviation = offset.abbreviation();

        let formatted_timestamp = format!(
            "{} - {} {}",
            self.start_time.format("%A %I:%M%P"),
            self.end_time.format("%I:%M%P"),
            abbreviation
        );

        writeln!(f, "Timestamp: {}", formatted_timestamp)?;

        let (available_people, unavailable_people): (Vec<&Person>, Vec<&Person>) =
            self.people.iter().partition(|&person| person.available);

        if !available_people.is_empty() {
            writeln!(f, "Available People:")?;

            for person in available_people {
                writeln!(f, "- {}", person.name)?;
            }
        }

        if !unavailable_people.is_empty() {
            writeln!(f, "Unavailable People:")?;

            for person in unavailable_people {
                writeln!(f, "- {}", person.name)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fetch_availability::fold;
    use crate::fetch_availability::model::{Person, Slot};
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use chrono_tz::{OffsetName, Tz};
    use iana_time_zone::get_timezone;

    #[test]
    fn test_fold() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
            Slot::new(
                DateTime::parse_from_str("1693746900", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
            Slot::new(
                DateTime::parse_from_str("1693747800", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
        ];

        let folded_slots = fold(slots);

        assert_eq!(
            folded_slots,
            vec![
                Slot {
                    start_time: DateTime::parse_from_str("1693746000", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    end_time: DateTime::parse_from_str("1693747800", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    people: vec![
                        Person {
                            name: "Muneer".to_string(),
                            available: false,
                        },
                        Person {
                            name: "Brian".to_string(),
                            available: false,
                        },
                        Person {
                            name: "Garrett".to_string(),
                            available: false,
                        },
                    ],
                },
                Slot::new(
                    DateTime::parse_from_str("1693747800", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    vec![
                        Person {
                            name: "Muneer".to_string(),
                            available: true,
                        },
                        Person {
                            name: "Brian".to_string(),
                            available: false,
                        },
                        Person {
                            name: "Garrett".to_string(),
                            available: false,
                        },
                    ],
                ),
            ]
        );
    }

    #[test]
    fn test_fold_empty() {
        assert_eq!(fold(vec![]), vec![]);
    }

    #[test]
    fn test_slot_display() {
        let slot = Slot {
            start_time: Utc::now(),
            end_time: Utc::now() + Duration::hours(2),
            people: vec![
                Person {
                    name: String::from("Muneer"),
                    available: true,
                },
                Person {
                    name: String::from("Garrett"),
                    available: false,
                },
            ],
        };

        let tz_str = get_timezone().unwrap();

        let tz: Tz = tz_str.parse().unwrap();

        let offset = tz.offset_from_utc_date(&Utc::now().date_naive());
        let abbreviation = offset.abbreviation();

        let expected_output = format!(
            "Timestamp: {} - {} {}\nAvailable People:\n- Muneer\nUnavailable People:\n- Garrett\n",
            slot.start_time.format("%A %I:%M%P"),
            slot.end_time.format("%I:%M%P"),
            abbreviation
        );

        let actual_output = format!("{}", slot);

        assert_eq!(expected_output, actual_output);
    }
}
