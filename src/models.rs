#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;

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
#[table_name = "study"]
#[primary_key(study_id)]
pub struct DbStudy {
    pub study_id: i32,
    pub study_type_id: i32,
    pub phase_id: i32,
    pub nct_id: String,
    pub brief_title: Option<String>,
    pub org_study_id: Option<String>,
    pub official_title: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "study"]
pub struct DbStudyInsert<'a> {
    pub phase_id: &'a i32,
    pub study_type_id: i32,
    pub nct_id: &'a str,
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
