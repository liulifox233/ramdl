#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    /// The average background color of the image.
    pub bg_color: Option<String>,
    /// The maximum height available for the image.
    pub height: i32,
    /// The maximum width available for the image.
    pub width: i32,
    /// The primary text color used if the background color gets displayed.
    pub text_color1: String,
    /// The secondary text color used if the background color gets displayed.
    pub text_color2: String,
    /// The tertiary text color used if the background color gets displayed.
    pub text_color3: String,
    /// The final post-tertiary text color used if the background color gets displayed.
    pub text_color4: String,
    /// The URL to request the image asset. {w}x{h}must precede image filename, as placeholders for the width and height values as described above. For example, {w}x{h}bb.jpeg).
    pub url: String,
    pub has_p3: Option<bool>,
}
