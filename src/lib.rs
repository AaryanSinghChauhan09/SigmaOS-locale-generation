// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-locale-generation: locale generation component model.
//!
//! Real implementation: locales, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locale {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocaleGenerationError {
    EmptyLocaleName,
    DuplicateLocale(String),
    EmptyEventSubject,
}

impl fmt::Display for LocaleGenerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLocaleName => write!(f, "locale name must not be empty"),
            Self::DuplicateLocale(id) => write!(f, "duplicate locale {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LocaleGenerationError {}

pub struct LocaleGeneration {
    pub name: String,
    locales: BTreeMap<String, Locale>,
    events: BTreeMap<String, Event>,
}

impl LocaleGeneration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            locales: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_locale(&mut self, item: Locale) -> Result<(), LocaleGenerationError> {
        if item.name.trim().is_empty() {
            return Err(LocaleGenerationError::EmptyLocaleName);
        }
        let id = item.id.clone();
        if self.locales.contains_key(&id) {
            return Err(LocaleGenerationError::DuplicateLocale(id.clone()));
        }
        self.locales.insert(id, item);
        Ok(())
    }

    pub fn locale_count(&self) -> usize {
        self.locales.len()
    }

    pub fn get_locale(&self, id: &str) -> Option<&Locale> {
        self.locales.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LocaleGenerationError> {
        if event.subject.trim().is_empty() {
            return Err(LocaleGenerationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_locale(id: &str, name: &str) -> Locale {
        Locale {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = LocaleGeneration::new("LocaleGeneration");
        assert_eq!(m.name, "LocaleGeneration");
        assert_eq!(m.locale_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_locale() {
        let mut m = LocaleGeneration::new("LocaleGeneration");
        m.add_locale(sample_locale("a1", "Alpha")).unwrap();
        assert_eq!(m.locale_count(), 1);
        assert!(m.get_locale("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = LocaleGeneration::new("LocaleGeneration");
        assert!(m.add_locale(sample_locale("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = LocaleGeneration::new("LocaleGeneration");
        m.add_locale(sample_locale("x", "One")).unwrap();
        assert!(m.add_locale(sample_locale("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = LocaleGeneration::new("LocaleGeneration");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
