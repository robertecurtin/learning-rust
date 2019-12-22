use std::collections::HashMap;

pub fn chapter8() {
  let mut v: Vec<i32> = Vec::new();
  v.push(1);
  v.push(2); // pop also exists, and other things
  let v2 = vec![1, 2];
  println!("v == v2? {}", v == v2); // true

  println!(
    "{:?}, {:?}", // 1, 2
    &v[0],
    match v.get(1) {
      Some(v) => *v,
      None => 0,
    }
  );

  for i in &mut v {
    *i += 10;
  }

  for i in &v {
    println!("{}", i);
  } // 11 12

  #[derive(Debug)]
  enum SomeEnum {
    Int(i32),
    Text(String)
  }

  let v = vec![
    SomeEnum::Int(3),
    SomeEnum::Text(String::from("deal with it"))
  ]; // vEcToRs CaN oNlY CoNtAiN oNe TyPe

  for i in &v {
    println!("{:?}", i);
  }; // Int(3) Text("deal with it")

  let s = "string"; // str
  let s = s.to_string();
  let s2 = "string".to_string();
  println!("s == s2? {}", s == s2); // true
  let mut s = String::from("string"); // another way
  s += " ";
  s.push_str("foo");
  println!("{}", s); // string foo

  let s2 = s + &s2; // s has been moved, s2 is coerced into &str and thus doesn't take ownership
  println!("{}", s2); // string foostring

  let a = String::from("a");
  let b = String::from("b");
  let ab = format!("{}{}", a,b); // same syntax as println!
  println!("{}", ab); // ab

  println!("{}", &ab[0..1]); // a

  for c in "doggo".chars() {
    println!("{}", c); // d o g g o
  }

  for b in "ab".bytes() {
    println!("{}", b); // 97 98
  }

  let mut a = HashMap::new();
  a.insert(String::from("one"), 1); // this takes ownership
  a.insert(String::from("two"), 2);

  let keys = vec![String::from("one"), String::from("two")];
  let values = vec![1, 2];
  let b: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

  for (key, value) in &b {
    println!("{}: {}", key, value);
  } // one: 1 two: 2

  a.insert(String::from("one"), 1); // overwrite value
  println!("{:?}", a); // {"one": 1, "two": 2}

  a.entry(String::from("one")).or_insert(2); // write if it doesn't exist
  // ^ how does this work?

  // Modify an existing entry
  let one = a.entry(String::from("one")).or_insert(0);
  *one += 1;
  println!("{:?}", a.entry(String::from("one")).or_insert(0));

}
