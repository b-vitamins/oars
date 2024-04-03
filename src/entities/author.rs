use crate::entities::common::{Affiliation, AuthorIds, DehydratedInstitution, DehydratedConcept, SummaryStats, CountByYear};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    affiliations: Vec<Affiliation>,
    cited_by_count: i32,
    counts_by_year: Vec<CountByYear>,
    created_date: String,
    display_name: String,
    display_name_alternatives: Vec<String>,
    id: String,
    ids: AuthorIds,
    last_known_institution: Option<DehydratedInstitution>,
    last_known_institutions: Vec<DehydratedInstitution>,
    orcid: Option<String>,
    summary_stats: SummaryStats,
    updated_date: String,
    works_api_url: String,
    works_count: i32,
    x_concepts: Vec<DehydratedConcept>,
}