#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EditorialNotes {
    /// Abbreviated notes shown inline or when the content appears alongside other content.
    pub short: Option<String>,
    /// Notes shown when the content is prominently displayed.
    pub standard: Option<String>,
    /// Name for the editorial notes.
    pub name: Option<String>,
    /// The tag line for the editorial notes.
    pub tagline: Option<String>,
}
