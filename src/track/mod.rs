pub mod switch;
pub mod straight;
pub mod buffer;

use defmt::{debug, error, trace};

/// A trait representing a track in a model railway system.
pub trait Track {
    /// Returns the identifier of the track.
    ///
    /// # Returns
    /// A string slice that holds the identifier of the track.
    fn id(&self) -> &str;
    fn debug_print(&self) {
        debug!(
            "Track {}: length: {}, connected with {}",
            self.id(),
            self.length(),
            self.get_connections()
        );
    }

    /// Returns the length of the track.
    ///
    /// # Returns
    /// A 32-bit unsigned integer representing the length of the track.
    fn length(&self) -> u32;

    /// Returns the next possible tracks from the current track.
    ///
    /// # Parameters
    /// - `previous_track`: A string slice that holds the identifier of the previous track.
    ///
    /// # Returns
    /// An array of string slices representing the identifiers of the next possible tracks.
    fn get_next_tracks<'a>(&self, previous_track: &str) -> [&str; 3];

    /// Returns the next track from the current track.
    ///
    /// # Parameters
    /// - `previous_track`: A string slice that holds the identifier of the previous track.
    ///
    /// # Returns
    /// A string slice representing the identifier of the next track.
    fn get_next_track(&self, previous_track: &str) -> &str;

    fn get_connections(&self) -> [&str; 4] {
        ["", "", "", ""]
    }
}