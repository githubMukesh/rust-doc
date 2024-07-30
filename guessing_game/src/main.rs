use std::{cmp::Ordering, io};
use rand::Rng;
fn main() {
    println!("------------Hey welcome to Guessing Game!---------------");

    let secret_number = rand::thread_rng().gen_range( 1, 101);

   loop { 
    let mut guess = String::new();
    println!("Guess the number");
   io::stdin().read_line(&mut guess).expect("Failed to read line ");


   let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

   println!("-----------output-------------");

   match guess.cmp(&secret_number) {
     Ordering::Less => println!("Too small"),
     Ordering::Equal => {
        println!("Whoa!! You have guessed the number");
        break;
     },
     Ordering::Greater => println!("Bit high"),
       
   }

   println!("Your secret number is : {}", secret_number);
   println!("Your guessed: {}", guess);

}
}