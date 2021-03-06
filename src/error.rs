//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Error module, containing error types

use std::error::Error;
use std::fmt::Error as FmtError;
use std::clone::Clone;
use std::fmt::{Display, Formatter};

///
/// Kind of task error
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TaskErrorKind {
    /// Error kind indicating that the JSON parser failed
    ParserError,

    /// Error kind indicating that the Status of a task is missing
    NoStatus,

    /// Error kind indicating that the Reader failed to read something
    ReaderError,
}

fn store_error_type_as_str(e: &TaskErrorKind) -> &'static str {
    match e {
        &TaskErrorKind::ParserError => "Parser Error",
        &TaskErrorKind::NoStatus    => "Task status is missing",
        &TaskErrorKind::ReaderError => "Reader Error",
    }
}

impl Display for TaskErrorKind {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "{}", store_error_type_as_str(self)));
        Ok(())
    }

}

///
/// Task error type
///
#[derive(Debug)]
pub struct TaskError {
    err_type: TaskErrorKind,
    cause: Option<Box<Error>>,
}

impl TaskError {

    ///
    /// Build a new TaskError from an TaskErrorKind, optionally with cause
    ///
    pub fn new(errtype: TaskErrorKind, cause: Option<Box<Error>>)
        -> TaskError
        {
            TaskError {
                err_type: errtype,
                cause: cause,
            }
        }

    ///
    /// Get the error type of this TaskError
    ///
    pub fn err_type(&self) -> TaskErrorKind {
        self.err_type.clone()
    }

}

impl Display for TaskError {

    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        try!(write!(fmt, "[{}]", store_error_type_as_str(&self.err_type.clone())));
        Ok(())
    }

}

impl Error for TaskError {

    fn description(&self) -> &str {
        store_error_type_as_str(&self.err_type.clone())
    }

    fn cause(&self) -> Option<&Error> {
        self.cause.as_ref().map(|e| &**e)
    }

}

