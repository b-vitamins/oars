use serde::{Serialize, Deserialize};
use crate::entities::common::{CountByYear, PublisherIds, Role, SummaryStats};

#[derive(Serialize, Deserialize, Debug)]
pub struct Publisher {
    alternate_titles: Vec<String>,
    cited_by_count: i64,
    country_codes: Vec<String>,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    hierarchy_level: i32,
    id: String,
    ids: PublisherIds,
    image_thumbnail_url: String,
    image_url: String,
    lineage: Vec<String>,
    parent_publisher: Option<String>,
    roles: Vec<Role>,
    sources_api_url: String,
    summary_stats: SummaryStats,
    updated_date: String,
    works_count: i32,
}
