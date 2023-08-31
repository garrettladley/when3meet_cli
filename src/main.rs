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

    let names = r#"
    (function () {
        return PeopleNames.join(",")
    })();
    "#;

    let names = tab.evaluate(names, false)?;

    let names = names.value.unwrap().to_string();

    let names = &names[1..names.len() - 1];

    let names: Vec<&str> = names.split(',').collect();

    let avail_matrix = r#"
    (function () {
        return AvailableAtSlot.map((slotData, i) => {
            return `${TimeOfSlot[i]},${PeopleIDs.map(id => slotData.includes(id) ? 1 : 0).join(",")}`;
        }).join("|");
    })();
    "#;

    let avail_matrix = tab.evaluate(avail_matrix, false)?;

    let avail_matrix = avail_matrix.value.unwrap().to_string();

    let avail_matrix = &avail_matrix[1..avail_matrix.len() - 1];

    let sections: Vec<&str> = avail_matrix.split('|').collect();

    let mut slots = Vec::new();

    for section in sections {
        let mut parts = section.split(',');

        let timestamp_str = parts.next().unwrap();
        let timestamp = DateTime::parse_from_str(timestamp_str, "%s")
            .unwrap()
            .with_timezone(&Utc);

        let mut people = Vec::new();
        for (name, available) in names.iter().zip(parts) {
            let available = available == "1";

            people.push(Person {
                name: name.to_string(),
                available,
            });
        }

        slots.push(Slot { timestamp, people });
    }

    println!("{:#?}", slots);

    Ok(())
}
