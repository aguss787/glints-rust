// @generated automatically by Diesel CLI.

diesel::table! {
    hubbers (id) {
        id -> Uuid,
        code -> Varchar,
        name -> Varchar,
    }
}
