// @generated automatically by Diesel CLI.

diesel::table! {
    experience (id) {
        id -> Integer,
        club_name -> Nullable<Text>,
        starting_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
        martial_artist_id -> Nullable<Integer>,
    }
}

diesel::table! {
    martial_artist (id) {
        id -> Integer,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
    }
}

diesel::table! {
    martial_arts (martialart_name) {
        martialart_name -> Text,
        punches -> Nullable<Integer>,
        kicks -> Nullable<Integer>,
        knees -> Nullable<Integer>,
        elbows -> Nullable<Integer>,
        standup_grappling -> Nullable<Integer>,
        ground_grappling -> Nullable<Integer>,
        ground_n_pound -> Nullable<Integer>,
        trapping -> Nullable<Integer>,
        weapon_manipulation -> Nullable<Integer>,
        weapon_defense -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    experience,
    martial_artist,
    martial_arts,
);
