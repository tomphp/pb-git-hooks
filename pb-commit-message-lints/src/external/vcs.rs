use std::{clone::Clone, collections::HashMap, error::Error, string::String};

pub trait Vcs {
    fn get_bool(&self, name: &str) -> Option<bool>;
    fn get_str(&self, name: &str) -> Option<&str>;
    fn get_i64(&self, name: &str) -> Option<i64>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>>;
    /// # Errors
    ///
    /// If the config fails to writ
    fn remove(&mut self, name: &str) -> Result<(), Box<dyn Error>>;
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
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.store
            .get(name)
            .cloned()
            .ok_or_else(|| ())
            .and_then(|x| x.parse().map_err(|_| ()))
            .ok()
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.store.get(name).map(String::as_str)
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.get_string(name)
            .ok_or_else(|| ())
            .and_then(|x| x.parse().map_err(|_| ()))
            .ok()
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.store.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.store.insert(name.into(), format!("{}", value));
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.store.remove(name);
        Ok(())
    }
}

impl InMemory<'_> {
    fn get_string(&self, name: &str) -> Option<String> {
        self.store.get(name).cloned()
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

    fn config_defined(&self, lint_name: &str) -> Result<bool, Box<dyn Error>> {
        self.config_snapshot
            .entries(Some(lint_name))
            .map(|entries| entries.count() > 0)
            .map_err(Box::from)
    }
}

impl Vcs for Git2 {
    fn get_bool(&self, name: &str) -> Option<bool> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config_snapshot.get_bool(name).ok())
    }

    fn get_str(&self, name: &str) -> Option<&str> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config_snapshot.get_str(name).ok())
    }

    fn get_i64(&self, name: &str) -> Option<i64> {
        self.config_defined(name)
            .ok()
            .filter(bool::clone)
            .and_then(|_| self.config_snapshot.get_i64(name).ok())
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Box<dyn Error>> {
        self.config_live
            .set_str(name, value)
            .map_err(Box::from)
            .and_then(|_| {
                self.config_live
                    .snapshot()
                    .map(|config| self.config_snapshot = config)
                    .map_err(Box::from)
            })
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Box<dyn Error>> {
        self.config_live
            .set_i64(name, value)
            .map_err(Box::from)
            .and_then(|_| {
                self.config_live
                    .snapshot()
                    .map(|config| self.config_snapshot = config)
                    .map_err(Box::from)
            })
    }

    fn remove(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        self.config_live
            .remove(name)
            .map_err(Box::from)
            .and_then(|_| {
                self.config_live
                    .snapshot()
                    .map(|config| self.config_snapshot = config)
                    .map_err(Box::from)
            })
    }
}
