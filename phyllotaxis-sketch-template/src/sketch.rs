//! Your sketch lives here. The runtime and raw ABI are in `phyllo.rs`.

use crate::phyllo::{self, Rgb, Frame, Input, LED_COUNT};

/// Use NONE, AUDIO, INPUT, or AUDIO | INPUT.
pub const CAPABILITIES: u32 = phyllo::capabilities::INPUT;

static mut idx: i32 = 0;
static mut level: i32 = 0;
static indices: [[usize;20];5] = [
    [87, 4, 5, 14, 15, 22, 23, 32, 33, 40, 41, 50, 51, 58, 59, 68, 69, 76, 77, 86],
    [88, 3, 6, 13, 16, 21, 24, 31, 34, 39, 42, 49, 52, 57, 60, 67, 70, 75, 78, 85],
    [89, 2, 7, 12, 17, 20, 25, 30, 35, 38, 43, 48, 53, 56, 61, 66, 71, 74, 79, 84],
    [0, 1, 8, 11, 18, 19, 26, 29, 36, 37, 44, 47, 54, 55, 62, 65, 72, 73, 80, 83],
    [999, 999, 9, 10, 999, 999, 27, 28, 999, 999, 45, 46, 999, 999, 63, 64, 999, 999, 81, 82]
];

pub fn logical_to_physical(ring: i32, index: i32) -> usize {
    return indices[ring as usize][index as usize];
}

pub fn render(input: &Input<'_>, frame: &mut Frame<'_>) {
    unsafe{
    let time = input.phase(8_000_000);

    idx += input.encoder_delta();
    if idx < 0 {
        idx = 19;
    }
    idx %= 20;
    if input.button_released() {
        level += 1;
    }
    level %= 5;

    for led in 0..LED_COUNT {
        let mod18 = led % 18;
        if logical_to_physical(level, idx) == led {
            frame.set(led, Rgb::new(255, 0, 0));
        } else if [4, 5, 14, 15].contains(&mod18) {
            frame.set(led, Rgb::new(0, 255, 0));
        } else if [3, 6, 13, 16].contains(&mod18) {
            frame.set(led, Rgb::new(0, 0, 255));
        } else if [2, 7, 12, 17].contains(&mod18) {
            frame.set(led, Rgb::new(200, 0, 200));
        } else if [0, 1, 8, 11].contains(&mod18) {
            frame.set(led, Rgb::new(200, 200, 0));
        } else if [9, 10].contains(&mod18) {
            frame.set(led, Rgb::new(0, 200, 200));
        } else {
            frame.set(led, Rgb::new(0, 0, 0));
        }
    }
    }
}

