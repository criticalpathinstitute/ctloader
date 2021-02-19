table! {
    phase (phase_id) {
        phase_id -> Int4,
        phase_name -> Varchar,
    }
}

table! {
    status (status_id) {
        status_id -> Int4,
        status_name -> Varchar,
    }
}

table! {
    study (study_id) {
        study_id -> Int4,
        study_type_id -> Int4,
        phase_id -> Int4,
        overall_status_id -> Int4,
        last_known_status_id -> Int4,
        nct_id -> Varchar,
        brief_title -> Nullable<Text>,
        official_title -> Nullable<Text>,
        org_study_id -> Nullable<Text>,
        acronym -> Nullable<Text>,
        source -> Nullable<Text>,
        rank -> Nullable<Text>,
        brief_summary -> Nullable<Text>,
        detailed_description -> Nullable<Text>,
        why_stopped -> Nullable<Text>,
        has_expanded_access -> Nullable<Text>,
        target_duration -> Nullable<Text>,
        biospec_retention -> Nullable<Text>,
        biospec_description -> Nullable<Text>,
        keywords -> Nullable<Text>,
        enrollment -> Nullable<Int4>,
        start_date -> Nullable<Date>,
        completion_date -> Nullable<Date>,
        all_text -> Nullable<Text>,
        record_last_updated -> Nullable<Timestamp>,
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

allow_tables_to_appear_in_same_query!(
    phase,
    status,
    study,
    study_type,
);
