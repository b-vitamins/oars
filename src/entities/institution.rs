use crate::entities::common::{DehydratedInstitutionWithRelationship, CountsByYear, Geo, InstitutionIds, Repository, Role, SummaryStats, DehydratedConcept};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Institution {
    associated_institutions: Vec<DehydratedInstitutionWithRelationship>,
    cited_by_count: i64,
    country_code: String,
    counts_by_year: Vec<CountsByYear>,
    created_date: String,
    display_name: String,
    display_name_acronyms: Vec<String>,
    display_name_alternatives: Vec<String>,
    geo: Geo,
    homepage_url: String,
    id: String,
    ids: InstitutionIds,
    image_thumbnail_url: String,
    image_url: String,
    international: HashMap<String, String>,
    lineage: Vec<String>,
    repositories: Vec<Repository>,
    roles: Vec<Role>,
    ror: String,
    summary_stats: SummaryStats,
    #[serde(rename = "type")]
    institution_type: String,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}
