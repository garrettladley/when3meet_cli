use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io;
use when3meet::cli::args::Args;
use when3meet::fetch_availability::parse_when2meet;
use when3meet::optimal::find_opt;
use when3meet::output::write_slots;

fn main() -> Result<()> {
    let args = Args::parse();
    let slots = parse_when2meet(&args.when2meet_url)?;
    let slots = find_opt(&slots, &args.required_people, &args.flexible_naming);

    match args.output_file_path {
        Some(path) => {
            let mut file = File::create(path).context("Failed to create output file")?;
            write_slots(&slots, &mut file)?;
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            write_slots(&slots, &mut handle)?;
        }
    }

    Ok(())
}
