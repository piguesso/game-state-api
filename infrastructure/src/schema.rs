// @generated automatically by Diesel CLI.

diesel::table! {
    demo (id) {
        id -> Integer,
        clerk_id -> Varchar,
        drawing -> Nullable<Jsonb>,
        term -> Nullable<Integer>,
        guess -> Nullable<Integer>,
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
        id -> Integer,
        user_id -> Varchar,
        friend_id -> Varchar,
        created_at -> Varchar,
    }
}

diesel::table! {
    games (id) {
        id -> Integer,
        status -> Nullable<Varchar>,
        game_slug -> Varchar,
        max_players -> Integer,
        rounds -> Integer,
        winner_id -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    player_scoring (id) {
        id -> Integer,
        player_id -> Varchar,
        total_xp -> Nullable<Integer>,
        highest_score_game -> Nullable<Integer>,
        highest_score_round -> Nullable<Integer>,
        games_played -> Nullable<Integer>,
        games_won -> Nullable<Integer>,
        games_lost -> Nullable<Integer>,
        games_top3 -> Nullable<Integer>,
        games_bottom3 -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    player_scoring_round (id) {
        id -> Integer,
        player_id -> Varchar,
        game_id -> Integer,
        score -> Nullable<Integer>,
        round_id -> Integer,
        place -> Nullable<Integer>,
        is_winner -> Nullable<Bool>,
        time_used_to_complete -> Nullable<BigInt>,
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
        id -> Integer,
        player_id -> Varchar,
        game_id -> Integer,
        is_host -> Nullable<Bool>,
        left_game_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rounds (id) {
        id -> Integer,
        game_id -> Integer,
        round_number -> Integer,
        #[max_length = 100]
        topic -> Varchar,
        start_time -> Timestamp,
        end_time -> Nullable<Timestamp>,
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
