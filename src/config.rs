use tokio::time::Duration;

static BASEPATH: &str = "https://api.openalex.org";
static USERAGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Clone, Debug)]
pub struct OARSConfig {
    pub basepath: String,
    pub useragent: String,
    pub email: Option<String>,
    pub apikey: Option<String>,
    pub maxretries: usize,
    pub politeness: f64,
    pub logornot: bool,
    pub dailylimit: usize,
    pub resetafter: Duration,
}

impl Default for OARSConfig {
    fn default() -> Self {
        Self {
            basepath: BASEPATH.to_owned(),
            useragent: USERAGENT.to_owned(),
            email: None,
            apikey: None,
            maxretries: 3,
            politeness: 0.1,
            logornot: false,
            dailylimit: 100_000,
            resetafter: Duration::from_secs(60 * 60 * 24),
        }
    }
}

impl OARSConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn basepath(mut self, basepath: String) -> Self {
        self.basepath = basepath;
        self
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

    pub fn dailylimit(mut self, limit: usize) -> Self {
        self.dailylimit = limit;
        self
    }

    pub fn resetafter(mut self, interval: Duration) -> Self {
        self.resetafter = interval;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::init_logging;

    #[test]
    fn test_configuration_defaults() {
        let config = OARSConfig::new();
        assert_eq!(config.basepath, BASEPATH);
        assert_eq!(config.useragent, USERAGENT);
        assert_eq!(config.maxretries, 3);
        assert_eq!(config.politeness, 0.1);
        assert_eq!(config.resetafter, Duration::from_secs(60 * 60 * 24));
        assert!(!config.logornot);
    }

    #[test]
    fn test_configuration_with_options() {
        init_logging();
        let config = OARSConfig::new()
            .email("test@example.com".to_string())
            .apikey("testapikey".to_string())
            .maxretries(5)
            .politeness(0.2)
            .enable_logging()
            .resetafter(Duration::from_secs(60 * 60 * 12)); // 12 hours

        assert_eq!(config.email, Some("test@example.com".to_string()));
        assert_eq!(config.apikey, Some("testapikey".to_string()));
        assert_eq!(config.maxretries, 5);
        assert_eq!(config.politeness, 0.2);
        assert!(config.logornot);
        assert_eq!(config.resetafter, Duration::from_secs(60 * 60 * 12));
    }
}
