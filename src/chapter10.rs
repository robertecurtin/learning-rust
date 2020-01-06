use std::ops::Add;

pub fn chapter10() {
  fn add<T: Add>(a: T, b: T) -> T::Output {
    a + b
  }

  let a: u8 = 10;
  let b: u8 = 15;
  println!("u8: 10 + 15 == {}", add(a, b));

  let a: u16 = 10;
  let b: u16 = 15;
  println!("u16: 10 + 15 == {}", add(a, b));

  struct Foo<T> { a: T }
  impl<T> Foo<T> {
    fn a(&self) -> &T {
      &self.a
    }
  }

  let foo = Foo { a: 11 };

  println!("foo.a() == {}", foo.a()); // 11

  impl Foo<u8> {
    fn b(&self) {
      println!(" This only prints for u8 values");
    }
  }

  foo.b(); // u8, only available for u8s

  struct FooWithLifetime<'a> {
    bar: &'a str
  }

  let foo = FooWithLifetime {
    bar: "str"
  };
  println!("foo.bar = {}", foo.bar);
}