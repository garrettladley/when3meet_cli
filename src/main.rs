use std::fs::File;
use std::io::Write;

use clap::Parser;
use when3meet::cli::args::Args;
use when3meet::fetch_availability::parse_when2meet;
use when3meet::optimal::find_opt;

fn main() {
    let args = Args::parse();

    let slots = match parse_when2meet(&args.when2meet_url) {
        Ok(slots) => slots,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let slots = find_opt(
        &slots,
        &args.required_people.unwrap_or(vec![]),
        &args.flexible_naming.unwrap_or(false),
    );

    match args.output_file_path {
        Some(path) => {
            let mut file = match File::create(path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };
            for (index, slot) in slots.iter().enumerate() {
                if index < slots.len() - 1 {
                    match file.write_all(format!("{}\n", slot).as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    };
                } else {
                    match file.write_all(format!("{}", slot).as_bytes()) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        }
                    };
                }
            }
        }
        None => {
            for (index, slot) in slots.iter().enumerate() {
                if index < slots.len() - 1 {
                    println!("{}", slot);
                } else {
                    print!("{}", slot);
                }
            }
        }
    }
}
