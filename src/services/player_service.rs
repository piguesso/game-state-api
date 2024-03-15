use crate::{
    database::{
        NewPlayer, NewPlayerScoringRound, Player, UpdatePlayer, UpdatePlayerScoring,
        UpdatePlayerScoringRound,
    },
    error::{CustomError, ErrorCodes},
    repository::{
        IPlayerRepository, IPlayerRoundScoringRepository, IPlayerScoringRepository,
        IRoundRepository,
    },
    utils::{
        date::{calculate_date_for_pg, get_current_date_time},
        types::{PlayerScorePerGame, PlayerScorePerRound, PlayerScoringValues},
        HTTPStatusCode,
    },
};

pub trait IPlayerService: Send {
    fn add_player_to_game(&mut self, data: NewPlayer) -> Result<(String, i32), CustomError>;
    fn update_player(
        &mut self,
        player_id: String,
        game_id: i32,
        data: UpdatePlayer,
    ) -> Result<(), CustomError>;
    fn remove_player_from_game(
        &mut self,
        requester_id: String,
        player_id: String,
        game_id: i32,
    ) -> Result<(), CustomError>;
    fn get_player(&mut self, player_id: String, game_id: i32) -> Result<Player, CustomError>;
    fn get_players(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError>;
    fn create_player_scoring_for_round(
        &mut self,
        game_id: i32,
        round_id: i32,
        player_id: String,
    ) -> Result<(), CustomError>;
    fn update_player_scoring_for_round(
        &mut self,
        player_id: String,
        game_id: i32,
        round_id: i32,
        data: PlayerScoringValues,
    ) -> Result<(), CustomError>;
    fn get_player_scoring_for_round(
        &mut self,
        game_id: i32,
        round_id: i32,
        player_id: String,
    ) -> Result<i32, CustomError>;
    fn get_player_score_per_game(
        &mut self,
        game_id: i32,
        player_id: String,
    ) -> Result<PlayerScorePerGame, CustomError>;
    fn save_final_game_score(&mut self, game_id: i32) -> Result<(), CustomError>;
}

pub struct PlayerService {
    player_repo: Box<dyn IPlayerRepository>,
    player_round_scoring_repo: Box<dyn IPlayerRoundScoringRepository>,
    player_scoring_repo: Box<dyn IPlayerScoringRepository>,
    round_repo: Box<dyn IRoundRepository>,
}

impl IPlayerService for PlayerService {
    fn add_player_to_game(&mut self, data: NewPlayer) -> Result<(String, i32), CustomError> {
        match self
            .player_repo
            .fetch_entry((data.player_id.clone(), data.game_id))
        {
            Ok(player) => return Ok((player.player_id, player.game_id)),
            Err(e) => {
                if HTTPStatusCode::from(e.status_code) != HTTPStatusCode::NotFound {
                    return Err(e);
                }
            }
        };

        if let Some(host) = data.is_host {
            if host {
                let host_obj = match self.player_repo.get_host(data.game_id) {
                    Ok(host) => host,
                    Err(e) => return Err(CustomError::new(e.message, e.code, e.status_code)),
                };

                if let Some(_) = host_obj {
                    return Err(CustomError::new(
                        "Host already exists".to_string(),
                        ErrorCodes::InvalidInput,
                        HTTPStatusCode::BadRequest,
                    ));
                }
            }
        }

        match self.player_repo.create(data) {
            Ok(player) => Ok(player),
            Err(e) => Err(e),
        }
    }

    fn update_player(
        &mut self,
        player_id: String,
        game_id: i32,
        data: UpdatePlayer,
    ) -> Result<(), CustomError> {
        if let Some(host_bool) = data.is_host {
            if host_bool {
                let host_obj = match self.player_repo.get_host(game_id) {
                    Ok(host) => host,
                    Err(e) => return Err(CustomError::new(e.message, e.code, e.status_code)),
                };

                if let Some(_) = host_obj {
                    return Err(CustomError::new(
                        "Host already exists".to_string(),
                        ErrorCodes::InvalidInput,
                        HTTPStatusCode::BadRequest,
                    ));
                }
            }
        }

        match self.player_repo.update((player_id, game_id), data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn remove_player_from_game(
        &mut self,
        requester_id: String,
        player_id: String,
        game_id: i32,
    ) -> Result<(), CustomError> {
        let current_date = get_current_date_time();
        let date = match calculate_date_for_pg(current_date) {
            Ok(date) => date,
            Err(e) => return Err(e),
        };

        if requester_id == player_id {
            let data = UpdatePlayer {
                player_id: player_id.clone(),
                game_id,
                is_host: None,
                left_game_at: Some(date),
            };
            match self.player_repo.update((player_id, game_id), data) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            let host = match self.player_repo.get_host(game_id) {
                Ok(host) => host,
                Err(e) => return Err(CustomError::new(e.message, e.code, e.status_code)),
            };

            if let None = host {
                return Err(CustomError::new(
                    String::from("You must be host to remove other players"),
                    ErrorCodes::NotAllowedToPerformAction,
                    HTTPStatusCode::Forbidden,
                ));
            } else {
                let data = UpdatePlayer {
                    player_id: player_id.clone(),
                    game_id,
                    left_game_at: Some(date),
                    is_host: None,
                };

                match self.player_repo.update((player_id, game_id), data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn get_player(&mut self, player_id: String, game_id: i32) -> Result<Player, CustomError> {
        match self.player_repo.fetch_entry((player_id, game_id)) {
            Ok(player) => Ok(player),
            Err(e) => Err(e),
        }
    }

    fn get_players(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError> {
        match self.player_repo.fetch_entries(game_id) {
            Ok(players) => Ok(players),
            Err(e) => Err(e),
        }
    }

    fn create_player_scoring_for_round(
        &mut self,
        game_id: i32,
        round_id: i32,
        player_id: String,
    ) -> Result<(), CustomError> {
        match self
            .player_round_scoring_repo
            .create(NewPlayerScoringRound {
                game_id,
                round_id,
                player_id,
            }) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn update_player_scoring_for_round(
        &mut self,
        player_id: String,
        game_id: i32,
        round_id: i32,
        data: PlayerScoringValues,
    ) -> Result<(), CustomError> {
        let round = match self.round_repo.fetch_entry(round_id) {
            Ok(round) => round,
            Err(e) => return Err(e),
        };

        let time_used = data.time_completed.and_utc().timestamp_micros() - round.start_time.0;

        // TODO calculate score

        let final_data = UpdatePlayerScoringRound {
            game_id,
            round_id,
            player_id: player_id.clone(),
            first_topic: Some(data.first_topic),
            second_topic: Some(data.second_topic),
            third_topic: Some(data.third_topic),
            score: None,
            place: None,
            is_winner: None,
            time_used_to_complete: Some(time_used),
            has_stopped_game: None,
        };

        match self
            .player_round_scoring_repo
            .update((player_id, game_id, round_id), final_data)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn get_player_score_per_game(
        &mut self,
        game_id: i32,
        player_id: String,
    ) -> Result<PlayerScorePerGame, CustomError> {
        let round_scores = match self
            .player_round_scoring_repo
            .fetch_entries_with_player(game_id, player_id.clone())
        {
            Ok(scores) => scores,
            Err(e) => return Err(e),
        };

        let mut total_score = 0;
        let mut score_per_round: Vec<PlayerScorePerRound> = Vec::new();

        for round_score in round_scores {
            total_score += round_score.score;

            let round = match self.round_repo.fetch_entry(round_score.round_id) {
                Ok(round) => round,
                Err(e) => return Err(e),
            };

            let current_round = PlayerScorePerRound {
                round_id: round_score.round_id,
                score: round_score.score,
                position: round_score.place,
                round_number: round.round_number,
            };
            score_per_round.push(current_round);
        }

        let data = PlayerScorePerGame {
            game_id,
            player_id,
            score: total_score,
            score_per_round,
            position: 0,
        };

        Ok(data)
    }

    fn get_player_scoring_for_round(
        &mut self,
        game_id: i32,
        round_id: i32,
        player_id: String,
    ) -> Result<i32, CustomError> {
        match self
            .player_round_scoring_repo
            .fetch_entry((player_id, game_id, round_id))
        {
            Ok(score) => Ok(score.score),
            Err(e) => Err(e),
        }
    }

    fn save_final_game_score(&mut self, game_id: i32) -> Result<(), CustomError> {
        let round_score_entries = match self.player_round_scoring_repo.fetch_entries(game_id) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };

        for entry in round_score_entries {
            let current_player_score = match self
                .player_scoring_repo
                .fetch_entry(entry.player_id.clone())
            {
                Ok(score) => score,
                Err(e) => return Err(e),
            };

            let player_score = UpdatePlayerScoring {
                player_id: entry.player_id.clone(),
                total_xp: Some(current_player_score.total_xp + entry.score),
                games_played: Some(current_player_score.games_played + 1),
                games_won: Some(
                    current_player_score.games_won + if entry.is_winner { 1 } else { 0 },
                ),
                games_lost: Some(
                    current_player_score.games_lost + if entry.is_winner { 0 } else { 1 },
                ),
                highest_score_game: Some(
                    if current_player_score.highest_score_game < entry.score {
                        entry.score
                    } else {
                        current_player_score.highest_score_game
                    },
                ),
                highest_score_round: None,
                games_bottom3: None,
                games_top3: None,
            };

            match self
                .player_scoring_repo
                .update(entry.player_id.clone(), player_score)
            {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
