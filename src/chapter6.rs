pub fn chapter6() {
  #[derive(Debug)]
  enum SomeEnum {
    ValueA,
    ValueB,
  }

  let a = SomeEnum::ValueA;

  fn print_enum(value: SomeEnum) {
    println!("enum is {:?}", value);
  }
  print_enum(a); // enum is ValueA

  #[derive(Debug)]
  struct SomeStruct {
    some_enum: SomeEnum,
    num: u32,
  };

  let instance = SomeStruct {
    some_enum: SomeEnum::ValueA,
    num: 3,
  };
  println!("{:?}", instance);

  fn print_enum_more(value: SomeEnum) {
    println!(
      "{}",
      match value {
        SomeEnum::ValueA => {
          1
        }
        SomeEnum::ValueB => 2,
      }
    )
  }
  print_enum_more(SomeEnum::ValueB); // 2

  enum OtherEnum {
    A(u8, u8),
    B(String),
  }

  fn print_other_enum(v: OtherEnum) {
    match v {
      OtherEnum::A(a, b) => println!("A: {}, {}", a, b),
      OtherEnum::B(b) => println!("B: {}", b),
    }
  }
  print_other_enum(OtherEnum::A(1, 2)); // A: 1, 2
  print_other_enum(OtherEnum::B("Bee".to_string())); // B: Bee

  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
print!("{}", plus_one(Some(5)).unwrap()); // 6

}
