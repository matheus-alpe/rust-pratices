// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
pub fn example() {
  println!("\n########################## ownership:");

  /* Rules:
  - Each value in Rust has an owner.
  - There can only be one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
  */
  variable_scope();
  string_type();
  stack_only_data_copy();
  variables_and_data_interact_move();
  variables_and_data_interact_clone();
  ownership_and_functions();
  return_values_and_scope();
  returning_multi_values();
}

fn variable_scope() {
  let s = "hello";

  {
    let s = "word";
    println!("inner: {}", s);
  }

  println!("outer: {}", s);
}

fn string_type() {
  let mut s = String::from("Hello");
  s.push_str(", world!");
  println!("{}", s);
}

fn stack_only_data_copy() {
  let x = 5;
  let y = x;

  println!("x = {}, y = {}", x, y);
}

fn variables_and_data_interact_move() {
  let s1 = String::from("hello");
  let s2 = s1;

  println!("{}, world!", s2);
}

fn variables_and_data_interact_clone() {
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {}", s1);
  println!("s2 = {}", s2);
}

fn ownership_and_functions() {
  let s = String::from("hi");
  takes_ownership(s);
  // println!("s = {}", s); // <- error, borrowed value
  
  let x = 5;
  makes_copy(x);
  println!("x = {}", x); /* <- doesn't error, because of this:
  So what types implement the Copy trait? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can implement Copy, and nothing that requires allocation or is some form of resource can implement Copy. Here are some of the types that implement Copy:

  All the integer types, such as u32.
  The Boolean type, bool, with values true and false.
  All the floating point types, such as f64.
  The character type, char.
  Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
   */
}

fn takes_ownership(some_string: String) {
  println!("takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("makes_copy: {}", some_integer);
}

fn return_values_and_scope() {
  let s1 = gives_ownership();// gives_ownership moves its return value into s1
  println!("s1 = {}", s1);

  let s2 = String::from("hello");// s2 comes into scope

  let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
  println!("s2 = {}", s3);

} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {// gives_ownership will move its return value into the function that calls it
  let some_string = String::from("yours"); // some_string comes into scope

  some_string// some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
  a_string  // a_string is returned and moves out to the calling function
}

fn returning_multi_values() {
  let s1 = String::from("Hello");

  let (s2, len) = calculate_length(s1);
  println!("The length of '{s2}' is {len}");
}

fn calculate_length(value: String) -> (String, usize) {
  let length = value.len();
  // returning values in a tuple
  (value, length)
}