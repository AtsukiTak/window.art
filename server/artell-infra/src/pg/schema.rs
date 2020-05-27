table! {
    artists (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        status_msg -> Text,
        description -> Text,
        instagram -> Text,
        twitter -> Text,
    }
}

table! {
    arts (id) {
        id -> Uuid,
        artist_id -> Uuid,
        title -> Text,
        image_name -> Text,
        portfolio_link -> Text,
        materials -> Text,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
    }
}

table! {
    schedules (id) {
        id -> Int4,
        art_id -> Uuid,
        activate_at -> Timestamptz,
        is_scheduled -> Bool,
    }
}

joinable!(arts -> artists (artist_id));
joinable!(schedules -> arts (art_id));

allow_tables_to_appear_in_same_query!(
    artists,
    arts,
    schedules,
);
