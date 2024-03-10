use crate::error::CustomError;
use redis::{Commands, Connection};

pub trait IStateHandler {
    fn write_value(&mut self, key: String, value: String) -> Option<CustomError>;
    fn read_value(&mut self, key: String) -> Result<String, CustomError>;
    fn delete_value(&mut self, key: String) -> Option<CustomError>;
    fn does_key_exist(&mut self, key: String) -> Result<bool, CustomError>;
    fn add_to_set(&mut self, key: String, value: String) -> Option<CustomError>;
    fn remove_from_set(&mut self, key: String, value: String) -> Option<CustomError>;
    fn get_members_of_set(&mut self, key: String) -> Result<Vec<String>, CustomError>;
    fn is_member_of_set(&mut self, key: String, value: String) -> Result<bool, CustomError>;
}

pub fn new_state_handler(connection: Connection) -> StateHandler {
    StateHandler {
        state_connection: connection,
    }
}

pub struct StateHandler {
    state_connection: redis::Connection,
}

impl IStateHandler for StateHandler {
    fn write_value(&mut self, key: String, value: String) -> Option<CustomError> {
        match self
            .state_connection
            .set::<String, String, String>(key, value)
        {
            Ok(_) => None,
            Err(_) => Some(CustomError::InternalServerError(None)),
        }
    }

    fn read_value(&mut self, key: String) -> Result<String, CustomError> {
        match self.state_connection.get::<String, String>(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(CustomError::InternalServerError(None)),
        }
    }

    fn delete_value(&mut self, key: String) -> Option<CustomError> {
        match self.state_connection.del::<String, bool>(key) {
            Ok(_) => None,
            Err(_) => Some(CustomError::InternalServerError(None)),
        }
    }

    fn does_key_exist(&mut self, key: String) -> Result<bool, CustomError> {
        match self.state_connection.exists::<String, bool>(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(CustomError::InternalServerError(None)),
        }
    }

    fn add_to_set(&mut self, key: String, value: String) -> Option<CustomError> {
        match self
            .state_connection
            .sadd::<String, String, bool>(key, value)
        {
            Ok(_) => None,
            Err(_) => Some(CustomError::InternalServerError(None)),
        }
    }

    fn remove_from_set(&mut self, key: String, value: String) -> Option<CustomError> {
        match self
            .state_connection
            .srem::<String, String, bool>(key, value)
        {
            Ok(_) => None,
            Err(_) => Some(CustomError::InternalServerError(None)),
        }
    }

    fn get_members_of_set(&mut self, key: String) -> Result<Vec<String>, CustomError> {
        match self.state_connection.smembers::<String, Vec<String>>(key) {
            Ok(val) => Ok(val),
            Err(_) => Err(CustomError::InternalServerError(None)),
        }
    }

    fn is_member_of_set(&mut self, key: String, value: String) -> Result<bool, CustomError> {
        match self
            .state_connection
            .sismember::<String, String, bool>(key, value)
        {
            Ok(val) => Ok(val),
            Err(_) => Err(CustomError::InternalServerError(None)),
        }
    }
}
