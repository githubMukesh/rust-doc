pub fn multiple_condition() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

pub fn printTable(number: i32) {
   let mut multiplyBy = 1;
   println!("Printing {} Table", number);
   loop {
        if(multiplyBy > 10){
            println!("--------End ------");
            break;
        }
        println!("{} * {} = {}", number, multiplyBy, number * multiplyBy);
        multiplyBy+=1;  
   }

   println!("--------Start of for loop ------");
   for multiple  in (1..11){
  
    println!("{} * {} = {}", number, multiple, number * multiple);

   }
   println!("--------End of for loop ------");

}

pub fn printTableWithWhile(number: i32) {
    let mut multiplyBy = 1;

    while multiplyBy <= 10 {
        println!("{} * {} = {}", number, multiplyBy, number * multiplyBy); 
        multiplyBy+=1;
    }
    println!("--------End ------");
}

pub fn iterateOverArray(arr: &mut [i32]) {
  
  for element in arr {
    println!("Value of item {}", element);
  } 
}