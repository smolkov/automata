#![allow(unused_variables)]

use failure::{Fail};
use std::io;
// use std::string::FromUtf8Error;

// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum ApiError {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

}

impl From<io::Error> for ApiError {
    fn from(kind:io::Error) -> ApiError {
        ApiError::IOError{err: kind}
    }
}
