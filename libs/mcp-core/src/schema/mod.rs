// @generated automatically by Diesel CLI.

diesel::table! {
    server_tools (server_id) {
        server_id -> Text,
        tool_data -> Text,
    }
}

diesel::table! {
    tools (id) {
        id -> Text,
        data -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    server_tools,
    tools,
);
