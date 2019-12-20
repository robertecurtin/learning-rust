fn plus_one(x: i32) -> i32 {
    x + 1 // No semicolon!
}

pub fn chapter3() {
  println!("12_345 == 12345? {}", 12_345 == 12345); // true

  let y = {
    let x = 3;
    x + 1 // No semicolon means this is an expression instead of a statement, and has a return value
  };
  println!("y == {}", y); // y == 4

  println!("1 + 1 == {}", plus_one(1));

  if y == 4 {
    println!("it's four");
  } else {
    println!("how did you do that");
  }

  // if is an expression
  let foo = if y == 4 { 1 } else { 2 };
  println!("foo == {}", foo); // foo == 1

  // this is invalid, types must match
  // let foo = if y == 4 { 1 } else { '2' };

  let mut foo = 2;
  loop {
    foo -= 1;
    println!("let's go!");
    if foo == 0 { break; };
  } // let's go! let's go!

  let bar = loop {
    foo += 1;
    if foo == 10 { break foo * 2; }
  };
  println!("bar == {}", bar); // bar == 20

  while foo > 0 {
    foo -= 5;
    println!("down with foo {}", foo)
  } // down with foo 5 down with foo 0

  let array = [0, 1, 2, 3];
  println!("array[2] = {}", array[2]); // array[2] == 2

  for element in ["hello", "goodbye"].iter() {
    println!("I say {}", element);
  } // I say hello I say goodbye

  for element in ["goodbye", "hello"].iter().rev() {
    println!("I say {}", element);
  } // I say hello I say goodbye


}

