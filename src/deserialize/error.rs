// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DeserializeError<'a> {
    pub message: Cow<'a, str>,
}

impl<'a> DeserializeError<'a> {
    #[cold]
    pub fn new(message: Cow<'a, str>) -> Self {
        DeserializeError { message }
    }
}
