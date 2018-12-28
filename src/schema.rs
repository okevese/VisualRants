table! {
    rant_info (id) {
        id -> Integer,
        text -> Text,
        score -> Integer,
        created_time -> Integer,
        num_comments -> Integer,
        tags -> Text,
        vote_state -> Integer,
        edited -> Bool,
        user_username -> Text,
        user_score -> Integer,
    }
}
