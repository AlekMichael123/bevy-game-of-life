use bevy::{prelude::*, sprite::Wireframe2dPlugin};

pub mod cells;

// run with cargo run --features bevy/dynamic_linking
// or use cargo add bevy -F dynamic_linking


fn main() {
  App::new()
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
          primary_window: Some(Window {
              resolution: (cells::WINDOW_SIZE, cells::WINDOW_SIZE).into(),
              resizable: false,
              ..default()
          }),
          ..default()
      }),
      cells::CellsPlugin,
      Wireframe2dPlugin,
    ))
    .run();
}
