use defmt::{error, trace};

use super::{BaseTrack, OccupiedState, Track, PowerState};


pub struct Buffer {
    pub base: BaseTrack,
    pub connection: &'static str,
    pub length: u32,
}
impl Buffer {
    pub const fn new(id: &'static str, connection: &'static str, length: u32) -> Self {
        Self {
            base: BaseTrack::new(id),
            connection,
            length,
        }
    }
}

impl Track for Buffer {
    fn power_state(&self) -> PowerState {
        self.base.power_state()
    }
    fn set_power_state(&mut self, power_state: PowerState) {
        self.base.set_power_state(power_state);
    }

    fn occupied_state(&self) -> OccupiedState {
        self.base.occupied_state()
    }

    fn id(&self) -> &str {
        self.base.id()
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return ["", "", ""];
        }
        error!("Buffer {}: Not connected to {}", self.id(), previous_track);
        ["", "", ""]
    }

    fn next_track(&self, previous_track: &str) -> &'static str {
        if previous_track == self.connection {
            trace!("Buffer {}: from {} to end", self.id(), previous_track);
            return "";
        }
        error!("Buffer {}: Not connected to {}", self.id(), previous_track);
        ""
    }
    fn connections(&self) -> [&str; 4] {
        [self.connection, "", "", ""]
    }
    fn planing_state(&self) -> super::PlaningState {
        self.base.planing_state()
    }
}
