// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html
pub fn example() {
  println!("\n########################## functions:");
  
  another_function();
  
  fn_parameters(26);

  print_labeled_measurement(5, 'h');

  statements_and_expressions();

  print_value();
}

fn another_function() {
  println!("Execution of another_function()");
}

fn fn_parameters(x: i32) {
  println!("Value of X is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn statements_and_expressions() {
  let y = {
    let x = 3;
    // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.
    x + 1
  };

  println!("The value of y is: {y}");
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn print_value() {
  let x = five();

  println!("The value of x is: {x}");

  let z = plus_one(five());

  println!("The value of z is: {z}")
}