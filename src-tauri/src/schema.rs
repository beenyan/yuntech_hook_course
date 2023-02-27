// @generated automatically by Diesel CLI.

diesel::table! {
    cookies (id) {
        id -> Integer,
        name -> Text,
        value -> Text,
    }
}
