use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;


#[derive(Debug, Error)]
pub enum  PcapError{

    #[error("File not saved")]
    FileNoSavedError,

    #[error("File too large.")]
    FileTooLarge,

    #[error("Invalid File Extension.")]
    InvalidExtension,

    #[error("Filename too short")]
    InvalidFilename,

    #[error("Something went wrong.")]
    Other,

    #[error("Problem with file")]
    IOError,
}

