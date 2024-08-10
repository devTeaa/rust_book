use std::io;

fn main() {
    // println!("Convert Fahrenheit to Celcius");

    // let mut input_degree = String::new();

    // io::stdin()
    //     .read_line(&mut input_degree)
    //     .expect("Unable to read line");

    // let input_degree: i32 = input_degree.trim().parse().expect("input is not a number");
    // // let input_degree: i32 = match input_degree.trim().parse() {
    // //   Ok(num) => num,
    // //   Err(_) => ""
    // // };

    // println!("Celcius value is {}", input_degree - 30 / 2);
    // =========================================================================

    println!("Input fibonacci numbers");

    let mut input_number = String::new();

    io::stdin()
      .read_line(&mut input_number)
      .expect("Unable to read line");

    let input_number: i32 = input_number.trim().parse().expect("input is not a number");

    let mut fib_a = 0;
    let mut fib_b = 1;

    println!("{}", fib_a);
    println!("{}", fib_b);

    for _ in 0..input_number {
      let fib_c = fib_a + fib_b;
      println!("{}", fib_c);

      fib_a = fib_b;
      fib_b = fib_c;
    };
    // =========================================================================
}
