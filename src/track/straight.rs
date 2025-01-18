use defmt::{debug, error, trace};

use super::Track;
pub struct Straight {
    pub id: &'static str,
    pub length: u32,
    pub connection_a: &'static str,
    pub connection_b: &'static str,
}

impl Straight {
    pub fn new(
        id: &'static str,
        connection_a: &'static str,
        connection_b: &'static str,
        length: u32,
    ) -> Self {
        Self {
            id,
            length,
            connection_a,
            connection_b,
        }
    }
}

impl Track for Straight {
    // Constructor for Straight

    fn id(&self) -> &str {
        self.id
    }

    fn debug_print(&self) {
        debug!(
            "Straight {}: length: {}, connected with {} and {}",
            self.id(),
            self.length(),
            self.connection_a,
            self.connection_b
        );
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection_a {
            trace!(
                "Straight {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_b
            );
            return [self.connection_b, "", ""];
        }
        if previous_track == self.connection_b {
            trace!(
                "Straight {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_a
            );
            return [self.connection_a, "", ""];
        }
        error!("Straight {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }

    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection_a {
            trace!(
                "Straight {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_b
            );
            return self.connection_b;
        }
        if previous_track == self.connection_b {
            trace!(
                "Straight {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_a
            );
            return self.connection_a;
        }
        error!("Straight {}: Not connected to {}", self.id, previous_track);
        ""
    }
    fn get_connections(&self) -> [&str; 4] {
        [self.connection_a, self.connection_b, "", ""]
    }
}