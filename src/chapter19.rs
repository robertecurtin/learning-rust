macro_rules! a_macro {
  ( $($x:expr), * ) => {
    // Match any expression, load into x, allow 0+ matches separated by commas
    {
      println!("here we go");
      $( // Repeat for each comma separated value $x
        println!("{}", $x);
      )*
    }
  };
}
pub fn chapter19() {
  a_macro!("doggo"); // here we go doggo
  a_macro!("doggo", "here we go!"); // here we go doggo here we go!
}
