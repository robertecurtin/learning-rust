
struct Foo {
  a: u8
}

impl Foo {
  fn a(&self) -> u8 {
    self.a
  }
}

fn foo_init() -> Foo {
  Foo {
    a: 1
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

fn bar_init() -> Bar {
  Bar {
    foo: foo_init()
  }
}

pub fn chapter17() {
  let bar = Bar {
    foo: Foo {
      a: 10
    }
  };
  println!("foo says: {}", bar.foo().a()); // 10


  let bar = bar_init();
  println!("foo says: {}", bar.foo().a()); // 1
}
