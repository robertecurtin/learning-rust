
struct Foo {
  a: u8
}

impl Foo {
  fn a(&self) -> u8 {
    self.a
  }
}

struct Bar {
  foo: Foo
}

impl Bar {
  fn foo(&self) -> &Foo {
    &self.foo
  }
}

pub fn chapter17() {
  let bar = Bar {
    foo: Foo {
      a: 1
    }
  };
  println!("foo says: {}", bar.foo().a()) // 1
}
