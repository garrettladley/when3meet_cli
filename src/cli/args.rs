use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The people required at the meeting. If not provided, assumed to be all people.
    #[arg(short, long, value_parser, num_args = 1..)]
    pub required_people: Vec<String>,

    /// Perform case insensitive contains based matching on required people.
    #[arg(short, long, requires("required_people"))]
    pub flexible_naming: bool,

    /// The URL to the when2meet page.
    #[arg(short, long)]
    pub when2meet_url: Url,

    /// The output file path. If not provided, it will be printed to stdout.
    #[arg(short, long)]
    pub output_file_path: Option<std::path::PathBuf>,
}
