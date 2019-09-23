use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq)]
pub enum Tier {
    Main,
    High,
}

impl Default for Tier {
    fn default() -> Self {
        Tier::Main
    }
}

pub struct SequenceContext {
    pub tier: Tier,
    pub pic_size: (u16, u16), // (width, height)
    pub display_rate: u64,
    pub decode_rate: u64,
    pub header_rate: u16,
    pub mbps: f64,
    pub cr: u8,
    pub tiles: (u8, u8), // (cols, rows)
}

#[derive(Copy, Clone)]
struct LevelLimits {
    max_pic_size: u32,
    max_h_size: u16,
    max_v_size: u16,
    max_display_rate: u64,
    max_decode_rate: u64,
    max_header_rate: u16,
    main_mbps: f64,
    high_mbps: f64,
    main_cr: u8,
    high_cr: u8,
    max_tiles: u8,
    max_tile_cols: u8,
}

#[derive(Copy, Clone)]
pub struct Level(pub u8, Option<LevelLimits>);

impl Level {
    pub fn is_valid(&self) -> bool {
        self.1.is_some()
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let index = self.0;

        if index == 31 {
            write!(f, "Maximum parameters")
        } else if index >= 24 {
            write!(f, "Reserved")
        } else {
            let x = 2 + (index >> 2);
            let y = index & 3;

            write!(f, "{}.{}", x, y)
        }
    }
}

macro_rules! level {
    ($level: expr, $limits: expr) => {
        Level($level, Some($limits))
    };
    ($level: expr) => {
        Level($level, None)
    };
}

pub const LEVELS: [Level; 32] = [
    level!(
        0,
        LevelLimits {
            max_pic_size: 147456,
            max_h_size: 2048,
            max_v_size: 1152,
            max_display_rate: 4423680,
            max_decode_rate: 5529600,
            max_header_rate: 150,
            main_mbps: 1.5,
            high_mbps: 0.0,
            main_cr: 2,
            high_cr: 0,
            max_tiles: 8,
            max_tile_cols: 4,
        }
    ),
    level!(
        1,
        LevelLimits {
            max_pic_size: 278784,
            max_h_size: 2816,
            max_v_size: 1584,
            max_display_rate: 8363520,
            max_decode_rate: 10454400,
            max_header_rate: 150,
            main_mbps: 3.0,
            high_mbps: 0.0,
            main_cr: 2,
            high_cr: 0,
            max_tiles: 8,
            max_tile_cols: 4,
        }
    ),
    level!(2),
    level!(3),
    level!(
        4,
        LevelLimits {
            max_pic_size: 665856,
            max_h_size: 4352,
            max_v_size: 2448,
            max_display_rate: 19975680,
            max_decode_rate: 24969600,
            max_header_rate: 150,
            main_mbps: 6.0,
            high_mbps: 0.0,
            main_cr: 2,
            high_cr: 0,
            max_tiles: 16,
            max_tile_cols: 6,
        }
    ),
    level!(
        5,
        LevelLimits {
            max_pic_size: 1065024,
            max_h_size: 5504,
            max_v_size: 3096,
            max_display_rate: 31950720,
            max_decode_rate: 39938400,
            max_header_rate: 150,
            main_mbps: 10.0,
            high_mbps: 0.0,
            main_cr: 2,
            high_cr: 0,
            max_tiles: 8,
            max_tile_cols: 4,
        }
    ),
    level!(6),
    level!(7),
    level!(
        8,
        LevelLimits {
            max_pic_size: 2359296,
            max_h_size: 6144,
            max_v_size: 3456,
            max_display_rate: 70778880,
            max_decode_rate: 77856768,
            max_header_rate: 300,
            main_mbps: 12.0,
            high_mbps: 30.0,
            main_cr: 4,
            high_cr: 4,
            max_tiles: 32,
            max_tile_cols: 8,
        }
    ),
    level!(
        9,
        LevelLimits {
            max_pic_size: 2359296,
            max_h_size: 6144,
            max_v_size: 3456,
            max_display_rate: 141557760,
            max_decode_rate: 155713536,
            max_header_rate: 300,
            main_mbps: 20.0,
            high_mbps: 50.0,
            main_cr: 4,
            high_cr: 4,
            max_tiles: 32,
            max_tile_cols: 8,
        }
    ),
    level!(10),
    level!(11),
    level!(
        12,
        LevelLimits {
            max_pic_size: 8912896,
            max_h_size: 8192,
            max_v_size: 4352,
            max_display_rate: 267386880,
            max_decode_rate: 273715200,
            max_header_rate: 300,
            main_mbps: 30.0,
            high_mbps: 100.0,
            main_cr: 6,
            high_cr: 4,
            max_tiles: 64,
            max_tile_cols: 8,
        }
    ),
    level!(
        13,
        LevelLimits {
            max_pic_size: 8912896,
            max_h_size: 8192,
            max_v_size: 4352,
            max_display_rate: 534773760,
            max_decode_rate: 547430400,
            max_header_rate: 300,
            main_mbps: 40.0,
            high_mbps: 160.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 64,
            max_tile_cols: 8,
        }
    ),
    level!(
        14,
        LevelLimits {
            max_pic_size: 8912896,
            max_h_size: 8192,
            max_v_size: 4352,
            max_display_rate: 1069547520,
            max_decode_rate: 1094860800,
            max_header_rate: 300,
            main_mbps: 60.0,
            high_mbps: 240.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 64,
            max_tile_cols: 8,
        }
    ),
    level!(
        15,
        LevelLimits {
            max_pic_size: 8912896,
            max_h_size: 8192,
            max_v_size: 4352,
            max_display_rate: 1069547520,
            max_decode_rate: 1176502272,
            max_header_rate: 300,
            main_mbps: 60.0,
            high_mbps: 240.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 64,
            max_tile_cols: 8,
        }
    ),
    level!(
        16,
        LevelLimits {
            max_pic_size: 35651584,
            max_h_size: 16384,
            max_v_size: 8704,
            max_display_rate: 1069547520,
            max_decode_rate: 1176502272,
            max_header_rate: 300,
            main_mbps: 60.0,
            high_mbps: 240.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 128,
            max_tile_cols: 16,
        }
    ),
    level!(
        17,
        LevelLimits {
            max_pic_size: 35651584,
            max_h_size: 16384,
            max_v_size: 8704,
            max_display_rate: 2139095040,
            max_decode_rate: 2189721600,
            max_header_rate: 300,
            main_mbps: 100.0,
            high_mbps: 480.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 128,
            max_tile_cols: 16,
        }
    ),
    level!(
        18,
        LevelLimits {
            max_pic_size: 35651584,
            max_h_size: 16384,
            max_v_size: 8704,
            max_display_rate: 4278190080,
            max_decode_rate: 4379443200,
            max_header_rate: 300,
            main_mbps: 160.0,
            high_mbps: 800.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 128,
            max_tile_cols: 16,
        }
    ),
    level!(
        19,
        LevelLimits {
            max_pic_size: 35651584,
            max_h_size: 16384,
            max_v_size: 8704,
            max_display_rate: 4278190080,
            max_decode_rate: 4706009088,
            max_header_rate: 300,
            main_mbps: 160.0,
            high_mbps: 800.0,
            main_cr: 8,
            high_cr: 4,
            max_tiles: 128,
            max_tile_cols: 16,
        }
    ),
    level!(20),
    level!(21),
    level!(22),
    level!(23),
    level!(24),
    level!(25),
    level!(26),
    level!(27),
    level!(28),
    level!(29),
    level!(30),
    level!(
        31,
        LevelLimits {
            max_pic_size: std::u32::MAX,
            max_h_size: std::u16::MAX,
            max_v_size: std::u16::MAX,
            max_display_rate: std::u64::MAX,
            max_decode_rate: std::u64::MAX,
            max_header_rate: std::u16::MAX,
            main_mbps: std::f64::MAX,
            high_mbps: std::f64::MAX,
            main_cr: std::u8::MAX,
            high_cr: std::u8::MAX,
            max_tiles: std::u8::MAX,
            max_tile_cols: std::u8::MAX,
        }
    ),
];

pub fn calculate_level(context: &SequenceContext) -> Level {
    let mut idx = 0;

    for level in LEVELS[idx..].iter() {
        if let Some(limits) = level.1 {
            if limits.max_pic_size < context.pic_size.0 as u32 * context.pic_size.1 as u32
                || limits.max_h_size < context.pic_size.0
                || limits.max_v_size < context.pic_size.1
                || limits.max_display_rate < context.display_rate
                || limits.max_decode_rate < context.decode_rate
                || limits.max_header_rate < context.header_rate
                || context.tier == Tier::Main
                    && (limits.main_mbps < context.mbps || limits.main_cr < context.cr)
                || context.tier == Tier::High
                    && (limits.high_mbps < context.mbps || limits.high_cr < context.cr)
                || limits.max_tiles < context.tiles.0 * context.tiles.1
                || limits.max_tile_cols < context.tiles.0
            {
                idx += 1;
            }
        }
    }

    LEVELS[idx]
}