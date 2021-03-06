use crate::errors::PbCommitMessageLintsError;
use git2::{Config, Repository};
use serde::export::TryFrom;
use std::{collections::HashMap, path::PathBuf, string::String};

pub trait Vcs {
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a bool
    fn get_bool(&self, name: &str) -> Result<Option<bool>, PbCommitMessageLintsError>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a &str
    fn get_str(&self, name: &str) -> Result<Option<&str>, PbCommitMessageLintsError>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a i64
    fn get_i64(&self, name: &str) -> Result<Option<i64>, PbCommitMessageLintsError>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_str(&mut self, name: &str, value: &str) -> Result<(), PbCommitMessageLintsError>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), PbCommitMessageLintsError>;
    /// # Errors
    ///
    /// If the config fails to writ
    fn remove(&mut self, name: &str) -> Result<(), PbCommitMessageLintsError>;
}

pub struct InMemory<'a> {
    store: &'a mut HashMap<String, String>,
}

impl InMemory<'_> {
    #[must_use]
    pub fn new(store: &mut HashMap<String, String>) -> InMemory {
        InMemory { store }
    }
}

impl Vcs for InMemory<'_> {
    fn get_bool(&self, name: &str) -> Result<Option<bool>, PbCommitMessageLintsError> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse()?)),
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>, PbCommitMessageLintsError> {
        Ok(self.store.get(name).map(String::as_str))
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, PbCommitMessageLintsError> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse()?)),
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), PbCommitMessageLintsError> {
        self.store.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), PbCommitMessageLintsError> {
        self.store.insert(name.into(), format!("{}", value));
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), PbCommitMessageLintsError> {
        self.store.remove(name);
        Ok(())
    }
}

pub struct Git2 {
    config_snapshot: git2::Config,
    config_live: git2::Config,
}

impl Git2 {
    #[must_use]
    pub fn new(mut config: git2::Config) -> Git2 {
        Git2 {
            config_snapshot: config.snapshot().unwrap(),
            config_live: config,
        }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool, PbCommitMessageLintsError> {
        self.config_snapshot
            .entries(Some(lint_name))
            .map(|entries| entries.count() > 0)
            .map_err(PbCommitMessageLintsError::from)
    }
}

impl Vcs for Git2 {
    fn get_bool(&self, name: &str) -> Result<Option<bool>, PbCommitMessageLintsError> {
        if self.config_defined(name)? {
            Ok(Some(self.config_snapshot.get_bool(name)?))
        } else {
            Ok(None)
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>, PbCommitMessageLintsError> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_str(name)
                .map(Some)
                .map_err(PbCommitMessageLintsError::from)
        } else {
            Ok(None)
        }
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, PbCommitMessageLintsError> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_i64(name)
                .map(Some)
                .map_err(PbCommitMessageLintsError::from)
        } else {
            Ok(None)
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), PbCommitMessageLintsError> {
        self.config_live.set_str(name, value)?;

        let config = self.config_live.snapshot()?;

        self.config_snapshot = config;

        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), PbCommitMessageLintsError> {
        self.config_live.set_i64(name, value)?;

        let config = self.config_live.snapshot()?;
        self.config_snapshot = config;

        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), PbCommitMessageLintsError> {
        self.config_live.remove(name)?;

        let config = self.config_live.snapshot()?;
        self.config_snapshot = config;

        Ok(())
    }
}

impl TryFrom<PathBuf> for Git2 {
    type Error = PbCommitMessageLintsError;

    fn try_from(current_dir: PathBuf) -> Result<Self, Self::Error> {
        Repository::discover(current_dir)
            .and_then(|x| x.config())
            .or_else(|_| Config::open_default())
            .map(Git2::new)
            .map_err(PbCommitMessageLintsError::from)
    }
}
