use crate::config::OARSConfig;
use crate::entities::{
    author::Author, funder::Funder, institution::Institution, publisher::Publisher, source::Source,
    topic::Topic, work::Work,
};
use crate::error::OARSError;
use crate::traits::*;
use async_trait::async_trait;
use reqwest::Client;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::time::{interval, Interval};
use tracing::{info, instrument};

#[derive(Clone, Debug)]
pub struct OARSClient {
    config: Arc<OARSConfig>,
    querycount: Arc<AtomicUsize>,
    client: Client,
}

impl OARSClient {
    pub fn new(config: OARSConfig) -> Self {
        let config = Arc::new(config);
        let client = Client::new();
        let oars_client = Self {
            config: config.clone(),
            querycount: Arc::new(AtomicUsize::new(0)),
            client,
        };

        // Spawn the reset task
        let reset_interval = config.resetafter;
        let cloned_client = oars_client.clone();
        tokio::spawn(async move {
            cloned_client.reset_task(interval(reset_interval)).await;
        });

        oars_client
    }

    #[instrument]
    fn get_querycount(&self) -> usize {
        let count = self.querycount.load(Ordering::SeqCst);
        if self.config.logornot {
            info!("Query Count: {}", count);
        }
        count
    }

    #[instrument]
    fn bump_querycount(&self, increment: Option<usize>) -> Result<(), OARSError> {
        let increment = increment.unwrap_or(1);
        let current_count = self.querycount.load(Ordering::SeqCst);
        if current_count + increment > self.config.dailylimit {
            return Err(OARSError::QueryLimitReached);
        }

        self.querycount.fetch_add(increment, Ordering::SeqCst);
        if self.config.logornot {
            info!(
                "Query Count: {} (+{})",
                current_count + increment,
                increment
            );
        }
        Ok(())
    }

    #[instrument]
    fn reset_querycount(&self) {
        self.querycount.store(0, Ordering::SeqCst);
        if self.config.logornot {
            info!("Query Count reset to 0");
        }
    }

    async fn reset_task(&self, mut interval: Interval) {
        loop {
            interval.tick().await;
            self.reset_querycount();
        }
    }
}

#[async_trait]
impl WorkAPIClient for OARSClient {
    async fn get_work_by_id(&self, work_id: &str) -> Result<Work, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/works/{}", self.config.basepath, work_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Work>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl AuthorAPIClient for OARSClient {
    async fn get_author_by_id(&self, author_id: &str) -> Result<Author, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/authors/{}", self.config.basepath, author_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Author>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl FunderAPIClient for OARSClient {
    async fn get_funder_by_id(&self, funder_id: &str) -> Result<Funder, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/funders/{}", self.config.basepath, funder_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Funder>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl InstitutionAPIClient for OARSClient {
    async fn get_institution_by_id(&self, institution_id: &str) -> Result<Institution, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/institutions/{}", self.config.basepath, institution_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Institution>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl PublisherAPIClient for OARSClient {
    async fn get_publisher_by_id(&self, publisher_id: &str) -> Result<Publisher, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/publishers/{}", self.config.basepath, publisher_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Publisher>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl SourceAPIClient for OARSClient {
    async fn get_source_by_id(&self, source_id: &str) -> Result<Source, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/sources/{}", self.config.basepath, source_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Source>()
            .await?;
        Ok(response)
    }
}

#[async_trait]
impl TopicAPIClient for OARSClient {
    async fn get_topic_by_id(&self, topic_id: &str) -> Result<Topic, OARSError> {
        self.bump_querycount(None)?;

        let url = format!("{}/topics/{}", self.config.basepath, topic_id);
        let response = self
            .client
            .get(&url)
            .header("User-Agent", &self.config.useragent)
            .send()
            .await?
            .json::<Topic>()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::OARSConfig;

    #[tokio::test]
    async fn increment_querycount() {
        let config = OARSConfig::new();
        let client = OARSClient::new(config);
        client.bump_querycount(None).unwrap();
        assert_eq!(client.get_querycount(), 1);
    }

    #[tokio::test]
    async fn increment_querycount_with_value() {
        let config = OARSConfig::new();
        let client = OARSClient::new(config);
        client.bump_querycount(Some(5)).unwrap();
        assert_eq!(client.get_querycount(), 5);
    }

    #[tokio::test]
    async fn reset_querycount() {
        let config = OARSConfig::new();
        let client = OARSClient::new(config);
        client.bump_querycount(None).unwrap();
        client.reset_querycount();
        assert_eq!(client.get_querycount(), 0);
    }

    #[tokio::test]
    async fn query_limit() {
        let config = OARSConfig::new().dailylimit(1000);

        let client = OARSClient::new(config);
        for _ in 0..client.config.dailylimit {
            assert!(client.bump_querycount(None).is_ok());
        }
        assert!(client.bump_querycount(None).is_err());
    }

    #[tokio::test]
    async fn test_get_work_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/works/W2741809807")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/work.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let work_id = "W2741809807";
        let work = client
            .get_work_by_id(work_id)
            .await
            .expect("Failed to get work by ID");

        crate::check_entity_against_json!(Work, "testdata/work.json", work);
    }

    #[tokio::test]
    async fn test_get_author_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/authors/A5023888391")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/author.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let author_id = "A5023888391";
        let author = client
            .get_author_by_id(author_id)
            .await
            .expect("Failed to get author by ID");

        crate::check_entity_against_json!(Author, "testdata/author.json", author);
    }

    #[tokio::test]
    async fn test_get_funder_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/funders/F4320332161")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/funder.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let funder_id = "F4320332161";
        let funder = client
            .get_funder_by_id(funder_id)
            .await
            .expect("Failed to get funder by ID");

        crate::check_entity_against_json!(Funder, "testdata/funder.json", funder);
    }

    #[tokio::test]
    async fn test_get_institution_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/institutions/I27837315")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/institution.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let institution_id = "I27837315";
        let institution = client
            .get_institution_by_id(institution_id)
            .await
            .expect("Failed to get institution by ID");

        crate::check_entity_against_json!(Institution, "testdata/institution.json", institution);
    }

    #[tokio::test]
    async fn test_get_publisher_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/publishers/P4310319965")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/publisher.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let publisher_id = "P4310319965";
        let publisher = client
            .get_publisher_by_id(publisher_id)
            .await
            .expect("Failed to get publisher by ID");

        crate::check_entity_against_json!(Publisher, "testdata/publisher.json", publisher);
    }

    #[tokio::test]
    async fn test_get_source_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/sources/S137773608")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/source.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let source_id = "S137773608";
        let source = client
            .get_source_by_id(source_id)
            .await
            .expect("Failed to get source by ID");

        crate::check_entity_against_json!(Source, "testdata/source.json", source);
    }

    #[tokio::test]
    async fn test_get_topic_by_id() {
        let mut server = mockito::Server::new_async().await;
        let _m = server
            .mock("GET", "/topics/T11636")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body_from_file("testdata/topic.json")
            .create();

        let config = OARSConfig::new().basepath(server.url());
        let client = OARSClient::new(config);
        let topic_id = "T11636";
        let topic = client
            .get_topic_by_id(topic_id)
            .await
            .expect("Failed to get topic by ID");

        crate::check_entity_against_json!(Topic, "testdata/topic.json", topic);
    }
}
