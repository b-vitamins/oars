use crate::entities::common::{DehydratedConcept, ConceptIds, InternationalDisplayNames, ConceptSummaryStats, CountByYear};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Concept {
    ancestors: Vec<DehydratedConcept>,
    cited_by_count: i32,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    description: String,
    display_name: String,
    id: String,
    ids: ConceptIds,
    image_thumbnail_url: Option<String>,
    image_url: Option<String>,
    international: Option<InternationalDisplayNames>,
    level: i32,
    related_concepts: Vec<DehydratedConcept>,
    summary_stats: ConceptSummaryStats,
    updated_date: String,
    wikidata: String,
    works_api_url: String,
    works_count: i32,
}
