table! {
    condition (condition_id) {
        condition_id -> Int4,
        condition_name -> Varchar,
    }
}

table! {
    dataload (dataload_id) {
        dataload_id -> Int4,
        updated_on -> Nullable<Date>,
    }
}

table! {
    intervention (intervention_id) {
        intervention_id -> Int4,
        intervention_name -> Varchar,
    }
}

table! {
    phase (phase_id) {
        phase_id -> Int4,
        phase_name -> Varchar,
    }
}

table! {
    saved_search (saved_search_id) {
        saved_search_id -> Int4,
        web_user_id -> Int4,
        search_name -> Varchar,
        full_text -> Text,
        full_text_bool -> Int4,
        conditions -> Text,
        conditions_bool -> Int4,
        sponsors -> Text,
        sponsors_bool -> Int4,
        phase_ids -> Text,
        study_type_ids -> Text,
        enrollment -> Int4,
        email_to -> Varchar,
    }
}

table! {
    sponsor (sponsor_id) {
        sponsor_id -> Int4,
        sponsor_name -> Varchar,
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
        record_last_updated -> Nullable<Timestamp>,
        fulltext_load -> Nullable<Text>,
    }
}

table! {
    study_doc (study_doc_id) {
        study_doc_id -> Int4,
        study_id -> Int4,
        doc_id -> Nullable<Varchar>,
        doc_type -> Nullable<Varchar>,
        doc_url -> Nullable<Text>,
        doc_comment -> Nullable<Text>,
    }
}

table! {
    study_outcome (study_outcome_id) {
        study_outcome_id -> Int4,
        study_id -> Int4,
        outcome_type -> Varchar,
        measure -> Text,
        time_frame -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    study_to_condition (study_to_condition_id) {
        study_to_condition_id -> Int4,
        study_id -> Int4,
        condition_id -> Int4,
    }
}

table! {
    study_to_intervention (study_to_intervention_id) {
        study_to_intervention_id -> Int4,
        study_id -> Int4,
        intervention_id -> Int4,
    }
}

table! {
    study_to_sponsor (study_to_sponsor_id) {
        study_to_sponsor_id -> Int4,
        study_id -> Int4,
        sponsor_id -> Int4,
    }
}

table! {
    study_type (study_type_id) {
        study_type_id -> Int4,
        study_type_name -> Varchar,
    }
}

table! {
    web_user (web_user_id) {
        web_user_id -> Int4,
        email -> Varchar,
        name -> Varchar,
        picture -> Varchar,
    }
}

joinable!(saved_search -> web_user (web_user_id));
joinable!(study -> phase (phase_id));
joinable!(study -> study_type (study_type_id));
joinable!(study_doc -> study (study_id));
joinable!(study_outcome -> study (study_id));
joinable!(study_to_condition -> condition (condition_id));
joinable!(study_to_condition -> study (study_id));
joinable!(study_to_intervention -> intervention (intervention_id));
joinable!(study_to_intervention -> study (study_id));
joinable!(study_to_sponsor -> sponsor (sponsor_id));
joinable!(study_to_sponsor -> study (study_id));

allow_tables_to_appear_in_same_query!(
    condition,
    dataload,
    intervention,
    phase,
    saved_search,
    sponsor,
    status,
    study,
    study_doc,
    study_outcome,
    study_to_condition,
    study_to_intervention,
    study_to_sponsor,
    study_type,
    web_user,
);
