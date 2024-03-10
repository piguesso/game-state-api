mod game_repository;
mod player_repository;
mod round_repository;
mod state_handler;

use crate::error::CustomError;
use crate::types::RequestOptions;

pub trait IRepository<C, U, F, PK, FT> {
    fn create_entry(&mut self, entry: C) -> Result<PK, CustomError>;
    fn delete_entry(&mut self, entry_id: PK) -> Option<CustomError>;
    fn update_entry(&mut self, entry_id: PK, entry: U) -> Result<F, CustomError>;
    fn fetch_entry(&mut self, entry_id: PK) -> Result<F, CustomError>;
    fn fetch_entries(&mut self, request_options: RequestOptions<FT>)
        -> Result<Vec<F>, CustomError>;
}

pub use self::game_repository::*;
pub use self::player_repository::*;
pub use self::round_repository::*;
pub use self::state_handler::*;
