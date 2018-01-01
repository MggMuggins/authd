use std::result;

use failure::Error;
use syscall::error::Error as SyscallError;

pub type Result<T> = result::Result<T , Error>;

/// Errors that might happen while using this crate
#[derive(Debug, Fail, PartialEq)]
pub enum UsersError {
    #[fail(display = "os error: code {}", reason)]
    Os { reason: String },
    #[fail(display = "parse error: {}", reason)]
    Parsing { reason: String },
    #[fail(display = "user/group not found")]
    NotFound,
    #[fail(display = "user/group already exists")]
    AlreadyExists,
}

pub fn parse_error(reason: &str) -> UsersError {
    UsersError::Parsing { reason: reason.into() }
}

pub fn os_error(reason: &str) -> UsersError {
    UsersError::Os { reason: reason.into() }
}

impl From<SyscallError> for UsersError {
    fn from(syscall_error: SyscallError) -> UsersError {
        UsersError::Os { reason: format!("{}", syscall_error) }
    }
}
