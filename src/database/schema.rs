diesel::table! {
    games (id) {
        id -> Serial,
        status -> VarChar,
        winner_id -> Nullable<VarChar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    players (player_id, game_id) {
        player_id -> VarChar,
        game_id -> Integer,
        is_host -> Bool,
        left_game_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    player_scoring (player_id) {
        player_id -> VarChar,
        total_xp -> Integer,
        highest_score_game -> Integer,
        highest_score_round -> Integer,
        games_played -> Integer,
        games_won -> Integer,
        games_lost -> Integer,
        games_top3 -> Integer,
        games_bottom3 -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    rounds (id) {
        id -> Serial,
        game_id -> Integer,
        round_number -> Integer,
        topic -> VarChar,
        start_time -> Timestamp,
        end_time -> Timestamp
    }
}

diesel::table! {
    player_scoring_round (player_id, game_id, round_id) {
        player_id -> VarChar,
        game_id -> Integer,
        round_id -> Integer,
        score -> Integer,
        place -> Integer,
        is_winner -> Bool,
        time_used_to_complete -> BigInt,
        first_topic -> VarChar,
        second_topic -> VarChar,
        third_topic -> VarChar,
        has_stopped_game -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
