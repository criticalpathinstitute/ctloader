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
    source: String,
    study_type: String,
    phase: Option<String>,
    study_design_info: Option<StudyDesignInfo>,
    detailed_description: Option<Textblock>,
    has_expanded_access: Option<String>,
    overall_status: Option<String>,
    last_known_status: Option<String>,
    rank: Option<String>,
    why_stopped: Option<String>,
    target_duration: Option<String>,
    acronym: Option<String>,
    number_of_arms: Option<i32>,
    number_of_groups: Option<i32>,
    condition: Option<Vec<String>>,
    keyword: Option<Vec<String>>,
    id_info: IdInfo,
    brief_title: String,
    official_title: Option<String>,
    sponsors: Sponsors,
    oversight_info: OversightInfo,
    expanded_access_info: Option<ExpandedAccessInfo>,
    start_date: Option<String>,
    completion_date: Option<String>,
    primary_completion_date: Option<String>,
    verification_date: Option<String>,
    study_first_submitted: Option<String>,
    study_first_submitted_qc: Option<String>,
    study_first_posted: Option<String>,
    results_first_submitted: Option<String>,
    results_first_submitted_qc: Option<String>,
    results_first_posted: Option<String>,
    disposition_first_submitted: Option<String>,
    disposition_first_submitted_qc: Option<String>,
    disposition_first_posted: Option<String>,
    last_update_submitted: Option<String>,
    last_update_submitted_qc: Option<String>,
    last_update_posted: Option<String>,
    primary_outcome: Option<Vec<ProtocolOutcome>>,
    secondary_outcome: Option<Vec<ProtocolOutcome>>,
    other_outcome: Option<Vec<ProtocolOutcome>>,
    enrollment: Option<i32>,
    arm_group: Option<Vec<ArmGroup>>,
    intervention: Option<Vec<Intervention>>,
    biospec_retention: Option<String>,
    biospec_descr: Option<Textblock>,
    eligibility: Option<Eligibility>,
    overall_official: Option<Vec<Investigator>>,
    overall_contact: Option<Contact>,
    overall_contact_backup: Option<Contact>,
    location: Option<Vec<Location>>,
    location_countries: Option<Vec<Country>>,
    removed_countries: Option<Vec<Country>>,
    link: Option<Vec<Link>>,
    reference: Option<Vec<Reference>>,
    results_reference: Option<Vec<Reference>>,
    responsible_party: Option<ResponsibleParty>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Address {
    city: String,
    state: Option<String>,
    zip: Option<String>,
    country: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ArmGroup {
    arm_group_label: String,
    arm_group_type: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Contact {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    degrees: Option<String>,
    phone: Option<String>,
    phone_ext: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Country {
    country: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Eligibility {
    study_pop: Option<String>,
    sampling_method: Option<String>,
    criteria: Option<Textblock>,
    gender: String,
    gender_based: Option<String>,
    gender_description: Option<String>,
    minimum_age: String,
    maximum_age: String,
    healthy_volunteers: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ExpandedAccessInfo {
    expanded_access_type_individual: Option<String>,
    expanded_access_type_intermediate: Option<Vec<String>>,
    expanded_access_type_treatment: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct IdInfo {
    nct_id: String,
    org_study_id: Option<String>,
    secondary_id: Option<Vec<String>>,
    nct_alias: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Intervention {
    intervention_type: String,
    intervention_name: String,
    description: Option<String>,
    arm_group_label: Option<Vec<String>>,
    other_name: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Investigator {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: String,
    degrees: Option<String>,
    role: Option<String>,
    affiliation: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Facility {
    name: Option<String>,
    address: Option<Address>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Link {
    url: Option<String>,
    description: Option<Address>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Location {
    facility: Option<Facility>,
    status: Option<String>,
    contact: Option<Contact>,
    contact_backup: Option<Contact>,
    investigator: Option<Investigator>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ProtocolOutcome {
    measure: String,
    time_frame: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct OversightInfo {
    has_dmc: Option<String>,
    is_fda_regulated_drug: Option<String>,
    is_fda_regulated_device: Option<String>,
    is_unapproved_device: Option<String>,
    is_ppsd: Option<String>,
    is_us_export: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Reference {
    citation: Option<String>,
    #[serde(rename(deserialize = "PMID"))]
    pmid: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct RequiredHeader {
    download_date: String,
    link_text: String,
    url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ResponsibleParty {
    name_title: Option<String>,
    organization: Option<String>,
    responsible_party_type: Option<String>,
    investigator_affiliation: Option<String>,
    investigator_full_name: Option<String>,
    investigator_title: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StudyDesignInfo {
    allocation: Option<String>,
    intervention_model: Option<String>,
    intervention_model_description: Option<String>,
    primary_purpose: Option<String>,
    observational_model: Option<String>,
    time_perspective: Option<String>,
    masking: Option<String>,
    masking_description: Option<String>,
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

#[derive(Debug, Deserialize, PartialEq)]
struct Textblock {
    textblock: String,
}

#[derive(Debug, Deserialize, PartialEq)]
enum YesNo {
    #[serde(rename(deserialize = "Yes"))]
    Yes,
    #[serde(rename(deserialize = "No"))]
    No,
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
        println!("{:#?}", study);
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
