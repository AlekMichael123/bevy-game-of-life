use bevy::prelude::*;

const DEFAULT_CELLS_GRID_SIZE: usize = 10;

#[derive(Clone, Debug)]
pub enum CellState {
  Alive,
  Dead,
}

type CellsRow = Vec<CellState>;
type CellsGrid = Vec<CellsRow>;

#[derive(Component, Debug)]
pub struct Cells(CellsGrid);

pub fn init_cells(mut commands: Commands) {
  commands.spawn(generate_random_cells());
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

pub fn print_cells(cells_query: Query<&Cells>) {
  println!("{:?}", &cells_query.single().0);
}

