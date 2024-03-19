// @generated automatically by Diesel CLI.

diesel::table! {
    demo (id) {
        id -> Int4,
        clerk_id -> Varchar,
        drawing -> Nullable<Jsonb>,
        term -> Nullable<Int4>,
        guess -> Nullable<Int4>,
        term_confidence -> Nullable<Numeric>,
        host -> Bool,
        image_url -> Nullable<Text>,
        username -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    friends (id) {
        id -> Int4,
        user_id -> Varchar,
        friend_id -> Varchar,
        created_at -> Varchar,
    }
}

diesel::table! {
    games (id) {
        id -> Int4,
        status -> Nullable<Varchar>,
        game_slug -> Varchar,
        winner_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    player_scoring (id) {
        id -> Int4,
        player_id -> Varchar,
        total_xp -> Nullable<Int4>,
        highest_score_game -> Nullable<Int4>,
        highest_score_round -> Nullable<Int4>,
        games_played -> Nullable<Int4>,
        games_won -> Nullable<Int4>,
        games_lost -> Nullable<Int4>,
        games_top3 -> Nullable<Int4>,
        games_bottom3 -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    player_scoring_round (id) {
        id -> Int4,
        player_id -> Varchar,
        game_id -> Int4,
        score -> Nullable<Int4>,
        round_id -> Int4,
        place -> Nullable<Int4>,
        is_winner -> Nullable<Bool>,
        time_used_to_complete -> Nullable<Int4>,
        #[max_length = 100]
        first_topic -> Nullable<Varchar>,
        #[max_length = 100]
        second_topic -> Nullable<Varchar>,
        #[max_length = 100]
        third_topic -> Nullable<Varchar>,
        has_stopped_game -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        player_id -> Varchar,
        game_id -> Int4,
        is_host -> Nullable<Bool>,
        left_game_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rounds (id) {
        id -> Int4,
        game_id -> Int4,
        round_number -> Int4,
        #[max_length = 100]
        topic -> Varchar,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

diesel::table! {
    training (clerk_id) {
        clerk_id -> Varchar,
        drawing -> Jsonb,
    }
}

diesel::table! {
    users (clerk_id) {
        clerk_id -> Varchar,
        #[max_length = 1000]
        biography -> Nullable<Varchar>,
        tag -> Varchar,
    }
}

diesel::joinable!(players -> users (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    demo,
    friends,
    games,
    player_scoring,
    player_scoring_round,
    players,
    rounds,
    training,
    users,
);
