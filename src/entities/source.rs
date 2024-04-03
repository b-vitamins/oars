use crate::entities::common::{ApcPrice, CountsByYear, SourceIds, Society, SummaryStats, SourceType, DehydratedConcept};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    abbreviated_title: String,
    alternate_titles: Vec<String>,
    apc_prices: Vec<ApcPrice>,
    apc_usd: Option<i32>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    display_name: String,
    homepage_url: String,
    host_organization: String,
    host_organization_lineage: Vec<String>,
    host_organization_name: String,
    id: String,
    ids: SourceIds,
    image_thumbnail_url: String,
    image_url: String,
    is_in_doaj: bool,
    is_oa: bool,
    issn: Vec<String>,
    issn_l: String,
    societies: Vec<Society>,
    summary_stats: SummaryStats,
    #[serde(rename = "type")]
    source_type: SourceType,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}
