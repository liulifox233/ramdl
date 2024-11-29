//! some decryption functions.

use crate::{error::Result, AppleMusicDownloader};
use base64::Engine;
use m3u8_rs::MasterPlaylist;
use pssh_box::{widevine::WidevinePsshData, PsshBox, PsshData, ToBytes};
use widevine::{self, Cdm, LicenseType, Pssh};

/// Requests a Widevine license from Apple Music and retrieves the decryption key.
pub async fn get_decrypt_key(
    cdm: &Cdm,
    pssh: &str,
    id: &str,
    apple_music_downloader: &AppleMusicDownloader,
) -> Result<String> {
    let session = cdm.open();
    let key_id = base64::engine::general_purpose::STANDARD
        .decode(pssh.split(',').collect::<Vec<&str>>().last().unwrap())?;
    let widevine_pssh_data = WidevinePsshData {
        key_id: vec![key_id],
        algorithm: Some(1),
        ..Default::default()
    };
    let pssh_obj = PsshData::Widevine(widevine_pssh_data);
    let mut pssh_box = PsshBox::new_widevine();
    // let key_id: DRMKeyId = key_id.try_into().unwrap();
    // pssh_box.add_key_id(key_id);
    pssh_box.pssh_data = pssh_obj;
    pssh_box.version = 0;
    let pssh_obj = Pssh::from_bytes(&pssh_box.to_bytes()).unwrap();
    let cdm_license_request = session
        .get_license_request(pssh_obj, LicenseType::STREAMING)
        .unwrap();

    let license_message = apple_music_downloader
        .get_widevine_license(id, pssh, cdm_license_request.challenge().unwrap())
        .await?;
    let keys = cdm_license_request.get_keys(&license_message).unwrap();
    let key = keys.first_of_type(widevine::KeyType::CONTENT).unwrap();
    let key_hex = hex::encode(key.key.clone());
    Ok(key_hex)
}

/// Retrieves asset information from the provided M3U8 master playlist data.
pub fn get_assert_info(m3u8_data: &MasterPlaylist) -> Result<serde_json::Value> {
    let assert_info = m3u8_data
        .session_data
        .iter()
        .find(|data| data.data_id == "com.apple.hls.audioAssetMetadata")
        .ok_or_else(|| crate::error::Error::Decrypt("DATA-ID not found".to_string()))?;
    if let m3u8_rs::SessionDataField::Value(value) = &assert_info.field {
        let assert_info = base64::engine::general_purpose::STANDARD.decode(value)?;
        Ok(serde_json::from_slice(&assert_info)?)
    } else {
        Err(crate::error::Error::Decrypt(
            "DATA-ID not found".to_string(),
        ))
    }
}

/// Retrieves the PSSH (Protection System Specific Header) from the provided DRM information and DRM IDs.
pub fn get_pssh(drm_infos: &serde_json::Value, drm_ids: &serde_json::Value) -> Option<String> {
    for drm_id in drm_ids.as_array()? {
        if let Some(drm_info) = drm_infos.get(drm_id.as_str()?) {
            if let Some(uri) = drm_info.get("urn:uuid:edef8ba9-79d6-4ace-a3c8-27dcd51d21ed") {
                if drm_id != "1" {
                    return uri
                        .get("URI")
                        .and_then(|u| u.as_str().map(|s| s.to_string()));
                }
            }
        }
    }
    None
}

/// Retrieves DRM information from the provided M3U8 master playlist data.
pub fn get_drm_info(m3u8_data: &MasterPlaylist) -> Result<serde_json::Value> {
    let drm_info = m3u8_data
        .session_data
        .iter()
        .find(|data| data.data_id == "com.apple.hls.AudioSessionKeyInfo")
        .ok_or_else(|| crate::error::Error::Decrypt("DATA-ID not found".to_string()))?;
    if let m3u8_rs::SessionDataField::Value(value) = &drm_info.field {
        let drm_info = base64::engine::general_purpose::STANDARD.decode(value)?;
        Ok(serde_json::from_slice(&drm_info)?)
    } else {
        Err(crate::error::Error::Decrypt(
            "DATA-ID not found".to_string(),
        ))
    }
}
