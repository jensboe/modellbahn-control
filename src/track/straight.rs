use defmt::{error, trace};

use super::{BaseTrack, OccupiedState, Track, PowerState};
pub struct Straight {
    pub base: BaseTrack,
    pub length: u32,
    pub connection_a: &'static str,
    pub connection_b: &'static str,
    pub is_pattform: bool,
}

impl Straight {
    pub const fn new(
        id: &'static str,
        connection_a: &'static str,
        connection_b: &'static str,
        length: u32,
    ) -> Self {
        Self {
            base: BaseTrack::new(id),
            length,
            connection_a,
            connection_b,
            is_pattform: false,
        }
    }
    
    pub const fn new_platform(
        id: &'static str,
        connection_a: &'static str,
        connection_b: &'static str,
        length: u32,
        
    ) -> Self {
        Self {
            base: BaseTrack::new(id),
            length,
            connection_a,
            connection_b,
            is_pattform: true,
        }
    }
}

impl Track for Straight {
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

    fn debug_print(&self) {
        self.base.debug_print();
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn next_tracks(&self, previous_track: &str) -> [&'static str; 3] {
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
        error!("Straight {}: Not connected to {}", self.id(), previous_track);
        ["", "", ""]
    }

    fn next_track(&self, previous_track: &str) -> &'static str {
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
        error!("Straight {}: Not connected to {}", self.id(), previous_track);
        ""
    }
    fn connections(&self) -> [&str; 4] {
        [self.connection_a, self.connection_b, "", ""]
    }

    fn is_plattform(&self) -> bool {
        self.is_pattform
    }
    fn planing_state(&self) -> super::PlaningState {
        self.base.planing_state()
    }

}