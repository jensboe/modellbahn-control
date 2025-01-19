pub mod switch;
pub mod straight;
pub mod buffer;


use defmt::{debug, error};


#[derive(Clone,Copy, defmt::Format)]
pub enum PowerState {
    Off,
    On,
}
#[derive(Clone,Copy, defmt::Format)]
pub enum PlaningState {
    Unused,
    Planed,
    Locked,
}
#[derive(Clone,Copy, defmt::Format)]
pub enum OccupiedState {
    Free,
    Occupied,
    Locked,
    Unknown
}

pub trait Track {
    fn power_state(&self) -> PowerState;
    fn set_power_state(&mut self, power_state: PowerState);

    fn occupied_state(&self) -> OccupiedState;

    fn id(&self) -> &str;

    fn debug_print(&self) {
        debug!(
            "Track {}: planing state: {:?}, connections: {:?}",
            self.id(),
            self.planing_state(),
            self.connections()
        );
    }

    fn next_tracks(&self, previous_track: &str) -> [&str; 3];
    
    fn next_track(&self, previous_track: &str) -> &'static str;
    
    fn connections(&self) -> [&str; 4] {
        ["", "", "", ""]
    }
    fn is_plattform(&self) -> bool {
        false
    }
    fn planing_state(&self) -> PlaningState;
    fn length(&self) -> u32;

}

pub struct BaseTrack {
    id: &'static str,
    power_state: PowerState,
    occupied_state: OccupiedState,
    planing_state: PlaningState,
}
impl BaseTrack {
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            power_state: PowerState::Off,
            occupied_state: OccupiedState::Unknown,
            planing_state: PlaningState::Unused,
        }
    }
}
impl Track for BaseTrack {
    fn power_state(&self) -> PowerState {
        self.power_state
    }
    fn set_power_state(&mut self, power_state: PowerState) {
        self.power_state = power_state;
    }

    fn occupied_state(&self) -> OccupiedState {
        self.occupied_state
    }

    fn id(&self) -> &str {
        self.id
    }

    fn next_tracks(&self, previous_track: &str) -> [&str; 3] {
        if previous_track == self.id {
            return ["", "", ""];
        }
        error!("Track {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }

    fn next_track(&self, previous_track: &str) -> &'static str {
        if previous_track == self.id {
            return "";
        }
        error!("Track {}: Not connected to {}", self.id, previous_track);
        ""
    }

    fn planing_state(&self) -> PlaningState {
        self.planing_state
    }

    fn length(&self) -> u32 {
        10
    }
    
}