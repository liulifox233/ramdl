use super::artwork::Artwork;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    /// The preview artwork for the associated preview music video.
    pub artwork: Option<Artwork>,
    /// The preview URL for the content.
    pub url: String,
    /// The HLS preview URL for the content.
    pub hls_url: Option<String>,
}
