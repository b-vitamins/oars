use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Authorship {
    pub author: Option<Author>,
    pub author_position: Option<AuthorPosition>,
    pub countries: Option<Vec<String>>,
    pub institutions: Option<Vec<Institution>>,
    pub is_corresponding: Option<bool>,
    pub raw_affiliation_string: Option<String>,
    pub raw_affiliation_strings: Option<Vec<String>>,
    pub raw_author_name: Option<String>,
}

impl Authorship {
    pub fn new() -> Self {
        Self::default()
    }
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Author {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub orcid: Option<String>,
}

impl Author {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Institution {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub ror: Option<String>,
    pub country_code: Option<String>,
    pub lineage: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

impl Institution {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Apc {
    pub value: Option<usize>,
    pub currency: Option<String>,
    pub provenance: Option<String>,
    pub value_usd: Option<usize>,
}

impl Apc {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    pub is_accepted: Option<bool>,
    pub is_oa: Option<bool>,
    pub is_published: Option<bool>,
    pub landing_page_url: Option<String>,
    pub license: Option<License>,
    pub pdf_url: Option<String>,
    pub source: Option<Source>,
    pub version: Option<Version>,
}

impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Source {
    pub display_name: Option<String>,
    pub host_organization: Option<String>,
    pub host_organization_lineage: Option<Vec<String>>,
    pub host_organization_lineage_names: Option<Vec<String>>,
    pub host_organization_name: Option<String>,
    pub id: Option<String>,
    pub is_in_doaj: Option<bool>,
    pub is_oa: Option<bool>,
    pub issn: Option<Vec<String>>,
    #[serde(rename = "issn_l")]
    pub issn_l: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<SourceType>,
}

impl Source {
    pub fn new() -> Self {
        Self::default()
    }
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    #[serde(rename = "submittedVersion")]
    SubmittedVersion,
    #[serde(rename = "acceptedVersion")]
    AcceptedVersion,
    #[serde(rename = "publishedVersion")]
    PublishedVersion,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Biblio {
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub first_page: Option<String>,
    pub last_page: Option<String>,
}

impl Biblio {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct YearCount {
    pub year: Option<usize>,
    pub cited_by_count: Option<usize>,
}

impl YearCount {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PercentileYear {
    pub max: Option<usize>,
    pub min: Option<usize>,
}

impl PercentileYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Concept {
    pub id: Option<String>,
    pub wikidata: Option<String>,
    pub display_name: Option<String>,
    pub level: Option<usize>,
    pub score: Option<f64>,
}

impl Concept {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grant {
    pub funder: Option<String>,
    pub funder_display_name: Option<String>,
    pub award_id: Option<String>,
}

impl Grant {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Keyword {
    pub keyword: Option<String>,
    pub score: Option<f64>,
}

impl Keyword {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MeshTag {
    pub descriptor_ui: Option<String>,
    pub descriptor_name: Option<String>,
    pub qualifier_ui: Option<String>,
    pub qualifier_name: Option<String>,
    pub is_major_topic: Option<bool>,
}

impl MeshTag {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WorkIds {
    pub openalex: Option<String>,
    pub doi: Option<String>,
    pub mag: Option<String>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
}

impl WorkIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OpenAccess {
    pub is_oa: Option<bool>,
    pub oa_status: Option<OaStatus>,
    pub oa_url: Option<String>,
    pub any_repository_has_fulltext: Option<bool>,
}

impl OpenAccess {
    pub fn new() -> Self {
        Self::default()
    }
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Sdg {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub score: Option<f64>,
}

impl Sdg {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    pub display_name: Option<String>,
    pub domain: Option<Domain>,
    pub field: Option<Field>,
    pub id: Option<String>,
    pub score: Option<f64>,
    pub subfield: Option<Subfield>,
}

impl Topic {
    pub fn new() -> Self {
        Self::default()
    }
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CountByYear {
    pub year: Option<i32>,
    pub works_count: Option<i32>,
    pub cited_by_count: Option<i32>,
}

impl CountByYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PublisherIds {
    pub openalex: Option<String>,
    pub ror: Option<String>,
    pub wikidata: Option<String>,
}

impl PublisherIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Role {
    pub role: Option<String>,
    pub id: Option<String>,
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
    pub two_year_mean_citedness: Option<f64>,
    pub h_index: Option<i32>,
    pub i10_index: Option<i32>,
}

impl SummaryStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CountsByYear {
    pub year: Option<i32>,
    pub works_count: Option<i32>,
    pub cited_by_count: Option<i32>,
}

impl CountsByYear {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FunderIds {
    pub crossref: Option<String>,
    pub doi: Option<String>,
    pub openalex: Option<String>,
    pub ror: Option<String>,
    pub wikidata: Option<String>,
}

impl FunderIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Affiliation {
    pub institution: Option<DehydratedInstitution>,
    pub years: Option<Vec<i32>>,
}

impl Affiliation {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AuthorIds {
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
    pub id: Option<String>,
    pub ror: Option<String>,
    pub display_name: Option<String>,
    pub country_code: Option<String>,
    #[serde(rename = "type")]
    pub institution_type: Option<String>,
    pub lineage: Option<Vec<String>>,
}

impl DehydratedInstitution {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DehydratedConcept {
    pub id: Option<String>,
    pub wikidata: Option<String>,
    pub display_name: Option<String>,
    pub level: Option<i32>,
    pub score: Option<f64>,
}

impl DehydratedConcept {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApcPrice {
    pub price: Option<i32>,
    pub currency: Option<String>,
}

impl ApcPrice {
    pub fn new() -> Self {
        Self::default()
    }
}

fn is_none_or_empty<T>(opt: &Option<Vec<T>>) -> bool {
    match opt {
        Some(vec) => vec.is_empty(),
        None => true,
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SourceIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fatcat: Option<String>,
    #[serde(skip_serializing_if = "is_none_or_empty", default)]
    pub issn: Option<Vec<String>>,
    pub issn_l: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag: Option<String>,
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
    pub url: Option<String>,
    pub organization: Option<String>,
}

impl Society {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Journal,
    Repository,
    Publisher,
    Conference,
    EbookPlatform,
    BookSeries,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConceptIds {
    pub mag: Option<i64>,
    pub openalex: Option<String>,
    pub umls_cui: Option<Vec<String>>,
    pub umls_aui: Option<Vec<String>>,
    pub wikidata: Option<String>,
    pub wikipedia: Option<String>,
}

impl ConceptIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InternationalDisplayNames {
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
    pub two_year_mean_citedness: Option<f64>,
    pub h_index: Option<i32>,
    pub i10_index: Option<i32>,
}

impl ConceptSummaryStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DehydratedInstitutionWithRelationship {
    pub id: Option<String>,
    pub ror: Option<String>,
    pub display_name: Option<String>,
    pub country_code: Option<String>,
    #[serde(rename = "type")]
    pub institution_type: Option<String>,
    pub relationship: Option<String>,
}

impl DehydratedInstitutionWithRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Geo {
    pub city: Option<String>,
    pub geonames_city_id: Option<String>,
    pub region: Option<String>,
    pub country_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Geo {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InstitutionIds {
    pub grid: Option<String>,
    pub mag: Option<String>,
    pub openalex: Option<String>,
    pub ror: Option<String>,
    pub wikipedia: Option<String>,
    pub wikidata: Option<String>,
}

impl InstitutionIds {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Repository {
    pub id: Option<String>,
    pub display_name: Option<String>,
    pub host_organization: Option<String>,
    pub host_organization_name: Option<String>,
    pub host_organization_lineage: Option<Vec<String>>,
}

impl Repository {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Domain {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

impl Domain {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Field {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

impl Field {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Subfield {
    pub id: Option<String>,
    pub display_name: Option<String>,
}

impl Subfield {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TopicIds {
    pub openalex: Option<String>,
    pub wikipedia: Option<String>,
}

impl TopicIds {
    pub fn new() -> Self {
        Self::default()
    }
}
