#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SearchResults {
    pub songs: Vec<Song>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub playlists: Vec<Playlist>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Song {
    pub id: String,
    pub href: String,
    pub attributes: SongAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SongAttributes {
    pub has_time_synced_lyrics: bool,
    pub album_name: String,
    pub genre_names: Vec<String>,
    pub track_number: u32,
    pub release_date: String,
    pub duration_in_millis: u32,
    pub is_vocal_attenuation_allowed: bool,
    pub is_mastered_for_itunes: bool,
    pub isrc: String,
    pub artwork: Artwork,
    pub audio_locale: String,
    pub composer_name: Option<String>,
    pub play_params: PlayParams,
    pub url: String,
    pub disc_number: u32,
    pub has_lyrics: bool,
    pub is_apple_digital_master: bool,
    pub audio_traits: Vec<String>,
    pub name: String,
    pub previews: Vec<Preview>,
    pub artist_name: String,
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
pub struct PlayParams {
    pub id: String,
    pub kind: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Preview {
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Album {
    pub id: String,
    pub href: String,
    pub attributes: AlbumAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumAttributes {
    pub copyright: String,
    pub genre_names: Vec<String>,
    pub release_date: String,
    pub upc: String,
    pub is_mastered_for_itunes: bool,
    pub artwork: Artwork,
    pub url: String,
    pub play_params: PlayParams,
    pub record_label: String,
    pub is_compilation: bool,
    pub track_count: u32,
    pub is_prerelease: bool,
    pub audio_traits: Vec<String>,
    pub is_single: bool,
    pub name: String,
    pub artist_name: String,
    pub is_complete: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Playlist {
    pub id: String,
    pub href: String,
    pub attributes: PlaylistAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistAttributes {
    pub last_modified_date: String,
    pub supports_sing: bool,
    pub description: Description,
    pub artwork: Artwork,
    pub play_params: PlayParams,
    pub url: String,
    pub has_collaboration: bool,
    pub curator_name: String,
    pub audio_traits: Vec<String>,
    pub name: String,
    pub is_chart: bool,
    pub playlist_type: String,
    pub editorial_notes: EditorialNotes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Description {
    pub standard: String,
    pub short: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct EditorialNotes {
    pub name: String,
    pub standard: String,
    pub short: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Artist {
    pub id: String,
    pub href: String,
    pub attributes: ArtistAttributes,
    pub relationships: ArtistRelationships,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtistAttributes {
    pub name: String,
    pub genre_names: Vec<String>,
    pub artwork: Artwork,
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ArtistRelationships {
    pub albums: AlbumsRelationship,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AlbumsRelationship {
    pub href: String,
    pub next: Option<String>,
    pub data: Vec<AlbumReference>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AlbumReference {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub href: String,
}

#[cfg(test)]
mod tests {
    use crate::AppleMusicDownloader;

    #[tokio::test]
    async fn test_search() {
        let media_user_token = std::env::var("MEDIA_USER_TOKEN").unwrap();
        let apple_music = AppleMusicDownloader::new_with_media_user_token(&media_user_token)
            .await
            .unwrap();
        apple_music.search("サイレンは彼方より").await.unwrap();
    }
}
