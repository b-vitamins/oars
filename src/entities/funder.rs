use serde::{Serialize, Deserialize};
use crate::entities::common::{CountsByYear, FunderIds, Role, SummaryStats};

#[derive(Serialize, Deserialize, Debug)]
pub struct Funder {
    alternate_titles: Vec<String>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    description: String,
    display_name: String,
    grants_count: i32,
    homepage_url: String,
    id: String,
    ids: FunderIds,
    image_thumbnail_url: String,
    image_url: String,
    roles: Vec<Role>,
    summary_stats: SummaryStats,
    updated_date: String,
    works_count: i32,
}