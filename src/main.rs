use bevy::prelude::*;

pub mod cells;

// run with cargo run --features bevy/dynamic_linking
// or use cargo add bevy -F dynamic_linking


fn main() {
    App::new()
      .add_systems(Startup, cells::init_cells)
      .add_systems(Update, cells::print_cells)
      .run();
}
