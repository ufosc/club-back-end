table! {
    attendance (ufl_username, start_timestamp) {
        ufl_username -> Text,
        start_timestamp -> Timestamptz,
    }
}

table! {
    event (start_timestamp) {
        start_timestamp -> Timestamptz,
        title -> Text,
        location -> Text,
        description -> Nullable<Text>,
        end_timestamp -> Timestamptz,
        image -> Nullable<Bytea>,
    }
}

table! {
    member (ufl_username) {
        ufl_username -> Text,
        is_info_filled_out -> Nullable<Bool>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        discord_username -> Nullable<Text>,
        github_username -> Nullable<Text>,
        server_username -> Nullable<Text>,
        server_key -> Nullable<Text>,
        is_acm_shareable -> Nullable<Bool>,
        is_in_email_list -> Nullable<Bool>,
    }
}

joinable!(attendance -> event (start_timestamp));
joinable!(attendance -> member (ufl_username));

allow_tables_to_appear_in_same_query!(
    attendance,
    event,
    member,
);
