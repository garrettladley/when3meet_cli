use std::error::Error;

use clap::Parser;
use headless_chrome::Browser;

use chrono::{DateTime, Utc};
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The people required at the meeting. If not provided, assumed to be all people.
    #[arg(short, long, required = false)]
    required_people: Option<Vec<String>>,

    /// Flexible naming. Perform case insensitive contains based matching.
    #[arg(short, long, required = false)]
    flexible_naming: Option<bool>,

    /// The URL to the when2meet page.
    #[arg(short, long)]
    when2meet_url: Url,

    /// The output file path. If not provided, it will be printed to stdout.
    #[arg(short, long, required = false)]
    output_file_path: Option<String>,
}

#[derive(Debug)]
struct Slot {
    timestamp: DateTime<Utc>,
    people: Vec<Person>,
}

#[derive(Debug)]
struct Person {
    name: String,
    available: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    tab.navigate_to("")?;

    tab.wait_until_navigated()?;

    let js_code = r#"
    (function () {
        return PeopleNames.join(",") + "|" + AvailableAtSlot.map((slotData, i) => {
            return `${TimeOfSlot[i]},${PeopleIDs.map(id => slotData.includes(id) ? 1 : 0).join(",")}`;
        }).join("|");
    })();
    "#;

    let evaluate_response = tab.evaluate(js_code, false)?;

    let evaluate_response = evaluate_response.value.unwrap().to_string();

    let evaluate_response = &evaluate_response[1..evaluate_response.len() - 1];

    print!("{}", evaluate_response);

    let mut parts = evaluate_response.split('|');
    let names = parts.next().unwrap();
    let slots_data = parts.next().unwrap();

    let names = names.split(',').collect::<Vec<&str>>();
    let slots = slots_data
        .split('|')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut values = s.split(',');

            let timestamp_str = values.next().unwrap();

            let format = "%s";
            let datetime = DateTime::parse_from_str(timestamp_str, format).unwrap();

            let timestamp = datetime.with_timezone(&Utc);

            let people = values
                .zip(names.iter())
                .map(|(val, name)| Person {
                    name: name.to_string(),
                    available: match val {
                        "1" => true,
                        "0" => false,
                        _ => unreachable!("impossible based on JS"),
                    },
                })
                .collect();

            Slot { timestamp, people }
        })
        .collect::<Vec<Slot>>();

    println!("{:#?}", slots);

    Ok(())
}
