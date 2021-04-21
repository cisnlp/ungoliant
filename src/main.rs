use download::Downloader;
use std::{fs::File, path::PathBuf};
use structopt::StructOpt;

extern crate fasttext;

#[macro_use]
extern crate log;

mod classify;
mod download;
mod warc;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ungoliant",
    about = "A mysterious project named after a spider that consumes everything 🕷️."
)]
struct UngoliantCli {
    #[structopt(help = "paths to download, ending in wet.paths.")]
    file: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let opt = UngoliantCli::from_args();
    debug!("cli args\n{:#?}", opt);

    let mut err_file = File::create("errors.txt").expect("failed to create error file");
    let mut log_file = File::create("log.txt").expect("failed to create log file");

    let warc_record = warc::Wet::from_path_gzip(opt.file)?;
    let mut classifier = classify::Classifier::new_lid().expect("oops");

    // FIX for robots: line
    let mut warc_record = warc_record.into_iter().skip(1);
    println!("{:?}", warc_record.next());

    for record in warc_record {
        let record = record.unwrap();
        // println!("{:?}", record);
        let predictions: Vec<(Result<Vec<fasttext::Prediction>, String>, &str)> = record
            .lines()
            .map(|line| (classifier.predict(line), line))
            .filter(|pair| !pair.0.as_ref().unwrap_or(&vec![]).is_empty())
            .collect();
        // println!("{:#?}", predictions);
    }
    // let d = Downloader::from_paths_file(&File::open(opt.file)?)?;

    // let results = d.download_all_blocking();

    // // print eventual errors
    // for error in results.iter().filter(|x| Result::is_err(x)) {
    //     eprintln!("{:?}", error);
    // }

    Ok(())
}
