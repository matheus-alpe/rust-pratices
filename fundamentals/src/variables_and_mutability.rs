// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
pub fn example() {
    println!("\n########################## variables_and_mutability:");

    let mut x = 5;
    println!("Value of x is: {}", x);
    x = 6;
    println!("Now x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {}s", THREE_HOURS_IN_SECONDS);

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("Value of y i the inner scope is: {}", y);
    }

    println!("Value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Lenght of spaces: {}", spaces);
}
