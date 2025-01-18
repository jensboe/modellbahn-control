use defmt::{debug, error, trace};

use super::Track;

#[derive(PartialEq, defmt::Format)]
pub enum SwitchState {
    Straight,
    Turnout,
    Unknown,
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
            id,
            connection_common,
            connection_straight,
            length_straight,
            connection_turnout,
            length_turnout,
            state: SwitchState::Straight,
        }
    }
}

impl Track for Switch {
    fn id(&self) -> &str {
        self.id
    }

    fn debug_print(&self) {
        debug!("Switch   {}, current state: {}, length: {} going from {} straight to {} or turnover to {}", self.id(), self.state, self.length(), self.connection_common, self.connection_straight, self.connection_turnout);
    }

    fn length(&self) -> u32 {
        if self.state == SwitchState::Straight {
            return self.length_straight;
        } else {
            return self.length_turnout;
        }
    }

    fn get_next_tracks(&self, previous_track: &str) -> [&str; 3] {
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
        error!("Switch   {}: Not connected to {}", self.id, previous_track);
        ["", "", ""]
    }

    fn get_next_track(&self, previous_track: &str) -> &str {
        if previous_track == self.connection_common {
            match self.state {
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
                    error!("Switch   {}: Unknown state", self.id);
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
        error!("Switch   {}: Not connected to {}", self.id, previous_track);
        ""
    }
    fn get_connections(&self) -> [&str; 4] {
        [
            self.connection_common,
            self.connection_straight,
            self.connection_turnout,
            "",
        ]
    }
}