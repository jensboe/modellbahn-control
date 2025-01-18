use defmt::{error, trace};

/// A trait representing a track in a model railway system.
pub trait Track {
    /// Returns the identifier of the track.
    ///
    /// # Returns
    /// A string slice that holds the identifier of the track.
    fn id(&self) -> &str;
    fn trace(&self) {
        trace!("Track {}: length: {}, connected with {}", self.id(), self.length(), self.get_connections());
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

pub struct Straight {
    pub id: &'static str,
    pub length: u32,
    pub connection_a: &'static str,
    pub connection_b: &'static str,
}

impl Track for Straight {
    fn id(&self) -> &str {
            self.id
    }

    fn trace(&self) {
        trace!("Straight {}: length: {}, connected with {} and {}", self.id(), self.length(), self.connection_a, self.connection_b);
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection_a {
            trace!("Straight {}: from {} to {}", self.id(), previous_track, self.connection_b);
            return [self.connection_b, "", ""]
        }
        if previous_track == self.connection_b {
            trace!("Straight {}: from {} to {}", self.id(), previous_track, self.connection_a);
            return [self.connection_a, "", ""]
        }
        error!("Straight {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }
    
    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection_a {
            trace!("Straight {}: from {} to {}", self.id(), previous_track, self.connection_b);
            return self.connection_b
        }
        if previous_track == self.connection_b {
            trace!("Straight {}: from {} to {}", self.id(), previous_track, self.connection_a);
            return self.connection_a
        }
        error!("Straight {}: Not connected to {}", self.id, previous_track);
        ""
    }
    fn get_connections(&self) -> [&str; 4] {
        [self.connection_a, self.connection_b, "", ""]
    }
}

#[derive(PartialEq, defmt::Format)]
pub enum SwitchState {
    Straight,
    Turnout,
}

pub struct Switch {
    pub id: &'static str,
    pub connection_common: &'static str,
    pub connection_straight: &'static str,
    pub length_straight: u32,
    pub connection_turnout: &'static str,
    pub length_turnout: u32,
    pub state: SwitchState,
}

impl Track for Switch {
    fn id(&self) -> &str {
        self.id
    }
    
    fn trace(&self) {
        trace!("Switch   {}, current state: {}, length: {} going from {} straight to {} or turnover to {}", self.id(), self.state, self.length(), self.connection_common, self.connection_straight, self.connection_turnout);
    }

    fn length(&self) -> u32 {
        if self.state == SwitchState::Straight {
            return self.length_straight
        }
        else {
            return self.length_turnout
        }
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection_common {
            trace!("Switch   {}: from {} to {} or {}", self.id(), previous_track, self.connection_straight, self.connection_turnout);
            return [self.connection_straight, self.connection_turnout, ""]
        }
        if previous_track == self.connection_straight {
            trace!("Switch   {}: from {} to {}", self.id(), previous_track, self.connection_common);
            return [self.connection_common, "", ""]
        }
        if previous_track == self.connection_turnout {
            trace!("Switch   {}: from {} to {}", self.id(), previous_track, self.connection_common);
            return [self.connection_common, "", ""]
        }
        error!("Switch   {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }

    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection_common {
            match self.state {
                SwitchState::Straight => {
                    trace!("Switch   {}: from {} straight to {}", self.id(), previous_track, self.connection_straight);
                    return self.connection_straight
                },
                SwitchState::Turnout => {
                    trace!("Switch   {}: from {} turnout to {}", self.id(), previous_track, self.connection_turnout);
                    return self.connection_turnout
                }
            }
        }
        if previous_track == self.connection_straight {
            return self.connection_common
        }
        if previous_track == self.connection_turnout {
            return self.connection_common
        }
        error!("Switch   {}: Not connected to {}", self.id, previous_track);
        ""
    }
    fn get_connections(&self) -> [&str; 4] {
        [self.connection_common, self.connection_straight, self.connection_turnout, ""]
    }
}

pub struct Buffer {
    pub id: &'static str,
    pub connection: &'static str,
}

impl Track for Buffer {
    fn id(&self) -> &str {
        self.id
    }

    fn trace(&self) {
        trace!("Buffer {}: connected with {}", self.id(), self.connection);
    }

    fn length(&self) -> u32 {
        1
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return ["", "", ""]
        }
        error!("Buffer {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }
    
    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return ""
        }
        error!("Buffer {}: Not connected to {}", self.id, previous_track);
        ""
    }
}