// use std::io;
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// 3.1
fn main() {
    // let x = 5;

    // let x = x + 1;

    // {
    //   let x = x * 2;
    //   println!("The value of x in this scope is {x}");
    // }

    // println!("The value of x here is {x}");
    // =========================================================================

    // let spaces = "   ";
    // let spaces = spaces.len();

    // println!("spaces len is {spaces}");

    // =========================================================================

    // let mut spaces = "   ";

    // spaces = "123";
    // println!("spaces value is {spaces}");

    // =========================================================================

    // let mut x:u8 = 255;

    // println!("value of x is {x}");

    // let mut y = String::new();

    // io::stdin()
    //     .read_line(&mut y)
    //     .expect("Failed to readline");

    // let y: u8 = y.trim().parse().expect("Failed to parse number");

    // x = 255 + y;

    // println!("value of x is {x}"); // cargo build --release
    // =========================================================================

    // let sum = 5 + 10;
    // println!("sum is {sum}");

    // let difference = 95.5 -4.3;
    // println!("difference is {difference}");

    // let product = 4 * 30;
    // println!("product is {product}");

    // let quotient = 56.7 / 32.2;
    // println!("quotient is {quotient}");

    // let truncated = -5 / 3;
    // println!("truncated is {truncated}");

    // let remainder = 43 % 5;
    // println!("remainder is {remainder}");
    // =========================================================================

    // let t = true;

    // let f: bool = false;

    // println!("t {t} f {f}");
    // =========================================================================

    // let c = 'z';
    // let z = 'Z';
    // let emoji = '😻';

    // println!("c {c} z {z} emoji {emoji}");
    // =========================================================================

    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // let tup1 = tup.0;
    // println!("tup1 is {tup1}");
    // =========================================================================

    // let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [2, 2, 2, 2, 2];
    // let a = [3; 5];

    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];
    // =========================================================================

    // let a  = [1, 2, 3, 4, 5];
    // println!("Please enter an array index");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered is not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
    // =========================================================================

    // another_function(5);
    // print_labeled_measurement(5, 'h');
    let x = five();
    println!("The value of x is {x}");

    let x: i32 = plus_one(x);
    println!("The value of x is {x}");
}

// fn another_function(x: i32) {
//     println!("The value of x is {x}");
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is {value}{unit_label}");
// }

fn five () -> i32 {
  5
}

fn plus_one (x: i32) -> i32 {
  x + 1
}
