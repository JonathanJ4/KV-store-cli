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
                print_help();
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

        handle_get(&store, parts[1]);
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


    //Delete 
    if parts[0].eq_ignore_ascii_case("Delete") {

    if  parts.len()!=2{
        println!("To use the Delete function use this format : Delete key");
        continue;
    }

    match store.remove(parts[1]){
        Some(_) => println!("Ok removed"),
        None => println!("Key does not exist")

    }
    continue;

}


    //Count 
    if parts[0].eq_ignore_ascii_case("COUNT") {

    if parts.len()!=1 {
        println!("To use the count function the format is: COUNT");
        continue;
    }
    handle_count(&store);
    

    continue;
}
    //Clear 
    if parts[0].eq_ignore_ascii_case("CLEAR") {
    if parts.len()!=1 {
        println!("To use the Clear function the format is: Clear");
        continue;
    }

    handle_clear(&mut store);
    continue;
}

    println!("Unkown command. Type Help for available commands.");
    }
}


fn print_help(){
      println!("SET key value 
                GET key 
                DELETE key
                EXISTS key
                COUNT
                CLEAR
                EXIT");
                
    }

fn handle_count(store: &HashMap<String,String>){

    let count = store.len();

    println!("{count}");
    
}

fn handle_clear(store: &mut HashMap<String,String>){
        store.clear();
        println!("OK");
}

fn handle_get(store: &HashMap<String,String>, key: &str){

    match store.get(key){
            Some(value) => println!("Found:{}",value),
            None => println!("Key does not exist"),

        }

        
}


