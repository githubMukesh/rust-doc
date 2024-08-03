use std::{io::stdin, string};

fn main() {
    println!("Hello, world!");

    // Shadowing
    // let a = 5;

    // let a = 5;

    // let guess: String = "abcs".parse().expect("Not a number!");
    
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // {
    //    let a = a * 9;

    //    println!("I am printing from curly braces {}", a);
    // }

    // println!("The value of a {} and seconds {} and guess {}", a, THREE_HOURS_IN_SECONDS, guess);

    /* ----------------------------------------------Tuples and Array---------------------------- */

    println!("----------------------------------------------Tuples and Array----------------------------");
 
    // let tup = ("Thomas", 18, 99.99);

    // let (name, age, wealth) = tup;

    // println!("Name {} Age {} Wealth {}", name, age, wealth);

    println!("----------------------Generating error---------------------");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

      stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}
