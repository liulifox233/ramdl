#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayParameters {
    /// The ID of the content to use for playback.
    pub id: String,
    /// The kind of the content to use for playback.
    pub kind: String,
}
