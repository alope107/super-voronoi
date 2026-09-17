//! Your sketch lives here. The runtime and raw ABI are in `phyllo.rs`.

use crate::{frame_ptr, phyllo::{self, random, Frame, Input, LED_COUNT, Rgb}};

/// Use NONE, AUDIO, INPUT, or AUDIO | INPUT.
pub const CAPABILITIES: u32 = phyllo::capabilities::INPUT;

pub const SPAWN_RATE: f32 = 0.01;

static mut difficulty: f32 = 1.0;
static mut initialized: bool = false;
static mut dying: u32 = 0;
static mut idx: usize = 0;
static mut level: usize = 0;
static indices: [[usize;20];5] = [
    [87, 4, 5, 14, 15, 22, 23, 32, 33, 40, 41, 50, 51, 58, 59, 68, 69, 76, 77, 86],
    [88, 3, 6, 13, 16, 21, 24, 31, 34, 39, 42, 49, 52, 57, 60, 67, 70, 75, 78, 85],
    [89, 2, 7, 12, 17, 20, 25, 30, 35, 38, 43, 48, 53, 56, 61, 66, 71, 74, 79, 84],
    [0, 1, 8, 11, 18, 19, 26, 29, 36, 37, 44, 47, 54, 55, 62, 65, 72, 73, 80, 83],
    [999, 999, 9, 10, 999, 999, 27, 28, 999, 999, 45, 46, 999, 999, 63, 64, 999, 999, 81, 82]
];

#[derive(Default, Clone, Copy)]
struct Obstacle {
    frames_remaining: u64,
    color: Rgb,
    frames_per_level: u64,
}
impl Obstacle {
    fn new() -> Obstacle {
        Obstacle {
        color: Rgb {r: 0, g: 0, b: 0},
        frames_per_level: 0,
        frames_remaining: 0,
    }
    }
}

static mut obstacles: [[Obstacle;20]; 5] = [[Obstacle {
    frames_remaining: 0,
    color: Rgb {r: 0, g: 0, b: 0},
    frames_per_level: 0,
}; 20];5];

pub fn logical_to_physical(ring: usize, index: usize) -> usize {
    return indices[ring as usize][index as usize];
}

pub unsafe fn update_obstacles() {
    for ring in 0..5 {
        for position in 0..20 {
            let obs = &mut obstacles[ring][position];
            if obs.frames_remaining != 0 {
                obs.frames_remaining -= 1;
                if obs.frames_remaining == 0 {
                    if ring != 0 {
                        obs.frames_remaining = obs.frames_per_level;
                        obstacles[ring - 1][position] = *obs;
                    }
                    obstacles[ring][position] = Obstacle::new();
                }
            }
        }
    }
}

pub unsafe fn spawn_obstacles() {
    let should_spawn = random();
    if should_spawn < SPAWN_RATE * difficulty {
        for _ in 0..3 {
            let spawn_idx = (random() * 20.0) as usize;
            let obs_time = ((40.0 + random() * 20.0) / difficulty) as u64;

            let obs = Obstacle {
                frames_remaining: obs_time,
                frames_per_level: obs_time,
                color: Rgb::new(255, 0, 0),
            };
            if difficulty > 3.0 {
                if random() < 0.03 * difficulty {
                    obstacles[4][(spawn_idx + 1) % 20] = obs;
                }
            }
            obstacles[4][spawn_idx] = obs;
        }
    }
}

pub fn render(input: &Input<'_>, frame: &mut Frame<'_>) {
    unsafe{
    if !initialized {
        for i in 0..5 {
            for j in 0..20 {
                obstacles[i][j] = Obstacle::new();
            }
        }
        difficulty = 1.0;
        initialized = true;
    }
    let time = input.phase(8_000_000);
    difficulty += 0.0004;

    let mut tmp_idx = idx as i32 - input.encoder_delta();
    if tmp_idx < 0 {
        tmp_idx = 19;
    }
    tmp_idx %= 20;
    idx = tmp_idx as usize;
    if input.button_released() {
        // level += 1;
    }
    level %= 5;

    spawn_obstacles();
    update_obstacles();

    let test_obstacle = obstacles[level][idx];
    if test_obstacle.frames_remaining != 0 {
        dying = 60;
    }

    if dying > 0 {
        dying -= 1;
        let col = Rgb::new(0, 0, 0).lerp(Rgb::new(255, 0, 0), dying as f32 / 60.0);
        for led in 0..LED_COUNT {
            frame.set(led, col);
        }
        if dying == 0 {
            initialized = false;
        }
    } else {
        for pos in 0..20 {
            for ring in 0..5 {
                let led_idx = logical_to_physical(ring, pos);
                let col_idx = pos % 4;
                let cols3 = [Rgb::new(0, 140, 0), Rgb::new(0, 0, 140), Rgb::new(120, 0, 120), Rgb::new(120, 120, 0)];
                let cols = [Rgb::new(0, 180, 0), Rgb::new(0, 0, 180), Rgb::new(140, 0, 140), Rgb::new(140, 140, 0)];
                let cols2 = [Rgb::new(0, 100, 0), Rgb::new(0, 0, 100), Rgb::new(80, 0, 80), Rgb::new(80, 80, 0)];
                if led_idx < 90 {
                    frame.set(led_idx, if ring == 0 {cols[col_idx]} else {cols2[col_idx]});
                }
            }
        }

        frame.set(logical_to_physical(level, idx), Rgb::new(255, 255, 255));

        for ring in 0..5 {
            for pos in 0..20 {
                let obs = &obstacles[ring][pos];
                if obs.frames_remaining > 0 {
                    let obs_idx = logical_to_physical(ring, pos);
                    if obs_idx < 90 {
                        frame.set(logical_to_physical(ring, pos), obs.color);
                    }
                }
            }
        }
    }


    }
}

