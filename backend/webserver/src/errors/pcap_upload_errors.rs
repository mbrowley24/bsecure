use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Debug, Error, Serialize, Deserialize)]
pub enum  PcapError{

    #[error("failed to remove file: {0}")]
    FailedToRemoveFile(String),

    #[error("File not saved")]
    FileNotSavedError,

    #[error("File too large.")]
    FileTooLarge,

    #[error("Invalid File Extension.")]
    InvalidExtension,

    #[error("Filename too short")]
    InvalidFilename,

    #[error("Something went wrong.")]
    Other,

    #[error("Something went wrong")]
    IOError,
}

