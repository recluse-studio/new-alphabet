use serde::{Deserialize, Serialize};

use crate::{Severity, ValidationCategory};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationMessage {
    pub rule_id: String,
    pub severity: Severity,
    pub category: ValidationCategory,
    pub target: String,
    pub message: String,
    pub repair: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationReport {
    pub valid: bool,
    pub messages: Vec<ValidationMessage>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            valid: true,
            messages: Vec::new(),
        }
    }

    pub fn push(&mut self, message: ValidationMessage) {
        if matches!(message.severity, Severity::Error) {
            self.valid = false;
        }

        self.messages.push(message);
    }
}
