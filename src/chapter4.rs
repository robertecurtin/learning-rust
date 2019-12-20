pub fn chapter4() {
  let x = 5;
  let y = x;
  let z = x;
  println!("{} {} {}", x, y, z); // 5 5

  // Heap types must be cloned:
  let x = String::from("hi");
  let y = x.clone();
  let z = x; // this is fine
  println!("{} {}", y, z); // hi hi

  // Arrays can be copied:
  let x = [1, 2, 3];
  let y = x;
  let z = x;
  println!("{}, {}, {}", x[0], y[1], z[2]);
  // 1, 2, 3

  fn length(s: &String) -> usize {
    s.len()
  }
  let x = String::from("foo");
  println!("{}", length(&x)); // 3

  fn append_something(s: &mut String) {
    s.push_str("bar");
  }
  let mut x = String::from("foo");
  append_something(&mut x);
  println!("{}", x); // foobar

  let s = String::from("hello world");
  let hello = &s[..5];
  let world = &s[6..];
  println!("{} {}", hello, world); // hello world

  let a = [0, 1, 2];
  let slice = &a[0..2]; // [0, 1]
  println!("{}", slice[1]); // 1

}
