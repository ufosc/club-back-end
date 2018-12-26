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
		description -> Text,
		end_timestamp -> Timestamptz,
		image -> Bytea,
	}
}

table! {
	member (ufl_username) {
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

joinable!(attendance -> event (start_timestamp));
joinable!(attendance -> member (ufl_username));

allow_tables_to_appear_in_same_query!(attendance, event, member,);
