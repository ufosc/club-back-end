table! {
    attendances (ufl_username, start_timestamp) {
        ufl_username -> Text,
        start_timestamp -> Timestamptz,
    }
}

table! {
    contributors (ufl_username, github_url) {
        ufl_username -> Text,
        github_url -> Text,
        is_project_lead -> Nullable<Bool>,
    }
}

table! {
    events (start_timestamp) {
        start_timestamp -> Timestamptz,
        title -> Text,
        location -> Text,
        description -> Text,
        end_timestamp -> Timestamptz,
        image -> Bytea,
    }
}

table! {
    members (ufl_username) {
        ufl_username -> Text,
        is_info_filled_out -> Bool,
        first_name -> Text,
        last_name -> Text,
        discord_username -> Text,
        github_username -> Text,
        server_username -> Text,
        server_key -> Text,
        is_acm_shareable -> Bool,
        is_in_email_list -> Bool,
    }
}

table! {
    officers (ufl_username) {
        ufl_username -> Text,
        position -> Nullable<Text>,
    }
}

table! {
    projects (github_url) {
        github_url -> Text,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        technologies -> Nullable<Array<Text>>,
        discord_channel -> Nullable<Text>,
        is_active -> Nullable<Bool>,
        next_milestone_date -> Nullable<Timestamptz>,
        image -> Nullable<Bytea>,
    }
}

joinable!(attendances -> events (start_timestamp));
joinable!(attendances -> members (ufl_username));
joinable!(contributors -> members (ufl_username));
joinable!(contributors -> projects (github_url));
joinable!(officers -> members (ufl_username));

allow_tables_to_appear_in_same_query!(
    attendances,
    contributors,
    events,
    members,
    officers,
    projects,
);
