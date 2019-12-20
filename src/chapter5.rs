pub fn chapter5() {
  #[derive(Debug)] // this trait lets us pretty-print this type
  struct SomeStruct {
    foo: String,
    bar: u32
  }

  let instance = SomeStruct {
    foo: String::from("foo"),
    bar: 12
  };
  println!("{}", instance.foo); // foo

  fn build_struct(bar: u32) -> SomeStruct {
    SomeStruct {
      foo: String::from("foo"),
      bar // field init shorthand
    }
  }

  let instance2 = build_struct(12);
  println!("{} == {}", instance.bar, instance2.bar); // true

  let instance2 = SomeStruct {
    foo: String::from("bloo"),
    ..instance2
  };
  println!("{} != {}", instance.foo, instance2.foo); // false
  println!("{:?}", instance); // single-line pretty-print array contents
  println!("{:#?}", instance); // multi-line pretty-print

  struct Color(i32, i32, i32); // Tuple struct
  let color = Color(0, 1, 2);
  println!("0 == {}", color.0);
  // 0 == 0

  #[derive(Debug)]
  struct AType {
    foo: u32,
    bar: u32
  }

  impl AType {
    fn sum(&self) -> u32 {
      self.foo + self.bar
    }

    fn has_larger_sum_than(&self, other: &AType) -> bool {
      self.sum() > other.sum()
    }
  }

  let instance = AType { foo: 0, bar: 1 };
  println!("the sum is {}", instance.sum());
  // the sum is 1
  let instance2 = AType { foo: 1, bar: 2};
  println!("instance 2 is larger: {}",
    instance2.has_larger_sum_than(&instance));
  // instance 2 is larger: true

  struct ALibrary {};
  impl ALibrary {
    fn a_function(foo: u32) {
      println!("printing {}", foo);
    }
    fn another_function(foo: u32) {
      println!("another {}", foo);
    }
  };

  impl ALibrary { // You can have multiple impl blocks
    fn yet_another_function(foo: u32) {
      println!("yet another {}", foo);
    }
  };

  ALibrary::a_function(12);
  // printing 12
  ALibrary::another_function(32);
  // printing 32
  ALibrary::yet_another_function(45);
  // printing 32
}