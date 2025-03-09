// @generated automatically by Diesel CLI.

diesel::table! {
    server_tools (server_id) {
        server_id -> Text,
        tool_data -> Text,
    }
}

diesel::table! {
    server_env (server_id, env_key) {
        server_id -> Text,
        env_key -> Text,
        env_value -> Text,
        env_description -> Text,
        env_required -> Bool,
    }
}

diesel::table! {
    servers (id) {
        id -> Text,
        name -> Text,
        description -> Text,
        tools_type -> Text,
        enabled -> Bool,
        entry_point -> Nullable<Text>,
        command -> Nullable<Text>,
        args -> Nullable<Text>,
        distribution_type -> Nullable<Text>,
        distribution_package -> Nullable<Text>,
    }
}

diesel::joinable!(server_env -> servers (server_id));

diesel::allow_tables_to_appear_in_same_query!(server_tools, server_env, servers,);
