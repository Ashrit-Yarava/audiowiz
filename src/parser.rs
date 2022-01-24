use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory for mp3 files
    #[clap(short, long)]
    input: String,

    /// Output directory for mp3 files
    #[clap(short, long)]
    output: String,
}

pub fn parse()  -> Vec<String> {
    let args = Args::parse();
    return vec![args.input, args.output];
}
