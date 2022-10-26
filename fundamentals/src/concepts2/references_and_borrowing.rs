// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
pub fn example() {
  println!("\n########################## references and borrowing:");

  using_references(); // & (ampersands) represents `references`, a pointer.
  mutable_references();
}

fn using_references() {
  let s1 = String::from ("Hello");
  let len = calculate_length(&s1);
  println!("The length of '{s1}' is {len}.");
}

fn calculate_length(value: &String) -> usize {
  value.len()
}

fn mutable_references() {
  let mut s = String::from("Hello");
  change(&mut s);
  println!("{s}");
}

fn change(value: &mut String) {
  value.push_str(", mutable world");
}