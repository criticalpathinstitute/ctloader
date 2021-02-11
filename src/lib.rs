extern crate clap;
extern crate serde;

//use quick_xml::events::Event;
//use quick_xml::Reader;

use clap::{App, Arg};
use quick_xml::de::from_reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ClinicalStudy {
    required_header: RequiredHeader,
    id_info: IdInfo,
    acronym: Option<String>,
    brief_title: String,
    official_title: Option<String>,
    sponsors: Sponsors,
}

#[derive(Debug, Deserialize, PartialEq)]
struct IdInfo {
    nct_id: String,
    org_study_id: Option<String>,
    secondary_id: Option<Vec<String>>,
    nct_alias: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct RequiredHeader {
    download_date: String,
    link_text: String,
    url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Sponsors {
    lead_sponsor: Sponsor,
    collaborator: Option<Vec<Sponsor>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Sponsor {
    agency: String,
    agency_class: Option<String>,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = App::new("ctloader")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@c-path.org>")
        .about("Load Clinical Trials XML")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("File input")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    let files = matches.values_of_lossy("file").unwrap();

    Ok(Config { files: files })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    for (fnum, filename) in config.files.into_iter().enumerate() {
        println!("{}: {}", fnum + 1, &filename);
        let study = parse_file(&filename)?;
        println!("{}: {}", study.id_info.nct_id, study.brief_title);
        println!("{:?}", study.sponsors);
    }

    Ok(())
}

// --------------------------------------------------
fn parse_file(filename: &str) -> MyResult<ClinicalStudy> {
    let path = Path::new(&filename);
    if path.is_file() {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        match from_reader(&mut reader) {
            Ok(study) => Ok(study),
            Err(err) => Err(From::from(format!("Failed to parse: {:?}", err))),
        }
    } else {
        Err(From::from(format!("'{}' not a valid file", filename)))
    }
}
