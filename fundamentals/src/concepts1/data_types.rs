// https://doc.rust-lang.org/book/ch03-02-data-types.html
pub fn example() {
  println!("\n########################## data_types:");

  let number: u32 = 2023230222;
  println!("Integer: {}", number);
  
  let float_number = 2.1 - 2.0;
  println!("Float: {}", float_number);

  // addition
  let sum = 5 + 10;
  println!("Sum: {}", sum);

  // subtraction
  let difference = 95.5 - 4.3;
  println!("Subtraction: {}", difference);

  // multiplication
  let product = 4 * 30;
  println!("Multiplication: {}", product);

  // division
  let quotient = 56.7 / 32.2;
  println!("Division (Float): {}", quotient);
  
  let floored = 2 / 3; // Results in 0
  println!("Division (Int floored): {}", floored);

  // remainder
  let remainder = 43 % 5;
  println!("Remainder: {}", remainder);

  let is_true: bool = false; // with explicit type annotation
  println!("Boolean: {}", is_true);

  // singles quotes only for char values
  let char_z = 'z';
  let char_zz = 'ℤ';
  let heart_eyed_cat = '😻';
  println!("Char: {} {} {}", char_z, char_zz, heart_eyed_cat);

  let tuple: (i32, f64, u8) = (500, 6.4, 1);
  let (_tuple_x, _tuple_y, tuple_z) = tuple;
  println!("Tuple: {}", tuple_z);
  let five_hundred = tuple.0;
  println!("Tuple index: {}", five_hundred);
  
  let numbers = [10, 20, 30, 40, 50];
  println!("Array (numbers): {}", numbers[4]);


  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  println!("Array (months): {}", months[3]);

  //i32 is the type of each element. After the semicolon,
  //the number 5 indicates the array contains five elements.
  let array_typed_length: [i32; 5] = [1, 2, 3, 4, 5];
  for number in array_typed_length {
      print!("{}", number);
  }

  println!("");

  //will contain 5 elements that will all be set to the value 3 initially. 
  //This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
  let array_same_number = [3; 5];
  for number in array_same_number {
      print!("{}", number);
  }

  println!("");

  let array_test = [1, 2, 3, 4, 5];

  let first = array_test[0];
  let second = array_test[1];
  println!("{} {}", first, second);
}
