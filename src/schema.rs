table! {
    events (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Nullable<Text>,
        location -> Varchar,
        host -> Varchar,
    }
}

table! {
    invitations (id) {
        id -> Varchar,
        event_id -> Varchar,
        invitee -> Varchar,
    }
}

table! {
    options (id) {
        id -> Integer,
        event_id -> Varchar,
        description -> Text,
        count -> Nullable<Integer>,
    }
}

joinable!(invitations -> events (event_id));
joinable!(options -> events (event_id));

allow_tables_to_appear_in_same_query!(
    events,
    invitations,
    options,
);
