//! Your sketch lives here. The runtime and raw ABI are in `phyllo.rs`.

use crate::phyllo::{self, Rgb, Frame, Input, LED_COUNT};

/// Use NONE, AUDIO, INPUT, or AUDIO | INPUT.
pub const CAPABILITIES: u32 = phyllo::capabilities::INPUT;

#[unsafe(no_mangle)]
static mut idx: usize = 0;

pub fn render(input: &Input<'_>, frame: &mut Frame<'_>) {
    unsafe{
    let time = input.phase(8_000_000);

    for led in 0..LED_COUNT {
        if led == idx {
            frame.set(led, Rgb::new(255, 0, 0));
        }
        else {
            frame.set(led, Rgb::new(0, 0, 0));
        }
    }
    idx += 1;
    idx = idx % 90;
    }
}

