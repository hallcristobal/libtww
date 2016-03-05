use system::memory::{write, write_str};

pub const NO_LAYER_OVERRIDE: i8 = -1;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FadeOut {
    Default = 0,
}

#[repr(C)]
#[derive(Clone)]
pub struct Entrance {
	pub stage: [u8; 8],
    pub entrance: u16,
    pub room: u8,
}

#[repr(C)]
#[derive(Clone)]
pub struct Warp {
    pub entrance: Entrance,
    pub layer_override: i8,
    pub enabled: bool,
    pub fadeout: FadeOut,
}

impl Warp {
    pub fn new(stage: &str,
               entrance: u16,
               room: u8,
               layer_override: i8,
               fadeout: FadeOut,
               enabled: bool)
               -> Self {
        let mut warp = Warp {
            entrance: Entrance {
            	stage: [0; 8],
            	entrance: entrance,
            	room: room
            },
            layer_override: layer_override,
            enabled: enabled,
            fadeout: fadeout,
        };
        write_str(warp.entrance.stage.as_mut_ptr(), stage);
        warp
    }

    pub fn execute(self) {
        write(0x803BD248, self);
    }
}