//! the stream info struct and functions.

use crate::error::Result;
use crate::{api::*, decrypter};

/// A struct representing the stream information.
#[derive(Debug, Clone)]
pub struct StreamInfo {
    pub stream_url: String,
    pub pssh: String,
    pub codec: String,
}

impl StreamInfo {
    /// Creates a new `StreamInfo` instance from the provided M3U8 data and base URI.
    pub fn new(m3u8: Vec<u8>, base_uri: &str) -> Result<Self> {
        let (_, m3u8_data) = m3u8_rs::parse_master_playlist(&m3u8)
            .map_err(|e| crate::error::Error::Decrypt(e.to_string()))?;
        let drm_infos = decrypter::get_drm_info(&m3u8_data)?;
        let assert_info = decrypter::get_assert_info(&m3u8_data)?;
        let base_uri = match base_uri.rfind('/') {
            Some(index) => &base_uri[..index + 1],
            None => base_uri,
        };
        let stream_url = format!("{}{}", base_uri, m3u8_data.variants[0].uri.clone());
        let variant_id = m3u8_data.variants[0]
            .other_attributes
            .as_ref()
            .and_then(|attrs| attrs.get("STABLE-VARIANT-ID").cloned())
            .expect("STABLE-VARIANT-ID not found");
        let drm_ids = &assert_info[variant_id.as_str()]["AUDIO-SESSION-KEY-IDS"];
        let pssh = decrypter::get_pssh(&drm_infos, drm_ids).unwrap().clone();
        let codec = m3u8_data.variants[0].codecs.as_ref().unwrap().clone();
        Ok(Self {
            stream_url,
            pssh,
            codec,
        })
    }

    // pub async fn new_with_song(song: Song, client: &reqwest::Client) -> Result<Self> {
    //     let m3u8_url = song.attributes.extended_asset_urls.enhanced_hls;
    //     let m3u8 = client.get(&m3u8_url).send().await?.bytes().await?.to_vec();
    //     let stream_info = Self::new(m3u8, &m3u8_url);
    //     stream_info
    // }

    /// Creates a new `StreamInfo` instance from a `WebPlayBack` instance.
    pub async fn new_with_webplayback(webplayback: &webplayback::WebPlayBack) -> Result<Self> {
        let webplayback = webplayback
            .song_list
            .first()
            .unwrap()
            .assets
            .iter()
            .find(|t| t.flavor == "28:ctrp256")
            .unwrap();

        let m3u8 = reqwest::get(&webplayback.url)
            .await?
            .bytes()
            .await?
            .to_vec();

        let media_playlist = m3u8_rs::parse_media_playlist(&m3u8)
            .map_err(|e| crate::error::Error::Decrypt(e.to_string()))?
            .1;

        if let Some(pssh) = media_playlist
            .segments
            .first()
            .and_then(|segment| segment.key.as_ref())
            .and_then(|key| key.uri.as_ref())
        {
            Ok(Self {
                stream_url: webplayback.url.clone(),
                pssh: pssh.clone(),
                codec: String::new(),
            })
        } else {
            Err(crate::error::Error::Other("Source not exists".to_string()))
        }
    }
}
