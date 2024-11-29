#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
    pub attributes: Attributes,
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
    pub album_name: String,
    pub has_time_synced_lyrics: bool,
    pub genre_names: Vec<String>,
    pub track_number: u32,
    pub duration_in_millis: u64,
    pub release_date: String,
    pub is_vocal_attenuation_allowed: bool,
    pub is_mastered_for_itunes: bool,
    pub isrc: String,
    pub artwork: Artwork,
    pub composer_name: Option<String>,
    pub audio_locale: String,
    pub url: String,
    pub play_params: PlayParams,
    pub disc_number: u32,
    pub has_lyrics: bool,
    pub is_apple_digital_master: bool,
    pub audio_traits: Vec<String>,
    pub name: String,
    pub previews: Vec<Preview>,
    pub artist_name: String,
    pub extended_asset_urls: ExtendedAssetUrls,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct PlayParams {
    pub id: String,
    pub kind: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Preview {
    pub url: String,
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
    pub albums: RelationshipData,
    pub artists: RelationshipData,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct RelationshipData {
    pub href: String,
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
