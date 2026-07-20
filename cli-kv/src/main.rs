use std::io::{self,Write};

fn main(){
    println!("Simple Rust KV store");
    println!("Type exit to close the program");

    loop{
        print!("kv> ");
    

    io::stdout()
        .flush() 
        .expect("Failed to flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");


    let input = input.trim();


    if input.is_empty(){
        continue;

    }

    if input.eq_ignore_ascii_case("EXIT"){
        println!("Goodbye");
        break;
    }   

    println!("You entered:{}",input);
    }
}