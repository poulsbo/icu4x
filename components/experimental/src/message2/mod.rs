// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] MessageFormat 2.0 implementation.

use alloc::string::String;

/// A formatter for MessageFormat 2.0 messages.
///
/// See https://unicode-org.github.io/icu/userguide/format_parse/messages/mf2.html.
#[derive(Debug)]
pub struct MessageFormatter;

impl MessageFormatter {
    /// Creates a new [`MessageFormatter`] with a dummy implementation.
    pub fn try_new(_pattern: &str) -> Result<Self, ()> {
        // TODO: Implement minimal parsing to identify functions
        Ok(MessageFormatter)
    }

    /// Formats the message (dummy implementation returning non-matching string).
    pub fn format(&self) -> String {
        String::from("dummy return that matches no test")
    }
}
