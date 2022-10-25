pub fn example() {
  println!("\n########################## control_flow:");

  if_expression();
  multi_if_expression();
  if_in_statement();
  repetition_loop();
  loop_labels();
  loop_with_while();
  loop_collection();
  for_countdown();
}

fn if_expression() {
  let number = 7;

  if number < 5 {
    println!("Condition was true");
  } else {
    println!("Condition was false");
  }

  if number != 0 {
    println!("Number was something other than zero");
  }
}

fn multi_if_expression() {
  let number = 61;

  if number % 4 == 0 {
    println!("Number is divisible by 4");
  } else if number % 3 == 0 {
    println!("Number is divisible by 3");
  } else if number % 2 == 0 {
    println!("Number is divisible by 2");
  } else {
    println!("Number is not divisible by 4, 3 or 2");
  }
}

fn if_in_statement() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");
}

fn repetition_loop() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      print!(".{counter}\n");
      break counter * 3;
    } 
    
    if counter == 1 {
      print!("{counter}");
    } else {
      print!(".{counter}");
    }
  };

  println!("The result is {result}");
}

fn loop_labels() {
  let mut count = 0;

  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 8 {
        break;
      }
    
      if count == 2 {
        break 'counting_up;
      }

      remaining -= 1;
    }

    count += 1;
  }

  println!("End count = {count}");
}

fn loop_with_while() {
  let mut number = 3;

  while number != 0 {
    println!("{number}!");
    number -= 1;
  };

  println!("LIFTOFF!");
}

fn loop_collection() {
  let a = [10, 20, 30, 40, 50, 60];
  let mut index = 0;

  println!("Collection length: {}", a.len());

  while index < a.len() {
    println!("The {index}º value is: {}", a[index]);

    index += 1;
  }

  for element in a {
    println!("Element is: {element}")
  }
}

fn for_countdown() {

  for number in (1..6).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!");
}