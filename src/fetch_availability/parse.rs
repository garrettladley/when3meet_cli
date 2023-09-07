use crate::fetch_availability::errors::{
    FetchError, HeadlessChromeError, ParseError, ParseWhen2MeetError, ProcessResultError,
};
use crate::fetch_availability::model::{fold, Person, Slot};
use chrono::{DateTime, Utc};
use headless_chrome::{Browser, Tab};
use std::sync::Arc;
use url::Url;

pub fn parse_when2meet(url: &Url) -> Result<Vec<Slot>, ParseWhen2MeetError> {
    let browser = match Browser::default() {
        Ok(browser) => browser,
        Err(_) => {
            return Err(ParseWhen2MeetError::HeadlessChrome(
                HeadlessChromeError::FailedToLaunch,
            ))
        }
    };

    let tab = match browser.new_tab() {
        Ok(tab) => tab,
        Err(_) => {
            return Err(ParseWhen2MeetError::HeadlessChrome(
                HeadlessChromeError::FailedToNewTab,
            ))
        }
    };

    match tab.navigate_to(url.as_str()) {
        Ok(_) => {}
        Err(_) => {
            return Err(ParseWhen2MeetError::HeadlessChrome(
                HeadlessChromeError::FailedToNavigate,
            ))
        }
    }

    match tab.wait_until_navigated() {
        Ok(_) => {}
        Err(_) => {
            return Err(ParseWhen2MeetError::HeadlessChrome(
                HeadlessChromeError::FailedToWaitUntilNavigated,
            ))
        }
    }

    let raw_names = match fetch_people_names(&tab) {
        Ok(raw_names) => raw_names,
        Err(fetch_error) => return Err(ParseWhen2MeetError::Fetch(fetch_error)),
    };

    let names = match parse_people_names_from_result(raw_names) {
        Ok(names) => names,
        Err(parse_error) => return Err(ParseWhen2MeetError::Parse(parse_error)),
    };

    let raw_avail_matrix = match fetch_avail_matrix(&tab) {
        Ok(raw_avail_matrix) => raw_avail_matrix,
        Err(fetch_error) => return Err(ParseWhen2MeetError::Fetch(fetch_error)),
    };

    let avail_matrix = match parse_avail_matrix_from_result(raw_avail_matrix) {
        Ok(avail_matrix) => avail_matrix,
        Err(parse_error) => return Err(ParseWhen2MeetError::Parse(parse_error)),
    };

    let slots = match process_names_and_matrix(names, avail_matrix) {
        Ok(slots) => slots,
        Err(process_result_error) => {
            return Err(ParseWhen2MeetError::ProcessResult(process_result_error))
        }
    };

    Ok(slots)
}

fn process_names_and_matrix(
    names: Vec<String>,
    avail_matrix: Vec<String>,
) -> Result<Vec<Slot>, ProcessResultError> {
    let mut slots = Vec::new();

    for section in avail_matrix {
        let mut parts = section.split(',');

        let start_timestamp_str = match parts.next() {
            Some(timestamp) => timestamp,
            None => return Err(ProcessResultError::AvailMatrixNoNext { section }),
        };

        let start_timestamp = match DateTime::parse_from_str(start_timestamp_str, "%s") {
            Ok(timestamp) => timestamp.with_timezone(&Utc),
            Err(_) => {
                return Err(ProcessResultError::AvailMatrixFailedTimestampParse {
                    timestamp: start_timestamp_str.to_string(),
                })
            }
        };

        let people = names
            .iter()
            .zip(parts)
            .map(|(name, available)| {
                let available = available == "1";
                Person {
                    name: name.to_string(),
                    available,
                }
            })
            .collect();

        slots.push(Slot::new(start_timestamp, people));
    }

    Ok(fold(slots))
}

fn fetch_people_names(tab: &Arc<Tab>) -> Result<String, FetchError> {
    let js_func = r#"
    (function () {
        return PeopleNames.join(",")
    })();
    "#;

    let names = match tab.evaluate(js_func, false) {
        Ok(result) => result,
        Err(_) => return Err(FetchError::FailedEval),
    };

    let raw_names = match names.value {
        Some(value) => value.to_string(),
        None => return Err(FetchError::EvalNoValue),
    };

    Ok(raw_names)
}

fn parse_people_names_from_result(raw_names: String) -> Result<Vec<String>, ParseError> {
    if raw_names.len() <= 2 {
        return Err(ParseError::EmptyRaw);
    }

    let names = &raw_names[1..raw_names.len() - 1];

    Ok(names.split(',').map(|x| x.to_string()).collect())
}

fn fetch_avail_matrix(tab: &Arc<Tab>) -> Result<String, FetchError> {
    let js_func = r#"
    (function () {
        return AvailableAtSlot.map((slotData, i) => {
            return `${TimeOfSlot[i]},${PeopleIDs.map(id => slotData.includes(id) ? 1 : 0).join(",")}`;
        }).join("|");
    })();
    "#;

    let avail_matrix = match tab.evaluate(js_func, false) {
        Ok(result) => result,
        Err(_) => return Err(FetchError::FailedEval),
    };

    let raw_avail_matrix = match avail_matrix.value {
        Some(value) => value.to_string(),
        None => return Err(FetchError::EvalNoValue),
    };

    Ok(raw_avail_matrix)
}

fn parse_avail_matrix_from_result(raw_avail_matrix: String) -> Result<Vec<String>, ParseError> {
    if raw_avail_matrix.len() <= 2 {
        return Err(ParseError::EmptyRaw);
    }

    let avail_matrix = &raw_avail_matrix[1..raw_avail_matrix.len() - 1];

    Ok(avail_matrix.split('|').map(|x| x.to_string()).collect())
}

#[cfg(test)]
mod tests {
    use crate::fetch_availability::errors::{ParseError, ProcessResultError};
    use crate::fetch_availability::model::{Person, Slot};
    use crate::fetch_availability::parse::{
        parse_avail_matrix_from_result, parse_people_names_from_result, process_names_and_matrix,
    };
    use chrono::{DateTime, Utc};
    use claims::{assert_err, assert_ok};

    #[test]
    fn test_parse_avail_matrix_from_result_valid_str() {
        let raw_avail_matrix = "'1693746000,0,0,0|1693746900,1,0,0|1693747800,0,1,0'".to_string();

        let avail_matrix = parse_avail_matrix_from_result(raw_avail_matrix);

        assert_ok!(&avail_matrix);

        let avail_matrix = avail_matrix.unwrap();

        assert!(avail_matrix.len() == 3);
        assert!(avail_matrix[0] == "1693746000,0,0,0");
        assert!(avail_matrix[1] == "1693746900,1,0,0");
        assert!(avail_matrix[2] == "1693747800,0,1,0");
    }

    #[test]
    fn test_parse_avail_matrix_from_result_invalid_str() {
        let raw_avail_matrix = "''".to_string();

        let avail_matrix = parse_avail_matrix_from_result(raw_avail_matrix);

        assert_err!(&avail_matrix);

        let avail_matrix = avail_matrix.unwrap_err();

        assert!(matches!(avail_matrix, ParseError::EmptyRaw));
    }

    #[test]
    fn test_parse_people_names_from_result_valid_str() {
        let raw_names = "'Muneer,Brian,Garrett'".to_string();

        let names = parse_people_names_from_result(raw_names);

        assert_ok!(&names);

        let names = names.unwrap();

        assert!(names.len() == 3);
        assert!(names[0] == "Muneer");
        assert!(names[1] == "Brian");
        assert!(names[2] == "Garrett");
    }

    #[test]
    fn test_parse_people_names_from_result_invalid_str() {
        let raw_names = "''".to_string();

        let names = parse_people_names_from_result(raw_names);

        assert_err!(&names);

        let names = names.unwrap_err();

        assert!(matches!(names, ParseError::EmptyRaw));
    }

    #[test]
    fn test_process_names_and_matrix_valid() {
        let names = vec![
            "Muneer".to_string(),
            "Brian".to_string(),
            "Garrett".to_string(),
        ];

        let avail_matrix = vec![
            "1693746000,0,0,0".to_string(),
            "1693746900,1,0,0".to_string(),
            "1693747800,0,1,0".to_string(),
        ];

        let slots = process_names_and_matrix(names, avail_matrix);

        assert_ok!(&slots);

        let slots = slots.unwrap();

        assert!(slots.len() == 3);
        assert!(
            slots[0]
                == Slot::new(
                    DateTime::parse_from_str("1693746000", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    vec![
                        Person {
                            name: "Muneer".to_string(),
                            available: false
                        },
                        Person {
                            name: "Brian".to_string(),
                            available: false
                        },
                        Person {
                            name: "Garrett".to_string(),
                            available: false
                        }
                    ]
                )
        );
        assert!(
            slots[1]
                == Slot::new(
                    DateTime::parse_from_str("1693746900", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    vec![
                        Person {
                            name: "Muneer".to_string(),
                            available: true
                        },
                        Person {
                            name: "Brian".to_string(),
                            available: false
                        },
                        Person {
                            name: "Garrett".to_string(),
                            available: false
                        }
                    ]
                )
        );
        assert!(
            slots[2]
                == Slot::new(
                    DateTime::parse_from_str("1693747800", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    vec![
                        Person {
                            name: "Muneer".to_string(),
                            available: false
                        },
                        Person {
                            name: "Brian".to_string(),
                            available: true
                        },
                        Person {
                            name: "Garrett".to_string(),
                            available: false
                        }
                    ]
                )
        );
    }

    #[test]
    fn test_process_names_and_matrix_bad_timestamp() {
        let names = vec![
            "Muneer".to_string(),
            "Brian".to_string(),
            "Garrett".to_string(),
        ];

        let avail_matrix = vec!["".to_string()];

        let slots = process_names_and_matrix(names, avail_matrix);

        assert_err!(&slots);

        let slots = slots.unwrap_err();

        assert_eq!(
            slots,
            ProcessResultError::AvailMatrixFailedTimestampParse {
                timestamp: "".to_string(),
            }
        );
    }
}
