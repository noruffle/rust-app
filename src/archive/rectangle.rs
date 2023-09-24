fn main() {
  let rectangle = Rectangle {
    width: 2,
    height: 2,
    dimension: None,
  };

  println!(
    "fn check: {};",
    rectangle.area()
  )
}
struct Rectangle {
  width: u32,
  height: u32,
  dimension: Option<(u32, u32)>,
}

impl Rectangle {
  fn check(&self) -> bool {
    self.dimension == None
  }

  fn area(&self) -> u32 {
    if self.check() {
      self.dimension.map_or(
        self.width * self.height, 
        |dimension| dimension.0 * dimension.1
      )
    } else {
      0
    }
  }
}

/* fn main() {
/*   let width = 30;
  let height = 50; */

  let scale = 2;

  let rectangle = Rectangle { 
    width: dbg!(200 * scale), 
    height: 500 
  };

  dbg!(&rectangle);

 /*  println!(
    "the area is {}", 
    area(width, height, (width, height))
  );
  
  println!(
    "function rectangle check: {:?}", 
    area1(&rectangle)
  ); */
}

fn _area1(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn _area(width: u32, height: u32, dimension: (u32, u32)) -> u32 {
  if width != 0 && height != 0 {
    return width * height;
  } else {
    return dimension.0 * dimension.1;
  }
} */