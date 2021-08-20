const SIZE: usize = 100;

struct Game {
  cells: Vec<Vec<bool>>,
}

impl Game {
  fn new() -> Self {
    let g = Game { cells: vec![] };
    g.populate();
    return g;
  }

  fn populate(&self) {
    let rows = &mut vec![];
    for _i in 0..SIZE {
      rows.push(vec![false; SIZE]);
    }
  }

  fn take_turn(&mut self) {
    for row in self.cells.iter_mut() {
      for cell in row.iter_mut() {
        *cell = true;
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn compress_cells(game: &Game) -> Vec<bool> {
    return game
      .cells
      .clone()
      .iter()
      .flat_map(|v| v.iter().map(|&b| b))
      .collect::<Vec<bool>>();
  }

  #[test]
  fn test_take_turn() {
    let mut game = Game::new();
    let before_cells = compress_cells(&game);

    assert_eq!(before_cells, vec![false; SIZE]);

    &game.take_turn();

    let after_cells = compress_cells(&game);

    assert_eq!(after_cells, vec![true; SIZE]);
  }
}
