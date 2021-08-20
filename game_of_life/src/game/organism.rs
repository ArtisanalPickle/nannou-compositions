#[derive(Debug)]

struct Cell {
  x_rel: i32,
  y_rel: i32,
}

trait Organism {
  fn get_center(&self) -> (i32, i32);
  fn get_cells(&self) -> Vec<Box<Cell>>;

  fn grow_or_shrink(&self) {}

  fn take_turn(&self) -> &Self {
    self
  }
}

#[derive(Debug)]
struct MyOrg {
  center: (i32, i32),
  cells: Vec<Box<Cell>>,
}
impl MyOrg {
  fn new(x: i32, y: i32) -> MyOrg {
    return MyOrg {
      center: (x, y),
      cells: vec![],
    };
  }
}
impl Organism for MyOrg {
  fn get_center(&self) -> (i32, i32) {
    return self.center;
  }
  fn get_cells(&self) -> Vec<Box<Cell>> {
    return vec![];
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn test_turn() {
  //   let org = MyOrg::new(0, 0);
  //   assert_eq!(org, org.take_turn())
  // }
  // fn test_init() {
  //   let org = Organism::new(1, 1);
  //   assert_eq!(org.center, (1, 1));
  // }
}
