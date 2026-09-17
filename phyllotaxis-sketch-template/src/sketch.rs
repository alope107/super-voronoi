//! Your sketch lives here. The runtime and raw ABI are in `phyllo.rs`.

use crate::phyllo::{self, Rgb, Frame, Input, LED_COUNT};

/// Use NONE, AUDIO, INPUT, or AUDIO | INPUT.
pub const CAPABILITIES: u32 = phyllo::capabilities::INPUT;

#[unsafe(no_mangle)]
static mut idx: i32 = 0;

pub fn render(input: &Input<'_>, frame: &mut Frame<'_>) {
    unsafe{
    let time = input.phase(8_000_000);

    idx += input.encoder_delta();
    if idx < 0 {
        idx = 89;
    }
    idx %= 90;

    for led in 0..LED_COUNT {
        let mod18 = led % 18;
        if led == idx as usize {
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

