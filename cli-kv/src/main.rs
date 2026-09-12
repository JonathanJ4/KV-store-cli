use std::collections::HashMap;
use std::hash::Hash;
use crc32fast::Hasher;

use std::fs::remove_file;
use std::error;
use std::fs::OpenOptions;

use std::fs::File;
use std::io::ErrorKind::NotFound;
use std::io::{self, Write, BufRead, BufReader};


enum Command{
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    Count,
    Clear,
    Help,
    Exit,

}

#[derive(Debug)]
enum ParseError {
    UnknownCommand,
    InvalidArguments,
}

struct Store{
    data:HashMap<String,String>

}

impl Store{
    fn new() -> Self{
        Self{
            data:HashMap::new()
        }
    }
    fn count(&self) -> usize {
        self.data.len()
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn get(&self, key:&str) -> Option<&String>{
        self.data.get(key)
    }

    fn set(&mut self, key: &str, val: &str){
        self.data.insert(key.to_string(), val.to_string());
    }
    
    fn delete(&mut self, key: &str) -> Option<String>{
        self.data.remove(key)

    }
    fn exists(&self, key: &str)->bool{
        self.data.contains_key(key)

    }
}




fn main() {
    println!("Simple Rust KV Store CLI");
    println!("Type EXIT to close the program");
    

  
    let mut store= Store::new();

    
    load_log(&mut store,"store.log");


    let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("store.log")
            .unwrap();

    loop {


        

        print!("kv> ");

        io::stdout()
        .flush()
        .expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
        
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        


        let parts: Vec<&str> = input.split_whitespace().collect();
        
        
        let command = match parse_command(&parts) {
            Ok(command) => command,
            Err(_) => {
                print!("Garbage");
                continue;
            }
    };

        
        if !handle_commands(&mut store,command,&mut file ){
            break;
        }

        

        

    }
}

fn print_help() {
    println!(
        "SET key value 
                GET key 
                DELETE key
                EXISTS key
                COUNT
                CLEAR
                EXIT"
    );
}


/*This is where the recovery happens, so the info from the log file is 
loaded back into the hashmap and also checks for garbage

*/
fn load_log(store: &mut Store, path: &str){
    let file1 = match File::open(path){
  
    Ok(file1) =>file1,
    Err(error) =>{
        let error1 = error.kind();
        if error1 == NotFound{
            return;
        }else {
                panic!("Failed to open log file");
            }
    }   
    };
    let reader = BufReader::new(file1);
    
    
    for line in reader.lines(){ 
            let actual_line = match line {
            Ok(line) => line,
            Err(_) => {
            println!("Failed to read log entry");
            continue;
        }
    };      
            let c = match actual_line.split_once("|"){
                Some((first,second)) => (first,second),
                None => {
                    println!("Error_caught");
                    continue;
                }

            };
          
            let saved_checksum = match c.1.parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid checksum");
                break;
            }
        };
            if checksum(c.0) != saved_checksum {
                println!("Checksum mismatch");
                break;
            }
            

            let parts: Vec<&str> = c.0.split_whitespace().collect();
            match parse_command(&parts) {
            Ok(command) => {apply_command(store, &command);}
            Err(_) => {print!("Garbage_Command");
                        continue;                }
            }
             
            

        }





}
fn parse_command(parts: &[&str]) -> Result<Command,ParseError>{

    if parts.is_empty() {
        print!("Empty");
        return Err(ParseError::InvalidArguments);
        
    }

    let command = parts[0].to_ascii_uppercase();
  

    match command.as_str(){
        "GET" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Get(parts[1].to_string()))
        }
        "SET" => {
            if parts.len() <3 {
                return Err(ParseError::InvalidArguments)
            }
            let s = parts[2..].join(" ");
            Ok(Command::Set(parts[1].to_string(),s.to_string()))
        }

        "DELETE" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Delete(parts[1].to_string()))
        }

        "EXISTS" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Exists(parts[1].to_string()))
        }

        "COUNT" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Count)}

        "CLEAR" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Clear)}

        "HELP" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Help)}

        "EXIT" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Exit)}

        _ => Err(ParseError::UnknownCommand),
    }

    
}

fn apply_command(store: &mut Store, command: &Command,  ){
        match command{
        Command::Set(key, value) => {
                store.set(&key, &value);
                
                
            }
         Command::Delete(key) => {
                store.delete(&key);
                

            }

        Command::Clear => {
                store.clear();
            }
        _ => {}
        }
    }


fn handle_commands(store: &mut Store, command: Command, file: &mut File) -> bool{
match command {
            Command::Set(key, value) => {
                store.set(&key, &value);
                let record = format!("Set {} {}", key, value);
                let sum1 = checksum(&record);

                writeln!(file,"{}|{}",record,sum1).unwrap();
                println!("OK");
                true
            }

            Command::Get(key) => {
                match store.get(&key) {
                Some(value) => println!("Found:{}", value),
                None => println!("Key does not exist"),
                    }
                true
            }

            Command::Count => {
                let count = store.count();
                println!("{count}");
                true
            }

            Command::Clear => {
                store.clear();
                writeln!(file,"Clear|{}",checksum("Clear")).unwrap();
                println!("OK");
                true
            }

            Command::Delete(key) => {
                match store.delete(&key) {
                Some(_) => {
                let input = format!("Delete {}", key);
                writeln!(file,"{}|{}", input, checksum(&input)).unwrap();
                println!("Ok removed");}
                None => println!("Key does not exist"),
                }
                
                true
            }

            Command::Exists(key) => {
                let exist = store.exists(&key);
                println!("{}",exist);
                true
            }


            Command::Exit => {
                println!("Goodbye");
                false
            }

            Command::Help =>{ 
                print_help();
                true
            }
        }
    }

fn checksum(data: &str) -> u32{
    let mut hasher = Hasher::new();
    hasher.update(data.as_bytes());
    return hasher.finalize();
    

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_store_is_empty() {
        let store = Store::new();

        assert_eq!(store.count(), 0);
    }

    #[test]
    fn get_set_store(){
        let mut store = Store::new();

        store.set("Language", "Rust");
        
        match store.get("Language") {
            Some(value) => {
                assert_eq!(value,"Rust");
            }
            None => {
                panic!("Expected key to exist")
            }
        }

    }
    #[test]
    fn check_recovery_state(){
    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("test.log")
    .unwrap();
    let record = "SET language Rust";
    let sum = checksum(record);

    writeln!(file, "{}|{}", record, sum).unwrap();
    
    let record2 = "SET greeting Hello World";
    let sum2 = checksum(record2);

    writeln!(file, "{}|{}", record2, sum2).unwrap();
    
    drop(file);


    let mut store = Store::new();

    load_log(&mut store, "test.log");
    std::fs::remove_file("test.log").unwrap();
    }
}   