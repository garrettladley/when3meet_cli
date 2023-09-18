use crate::fetch_availability::model::Slot;

pub fn find_opt<'a>(
    slots: &'a [Slot],
    required_people: &[String],
    flexible_naming: &bool,
) -> Vec<&'a Slot> {
    if !required_people.is_empty() {
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
                            &*slot_person.name == required_name
                        };
                        name_match && slot_person.available
                    })
                })
            })
            .collect()
    } else {
        let max_available_count = slots
            .iter()
            .map(|slot| slot.people.iter().filter(|person| person.available).count())
            .max()
            .unwrap_or(0);

        if max_available_count == 0 {
            return vec![];
        }

        slots
            .iter()
            .filter(|slot| {
                slot.people.iter().filter(|person| person.available).count() == max_available_count
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: false,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt.len(), 1);

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| &*person.name == "Muneer")
                .unwrap()
                .available
        );

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| &*person.name == "Brian")
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: true,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = false;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        let slots: Vec<&Slot> = slots.iter().collect();

        let valid_slots = vec![slots[0], slots[2]];

        assert_eq!(opt, valid_slots);
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
    fn test_find_opt_no_required_people_1_opt() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: false,
                    },
                ],
            ),
            Slot::new(
                DateTime::parse_from_str("1693748700", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: true,
                    },
                ],
            ),
        ];

        let flexible_naming = false;

        let opt = find_opt(&slots, &[], &flexible_naming);

        print!("{:?}", opt);

        assert_eq!(opt.len(), 1);

        let max_available_count = slots
            .iter()
            .map(|slot| slot.people.iter().filter(|person| person.available).count())
            .max()
            .unwrap_or(0);

        assert_eq!(max_available_count, 3);

        let actual_count_available = opt[0]
            .people
            .iter()
            .filter(|person| person.available)
            .count();

        assert_eq!(actual_count_available, 3);
    }

    #[test]
    fn test_find_opt_no_required_people_multiple_opt() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693743800", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: true,
                    },
                ],
            ),
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: false,
                    },
                ],
            ),
            Slot::new(
                DateTime::parse_from_str("1693748700", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
                vec![
                    Person {
                        name: "Muneer".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: true,
                    },
                ],
            ),
        ];

        let flexible_naming = false;

        let opt = find_opt(&slots, &[], &flexible_naming);

        print!("{:?}", opt);

        assert_eq!(opt.len(), 2);

        let max_available_count = slots
            .iter()
            .map(|slot| slot.people.iter().filter(|person| person.available).count())
            .max()
            .unwrap_or(0);

        assert_eq!(max_available_count, 3);

        assert!(opt.iter().all(|opt_slot| {
            opt_slot
                .people
                .iter()
                .filter(|person| person.available)
                .count()
                == max_available_count
        }));
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
                        name: " mUnEeR lAlJi ".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: " reicher, brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: " mUnEeR lAlJi ".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: " reicher, brian".to_string().into_boxed_str(),
                        available: false,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
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
                        name: " mUnEeR lAlJi ".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: " reicher, brian".to_string().into_boxed_str(),
                        available: true,
                    },
                    Person {
                        name: "Garrett".to_string().into_boxed_str(),
                        available: false,
                    },
                ],
            ),
        ];

        let required_people = vec!["Muneer".to_string(), "Brian".to_string()];

        let flexible_naming = true;

        let opt = find_opt(&slots, &required_people, &flexible_naming);

        assert_eq!(opt.len(), 1);

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| &*person.name == " mUnEeR lAlJi ")
                .unwrap()
                .available
        );

        assert!(
            opt[0]
                .people
                .iter()
                .find(|person| &*person.name == " reicher, brian")
                .unwrap()
                .available
        );
    }
}
