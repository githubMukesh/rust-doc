fn main() {
    println!("Hello, world!");

    let a = 5;

    let a = 5;
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    {
       let a = a * 9;

       println!("I am printing from curly braces {}", a);
    }

    println!("The value of a {} and seconds {}", a, THREE_HOURS_IN_SECONDS)
}
