#!([​warn​(​clippy​::all, ​clippy​::pedantic)])

mod camera;
mod map; //letting rust know that i want the map module to compile as well
mod map_builder;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

use prelude::*;

// const FRAME_DURATION: f32 = 75.0; //updates the game physics in every 75ms
// const CENTER_HEADLINE: i32 = 5;
// const CENTER_OPTION_1: i32 = 8;
// const CENTER_OPTION_2: i32 = 9;
// const CENTER_OPTION_3: i32 = 10;

struct State {
    map: Map,
    player: Player,
    camera: Camera,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        Self {
            map: mb.map,
            player: Player::new(mb.player_start), //initail position of player ath the center
            camera: Camera::new(mb.player_start),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        
        self.map.render(ctx, &self.camera);
    
        ctx.set_active_console(1);
        ctx.cls();
        
        self.player.update(ctx, &self.map, &mut self.camera);
        self.player.render(ctx, &self.camera);
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
