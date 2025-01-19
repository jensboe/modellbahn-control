use crate::track::{buffer::Buffer, straight::Straight, switch::Switch, Track};

static mut track_layout: [&mut dyn Track; 22] = [
    &mut Switch::new("A_d", "D_1a", "A_c", "A_3a", 100, 100),
    &mut Switch::new("A_c", "A_d", "A_1a", "A_2a", 100, 100),
    &mut Straight::new("A_1a", "A_c", "A_1b", 1_500),
    &mut Straight::new_platform("A_1b", "A_1a", "A_a", 200),
    &mut Switch::new("A_a", "B_1a", "A_1b", "A_b", 100, 100),

    &mut Straight::new("A_2a", "A_c", "A_2b", 1_500),
    &mut Straight::new_platform("A_2b", "A_2a", "A_b", 200),
    &mut Switch::new("A_b", "A_a", "A_3b", "A_2b", 100, 100),

    &mut Straight::new("A_3a", "A_d", "A_3b", 1_500),
    &mut Straight::new_platform("A_3b", "A_3a", "A_b", 200),

    &mut Straight::new("B_1a", "A_a", "C_a", 6_000),

    &mut Switch::new("C_a", "B_1a", "C_2a", "C_1a", 100, 100),
    &mut Straight::new("C_1a", "C_a", "C_1b", 1_500),
    &mut Straight::new_platform("C_1b", "C_1a", "C_b", 200),
    &mut Straight::new("C_2a", "C_a", "C_2b", 1_500),
    &mut Straight::new_platform("C_2b", "C_2a", "C_b", 200),

    &mut Switch::new("C_b", "C_c", "C_1b", "C_2b", 100, 100),
    &mut Switch::new("C_c", "D_1a", "C_b", "C_3c", 100, 100),
    &mut Buffer::new("C_3a", "C_3b", 200),
    &mut Straight::new_platform("C_3b", "C_3a", "C_3c", 200),
    &mut Straight::new("C_3c", "C_3b", "C_c", 200),

    &mut Straight::new("D_1a", "C_c", "A_d", 6_000),
];


pub fn get_track_layout() -> &'static mut [&'static mut dyn Track; 22] {
    unsafe { &mut track_layout }
}
