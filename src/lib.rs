extern crate chrono;
extern crate clap;
extern crate dotenv;
extern crate dtparse;
extern crate postgres;
extern crate quick_xml;
extern crate regex;
extern crate serde;
extern crate spectral;

#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

//use crate::schema::dataload;
use crate::schema::{
    condition, intervention, phase, sponsor, status, study_design, study_doc, study_eligibility,
    study_location, study_outcome, study_to_condition, study_to_intervention, study_to_sponsor,
    study_type, study_url,
};
use chrono::{NaiveDate, Utc};
use clap::{App, Arg};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::*;
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet, env, error::Error, fs::File, io::BufReader, path::Path};
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;
type DbResult<T> = Result<T, diesel::result::Error>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    force: bool,
    resume: Option<usize>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Analysis {
    //group_id_list: Option<Vec<GroupId>>,
    groups_desc: Option<String>,
    non_inferiority_type: Option<String>,
    non_inferiority_desc: Option<String>,
    p_value: Option<String>,
    p_value_desc: Option<String>,
    method: Option<String>,
    method_desc: Option<String>,
    param_type: Option<String>,
    param_value: Option<String>,
    dispersion_type: Option<String>,
    dispersion_value: Option<String>,
    ci_percent: Option<f64>,
    ci_n_sides: Option<String>,
    ci_lower_limit: Option<String>,
    ci_upper_limit: Option<String>,
    ci_upper_limit_na_comment: Option<String>,
    estimate_desc: Option<String>,
    other_analysis_desc: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AnalysisList {
    analysis: Vec<Analysis>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AnalyzeList {
    analyzed: Vec<MeasureAnalyzed>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Baseline {
    population: Option<String>,
    group_list: Option<GroupList>,
    analyzed_list: Option<AnalyzeList>,
    measure_list: Option<MeasureList>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClinicalStudy {
    required_header: RequiredHeader,
    source: String,
    study_type: String,
    phase: Option<String>,
    study_design_info: Option<StudyDesignInfo>,
    brief_summary: Option<Textblock>,
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
    sponsors: Option<Sponsors>,
    oversight_info: Option<OversightInfo>,
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
    condition_browse: Option<Browse>,
    intervention_browse: Option<Browse>,
    patient_data: Option<PatientData>,
    study_docs: Option<StudyDocs>,
    provided_document_section: Option<ProvidedDocuments>,
    clinical_results: Option<ClinicalResults>,
    // pending_results: Option<Vec<PendingResult>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Address {
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Browse {
    mesh_term: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ArmGroup {
    arm_group_label: String,
    arm_group_type: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CertainAgreements {
    pi_employee: Option<String>,
    restrictive_agreement: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ClinicalResults {
    participant_flow: Option<ParticipantFlow>,
    baseline: Option<Baseline>,
    outcome_list: Option<OutcomeList>,
    reported_events: Option<ReportedEvents>,
    certain_agreements: Option<CertainAgreements>,
    limitations_and_caveats: Option<String>,
    point_of_contact: Option<PointOfContact>,
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
struct CountList {
    count: Option<Vec<MeasureCount>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DropWithdrawReason {
    drop_withdraw_reason: Vec<Milestone>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Eligibility {
    study_pop: Option<Textblock>,
    sampling_method: Option<String>,
    criteria: Option<Textblock>,
    gender: Option<String>,
    gender_based: Option<String>,
    gender_description: Option<String>,
    minimum_age: Option<String>,
    maximum_age: Option<String>,
    healthy_volunteers: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Events {
    frequency_threshold: Option<String>,
    default_vocab: Option<String>,
    default_assessment: Option<String>,
    category_list: Option<EventCategoryList>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EventCategory {
    title: String,
    event_list: Option<EventList>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EventCategoryList {
    category: Vec<EventCategory>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Event {
    // sub_title: Option<VocabTerm>, <-- TODO fix this?
    sub_title: Option<String>,
    assessment: Option<String>,
    description: Option<String>,
    counts: Option<Vec<EventCounts>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EventCounts {
    group_id: String,
    subjects_affected: Option<u32>,
    subjects_at_risk: Option<u32>,
    events: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct EventList {
    event: Vec<Event>,
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
struct Group {
    group_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct GroupId {
    group_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct GroupList {
    group: Vec<Group>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Link {
    url: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Location {
    facility: Option<Facility>,
    status: Option<String>,
    contact: Option<Contact>,
    contact_backup: Option<Contact>,
    investigator: Option<Vec<Investigator>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Measure {
    title: String,
    description: Option<String>,
    population: Option<String>,
    units: Option<String>,
    param: Option<String>,
    dispersion: Option<String>,
    units_analyzed: Option<String>,
    analyzed_list: Option<AnalyzeList>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MeasureAnalyzed {
    units: String,
    scope: String,
    count_list: Option<CountList>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MeasureCount {
    group_id: String,
    value: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MeasureList {
    measure: Vec<Measure>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Milestone {
    title: Option<String>,
    participants: Option<Vec<Participant>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MilestoneList {
    milestone: Vec<Milestone>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Participant {
    group_id: String,
    count: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ParticipantFlow {
    recruitment_details: Option<String>,
    pre_assignment_details: Option<String>,
    group_list: Option<GroupList>,
    period_list: Option<PeriodList>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PatientData {
    sharing_ipd: String,
    ipd_description: Option<String>,
    ipd_info_type: Option<Vec<String>>,
    ipd_time_frame: Option<String>,
    ipd_access_criteria: Option<String>,
    ipd_url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Period {
    title: Option<String>,
    milestone_list: Option<MilestoneList>,
    drop_withdraw_reason_list: Option<DropWithdrawReason>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PeriodList {
    period: Vec<Period>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PointOfContact {
    name_or_title: String,
    organization: Option<String>,
    phone: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ProtocolOutcome {
    measure: String,
    time_frame: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct OutcomeList {
    outcome: Vec<ResultsOutcome>,
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
struct PendingResult {
    submitted: Option<Variable>,
    returned: Option<Variable>,
    submission_canceled: Option<Variable>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ProvidedDocument {
    document_type: Option<String>,
    document_has_protocol: Option<String>,
    document_has_icf: Option<String>,
    document_has_sap: Option<String>,
    document_date: Option<String>,
    document_url: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ProvidedDocuments {
    provided_document: Vec<ProvidedDocument>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Reference {
    citation: Option<String>,
    #[serde(rename(deserialize = "PMID"))]
    pmid: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ReportedEvents {
    time_frame: Option<String>,
    desc: Option<String>,
    group_list: Option<GroupList>,
    serious_events: Option<Events>,
    other_events: Option<Events>,
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
struct ResultsOutcome {
    #[serde(rename(deserialize = "type"))]
    results_type: String,
    title: String,
    description: Option<String>,
    time_frame: Option<String>,
    safety_issue: Option<String>,
    posting_date: Option<String>,
    population: Option<String>,
    group_list: Option<GroupList>,
    measure: Option<Measure>,
    analysis_list: Option<AnalysisList>,
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
pub struct StudyDoc {
    doc_id: Option<String>,
    doc_type: Option<String>,
    doc_url: Option<String>,
    doc_comment: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct StudyDocs {
    study_doc: Vec<StudyDoc>,
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
struct Variable {
    #[serde(rename(deserialize = "type"))]
    variable_type: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct VocabTerm {
    vocab: Option<String>,
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
            Arg::with_name("resume")
                .value_name("INT")
                .short("r")
                .long("resume")
                .help("Resume after N records"),
        )
        .arg(
            Arg::with_name("force")
                .short("f")
                .long("force")
                .help("Always update from file")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILES or DIRS")
                .help("File input")
                .required(true)
                .min_values(1),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        force: matches.is_present("force"),
        resume: matches
            .value_of("resume")
            .and_then(|v| v.parse::<usize>().ok()),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    let conn = connection()?;
    let files = find_files(&config.files)?;
    let num_files = files.len();

    println!(
        "Processings {} file{}...",
        num_files,
        if num_files == 1 { "" } else { "s" }
    );

    for (fnum, filename) in files.into_iter().enumerate() {
        if let Some(resume) = &config.resume {
            if fnum + 1 < resume - 1 {
                continue;
            }
        }

        let result = match process_file(&conn, &filename, &config.force) {
            Ok(db_study) => {
                format!("{} ({})", db_study.nct_id, db_study.study_id)
            }
            Err(err) => {
                let err = err.to_string();
                eprintln!("{}: {}", filename, err);
                err
            }
        };

        let path = Path::new(&filename);
        let basename = path.file_name().expect("basename");
        let basename = &basename.to_string_lossy().to_string();
        println!("{:6}: {} => {}", fnum + 1, &basename, &result);
    }

    update_dataload(&conn)?;

    println!("Done.");
    Ok(())
}

// --------------------------------------------------
fn process_file(conn: &PgConnection, filename: &str, force: &bool) -> MyResult<DbStudy> {
    let path = Path::new(&filename);
    if !path.is_file() {
        return Err(From::from(format!("'{}' not a valid file", filename)));
    }

    let clinical_study = parse_xml(&path)?;

    // Skip updating the db if the file modtime is older than db updated.
    // The --force flag skips this check and will always update the db.
    if !force {
        if let (Some(last_update_posted), Some(last_up)) = (
            extract_date(&clinical_study.last_update_posted.as_ref()),
            study_last_updated(&conn, &path),
        ) {
            if last_update_posted < last_up {
                return Err(From::from("Study older than data, skipping."));
            }
        }
    }

    let new_phase_name = &clinical_study
        .phase
        .clone()
        .unwrap_or("N/A".to_string())
        .to_string();
    let db_phase = find_or_create_phase(&conn, &new_phase_name)?;

    let db_study_type = find_or_create_study_type(&conn, &clinical_study.study_type)?;

    let new_overall_status = &clinical_study
        .overall_status
        .clone()
        .unwrap_or("Unknown status".to_string())
        .to_string();

    let db_overall_status = find_or_create_status(&conn, &new_overall_status)?;

    let new_last_known_status = &clinical_study
        .last_known_status
        .clone()
        .unwrap_or("Unknown status".to_string())
        .to_string();

    let db_last_known_status = find_or_create_status(&conn, &new_last_known_status)?;

    let result = find_or_create_study(
        &conn,
        &db_phase,
        &db_study_type,
        &db_overall_status,
        &db_last_known_status,
        &clinical_study.id_info.nct_id,
    );

    match result {
        Ok(db_study) => {
            update_study(&conn, &db_study, &clinical_study)?;
            Ok(db_study)
        }
        Err(e) => Err(From::from(format!(
            "Failed to create {}: {:?}",
            clinical_study.id_info.nct_id, e
        ))),
    }
}

// --------------------------------------------------
fn connection() -> MyResult<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("URL {}", database_url);

    match PgConnection::establish(&database_url) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(From::from(format!(
            "Error connecting to {}: {}",
            database_url, err
        ))),
    }
}

// --------------------------------------------------
fn parse_xml(path: &Path) -> MyResult<ClinicalStudy> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    match quick_xml::de::from_reader(&mut reader) {
        Ok(study) => Ok(study),
        Err(err) => Err(From::from(format!("Failed to parse: {:?}", err))),
    }
}

// --------------------------------------------------
fn find_or_create_condition<'a>(
    conn: &PgConnection,
    new_condition_name: &'a str,
) -> DbResult<DbCondition> {
    let results = condition::table
        .filter(condition::condition_name.eq(new_condition_name))
        .first::<DbCondition>(conn);

    match results {
        Ok(c) => Ok(c),
        _ => {
            diesel::insert_into(condition::table)
                .values(DbConditionInsert {
                    condition_name: new_condition_name.to_string(),
                })
                .execute(conn)
                .expect("Error inserting condition");

            condition::table
                .filter(condition::condition_name.eq(new_condition_name))
                .first::<DbCondition>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_intervention<'a>(
    conn: &PgConnection,
    new_intervention_name: &'a str,
) -> DbResult<DbIntervention> {
    let results = intervention::table
        .filter(intervention::intervention_name.eq(new_intervention_name))
        .first::<DbIntervention>(conn);

    match results {
        Ok(c) => Ok(c),
        _ => {
            diesel::insert_into(intervention::table)
                .values(DbInterventionInsert {
                    intervention_name: new_intervention_name.to_string(),
                })
                .execute(conn)
                .expect("Error inserting intervention");

            intervention::table
                .filter(intervention::intervention_name.eq(new_intervention_name))
                .first::<DbIntervention>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_sponsor<'a>(
    conn: &PgConnection,
    new_sponsor_name: &'a str,
) -> DbResult<DbSponsor> {
    let results = sponsor::table
        .filter(sponsor::sponsor_name.eq(new_sponsor_name))
        .first::<DbSponsor>(conn);

    match results {
        Ok(c) => Ok(c),
        _ => {
            diesel::insert_into(sponsor::table)
                .values(DbSponsorInsert {
                    sponsor_name: new_sponsor_name.to_string(),
                })
                .execute(conn)
                .expect("Error inserting sponsor");

            sponsor::table
                .filter(sponsor::sponsor_name.eq(new_sponsor_name))
                .first::<DbSponsor>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_doc(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_doc: &StudyDoc,
) -> DbResult<DbStudyDoc> {
    fn opt_text(val: &Option<String>) -> Option<String> {
        Some(val.as_ref().unwrap_or(&"".to_string()).to_string())
    }

    let new_doc_id = opt_text(&new_doc.doc_id);
    let new_doc_type = opt_text(&new_doc.doc_type);
    let new_doc_url = opt_text(&new_doc.doc_url);
    let new_doc_comment = opt_text(&new_doc.doc_comment);
    let query = study_doc::table
        .filter(study_doc::study_id.eq(&new_study.study_id))
        .filter(study_doc::doc_id.eq(&new_doc_id))
        .filter(study_doc::doc_type.eq(&new_doc_type))
        .filter(study_doc::doc_url.eq(&new_doc_url))
        .filter(study_doc::doc_comment.eq(&new_doc_comment));

    match query.first::<DbStudyDoc>(conn) {
        Ok(c) => Ok(c),
        _ => {
            diesel::insert_into(study_doc::table)
                .values(DbStudyDocInsert {
                    study_id: new_study.study_id,
                    doc_id: new_doc_id.clone(),
                    doc_type: new_doc_type.clone(),
                    doc_url: new_doc_url.clone(),
                    doc_comment: new_doc_comment.clone(),
                })
                .execute(conn)
                .expect("Error inserting study_doc");

            query.first::<DbStudyDoc>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_url(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_link: &Link,
) -> DbResult<DbStudyUrl> {
    let query = study_url::table
        .filter(study_url::study_id.eq(&new_study.study_id))
        .filter(study_url::url.eq(&new_link.url));

    match query.first::<DbStudyUrl>(conn) {
        Ok(c) => Ok(c),
        _ => {
            diesel::insert_into(study_url::table)
                .values(DbStudyUrlInsert {
                    study_id: new_study.study_id,
                    url: new_link.url.clone(),
                })
                .execute(conn)
                .expect("Error inserting study_url");

            query.first::<DbStudyUrl>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_design(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_design: &StudyDesignInfo,
) -> DbResult<DbStudyDesign> {
    let query = study_design::table.filter(study_design::study_id.eq(&new_study.study_id));

    match query.first::<DbStudyDesign>(conn) {
        Ok(e) => Ok(e),
        _ => {
            diesel::insert_into(study_design::table)
                .values(DbStudyDesignInsert {
                    study_id: new_study.study_id,
                    allocation: new_design.allocation.clone(),
                    intervention_model: new_design.intervention_model.clone(),
                    intervention_model_description: new_design
                        .intervention_model_description
                        .clone(),
                    primary_purpose: new_design.primary_purpose.clone(),
                    observational_model: new_design.observational_model.clone(),
                    time_perspective: new_design.time_perspective.clone(),
                    masking: new_design.masking.clone(),
                    masking_description: new_design.masking_description.clone(),
                })
                .execute(conn)
                .expect("Error inserting study_design");

            query.first::<DbStudyDesign>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_eligibility(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_elig: &Eligibility,
) -> DbResult<DbStudyEligibility> {
    let query =
        study_eligibility::table.filter(study_eligibility::study_id.eq(&new_study.study_id));

    match query.first::<DbStudyEligibility>(conn) {
        Ok(e) => Ok(e),
        _ => {
            diesel::insert_into(study_eligibility::table)
                .values(DbStudyEligibilityInsert {
                    study_id: new_study.study_id,
                    study_pop: extract_textblock(&new_elig.study_pop.as_ref()),
                    sampling_method: new_elig.sampling_method.clone(),
                    criteria: extract_textblock(&new_elig.criteria.as_ref()),
                    gender: new_elig.gender.clone(),
                    gender_based: new_elig.gender_based.clone(),
                    gender_description: new_elig.gender_description.clone(),
                    minimum_age: new_elig.minimum_age.clone(),
                    maximum_age: new_elig.maximum_age.clone(),
                    healthy_volunteers: new_elig.healthy_volunteers.clone(),
                })
                .execute(conn)
                .expect("Error inserting study_eligibility");

            query.first::<DbStudyEligibility>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_location(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_location: &Location,
) -> DbResult<DbStudyLocation> {
    let query = study_location::table.filter(study_location::study_id.eq(&new_study.study_id));

    match query.first::<DbStudyLocation>(conn) {
        Ok(e) => Ok(e),
        _ => {
            let contact = new_location.contact.as_ref().and_then(|c| {
                Some(format!(
                    "{} {}",
                    c.first_name
                        .as_ref()
                        .map_or("".to_string(), |v| v.to_string()),
                    c.last_name
                        .as_ref()
                        .map_or("".to_string(), |v| v.to_string())
                ))
            });

            let investigators = new_location.investigator.as_ref().and_then(|v| {
                Some(
                    v.iter()
                        .map(|i| {
                            format!(
                                "{} {}",
                                i.first_name
                                    .as_ref()
                                    .map_or("".to_string(), |v| v.to_string()),
                                i.last_name
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            });

            let facility_name = new_location
                .facility
                .as_ref()
                .and_then(|f| f.name.as_ref().and_then(|v| Some(v.to_string())));

            diesel::insert_into(study_location::table)
                .values(DbStudyLocationInsert {
                    study_id: new_study.study_id,
                    facility_name: facility_name.clone(),
                    status: new_location.status.clone(),
                    contact_name: contact,
                    investigator_name: investigators,
                })
                .execute(conn)
                .expect("Error inserting study_location");

            query.first::<DbStudyLocation>(conn)
        }
    }
}

// --------------------------------------------------
//fn delete_study_conditions(
//    conn: &PgConnection,
//    new_study: &DbStudy,
//) -> DbResult<()> {
//    diesel::delete(
//        study_to_condition::table
//            .filter(study_to_condition::study_id.eq(new_study.study_id)),
//    )
//    .execute(conn)?;

//    Ok(())
//}

// --------------------------------------------------
//fn delete_study_docs(
//    conn: &PgConnection,
//    new_study: &DbStudy,
//) -> DbResult<()> {
//    diesel::delete(
//        study_doc::table.filter(study_doc::study_id.eq(new_study.study_id)),
//    )
//    .execute(conn)?;

//    Ok(())
//}

// --------------------------------------------------
fn find_files(paths: &Vec<String>) -> MyResult<Vec<String>> {
    let mut files: Vec<String> = vec![];
    for path in paths {
        files.extend(
            WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().map_or(false, |ext| ext == "xml"))
                .map(|e| e.path().display().to_string()),
        );
    }

    Ok(files)
}

// --------------------------------------------------
fn find_or_create_study_to_condition(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_condition: &DbCondition,
) -> DbResult<DbStudyToCondition> {
    let results = study_to_condition::table
        .filter(study_to_condition::study_id.eq(new_study.study_id))
        .filter(study_to_condition::condition_id.eq(new_condition.condition_id))
        .first::<DbStudyToCondition>(conn);

    match results {
        Ok(c2s) => Ok(c2s),
        _ => {
            diesel::insert_into(study_to_condition::table)
                .values(DbStudyToConditionInsert {
                    study_id: new_study.study_id,
                    condition_id: new_condition.condition_id,
                })
                .execute(conn)
                .expect("Error inserting condition_to_study");

            study_to_condition::table
                .filter(study_to_condition::study_id.eq(new_study.study_id))
                .filter(study_to_condition::condition_id.eq(new_condition.condition_id))
                .first::<DbStudyToCondition>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_to_intervention(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_intervention: &DbIntervention,
) -> DbResult<DbStudyToIntervention> {
    let results = study_to_intervention::table
        .filter(study_to_intervention::study_id.eq(new_study.study_id))
        .filter(study_to_intervention::intervention_id.eq(new_intervention.intervention_id))
        .first::<DbStudyToIntervention>(conn);

    match results {
        Ok(c2s) => Ok(c2s),
        _ => {
            diesel::insert_into(study_to_intervention::table)
                .values(DbStudyToInterventionInsert {
                    study_id: new_study.study_id,
                    intervention_id: new_intervention.intervention_id,
                })
                .execute(conn)
                .expect("Error inserting intervention_to_study");

            study_to_intervention::table
                .filter(study_to_intervention::study_id.eq(new_study.study_id))
                .filter(study_to_intervention::intervention_id.eq(new_intervention.intervention_id))
                .first::<DbStudyToIntervention>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_to_sponsor(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_sponsor: &DbSponsor,
) -> DbResult<DbStudyToSponsor> {
    let results = study_to_sponsor::table
        .filter(study_to_sponsor::study_id.eq(new_study.study_id))
        .filter(study_to_sponsor::sponsor_id.eq(new_sponsor.sponsor_id))
        .first::<DbStudyToSponsor>(conn);

    match results {
        Ok(c2s) => Ok(c2s),
        _ => {
            diesel::insert_into(study_to_sponsor::table)
                .values(DbStudyToSponsorInsert {
                    study_id: new_study.study_id,
                    sponsor_id: new_sponsor.sponsor_id,
                })
                .execute(conn)
                .expect("Error inserting sponsor_to_study");

            study_to_sponsor::table
                .filter(study_to_sponsor::study_id.eq(new_study.study_id))
                .filter(study_to_sponsor::sponsor_id.eq(new_sponsor.sponsor_id))
                .first::<DbStudyToSponsor>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_phase<'a>(conn: &PgConnection, new_phase_name: &'a str) -> DbResult<DbPhase> {
    let results = phase::table
        .filter(phase::phase_name.eq(new_phase_name))
        .first::<DbPhase>(conn);

    match results {
        Ok(s) => Ok(s),
        _ => {
            diesel::insert_into(phase::table)
                .values(DbPhaseInsert {
                    phase_name: new_phase_name,
                })
                .execute(conn)
                .expect("Error inserting phase");

            phase::table
                .filter(phase::phase_name.eq(new_phase_name))
                .first::<DbPhase>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_status<'a>(conn: &PgConnection, new_status_name: &'a str) -> DbResult<DbStatus> {
    let results = status::table
        .filter(status::status_name.eq(new_status_name))
        .first::<DbStatus>(conn);

    match results {
        Ok(s) => Ok(s),
        _ => {
            diesel::insert_into(status::table)
                .values(DbStatusInsert {
                    status_name: new_status_name.to_string(),
                })
                .execute(conn)
                .expect("Error inserting status");

            status::table
                .filter(status::status_name.eq(new_status_name))
                .first::<DbStatus>(conn)
        }
    }
}

// --------------------------------------------------
fn study_last_updated<'a>(conn: &PgConnection, path: &Path) -> Option<NaiveDate> {
    use crate::schema::study::dsl::*;

    match path.file_stem() {
        Some(stem) => {
            let study_nct_id = &stem.to_string_lossy().to_string();

            match study.filter(nct_id.eq(study_nct_id)).first::<DbStudy>(conn) {
                Ok(db_study) => db_study.record_last_updated.and_then(|d| Some(d.date())),
                _ => None,
            }
        }
        _ => None,
    }
}

// --------------------------------------------------
fn find_or_create_study<'a>(
    conn: &PgConnection,
    db_phase: &DbPhase,
    db_study_type: &DbStudyType,
    db_overall_status: &DbStatus,
    db_last_known_status: &DbStatus,
    new_nct_id: &'a str,
) -> DbResult<DbStudy> {
    use crate::schema::study::dsl::*;
    let results = study.filter(nct_id.eq(new_nct_id)).first::<DbStudy>(conn);

    match results {
        Ok(db_study) => {
            diesel::update(&db_study)
                .set((
                    phase_id.eq(&db_phase.phase_id),
                    study_type_id.eq(&db_study_type.study_type_id),
                    overall_status_id.eq(&db_overall_status.status_id),
                    last_known_status_id.eq(&db_last_known_status.status_id),
                    record_last_updated.eq(Some(Utc::now().naive_utc())),
                ))
                .execute(conn)
                .expect("Error updating study");
            Ok(db_study)
        }
        _ => {
            diesel::insert_into(study)
                .values(DbStudyInsert {
                    phase_id: &db_phase.phase_id,
                    study_type_id: db_study_type.study_type_id,
                    overall_status_id: db_overall_status.status_id,
                    last_known_status_id: db_last_known_status.status_id,
                    nct_id: new_nct_id,
                })
                .execute(conn)
                .expect("Error inserting study");

            study.filter(nct_id.eq(new_nct_id)).first::<DbStudy>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_type<'a>(
    conn: &PgConnection,
    new_study_type_name: &'a str,
) -> DbResult<DbStudyType> {
    let results = study_type::table
        .filter(study_type::study_type_name.eq(new_study_type_name))
        .first::<DbStudyType>(conn);

    match results {
        Ok(s) => Ok(s),
        _ => {
            diesel::insert_into(study_type::table)
                .values(DbStudyTypeInsert {
                    study_type_name: new_study_type_name.to_string(),
                })
                .execute(conn)
                .expect("Error inserting study_type");

            study_type::table
                .filter(study_type::study_type_name.eq(new_study_type_name))
                .first::<DbStudyType>(conn)
        }
    }
}

// --------------------------------------------------
fn find_or_create_study_outcome<'a>(
    conn: &PgConnection,
    new_study: &DbStudy,
    new_outcome: &ProtocolOutcome,
    outcome_type: String,
) -> DbResult<DbStudyOutcome> {
    fn opt_text(val: &Option<String>) -> Option<String> {
        Some(val.as_ref().unwrap_or(&"".to_string()).to_string())
    }

    let new_measure = &new_outcome.measure;
    let new_time_frame = opt_text(&new_outcome.time_frame);
    let new_desc = opt_text(&new_outcome.description);

    let query = study_outcome::table
        .filter(study_outcome::study_id.eq(&new_study.study_id))
        .filter(study_outcome::outcome_type.eq(&outcome_type))
        .filter(study_outcome::measure.eq(&new_measure))
        .filter(study_outcome::time_frame.eq(&new_time_frame))
        .filter(study_outcome::description.eq(&new_desc));

    match query.first::<DbStudyOutcome>(conn) {
        Ok(s) => Ok(s),
        _ => {
            diesel::insert_into(study_outcome::table)
                .values(DbStudyOutcomeInsert {
                    study_id: new_study.study_id,
                    outcome_type: outcome_type.clone(),
                    measure: new_measure.clone(),
                    time_frame: new_time_frame.clone(),
                    description: new_desc.clone(),
                })
                .execute(conn)
                .expect("Error inserting study_outcome");

            query.first::<DbStudyOutcome>(conn)
        }
    }
}

// --------------------------------------------------
fn update_study<'a>(
    conn: &PgConnection,
    db_study: &'a DbStudy,
    new_study: &ClinicalStudy,
) -> MyResult<()> {
    use crate::schema::study::dsl::*;

    // println!("{:#?}", new_study.link);
    // println!("all_text = {:?}", get_all_text(&new_study));

    diesel::update(db_study)
        .set((
            brief_title.eq(&new_study.brief_title),
            official_title.eq(&new_study.official_title),
            org_study_id.eq(&new_study.id_info.org_study_id),
            acronym.eq(&new_study.acronym),
            source.eq(&new_study.source),
            rank.eq(&new_study.rank),
            brief_summary.eq(extract_textblock(&new_study.brief_summary.as_ref())),
            detailed_description.eq(extract_textblock(&new_study.detailed_description.as_ref())),
            why_stopped.eq(&new_study.why_stopped),
            has_expanded_access.eq(&new_study.has_expanded_access),
            target_duration.eq(&new_study.target_duration),
            biospec_retention.eq(&new_study.biospec_retention),
            biospec_description.eq(extract_textblock(&new_study.biospec_descr.as_ref())),
            keywords.eq(&new_study.keyword.as_ref().and_then(|x| Some(x.join(", ")))),
            enrollment.eq(&new_study.enrollment),
            //start_date.eq(extract_date(&new_study.start_date.as_ref())),
            //completion_date
            //    .eq(extract_date(&new_study.completion_date.as_ref())),
            fulltext_load.eq(get_all_text(&new_study)),
        ))
        .execute(conn)
        .expect("Error updating study");

    //let sql = format!(
    //    r#"
    //        update study
    //        set fulltext=to_tsvector('english', fulltext_load)
    //        where study_id={}
    //    "#,
    //    db_study.study_id
    //);

    //let _: Result<Vec<String>, diesel::result::Error> =
    //    diesel::sql_query(sql).load(conn);

    //// Conditions
    //delete_study_conditions(&conn, &db_study)?;
    if let Some(new_conditions) = &new_study.condition {
        for new_condition in new_conditions {
            let db_condition = find_or_create_condition(&conn, &new_condition)?;

            find_or_create_study_to_condition(&conn, &db_study, &db_condition)?;
        }
    }

    // Interventions
    if let Some(new_interventions) = &new_study.intervention {
        for new_intervention in new_interventions {
            let db_intervention =
                find_or_create_intervention(&conn, &new_intervention.intervention_name)?;

            find_or_create_study_to_intervention(&conn, &db_study, &db_intervention)?;
        }
    }

    // Sponsors
    if let Some(new_sponsors) = &new_study.sponsors {
        let lead_sponsor = find_or_create_sponsor(&conn, &new_sponsors.lead_sponsor.agency)?;
        find_or_create_study_to_sponsor(&conn, &db_study, &lead_sponsor)?;

        if let Some(collaborators) = &new_sponsors.collaborator {
            for new_sponsor in collaborators {
                let db_sponsor = find_or_create_sponsor(&conn, &new_sponsor.agency)?;

                find_or_create_study_to_sponsor(&conn, &db_study, &db_sponsor)?;
            }
        }
    }

    // StudyDocs
    //delete_study_docs(&conn, &db_study)?;
    if let Some(new_study_docs) = &new_study.study_docs {
        for new_study_doc in new_study_docs.study_doc.iter() {
            find_or_create_study_doc(&conn, &db_study, &new_study_doc)?;
        }
    }

    // Link/StudyUrl
    if let Some(links) = &new_study.link {
        for link in links {
            find_or_create_study_url(&conn, &db_study, &link)?;
        }
    }

    // Locations
    if let Some(locations) = &new_study.location {
        for loc in locations {
            find_or_create_study_location(&conn, &db_study, &loc)?;
        }
    }

    // Eligibility
    if let Some(elig) = &new_study.eligibility {
        find_or_create_study_eligibility(&conn, &db_study, &elig)?;
    }

    // StudyDesign
    if let Some(design) = &new_study.study_design_info {
        find_or_create_study_design(&conn, &db_study, &design)?;
    }

    // Primary Outcomes
    if let Some(new_primary_outcomes) = &new_study.primary_outcome {
        for new_outcome in new_primary_outcomes.iter() {
            find_or_create_study_outcome(&conn, &db_study, &new_outcome, "primary".to_string())?;
        }
    }

    // Secondary Outcomes
    if let Some(new_secondary_outcomes) = &new_study.secondary_outcome {
        for new_outcome in new_secondary_outcomes.iter() {
            find_or_create_study_outcome(&conn, &db_study, &new_outcome, "secondary".to_string())?;
        }
    }

    // Other Outcomes
    if let Some(new_other_outcomes) = &new_study.other_outcome {
        for new_outcome in new_other_outcomes.iter() {
            find_or_create_study_outcome(&conn, &db_study, &new_outcome, "other".to_string())?;
        }
    }

    Ok(())
}

// --------------------------------------------------
fn get_all_text(study: &ClinicalStudy) -> Option<String> {
    fn opt_text(val: Option<&String>) -> String {
        val.unwrap_or(&"".to_string()).to_string()
    }

    fn tb_text(tb: Option<&Textblock>) -> String {
        extract_textblock(&tb).unwrap_or("".to_string()).to_string()
    }

    fn vec_text(val: Option<&Vec<String>>) -> String {
        match val {
            Some(items) => items.join(" ").to_string(),
            _ => "".to_string(),
        }
    }

    let references = match &study.reference {
        Some(refs) => {
            let mut vals: Vec<String> = vec![];
            for reference in refs {
                if let Some(cite) = &reference.citation {
                    vals.push(cite.to_string());
                }

                if let Some(pmid) = &reference.pmid {
                    vals.push(pmid.to_string());
                }
            }

            vals.join(" ")
        }
        _ => "".to_string(),
    };

    let interventions = match &study.intervention {
        Some(vals) => vals
            .into_iter()
            .map(|x| x.intervention_name.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        _ => "".to_string(),
    };

    let (lead_sponsor, collaborators) = match &study.sponsors {
        Some(val) => {
            let collabs = match &val.collaborator {
                Some(c) => c
                    .into_iter()
                    .map(|x| x.agency.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                _ => "".to_string(),
            };
            (val.lead_sponsor.agency.to_string(), collabs)
        }
        _ => ("".to_string(), "".to_string()),
    };

    let mut outcomes: Vec<String> = vec![];
    if let Some(primary_outcomes) = &study.primary_outcome {
        for outcome in primary_outcomes {
            let desc = match &outcome.description {
                Some(val) => val.to_string(),
                _ => "".to_string(),
            };
            outcomes.push(format!("{} {}", outcome.measure, desc));
        }
    }

    if let Some(secondary_outcomes) = &study.secondary_outcome {
        for outcome in secondary_outcomes {
            let desc = match &outcome.description {
                Some(val) => val.to_string(),
                _ => "".to_string(),
            };
            outcomes.push(format!("{} {}", outcome.measure, desc));
        }
    }

    if let Some(other_outcomes) = &study.other_outcome {
        for outcome in other_outcomes {
            let desc = match &outcome.description {
                Some(val) => val.to_string(),
                _ => "".to_string(),
            };
            outcomes.push(format!("{} {}", outcome.measure, desc));
        }
    }

    let all_fields = vec![
        study.id_info.nct_id.to_string(),
        study.brief_title.to_string(),
        opt_text(study.official_title.as_ref()),
        study.source.to_string(),
        interventions,
        lead_sponsor,
        collaborators,
        references,
        vec_text(study.condition.as_ref()),
        vec_text(study.keyword.as_ref()),
        opt_text(study.official_title.as_ref()),
        opt_text(study.acronym.as_ref()),
        tb_text(study.brief_summary.as_ref()),
        tb_text(study.detailed_description.as_ref()),
        outcomes.join(" ").to_string(),
    ];

    let re1 = Regex::new(r"[^a-z0-9.]").unwrap();
    let re2 = Regex::new(r"[.]$").unwrap();
    let mut words: HashSet<String> = HashSet::new();
    for fld in &all_fields {
        for word in fld.split_whitespace() {
            let clean = re1
                .replace_all(&re2.replace_all(&word.to_ascii_lowercase(), ""), "")
                .to_string();

            if clean.len() > 2 {
                words.insert(clean);
            }
        }
    }

    Some(words.into_iter().collect::<Vec<String>>().join(" "))
}

// --------------------------------------------------
fn extract_textblock(val: &Option<&Textblock>) -> Option<String> {
    match val {
        Some(tb) => {
            let re = Regex::new(r"\s+").unwrap();
            Some(re.replace_all(&tb.textblock, " ").to_string())
        }
        _ => None,
    }
}

// --------------------------------------------------
fn extract_date(val: &Option<&String>) -> Option<NaiveDate> {
    match val {
        Some(date) => {
            if let Ok((dt, _)) = dtparse::parse(date) {
                Some(dt.date())
            } else {
                None
            }
        }
        _ => None,
    }
}

// --------------------------------------------------
fn update_dataload<'a>(conn: &PgConnection) -> MyResult<()> {
    use crate::schema::dataload::dsl::*;

    let today = Some(Utc::now().naive_utc().date());
    let results = dataload
        .filter(updated_on.eq(today))
        .first::<DbDataload>(conn);

    if let Err(_) = results {
        diesel::insert_into(dataload)
            .values(DbDataloadInsert { updated_on: today })
            .execute(conn)
            .expect("Error inserting dataload");
    }

    Ok(())
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn test_1() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let file = manifest_dir.join(PathBuf::from("data/test.xml"));
        //let conf = Config {
        //    files: vec![file.display().to_string()],
        //};

        let _res = match parse_xml(&file.display().to_string()) {
            Ok(study) => {
                assert_eq!(
                    study.required_header.url,
                    "https://clinicaltrials.gov/show/NCT00000516"
                );

                assert_eq!(
                    study.brief_title,
                    "Studies of Left Ventricular Dysfunction (SOLVD)"
                );

                assert_eq!(study.id_info.nct_id, "NCT00000516");

                assert_eq!(study.enrollment, Some(49));

                assert_eq!(
                    study.sponsors.lead_sponsor.agency,
                    "National Heart, Lung, and Blood Institute (NHLBI)"
                );

                assert_eq!(
                    study.source,
                    "National Heart, Lung, and Blood Institute (NHLBI)"
                );

                assert_eq!(study.study_type, "Interventional");

                assert_eq!(
                    study.condition,
                    Some(vec![
                        "Cardiovascular Diseases".to_string(),
                        "Coronary Disease".to_string(),
                        "Heart Diseases".to_string(),
                        "Heart Failure".to_string(),
                        "Hypertension".to_string(),
                        "Myocardial Ischemia".to_string()
                    ]),
                );

                assert_that!(study.reference).is_some().has_length(54);

                assert_that!(study.condition_browse).is_some();

                if let Some(browse) = study.condition_browse {
                    assert_that!(browse.mesh_term).has_length(5);
                }

                assert_that!(study.study_docs).is_some();

                if let Some(docs) = study.study_docs {
                    assert_that!(docs.study_doc).has_length(4);
                }
            }
            Err(x) => panic!("{:?}", x),
        };
    }
}
