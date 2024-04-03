use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorship {
    pub author_position: Option<AuthorPosition>,
    pub author: Author,
    pub institutions: Vec<Institution>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthorPosition {
    First,
    Middle,
    Last,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: String,
    pub display_name: String,
    pub orcid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Institution {
    pub id: String,
    pub display_name: String,
    pub ror: Option<String>,
    pub country_code: Option<String>,
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apc {
    pub value: usize,
    pub currency: String,
    pub provenance: Option<String>,
    pub value_usd: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub is_oa: bool,
    pub landing_page_url: String,
    pub pdf_url: Option<String>,
    pub source: Source,
    pub license: Option<License>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub id: String,
    pub display_name: String,
    pub issn_l: Option<String>,
    pub issn: Option<Vec<String>>,
    pub host_organization: Option<String>,
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Biblio {
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YearCount {
    pub year: usize,
    pub cited_by_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Concept {
    pub id: String,
    pub wikidata: Option<String>,
    pub display_name: String,
    pub level: usize,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grant {
    pub funder: String,
    pub funder_display_name: String,
    pub award_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyword {
    pub keyword: String,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeshTag {
    pub descriptor_ui: String,
    pub descriptor_name: String,
    pub qualifier_ui: Option<String>,
    pub qualifier_name: Option<String>,
    pub is_major_topic: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkIds {
    pub openalex: String,
    pub doi: Option<String>,
    pub mag: Option<usize>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAccess {
    pub is_oa: bool,
    pub oa_status: OaStatus,
    pub oa_url: Option<String>,
    pub any_repository_has_fulltext: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OaStatus {
    Gold,
    Green,
    Hybrid,
    Bronze,
    Closed,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum License {
    CcBy,
    CcBySa,
    CcByNd,
    CcByNc,
    CcByNcSa,
    CcByNcNd,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sdg {
    pub id: String,
    pub display_name: String,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub id: String,
    pub display_name: String,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WorkType {
    Article,
    Preprint,
    Book,
    Chapter,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountByYear {
    pub year: i32,
    pub works_count: i32,
    pub cited_by_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublisherIds {
    pub openalex: String,
    pub ror: Option<String>,
    pub wikidata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub role: String,
    pub id: String,
    pub works_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    pub two_year_mean_citedness: f64,
    pub h_index: i32,
    pub i10_index: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountsByYear {
    pub year: i32,
    pub works_count: i32,
    pub cited_by_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunderIds {
    pub crossref: Option<String>,
    pub doi: Option<String>,
    pub openalex: String,
    pub ror: Option<String>,
    pub wikidata: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Affiliation {
    pub institution: DehydratedInstitution,
    pub years: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthorIds {
    pub openalex: String,
    pub orcid: Option<String>,
    pub scopus: Option<String>,
    pub twitter: Option<String>,
    pub wikipedia: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DehydratedInstitution {
    pub id: String,
    pub ror: Option<String>,
    pub display_name: String,
    pub country_code: Option<String>,
    pub type_: Option<String>,
    pub lineage: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DehydratedConcept {
    pub id: String,
    pub wikidata: Option<String>,
    pub display_name: String,
    pub level: Option<i32>,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApcPrice {
    pub price: i32,
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SourceIds {
    pub fatcat: Option<String>,
    pub issn: Vec<String>,
    pub issn_l: String,
    pub mag: Option<i64>,
    pub openalex: String,
    pub wikidata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Society {
    pub url: String,
    pub organization: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SourceType {
    Journal,
    Repository,
    Conference,
    EbookPlatform,
    BookSeries,
    #[serde(other)]
    Unknown, // For handling any unknown or new types that might appear in the future
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConceptIds {
    pub mag: Option<i64>,
    pub openalex: String,
    pub umls_cui: Option<Vec<String>>,
    pub umls_aui: Option<Vec<String>>,
    pub wikidata: String,
    pub wikipedia: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternationalDisplayNames {
    pub display_name: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConceptSummaryStats {
    #[serde(rename = "2yr_mean_citedness")]
    pub two_year_mean_citedness: f64,
    pub h_index: i32,
    pub i10_index: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DehydratedInstitutionWithRelationship {
    id: String,
    ror: Option<String>,
    display_name: String,
    country_code: String,
    #[serde(rename = "type")]
    institution_type: String,
    relationship: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geo {
    city: String,
    geonames_city_id: String,
    region: String,
    country_code: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstitutionIds {
    grid: Option<String>,
    mag: Option<i64>,
    openalex: String,
    ror: String,
    wikipedia: Option<String>,
    wikidata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    id: String,
    display_name: String,
    host_organization: String,
    host_organization_name: String,
    host_organization_lineage: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Domain {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subfield {
    id: i32,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicIds {
    openalex: String,
    wikipedia: Option<String>,
}