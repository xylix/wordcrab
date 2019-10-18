use clap::AppSettings;
use rayon::prelude::*;
use serde_json::json;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod wordcrab;
use wordcrab::{analyse_file, FileStatsOutput};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wordcrab",
    about = "A command-line tool for counting lines, words and characters in documents.",
    global_settings = &[AppSettings::ColoredHelp]
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Select the output format
    #[structopt(short, long, possible_values = &["text", "json"], default_value = "text")]
    output: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:#?}", opt);
    }

    // If output format is text, we can stream-print as we go.
    // If output format is specified to any other format, we'll collect values first
    // in order to output a correct file
    match opt.output.as_str() {
        "text" => opt.files.par_iter().for_each(|path| {
            let filename = path.to_str().unwrap();
            println!("{}", analyse_file(&filename));
        }),
        "json" => {
            let results: Vec<FileStatsOutput> = opt
                .files
                .par_iter()
                .map(|path| {
                    let filename = path.to_str().unwrap();
                    analyse_file(&filename)
                })
                .collect();
            println!("{}", json!(results))
        }
        _ => unreachable!(), // structopt has explicit list of possible_values and a default_value
    }
    Ok(())
}
