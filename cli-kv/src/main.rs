use std::io::{self,Write};

fn main(){
    println!("Simple Rust KV Store CLI");
    println!("Type EXIT to close the program");

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
        println!("SET key value \n
                GET key \n
                DELETE key\n
                EXISTS key\n
                COUNT\n
                CLEAR\n
                EXIT")

    }

    println!("You entered:{}",input);
    }
}