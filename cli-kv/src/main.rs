use std::collections::HashMap;

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


    
    
    
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    
    //Set
    
    if parts[0].eq_ignore_ascii_case("SET"){
        if parts.len()!=3{
            println!("To Use set function follow this format: Set Key Value");
            continue;
        }
        let key = parts[1].to_string();
        let val = parts[2].to_string();

        store.insert(key, val);

        println!("OK Done!");
        continue;
    }

    
    //Get 

    if parts[0].eq_ignore_ascii_case("Get"){
        if parts.len()!=2{
            println!("To use the Get function follow this format: Get Key");
            continue;
        }

        match store.get(parts[1]){
            Some(value) => println!("Found:{}",value),
            None => println!("Key does not exist"),

        }
    continue;

    }


    //Exists
    if parts[0].eq_ignore_ascii_case("EXISTS") {

    if  parts.len()!=2{
        println!("To use the Exists function use this format : EXISTS key");
        continue;
    }
    


    let key = parts[1];
    let exists = store.contains_key(key);

    println!("{exists}");

    continue;

}


    println!("You entered:{}",input);
    }
}