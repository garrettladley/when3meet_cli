use url::Url;

use crate::fetch_availability::model::Slot;
use crate::fetch_availability::parse::parse_when2meet;

fn run(required_people: &[String], flexible_naming: &bool, when2meet_url: &Url) {
    let slots = match parse_when2meet(when2meet_url) {
        Ok(slots) => slots,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let opt = find_opt(&slots, required_people, flexible_naming);
}

fn find_opt(slots: &[Slot], required_people: &[String], flexible_naming: &bool) -> Vec<Slot> {
    slots
        .iter()
        .filter(|slot| {
            required_people.iter().all(|required_name| {
                slot.people.iter().any(|slot_person| {
                    let name_match = if *flexible_naming {
                        slot_person
                            .name
                            .to_lowercase()
                            .contains(&required_name.to_lowercase())
                    } else {
                        slot_person.name == *required_name
                    };
                    name_match && slot_person.available
                })
            })
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::fetch_availability::model::{Person, Slot};
    use crate::optimal::algo::find_opt;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_find_opt() {
        let slots = vec![
            Slot {
                timestamp: DateTime::parse_from_str("1693746000", "%s")
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
            Slot {
                timestamp: DateTime::parse_from_str("1693746900", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                people: vec![
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
            },
            Slot {
                timestamp: DateTime::parse_from_str("1693747800", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                people: vec![
                    Person {
                        name: "Muneer".to_string(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            },
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt.len(), 1);

        assert_eq!(opt[0].timestamp.timestamp(), 1693747800);

        assert_eq!(opt[0].people.len(), 3);

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| person.name == *"Muneer")
                .unwrap()
                .available
        );

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| person.name == *"Brian")
                .unwrap()
                .available
        );
    }
}
