use defmt::{error, trace};

use super::{BaseTrack, OccupiedState, Track, PowerState};

#[derive(Clone,Copy, PartialEq, defmt::Format)]
pub enum SwitchState {
    Straight,
    Turnout,
    Unknown,
}

pub struct Switch {
    pub base: BaseTrack,
    pub connection_common: &'static str,
    pub connection_straight: &'static str,
    pub length_straight: u32,
    pub connection_turnout: &'static str,
    pub length_turnout: u32,
    pub switch_state: SwitchState,
}
impl Switch {
    pub const fn new(
        id: &'static str,
        connection_common: &'static str,
        connection_straight: &'static str,
        connection_turnout: &'static str,
        length_straight: u32,
        length_turnout: u32,
    ) -> Self {
        Self {
            base: BaseTrack::new(id),
            connection_common,
            connection_straight,
            length_straight,
            connection_turnout,
            length_turnout,
            switch_state: SwitchState::Straight,
        }
    }
}
impl Switch {
    pub fn switch_state(&self) -> SwitchState {
        self.switch_state
    }
    pub fn set_switch_state(&mut self, switch_state: SwitchState) {
        self.switch_state = switch_state;
    }
}

impl Track for Switch {
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
        self.base.id
    }

    fn length(&self) -> u32 {
        if self.switch_state() == SwitchState::Straight {
            return self.length_straight;
        } else {
            return self.length_turnout;
        }
    }

    fn next_tracks(&self, previous_track: &str) -> [&'static str; 3] {
        if previous_track == self.connection_common {
            trace!(
                "Switch   {}: from {} to {} or {}",
                self.id(),
                previous_track,
                self.connection_straight,
                self.connection_turnout
            );
            return [self.connection_straight, self.connection_turnout, ""];
        }
        if previous_track == self.connection_straight {
            trace!(
                "Switch   {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_common
            );
            return [self.connection_common, "", ""];
        }
        if previous_track == self.connection_turnout {
            trace!(
                "Switch   {}: from {} to {}",
                self.id(),
                previous_track,
                self.connection_common
            );
            return [self.connection_common, "", ""];
        }
        error!("Switch   {}: Not connected to {}", self.id(), previous_track);
        ["", "", ""]
    }

    fn next_track(&self, previous_track: &str) -> &'static str  {
        if previous_track == self.connection_common {
            match self.switch_state {
                SwitchState::Straight => {
                    trace!(
                        "Switch   {}: from {} straight to {}",
                        self.id(),
                        previous_track,
                        self.connection_straight
                    );
                    return self.connection_straight;
                }
                SwitchState::Turnout => {
                    trace!(
                        "Switch   {}: from {} turnout to {}",
                        self.id(),
                        previous_track,
                        self.connection_turnout
                    );
                    return self.connection_turnout;
                }
                SwitchState::Unknown => {
                    error!("Switch   {}: Unknown state", self.id());
                    return "";
                }
            }
        }
        if previous_track == self.connection_straight {
            return self.connection_common;
        }
        if previous_track == self.connection_turnout {
            return self.connection_common;
        }
        error!("Switch   {}: Not connected to {}", self.id(), previous_track);
        ""
    }
    fn connections(&self) -> [&str; 4] {
        [
            self.connection_common,
            self.connection_straight,
            self.connection_turnout,
            "",
        ]
    }
    fn planing_state(&self) -> super::PlaningState {
        self.base.planing_state()
    }
}