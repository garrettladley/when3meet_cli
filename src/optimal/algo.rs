use crate::fetch_availability::fold;
use crate::fetch_availability::model::Slot;

pub fn find_opt(slots: &[Slot], required_people: &[String], flexible_naming: &bool) -> Vec<Slot> {
    if !required_people.is_empty() {
        fold(
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
                .collect(),
        )
    } else {
        let max_available_count = slots
            .iter()
            .map(|slot| slot.people.iter().filter(|person| person.available).count())
            .max()
            .unwrap_or(0);

        slots
            .iter()
            .filter(|slot| {
                slot.people.iter().filter(|person| person.available).count() == max_available_count
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::fetch_availability::fold;
    use crate::fetch_availability::model::{Person, Slot};
    use crate::optimal::algo::find_opt;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_find_opt() {
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
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt.len(), 1);

        assert_eq!(opt[0].start_time.timestamp(), 1693747800);

        assert_eq!(opt[0].end_time.timestamp(), 1693748700);

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

    #[test]
    fn test_find_opt_no_opt() {
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
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert!(opt.is_empty());
    }

    #[test]
    fn test_find_opt_multiple_opts() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
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
                        available: true,
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
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: true,
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
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: true,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt, fold(slots));
    }

    #[test]
    fn test_find_opt_not_all_required_people_available() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
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
            ),
            Slot::new(
                DateTime::parse_from_str("1693746900", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
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
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
        ];

        let required_people = vec![
            "Muneer".to_string(),
            "Brian".to_string(),
            "Garrett".to_string(),
        ];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert!(opt.is_empty());
    }

    #[test]
    fn test_find_opt_flexible_naming() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: " mUnEeR lAlJi ".to_string(),
                        available: false,
                    },
                    Person {
                        name: " reicher, brian".to_string(),
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
                        name: " mUnEeR lAlJi ".to_string(),
                        available: true,
                    },
                    Person {
                        name: " reicher, brian".to_string(),
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
                        name: " mUnEeR lAlJi ".to_string(),
                        available: true,
                    },
                    Person {
                        name: " reicher, brian".to_string(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string(),
                        available: false,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = true;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt.len(), 1);

        assert_eq!(opt[0].start_time.timestamp(), 1693747800);

        assert_eq!(opt[0].end_time.timestamp(), 1693748700);

        assert_eq!(opt[0].people.len(), 3);

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| person.name == *" mUnEeR lAlJi ")
                .unwrap()
                .available
        );

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| person.name == *" reicher, brian")
                .unwrap()
                .available
        );
    }
}
