#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Lyrics {
    pub href: String,
    pub data: Vec<LyricData>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct LyricData {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub attributes: LyricAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LyricAttributes {
    pub ttml: String,
    pub play_params: PlayParams,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayParams {
    pub id: String,
    pub kind: String,
    pub catalog_id: String,
    pub display_type: i32,
}

#[cfg(test)]
mod tests {
    use crate::AppleMusicDownloader;

    #[tokio::test]
    async fn test_lyrics() {
        let media_user_token = std::env::var("MEDIA_USER_TOKEN").unwrap();
        let apple_music = AppleMusicDownloader::new_with_media_user_token(&media_user_token)
            .await
            .unwrap();
        let lyrics = apple_music.get_lyrics("1428083880").await.unwrap();
    }
}
