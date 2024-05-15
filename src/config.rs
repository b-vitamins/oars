use reqwest::{Client};
use std::sync::{Arc, Mutex};
use tracing::{info, warn, instrument};

// Static strings and constants
static BASEPATH: &str = "https://api.openalex.org";
static USERAGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

#[derive(Clone, Debug)]
pub struct OARSConfig {
    pub basepath: String,
    pub useragent: String,
    pub client: Arc<Client>,
    pub email: Option<String>,
    pub apikey: Option<String>,
    pub maxretries: usize,
    pub politeness: f64,
    pub querycount: Arc<Mutex<usize>>,
    pub logornot: bool,
}

impl Default for OARSConfig {
    fn default() -> Self {
        Self {
            basepath: BASEPATH.to_owned(),
            useragent: USERAGENT.to_owned(),
            client: Arc::new(Client::new()),
            email: None,
            apikey: None,
            maxretries: 3,
            politeness: 0.1,
            querycount: Arc::new(Mutex::new(0)),
            logornot: false,
        }
    }
}

impl OARSConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }

    pub fn apikey(mut self, key: String) -> Self {
        self.apikey = Some(key);
        self
    }

    pub fn maxretries(mut self, retries: usize) -> Self {
        self.maxretries = retries;
        self
    }

    pub fn politeness(mut self, backoffrate: f64) -> Self {
        self.politeness = backoffrate;
        self
    }

    pub fn enable_logging(mut self) -> Self {
        self.logornot = true;
        self
    }

    #[instrument]
    pub fn get_querycount(&self) -> usize {
        let count = self.querycount.lock().unwrap();
        if self.logornot {
            info!("Query Count: {}", *count);
        }
        *count
    }

    #[instrument]
    fn bump_querycount(&self) {
        let mut count = self.querycount.lock().unwrap();
        *count += 1;
        if self.logornot {
            info!("Query Count: {} (+1)", *count);
        }
    }

    #[instrument]
    pub fn reset_querycount(&self) {
        let mut count = self.querycount.lock().unwrap();
        *count = 0;
        if self.logornot {
            info!("Query Count reset to 0");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber;

    #[test]
    fn test_configuration_defaults() {
        tracing_subscriber::fmt::init();

        let config = OARSConfig::new();
        assert_eq!(config.basepath, BASEPATH);
        assert_eq!(config.useragent, USERAGENT);
        assert_eq!(config.maxretries, 3);
        assert_eq!(config.politeness, 0.1);
        assert_eq!(config.get_querycount(), 0);
        assert!(!config.logornot);
    }

    #[test]
    fn test_increment_querycount() {

        let config = OARSConfig::new().enable_logging();
        config.bump_querycount();
        assert_eq!(config.get_querycount(), 1);
    }

    #[test]
    fn test_reset_querycount() {

        let config = OARSConfig::new().enable_logging();
        config.bump_querycount();
        config.reset_querycount();
        assert_eq!(config.get_querycount(), 0);
    }
}
