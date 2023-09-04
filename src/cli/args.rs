use clap::Parser;
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
