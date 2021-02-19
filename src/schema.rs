table! {
    phase (phase_id) {
        phase_id -> Int4,
        phase_name -> Varchar,
    }
}

table! {
    study (study_id) {
        study_id -> Int4,
        study_type_id -> Int4,
        phase_id -> Int4,
        nct_id -> Varchar,
        brief_title -> Nullable<Text>,
        org_study_id -> Nullable<Text>,
        official_title -> Nullable<Text>,
    }
}

table! {
    study_type (study_type_id) {
        study_type_id -> Int4,
        study_type_name -> Varchar,
    }
}

joinable!(study -> phase (phase_id));
joinable!(study -> study_type (study_type_id));

allow_tables_to_appear_in_same_query!(phase, study, study_type,);
