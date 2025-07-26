use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main (){
    println!("Welcome to te gueesing game ");
    let secret_key = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter  a number ");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("can`t read the line ");

        let guess :u32= match guess.trim().parse(){
            Ok(num )=> num ,
            Err(_)=> continue,
        };

        println!("the number you have guessed {guess}");
        match guess.cmp(&secret_key)
        {
            Ordering::Greater=>println!("It is too big"),
            Ordering::Less=>println!("It is too small"),
            Ordering::Equal=> {
                println!("You have guessed coorect Hooray !!!!!");
                break;
            }
        }


        

    }
}