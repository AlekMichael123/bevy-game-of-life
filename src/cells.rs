//! TODO: YOU SHOULD GET RID OF CELLS AND JUST USE ENUM, THEN TIE TOGETHER THE ENUM WITH A SPRITE!!! FIX IT THEN YOUR GOLDEN BB!!!

use std::borrow::BorrowMut;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

const DEFAULT_CELLS_GRID_SIZE: usize = 10;
pub const WINDOW_SIZE: f32 = 1200.;
const DEFAULT_CELL_SIZE: f32 = (WINDOW_SIZE as usize / (DEFAULT_CELLS_GRID_SIZE)) as f32;
const CELLS_TIMER_DURATION_SECONDS: f32 = 1.;

pub struct CellsPlugin;

impl Plugin for CellsPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(CellsTimer(Timer::from_seconds(CELLS_TIMER_DURATION_SECONDS, TimerMode::Repeating)));
    app.add_systems(Startup, (init_cells, init_sprites));
    app.add_systems(Update, (update_grid, update_grid_colors).chain());
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellState {
  Alive,
  Dead,
}

type CellsRow = Vec<CellState>;
type CellsGrid = Vec<CellsRow>;

#[derive(Component, Debug)]
pub struct Cells(CellsGrid);

#[derive(Resource)]
pub struct CellsTimer(Timer);

pub fn init_cells(mut commands: Commands) {
  commands.spawn(generate_random_cells());
}

pub fn init_sprites(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2dBundle::default());
  (0..DEFAULT_CELLS_GRID_SIZE)
    .into_iter()
    .for_each(|i| {
      (0..DEFAULT_CELLS_GRID_SIZE)
        .into_iter()
        .for_each(|j| {
          commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(DEFAULT_CELL_SIZE, DEFAULT_CELL_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz( 
              (j as f32 * DEFAULT_CELL_SIZE) + (DEFAULT_CELL_SIZE / 2.) - (WINDOW_SIZE / 2.), 
              (i as f32 * DEFAULT_CELL_SIZE) + (DEFAULT_CELL_SIZE / 2.) - (WINDOW_SIZE / 2.), 
              0.
            ),
            ..default()
          });
        });
    });
}

fn generate_random_cells() -> Cells {
  let grid = 
    (0..DEFAULT_CELLS_GRID_SIZE)
      .map(|_| 
        (0..DEFAULT_CELLS_GRID_SIZE)
          .map(|_| {
            if rand::random() {
              CellState::Alive
            } else {
              CellState::Dead
            }
          })
          .collect()
      )
      .collect();
  Cells(grid)
}

pub fn update_grid(time: Res<Time>, mut timer: ResMut<CellsTimer>, mut cells_query: Query<&mut Cells>) {
  if timer.0.tick(time.delta()).just_finished() {
    let mut cells = cells_query.single_mut();
    println!("{:?}", &cells_query.single().0);
  }
}

pub fn update_grid_colors(cells_query: Query<&Cells>, mut query: Query<(&mut Transform, &mut Sprite)>) {
  let concat = cells_query.single().0.concat();
  for (i, (_, mut sprite)) in (&mut query).iter_mut().enumerate() {
    sprite.color = if concat[i] == CellState::Alive {
      Color::WHITE
    } else {
      Color::BLACK
    };
  }
}