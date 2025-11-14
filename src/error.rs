use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, PartialEq, Debug)]
pub enum AppError {
    NotFound,
    InternalError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "not found"),
            AppError::InternalError => write!(f, "internal server error"),
        }
    }
}
