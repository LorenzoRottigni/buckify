// @generated automatically by Diesel CLI.

diesel::table! {
    resources (id) {
        id -> Nullable<Integer>,
        name -> Text,
        slug -> Text,
        path -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        size -> Integer,
        public -> Bool,
    }
}

diesel::table! {
    seaql_migrations (version) {
        version -> Text,
        applied_at -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(resources, seaql_migrations,);
