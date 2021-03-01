#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(condition_id)]
#[table_name = "condition"]
pub struct DbCondition {
    pub condition_id: i32,
    pub condition_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "condition"]
pub struct DbConditionInsert {
    pub condition_name: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(intervention_id)]
#[table_name = "intervention"]
pub struct DbIntervention {
    pub intervention_id: i32,
    pub intervention_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "intervention"]
pub struct DbInterventionInsert {
    pub intervention_name: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "phase"]
#[primary_key(phase_id)]
pub struct DbPhase {
    pub phase_id: i32,
    pub phase_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "phase"]
pub struct DbPhaseInsert<'a> {
    pub phase_name: &'a str,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(sponsor_id)]
#[table_name = "sponsor"]
pub struct DbSponsor {
    pub sponsor_id: i32,
    pub sponsor_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "sponsor"]
pub struct DbSponsorInsert {
    pub sponsor_name: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "status"]
#[primary_key(status_id)]
pub struct DbStatus {
    pub status_id: i32,
    pub status_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "status"]
pub struct DbStatusInsert {
    pub status_name: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "study"]
#[primary_key(study_id)]
pub struct DbStudy {
    pub study_id: i32,
    pub study_type_id: i32,
    pub phase_id: i32,
    pub overall_status_id: i32,
    pub last_known_status_id: i32,
    pub nct_id: String,
    pub brief_title: Option<String>,
    pub official_title: Option<String>,
    pub org_study_id: Option<String>,
    pub acronym: Option<String>,
    pub source: Option<String>,
    pub rank: Option<String>,
    pub brief_summary: Option<String>,
    pub detailed_description: Option<String>,
    pub why_stopped: Option<String>,
    pub has_expanded_access: Option<String>,
    pub target_duration: Option<String>,
    pub biospec_retention: Option<String>,
    pub biospec_description: Option<String>,
    pub keywords: Option<String>,
    pub enrollment: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub completion_date: Option<NaiveDate>,
    pub all_text: Option<String>,
    pub record_last_updated: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "study"]
pub struct DbStudyInsert<'a> {
    pub phase_id: &'a i32,
    pub study_type_id: i32,
    pub overall_status_id: i32,
    pub last_known_status_id: i32,
    pub nct_id: &'a str,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "study_outcome"]
#[primary_key(study_outcome_id)]
pub struct DbStudyOutcome {
    pub study_outcome_id: i32,
    pub study_id: i32,
    pub outcome_type: String,
    pub measure: String,
    pub time_frame: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "study_outcome"]
pub struct DbStudyOutcomeInsert {
    pub study_id: i32,
    pub outcome_type: String,
    pub measure: String,
    pub time_frame: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[table_name = "study_type"]
#[primary_key(study_type_id)]
pub struct DbStudyType {
    pub study_type_id: i32,
    pub study_type_name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "study_type"]
pub struct DbStudyTypeInsert {
    pub study_type_name: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(study_to_condition_id)]
#[table_name = "study_to_condition"]
pub struct DbStudyToCondition {
    pub study_to_condition_id: i32,
    pub study_id: i32,
    pub condition_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "study_to_condition"]
pub struct DbStudyToConditionInsert {
    pub study_id: i32,
    pub condition_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(study_doc_id)]
#[table_name = "study_doc"]
pub struct DbStudyDoc {
    pub study_doc_id: i32,
    pub study_id: i32,
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub doc_url: Option<String>,
    pub doc_comment: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "study_doc"]
pub struct DbStudyDocInsert {
    pub study_id: i32,
    pub doc_id: Option<String>,
    pub doc_type: Option<String>,
    pub doc_url: Option<String>,
    pub doc_comment: Option<String>,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(study_to_intervention_id)]
#[table_name = "study_to_intervention"]
pub struct DbStudyToIntervention {
    pub study_to_intervention_id: i32,
    pub study_id: i32,
    pub intervention_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "study_to_intervention"]
pub struct DbStudyToInterventionInsert {
    pub study_id: i32,
    pub intervention_id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[primary_key(study_to_sponsor_id)]
#[table_name = "study_to_sponsor"]
pub struct DbStudyToSponsor {
    pub study_to_sponsor_id: i32,
    pub study_id: i32,
    pub sponsor_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "study_to_sponsor"]
pub struct DbStudyToSponsorInsert {
    pub study_id: i32,
    pub sponsor_id: i32,
}
