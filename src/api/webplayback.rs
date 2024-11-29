#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebPlayBack {
    pub song_list: Vec<Song>,
    pub status: u8,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    #[serde(rename = "artwork-urls")]
    pub artwork_urls: ArtworkUrls,
    pub assets: Vec<Asset>,
    #[serde(rename = "hls-key-cert-url")]
    pub hls_key_cert_url: String,
    #[serde(rename = "hls-key-server-url")]
    pub hls_key_server_url: String,
    #[serde(rename = "is-itunes-stream")]
    pub is_itunes_stream: bool,
    pub song_id: String,
    #[serde(rename = "widevine-cert-url")]
    pub widevine_cert_url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "artworkURL")]
    pub artwork_url: String,
    pub chunks: Chunks,
    pub download_key: String,
    #[serde(rename = "file-size")]
    pub file_size: u64,
    pub flavor: String,
    pub md5: String,
    pub metadata: Metadata,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chunks {
    pub chunk_size: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub artist_id: String,
    pub artist_name: String,
    pub bit_rate: u32,
    pub compilation: bool,
    pub composer_id: String,
    pub composer_name: String,
    pub copyright: String,
    pub disc_count: u32,
    pub disc_number: u32,
    pub duration: u64,
    pub explicit: u8,
    pub file_extension: String,
    pub gapless: bool,
    pub genre: String,
    pub genre_id: u32,
    pub is_mastered_for_itunes: bool,
    pub item_id: String,
    pub item_name: String,
    pub kind: String,
    pub playlist_artist_name: String,
    pub playlist_id: String,
    pub playlist_name: String,
    pub rank: u32,
    pub release_date: String,
    pub s: u32,
    pub sample_rate: u32,
    #[serde(rename = "sort-album")]
    pub sort_album: String,
    #[serde(rename = "sort-artist")]
    pub sort_artist: String,
    #[serde(rename = "sort-composer")]
    pub sort_composer: String,
    #[serde(rename = "sort-name")]
    pub sort_name: String,
    pub track_count: u32,
    pub track_number: u32,
    pub vendor_id: u32,
    pub xid: String,
    pub year: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ArtworkUrls {
    pub default: ArtWorkUrlDefault,
    #[serde(rename = "default@2x")]
    pub defaultx2: ArtWorkUrlDefaultx2,
    #[serde(rename = "image-type")]
    pub image_type: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ArtWorkUrlDefault {
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ArtWorkUrlDefaultx2 {
    pub url: String,
}
