use breachlet_core::error::{AppError, AppResult};
use garde::{Report, rules::email::parse_email};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn parse(value: String) -> AppResult<Self> {
        if let Err(inner) = parse_email(value.as_str()) {
            let mut report = Report::new();
            report.append(
                garde::Path::new("email"), //
                garde::Error::new(inner.to_string()),
            );

            return Err(AppError::Validation(report));
        }

        Ok(Self(value))
    }

    pub fn parse_str(value: &str) -> AppResult<Self> {
        Self::parse(value.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl Display for Email {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
