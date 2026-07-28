use std::collections::HashMap;
use std::hash::Hash;
use std::io::{self,Write};




fn main(){
    println!("Simple Rust KV Store CLI");
    println!("Type EXIT to close the program");


    let mut store:HashMap<String,String>= HashMap::new();
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
    
    if input.eq_ignore_ascii_case("HELP"){
        println!("SET key value 
                GET key 
                DELETE key
                EXISTS key
                COUNT
                CLEAR
                EXIT");
                continue;
    }

    println!("You entered:{}",input);
    }
}