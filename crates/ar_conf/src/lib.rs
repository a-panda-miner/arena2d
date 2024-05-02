// Physical FPS
pub const PFPS: f64 = 60.0;

// Sprites
pub const TILE_W: usize = 6;
pub const TILE_H: usize = 8;
pub const SPRITE_SHEET_W: usize = 242;
pub const SPRITE_SHEET_H: usize = 12;
pub const SPRITE_SCALE_FACTOR: usize = 5;
pub const PLAYER_SPRITE_INDEX: usize = 1;
pub const SPRITESHEET_PATH: &str = "spritesheet.png";
pub const SPRITE_PADDING: f32 = 2.0;
pub const SPRITE_SHEET_OFFSET: f32 = 2.0;

// Window
pub const GRID_COLS: usize = 800;
pub const GRID_ROWS: usize = 300;
pub const GRID_W: usize = GRID_COLS * TILE_W;
pub const GRID_H: usize = GRID_ROWS * TILE_H;
pub const BG_COLOR: (u8, u8, u8) = (181, 212, 220);
pub const VIRTUAL_W: i32 = 640;
pub const VIRTUAL_H: i32 = 360;
// Terrain
pub const NOISE_SCALE: f64 = 13.5;

// Player
pub const PLAYER_BASE_SPEED: f32 = 0.5;
