use crate::entities::{
    author::Author, funder::Funder, institution::Institution, publisher::Publisher, source::Source,
    topic::Topic, work::Work,
};
use crate::error::OARSError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthorAPIClient {
    async fn get_author_by_id(&self, author_id: &str) -> Result<Author, OARSError>;
}

#[async_trait]
pub trait FunderAPIClient {
    async fn get_funder_by_id(&self, funder_id: &str) -> Result<Funder, OARSError>;
}

#[async_trait]
pub trait InstitutionAPIClient {
    async fn get_institution_by_id(&self, institution_id: &str) -> Result<Institution, OARSError>;
}

#[async_trait]
pub trait PublisherAPIClient {
    async fn get_publisher_by_id(&self, publisher_id: &str) -> Result<Publisher, OARSError>;
}

#[async_trait]
pub trait SourceAPIClient {
    async fn get_source_by_id(&self, source_id: &str) -> Result<Source, OARSError>;
}

#[async_trait]
pub trait TopicAPIClient {
    async fn get_topic_by_id(&self, topic_id: &str) -> Result<Topic, OARSError>;
}

#[async_trait]
pub trait WorkAPIClient {
    async fn get_work_by_id(&self, work_id: &str) -> Result<Work, OARSError>;
}
