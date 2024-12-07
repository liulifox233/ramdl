//! the API struct of Apple Music.

/// A resource object that represents an album.
pub mod albums;
/// A resource object that represents the artist of an album where an artist can be one or more people.
pub mod artists;
/// An object that represents artwork.
pub mod artwork;
/// An object that represents a notes attribute.
/// ## Discussion
/// Notes may include XML tags for formatting (&lt;b&gt; for bold, &lt;i&gt; for italic, or &lt;br&gt; for line break) and special characters (&amp;amp; for &, &amp;lt; for <, &amp;gt; for >, &amp;apos; for ‘, and &amp;quot; for “).
pub mod editorial_notes;
/// A resource object that represents a music genre.
pub mod genres;
/// A resource object that represents a library album.
pub mod library_albums;
/// /v1/catalog/:store_front/songs/:song_id?include=lyrics,syllable-lyrics
pub mod lyrics;
/// /v1/catalog/:store_front/music-videos/:id
pub mod music_videos;
/// An object that represents play parameters for resources.
pub mod play_parameters;
/// /v1/catalog/:store_front/playlists/:id
pub mod playlists;
/// An object that represents a preview for resources.
pub mod previews;
/// /v1/catalog/:store_front/search
pub mod search;
/// /v1/catalog/:store_front/songs/:id?include=albums
pub mod songs;
/// WEBPLAYBACK_API_URL
pub mod webplayback;
