#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub attributes: Attributes,
    /// The relationships for a song resource.
    pub relationships: Relationships,
    pub meta: Meta,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    pub width: u32,
    pub url: String,
    pub height: u32,
    pub text_color3: String,
    pub text_color2: String,
    pub text_color4: String,
    pub text_color1: String,
    pub bg_color: String,
    pub has_p3: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The name of the album the song appears on.
    pub album_name: Option<String>,
    /// The artist’s name.
    pub artist_name: String,
    /// The album artwork.
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content. The possible values for this rating are clean and explicit. No value means no rating.
    pub content_rating: String,
    /// The disc number the song appears on.
    pub disc_number: u32,
    /// The approximate length of the song in milliseconds.
    pub duration_in_millis: u64,
    /// The notes about the song that appear in the Apple Music catalog.
    pub editorial_notes: Option<EditorialNotes>,
    /// The genre names the song is associated with.
    pub genre_names: Vec<String>,
    /// Indicates whether the song has lyrics available in the Apple Music catalog. If true, the song has lyrics available; otherwise, it doesn't.
    pub has_lyrics: bool,
    /// Indicates whether the response delivered the song as an [Apple Digital Master](https://www.apple.com/apple-music/apple-digital-masters/).
    pub is_apple_digital_master: bool,
    /// The International Standard Recording Code (ISRC) for the song.
    pub isrc: Option<String>,
    /// (Classical music only) The movement count of the song.
    pub movement_count: Option<i32>,
    /// (Classical music only) The movement name of the song.
    pub movement_name: Option<String>,
    /// (Classical music only) The movement number of the song.
    pub movement_number: Option<i32>,
    /// The localized name of the song.
    pub name: String,
    /// When present, this attribute indicates that the song is available to play with an Apple Music subscription. The value map may be used to initiate playback. Previews of the song audio may be available with or without an Apple Music subscription.
    pub play_params: Option<PlayParams>,
    /// The preview assets for the song.
    pub previews: Vec<Preview>,
    /// The release date of the song, when known, in YYYY-MM-DD or YYYY format. Prerelease songs may have an expected release date in the future.
    pub release_date: Option<String>,
    /// The number of the song in the album’s track list.
    pub track_number: Option<i32>,
    /// The URL for sharing the song in Apple Music.
    pub url: String,
    /// (Classical music only) The name of the associated work.
    pub work_name: Option<String>,
    pub has_time_synced_lyrics: Option<bool>,
    pub is_vocal_attenuation_allowed: bool,
    pub is_mastered_for_itunes: bool,
    pub composer_name: Option<String>,
    pub audio_locale: Option<String>,
    pub audio_traits: Option<Vec<String>>,
    pub extended_asset_urls: Option<ExtendedAssetUrls>,
}

/// An object that represents a notes attribute.
/// ## Discussion
/// Notes may include XML tags for formatting (&lt;b&gt; for bold, &lt;i&gt; for italic, or &lt;br&gt; for line break) and special characters (&amp;amp; for &, &amp;lt; for <, &amp;gt; for >, &amp;apos; for ‘, and &amp;quot; for “).
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayParams {
    /// The ID of the content to use for playback.
    pub id: String,
    /// The kind of the content to use for playback.
    pub kind: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub artwork: Option<Artwork>,
    pub url: String,
    pub hls_url: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedAssetUrls {
    pub plus: String,
    pub lightweight: String,
    pub super_lightweight: String,
    pub lightweight_plus: String,
    pub enhanced_hls: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Relationships {
    /// The albums associated with the song. By default, albums includes identifiers only.
    pub albums: Option<RelationshipData>,
    /// The artists associated with the song. By default, artists includes identifiers only.
    pub artists: Option<RelationshipData>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RelationshipData {
    pub href: Option<String>,
    pub next: Option<String>,
    pub data: Vec<RelationshipItem>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RelationshipItem {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Meta {
    pub content_version: Option<ContentVersion>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ContentVersion {
    pub rtci: u64,
    pub mz_indexer: u64,
}

#[cfg(test)]
mod tests {
    use crate::AppleMusicDownloader;

    #[tokio::test]
    async fn test_songs() {
        let media_user_token = std::env::var("MEDIA_USER_TOKEN").unwrap();
        let apple_music = AppleMusicDownloader::new_with_media_user_token(&media_user_token)
            .await
            .unwrap();
        let song = apple_music.get_songs("1214782673").await.unwrap();
        assert_eq!(
            song.attributes.name,
            "サイレンは彼方より (feat. Hatsune Miku)"
        )
    }
}
