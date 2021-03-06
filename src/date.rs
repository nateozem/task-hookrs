//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

//! Module for wrapping chrono::naive::datetime::NaiveDateTime

use std::error::Error;
use std::ops::{Deref, DerefMut};

use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::de::Visitor;
use serde::de::Error as SerdeError;
use chrono::naive::datetime::NaiveDateTime;

/// Date is a NaiveDateTime-Wrapper object to be able to implement foreign traits on it
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Date(NaiveDateTime);

impl Deref for Date {
    type Target = NaiveDateTime;

    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }

}

impl DerefMut for Date {

    fn deref_mut(&mut self) -> &mut NaiveDateTime {
        &mut self.0
    }

}

impl From<NaiveDateTime> for Date {

    fn from(ndt: NaiveDateTime) -> Date {
        Date(ndt)
    }

}

/// The date-time parsing template used to parse the date time data exported by taskwarrior.
pub static TASKWARRIOR_DATETIME_TEMPLATE : &'static str = "%Y%m%dT%H%M%SZ";

impl Serialize for Date {

    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer
    {
        let formatted = self.0.format(TASKWARRIOR_DATETIME_TEMPLATE);
        serializer.serialize_str(&format!("{}", formatted))
    }

}

impl Deserialize for Date {

    fn deserialize<D>(deserializer: &mut D) -> Result<Date, D::Error>
        where D: Deserializer
    {
        struct DateVisitor;

        impl Visitor for DateVisitor {
            type Value = Date;

            fn visit_str<E>(&mut self, value: &str) -> Result<Date, E>
                where E: SerdeError
            {
                NaiveDateTime::parse_from_str(value, TASKWARRIOR_DATETIME_TEMPLATE)
                    .map(|d| Date(d))
                    .map_err(|e| SerdeError::custom(e.description()))
            }
        }

        deserializer.deserialize(DateVisitor)
    }

}
