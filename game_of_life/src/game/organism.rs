#[derive(Debug)]
pub struct Organism {
  center: (i32, i32),
}

impl Organism {
  fn new(x: i32, y: i32) -> Organism {
    Organism { center: (x, y) }
  }

  fn take_turn(&self) -> Organism {
    Organism {
      center: self.center,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_init() {
    let org = Organism::new(1, 1);
    assert_eq!(org.center, (1, 1));
  }
}
