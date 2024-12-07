#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    /// The identifier for the genre.
    pub id: String,
    /// This value must always be genres.
    #[serde(rename = "type")]
    pub type_field: String,
    /// The relative location for the genre resource.
    pub href: String,
    /// The attributes for the genre.
    pub attributes: Attributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The localized name of the genre.
    pub name: String,
    /// The identifier of the parent for the genre.
    pub parent_id: Option<String>,
    /// The localized name of the parent genre.
    pub parent_name: Option<String>,
    /// A localized string to use when displaying the genre in relation to charts.
    pub chart_label: Option<String>,
}
