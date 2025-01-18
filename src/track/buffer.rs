use defmt::{debug, error, trace};

use super::Track;


pub struct Buffer {
    pub id: &'static str,
    pub connection: &'static str,
    pub length: u32,
}
impl Buffer {
    pub fn new(id: &'static str, connection: &'static str, length: u32) -> Self {
        Self {
            id,
            connection,
            length,
        }
    }
}

impl Track for Buffer {
    fn id(&self) -> &str {
        self.id
    }

    fn debug_print(&self) {
        debug!(
            "Buffer {}: length: {}, connected with {}",
            self.id(),
            self.length,
            self.connection
        );
    }

    fn length(&self) -> u32 {
        1
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return ["", "", ""];
        }
        error!("Buffer {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }

    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return "";
        }
        error!("Buffer {}: Not connected to {}", self.id, previous_track);
        ""
    }
}
