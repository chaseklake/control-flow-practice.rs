// Create functions to do the following:
// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
use std::io;

fn main() {
    println!("1. Convert F <-> C");
    println!("2. Generate Fibonacci number");
    println!("3. Print \"Twelve Days of Christmas\" lyrics with loops");
    
    println!("Select an option:");
    let input = get_int_input();

    match input {
        1 =>  {
            println!("1. Convert F to C");
            println!("2. Convert C to F");
            println!("Select an option: ");
            let option = get_int_input();
            match option {
                1 => {
                    println!("Enter degrees F to convert: ");
                    let degrees = get_float_input();
                    println!("{degrees} degrees F is equal to {} degrees C.", f_to_c(degrees));
                }
                2 => {
                    println!("Enter degrees C to convert: ");
                    let degrees = get_float_input();
                    println!("{degrees} degrees C is equal to {} degrees F.", c_to_f(degrees));
                }
                _ => {
                    println!("Invalid option!");
                }
            }

        }
        2 => {
            println!("Enter the number of Fibonacci numbers to generate: ");
            let x = get_int_input();
            fibonacci(x);
        }
        3 => christmas_carol(),
        _ => println!("invalid option!")
    }
}

// Gets a number value from a user
fn get_int_input() -> u32 {
    return loop {
       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read input");
       
       let input: u32 = match input.trim().parse() {
           Ok(num) => num,
           Err(_) => {
               println!("Input was not a number");
               continue;
           }
       };

       break input;
   };
}

// Gets a float value from a user
fn get_float_input() -> f64 {
    return loop {
       let mut input = String::new();
       io::stdin()
           .read_line(&mut input)
           .expect("Failed to read input");
       
       let input: f64 = match input.trim().parse() {
           Ok(num) => num,
           Err(_) => {
               println!("Input was not a number");
               continue;
           }
       };

       break input;
   };
}

// Converts Fahrenheit to Celcius
fn f_to_c(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

// Converts Celcius to Fahrenheit 
fn c_to_f(c: f64) -> f64 {
    return 9.0 / 5.0 * c + 32.0;
}

// Prints the fibonacci sequence up to the nth value
fn fibonacci(n: u32) {
   let mut fib = 1;
   let mut last_fib = 0;

   for _i in 0..n {
       print!("{fib} ");

       let temp = fib;
       fib += last_fib;
       last_fib = temp;
   }
   // Used to make the spacing prettier
   println!("");
}

// Prints the lyrics to "The Twelve Days of Christmas" using loops
fn christmas_carol() {
    // All of the unique lyrics in the song
    let lines = [
        "a partridge in a pear tree!",
        "two turtle doves, and",
        "three french hens",
        "four calling birds",
        "five golden rings!",
        "six geese a layin'",
        "seven swans a swimmin'",
        "eight maids a milkin'",
        "nine ladies dancin'",
        "ten lords a leapin'",
        "eleven pipers pipin'",
        "twelve drummers drummin'"
    ];

    // Loop through all the days
    for day in 1..13 {
        // Helps write the correct first line of each verse
        let nth = match day {
            1  => "first",
            2  => "second",
            3  => "third",
            4  => "fourth",
            5  => "fifth",
            6  => "sixth",
            7  => "seventh",
            8  => "eighth",
            9  => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelth",
            _  => "invalid"
        };

        // Loop through the relevant lines each verse
        println!("On the {nth} of Christmas, my true love gave to me:");
        let mut count: usize = day;
        while count != 0 {
            println!("{}", lines[count - 1]);
            count -= 1;
        }

        // Put an empty line between each verse
        println!("");
    }
}
