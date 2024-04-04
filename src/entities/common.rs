use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Authorship {
    pub author: Author,
    pub author_position: AuthorPosition,
    pub countries: Vec<String>,
    pub institutions: Vec<Institution>,
    pub is_corresponding: bool,
    pub raw_affiliation_string: String,
    pub raw_affiliation_strings: Vec<String>,
    pub raw_author_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthorPosition {
    #[serde(rename = "first")]
    First,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "last")]
    Last,
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
    pub lineage: Vec<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Apc {
    pub value: usize,
    pub currency: String,
    pub provenance: Option<String>,
    pub value_usd: usize,
}

// Represents the best available open access location for a work
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub is_accepted: bool,
    pub is_oa: bool,
    pub is_published: bool,
    pub landing_page_url: Option<String>,
    pub license: Option<License>,
    pub pdf_url: Option<String>,
    pub source: Source,
    pub version: Option<Version>,
}

// Represents the source of the OA location
#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub display_name: Option<String>,
    pub host_organization: Option<String>,
    pub host_organization_lineage: Option<Vec<String>>,
    pub host_organization_lineage_names: Option<Vec<String>>,
    pub host_organization_name: Option<String>,
    pub id: Option<String>,
    pub is_in_doaj: bool,
    pub is_oa: bool,
    pub issn: Option<Vec<String>>,
    #[serde(rename = "issn_l")]
    pub issn_l: Option<String>,
    #[serde(rename = "type")]
    pub type_: SourceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum License {
    #[serde(rename = "cc-by")]
    CcBy,
    #[serde(rename = "cc-by-sa")]
    CcBySa,
    #[serde(rename = "cc-by-nd")]
    CcByNd,
    #[serde(rename = "cc-by-nc")]
    CcByNc,
    #[serde(rename = "cc-by-nc-sa")]
    CcByNcSa,
    #[serde(rename = "cc-by-nc-nd")]
    CcByNcNd,
    #[serde(other)]
    Unknown,
}

// Enum to represent version precedence
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    #[serde(rename = "submittedVersion")]
    SubmittedVersion,
    #[serde(rename = "acceptedVersion")]
    AcceptedVersion,
    #[serde(rename = "publishedVersion")]
    PublishedVersion,
    // Extend with other versions if necessary
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
pub struct PercentileYear {
    pub max: usize,
    pub min: usize,
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
    pub mag: Option<String>,
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
#[serde(rename_all = "lowercase")]
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
pub struct Sdg {
    pub id: String,
    pub display_name: String,
    pub score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    pub display_name: Option<String>,
    pub domain: Option<Domain>,
    pub field: Option<Field>,
    pub id: Option<String>,
    pub score: Option<f64>,
    pub subfield: Option<Subfield>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum WorkType {
    Article,
    Preprint,
    Book,
    Chapter,
    #[serde(other)]
    Unknown, // To capture any unknown types gracefully
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orcid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopus: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikipedia: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DehydratedInstitution {
    pub id: String,
    pub ror: Option<String>,
    pub display_name: String,
    pub country_code: Option<String>,
    #[serde(rename = "type")]
    pub institution_type: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatcat: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub issn: Vec<String>,
    pub issn_l: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<String>,
    pub openalex: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Society {
    pub url: String,
    pub organization: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SourceType {
    #[serde(rename = "journal")]
    Journal,
    #[serde(rename = "repository")]
    Repository,
    #[serde(rename = "publisher")]
    Publisher,
    #[serde(rename = "conference")]
    Conference,
    #[serde(rename = "ebook-platform")]
    EbookPlatform,
    #[serde(rename = "book-series")]
    BookSeries,
    #[serde(other)]
    Unknown,
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
    region: Option<String>,
    country_code: String,
    country: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstitutionIds {
    grid: Option<String>,
    mag: Option<String>,
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
    id: String,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    id: Option<String>,
    display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subfield {
    id: Option<String>,
    display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopicIds {
    openalex: String,
    wikipedia: Option<String>,
}
