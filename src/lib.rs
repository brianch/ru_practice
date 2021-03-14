pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WordNotFoundError {
    FromUtf8Error(std::string::FromUtf8Error),
    SerdeError(serde_json::Error),
    ParseIntError(std::num::ParseIntError),
    ReqwestError(reqwest::Error),
}

impl From<std::string::FromUtf8Error> for WordNotFoundError {
    fn from(err: std::string::FromUtf8Error) -> WordNotFoundError {
        WordNotFoundError::FromUtf8Error(err)
    }
}

impl From<serde_json::Error> for WordNotFoundError {
    fn from(err: serde_json::Error) -> WordNotFoundError {
        WordNotFoundError::SerdeError(err)
    }
}
impl From<std::num::ParseIntError> for WordNotFoundError {
    fn from(err: std::num::ParseIntError) -> WordNotFoundError {
        WordNotFoundError::ParseIntError(err)
    }
}

impl From<reqwest::Error> for WordNotFoundError {
    fn from(err: reqwest::Error) -> WordNotFoundError {
        WordNotFoundError::ReqwestError(err)
    }
}

impl fmt::Display for WordNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordNotFoundError::FromUtf8Error(ref err) => write!(f, "FromUtf8Error error: {}", err),
            WordNotFoundError::SerdeError(ref err) =>  write!(f, "SerdeError error: {}", err),
            WordNotFoundError::ParseIntError(ref err) =>  write!(f, "ParseIntError error: {}", err),
            WordNotFoundError::ReqwestError(ref err) =>  write!(f, "ReqWestError error: {}", err),
        }
    }
}

impl error::Error for WordNotFoundError {

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            WordNotFoundError::FromUtf8Error(ref err) => Some(err),
            WordNotFoundError::SerdeError(ref err) => Some(err),
            WordNotFoundError::ParseIntError(ref err) => Some(err),
            WordNotFoundError::ReqwestError(ref err) => Some(err),
        }
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))

}