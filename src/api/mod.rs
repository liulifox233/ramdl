//! the API struct of Apple Music.

/// /v1/catalog/:store_front/songs/:song_id?include=lyrics,syllable-lyrics
pub mod lyrics;
/// /v1/catalog/:store_front/search
pub mod search;
/// /v1/catalog/:store_front/songs/:song_id?include=albums
pub mod songs;
/// WEBPLAYBACK_API_URL
pub mod webplayback;
