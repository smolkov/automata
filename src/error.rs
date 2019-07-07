#![allow(unused_variables)]

use failure::{Fail};
use std::io;
// use std::string::FromUtf8Error;

// use mut_guard::*;

#[derive(Fail, Debug)]
pub enum WqmError {
    #[fail(display = "io error - {}",err)]
    IOError {err: io::Error },

}



impl From<io::Error> for WqmError {
    fn from(kind:io::Error) -> WqmError {
        WqmError::IOError{err: kind}
    }
}


//
