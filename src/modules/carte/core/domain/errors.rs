use std::error::{Error};
use std::fmt;
use std::str::FromStr;


#[derive(Debug, Copy, Clone)]
pub enum RepositoryError {
    ConnectionError,
    NotConnected,
    InsertionError
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::ConnectionError => write!(f,"Error on open a connection with Database"),
            RepositoryError::NotConnected => write!(f,"Database Connection not established"),
            RepositoryError::InsertionError => write!(f,"Error on Insert into Database"),
        }
    }
}

impl Error for RepositoryError {
    fn description(&self) -> &str {
        match self {
            RepositoryError::ConnectionError => "Error on open a connection with Database",
            RepositoryError::NotConnected => "Database Connection not established",
            RepositoryError::InsertionError => "Error on Insert into Database",
        }
    }
}

impl FromStr for RepositoryError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"RepositoryError::ConnectionError\"" => Ok(RepositoryError::ConnectionError),
            "\"RepositoryError::NotConnected\"" => Ok(RepositoryError::NotConnected),
            "\"RepositoryError::InsertionError\"" => Ok(RepositoryError::InsertionError),
            _ => Err(())
        }
    }
}