// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        wiki_id -> Int4,
        title -> Text,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        id_session -> Int8,
        t_email -> Varchar,
        t_password -> Varchar,
        t_ip_address -> Varchar,
        d_visit_first -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
