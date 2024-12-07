use super::artwork::Artwork;
use super::editorial_notes::EditorialNotes;
use super::play_parameters::PlayParameters;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Albums {
    /// The identifier for the album.
    pub id: String,
    /// This value is always albums.
    #[serde(rename = "type")]
    pub type_: String,
    /// The relative location for the album resource.
    pub href: String,
    /// The attributes for the album.
    pub attributes: Option<Attributes>,
    /// The relationships for the album.
    pub relationships: Option<Relationships>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    /// The name of the primary artist associated with the album.
    pub artist_name: String,
    /// The URL of the artist for this content.
    pub artist_url: Option<String>,
    /// The artwork for the album.
    pub artwork: Artwork,
    /// Indicates the specific audio variant for the album.
    pub audio_variants: Option<Vec<String>>,
    /// The Recording Industry Association of America (RIAA) rating of the content.
    pub content_rating: Option<String>,
    /// The copyright text.
    pub copyright: Option<String>,
    /// The notes about the album that appear in the iTunes Store.
    pub editorial_notes: Option<EditorialNotes>,
    /// The names of the genres associated with the album.
    pub genre_names: Vec<String>,
    /// Indicates whether the album is marked as a compilation.
    pub is_compilation: bool,
    /// Indicates whether the album is complete.
    pub is_complete: bool,
    /// Indicates whether the response delivered the album as an Apple Digital Master.
    pub is_mastered_for_itunes: bool,
    /// Indicates whether the album contains a single song.
    pub is_single: bool,
    /// The localized name of the album.
    pub name: String,
    /// Indicates that one or more tracks on the album are available to play with an Apple Music subscription.
    pub play_params: Option<PlayParameters>,
    /// The name of the record label for the album.
    pub record_label: Option<String>,
    /// The release date of the album, when known.
    pub release_date: Option<String>,
    /// The number of tracks for the album.
    pub track_count: i32,
    /// The Universal Product Code for the album.
    pub upc: Option<String>,
    /// The URL for sharing the album in Apple Music.
    pub url: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relationships {
    // /// The artists associated with the album. By default, artists includes identifiers only.
    // pub artists: Option<AlbumsArtistsRelationship>,
    // /// The genres for the album. By default, genres not included.
    // pub genres: Option<AlbumsGenresRelationship>,
    // /// The songs and music videos on the album. By default, tracks includes objects.
    // pub tracks: Option<AlbumsTracksRelationship>,
    // /// The album in the user’s library for the catalog album, if any.
    // pub library: Option<AlbumsLibraryRelationship>,
    // /// The record labels for the album.
    // pub record_labels: Option<AlbumsRecordLabelsRelationship>,
}

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumsArtistsRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The artists for the album.
//     pub data: Vec<Artist>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumsGenresRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The album’s associated genre.
//     pub data: Vec<Genre>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumsTracksRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The tracks for the album.
//     pub data: Vec<TrackData>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub enum TrackData {
//     #[serde(rename = "songs")]
//     Songs(Songs),
//     #[serde(rename = "music-videos")]
//     MusicVideos(MusicVideos),
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumsLibraryRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The library content this album is associated with if added to the user’s library.
//     pub data: Vec<LibraryAlbums>,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct AlbumsRecordLabelsRelationship {
//     /// The relative location to fetch the relationship directly.
//     pub href: String,
//     /// The relative location to request the next page of resources in the collection, if additional resources are available for fetching.
//     pub next: Option<String>,
//     /// The album’s associated record label.
//     pub data: Vec<RecordLabels>,
// }
