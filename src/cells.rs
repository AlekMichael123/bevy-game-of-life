use bevy::prelude::*;

const DEFAULT_CELLS_GRID_SIZE: usize = 500;
pub const WINDOW_SIZE: f32 = 1000.;
const DEFAULT_CELL_SIZE: f32 = (WINDOW_SIZE as usize / (DEFAULT_CELLS_GRID_SIZE)) as f32;
const CELLS_TIMER_DURATION_SECONDS: f32 = 0.;
const DIRECTIONS: [(isize, isize); 8] = [
  (1, -1),  (1, 0),  (1, 1),
  (0, -1),           (0, 1), 
  (-1, -1), (-1, 0), (-1, 1), 
];

pub struct CellsPlugin;

impl Plugin for CellsPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(CellsTimer(Timer::from_seconds(CELLS_TIMER_DURATION_SECONDS, TimerMode::Repeating)));
    app.add_systems(Startup, init_cells);
    app.add_systems(Update, update_grid);
  }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
enum CellState {
  Alive,
  Dead,
}

#[derive(Bundle)]
struct CellBundle {
  cell_state: CellState,
  sprite: SpriteBundle,
}

#[derive(Resource)]
struct CellsTimer(Timer);

fn init_cells(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
  (0..DEFAULT_CELLS_GRID_SIZE).for_each(|i| {
    (0..DEFAULT_CELLS_GRID_SIZE).for_each(|j| {
      let cell_state = if rand::random() {
        CellState::Alive
      } else {
        CellState::Dead
      };
      let color = if cell_state == CellState::Alive { Color::WHITE } else { Color::BLACK };
      let sprite = SpriteBundle {
        sprite: Sprite {
          color,
          custom_size: Some(Vec2::new(DEFAULT_CELL_SIZE, DEFAULT_CELL_SIZE)),
          ..default()
        },
        transform: Transform::from_xyz( 
          (j as f32 * DEFAULT_CELL_SIZE) + (DEFAULT_CELL_SIZE / 2.) - (WINDOW_SIZE / 2.), 
          (i as f32 * DEFAULT_CELL_SIZE) + (DEFAULT_CELL_SIZE / 2.) - (WINDOW_SIZE / 2.), 
          0.
        ),
        ..default()
      };
      commands.spawn(CellBundle {
        cell_state,
        sprite,
      });
    });
  });
}

fn update_grid(
  time: Res<Time>, 
  mut timer: ResMut<CellsTimer>, 
  mut cells_query: Query<(&mut CellState, &mut Sprite)>,
) {
  if timer.0.tick(time.delta()).just_finished() {
    let cell_states: Vec<CellState> = cells_query.iter().map(|(cell, _)| cell.clone()).collect();
    cells_query
      .iter_mut()
      .enumerate()
      .map(|(i, q)| (i / DEFAULT_CELLS_GRID_SIZE, i % DEFAULT_CELLS_GRID_SIZE, q))
      .for_each(|(i, j, (mut state, mut sprite))| {
        *state = next_cell_states(i, j, &cell_states);
        (*sprite).color = if *state == CellState::Alive { Color::WHITE } else { Color::BLACK };
      });
  }
}

fn next_cell_states(i: usize, j: usize, cell_states: &Vec<CellState>) -> CellState {
  let mut alive_neighbor_count: u8 = 0;
  DIRECTIONS.iter().for_each(|(i_off, j_off)| {
    let i = ((i as isize + i_off + DEFAULT_CELLS_GRID_SIZE as isize) % DEFAULT_CELLS_GRID_SIZE as isize) as usize;
    let j = ((j as isize + j_off + DEFAULT_CELLS_GRID_SIZE as isize) % DEFAULT_CELLS_GRID_SIZE as isize) as usize;
    if cell_states[to_index(i, j)] == CellState::Alive { 
      alive_neighbor_count += 1;
    }
  });
  
  if alive_neighbor_count <= 1 || alive_neighbor_count >= 4 {
    CellState::Dead
  } else if alive_neighbor_count >= 3 {
    CellState::Alive
  } else {
    cell_states[to_index(i, j)].clone()
  }
}

fn to_index(i: usize, j: usize) -> usize {
  (i * DEFAULT_CELLS_GRID_SIZE) + j
}
