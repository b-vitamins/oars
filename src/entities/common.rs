use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Authorship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institutions: Option<Vec<Institution>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_corresponding: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_affiliation_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_affiliation_strings: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_author_name: Option<String>,
}

impl Authorship {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Author {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orcid: Option<String>,
}

impl Author {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Institution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Institution {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Apc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_usd: Option<usize>,
}

impl Apc {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_oa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization_lineage: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization_lineage_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_in_doaj: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_oa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<Vec<String>>,
    #[serde(rename = "issn_l")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn_l: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl Source {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Biblio {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_page: Option<String>,
}

impl Biblio {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct YearCount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cited_by_count: Option<usize>,
}

impl YearCount {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PercentileYear {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<usize>,
}

impl PercentileYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Concept {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

impl Concept {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grant {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funder_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub award_id: Option<String>,
}

impl Grant {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Keyword {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

impl Keyword {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MeshTag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_ui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier_ui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_major_topic: Option<bool>,
}

impl MeshTag {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmcid: Option<String>,
}

impl WorkIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_oa: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oa_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oa_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_repository_has_fulltext: Option<bool>,
}

impl OpenAccess {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sdg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

impl Sdg {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<Field>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subfield: Option<Subfield>,
}

impl Topic {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CountByYear {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cited_by_count: Option<i32>,
}

impl CountByYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PublisherIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
}

impl PublisherIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_count: Option<i32>,
}

impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_year_mean_citedness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i10_index: Option<i32>,
}

impl SummaryStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CountsByYear {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cited_by_count: Option<i32>,
}

impl CountsByYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FunderIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
}

impl FunderIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Affiliation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution: Option<DehydratedInstitution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<Vec<i32>>,
}

impl Affiliation {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthorIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orcid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<String>,
}

impl AuthorIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DehydratedInstitution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub institution_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage: Option<Vec<String>>,
}

impl DehydratedInstitution {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DehydratedConcept {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
}

impl DehydratedConcept {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApcPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl ApcPrice {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SourceIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatcat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issn_l: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
}

impl SourceIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Society {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

impl Society {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConceptIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umls_cui: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umls_aui: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<String>,
}

impl ConceptIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InternationalDisplayNames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<HashMap<String, String>>,
}

impl InternationalDisplayNames {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConceptSummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_year_mean_citedness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i10_index: Option<i32>,
}

impl ConceptSummaryStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DehydratedInstitutionWithRelationship {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub institution_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<String>,
}

impl DehydratedInstitutionWithRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Geo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geonames_city_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

impl Geo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstitutionIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ror: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
}

impl InstitutionIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Repository {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_organization_lineage: Option<Vec<String>>,
}

impl Repository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Domain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Domain {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Field {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Field {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Subfield {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl Subfield {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TopicIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openalex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<String>,
}

impl TopicIds {
    pub fn new() -> Self {
        Self::default()
    }
}
