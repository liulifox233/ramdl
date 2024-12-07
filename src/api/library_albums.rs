use super::albums::Albums;
use super::artwork::Artwork;
use super::play_parameters::PlayParameters;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LibraryAlbums {
    /// The identifier for the library album.
    pub id: String,
    /// This value is always library-albums.
    #[serde(rename = "type")]
    pub type_: String,
    /// The relative location for the library album resource.
    pub href: String,
    /// The attributes for the library album.
    pub attributes: Option<Attributes>,
    /// The relationships for the library album.
    pub relationships: Option<Relationships>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The artist’s name.
    pub artist_name: String,
    /// The album artwork.
    pub artwork: Artwork,
    /// The Recording Industry Association of America (RIAA) rating of the content.
    /// The possible values for this rating are clean and explicit. No value means no rating.
    pub content_rating: Option<String>,
    /// The date the album was added to the library, in YYYY-MM-DD or YYYY format.
    pub date_added: Option<String>,
    /// The localized name of the album.
    pub name: String,
    /// When present, this attribute indicates that tracks from the album are available to play.
    /// The value map may be used to initiate playback of available tracks on the album.
    pub play_params: Option<PlayParameters>,
    /// The release date of the album, when known, in YYYY-MM-DD or YYYY format.
    /// Pre-release albums may have an expected release date in the future.
    pub release_date: Option<String>,
    /// The number of tracks.
    pub track_count: i32,
    /// The names of the genres associated with this album.
    pub genre_names: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    // /// The library artists associated with the album. By default, artists not included.
    // /// Fetch limits: 10 default, 10 maximum
    // pub artists: Option<LibraryAlbumsArtistsRelationship>,
    // /// The album in the Apple Music catalog the library album is associated with, when known.
    // /// Fetch limits: None (associated with at most one catalog album)
    // pub catalog: Option<LibraryAlbumsCatalogRelationship>,
    // /// The library songs and library music videos on the album. Only available when fetching single library album resource by ID. By default, tracks includes objects.
    // /// Fetch limits: 300 default, 300 maximum.
    // pub tracks: Option<LibraryAlbumsTracksRelationship>,
}

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct LibraryAlbumsArtistsRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The library artists for the library album.
//     pub data: Vec<LibraryArtists>,
// }

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LibraryAlbumsCatalogRelationship {
    /// The relative location to fetch the relationship directly.
    pub href: String,
    /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
    pub next: Option<String>,
    /// The album from the Apple Music catalog associated with the library album, if any.
    pub data: Vec<Albums>,
}

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct LibraryAlbumsTracksRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The songs and music videos from the library album’s tracklist added to the user’s library.
//     pub data: Vec<LibraryTracks>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(tag = "type")]
// pub enum LibraryTracks {
//     LibraryMusicVideos(LibraryMusicVideos),
//     LibrarySongs(LibrarySongs),
// }
