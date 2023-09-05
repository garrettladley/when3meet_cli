use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The people required at the meeting. If not provided, assumed to be all people.
    #[arg(short, long, required = false, value_parser, num_args = 1..)]
    pub required_people: Option<Vec<String>>,

    /// Flexible naming. Perform case insensitive contains based matching.
    #[arg(short, long, required = false)]
    pub flexible_naming: Option<bool>,

    /// The URL to the when2meet page.
    #[arg(short, long)]
    pub when2meet_url: Url,

    /// The output file path. If not provided, it will be printed to stdout.
    #[arg(short, long, required = false)]
    pub output_file_path: Option<String>,
}
