// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Int4,
        wiki_id -> Int4,
        title -> Text,
        content -> Text,
    }
}
