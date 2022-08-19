pub fn example() {
  println!("\n########################## functions:");
  another_function();
  fn_parameters(26);
}

fn another_function() {
  println!("Execution of another_function()");
}

fn fn_parameters(x: i32) {
  println!("Value of X is: {x}");
}