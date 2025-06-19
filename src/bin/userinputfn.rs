// use this so that it works
use std::io; 
fn user_input () -> Option<String> {
// take in string as input

    // user input buffer, new string 
    let mut buffer = String::new();
    let input= buffer.trim().to_owned();// make string into owned string 
    
    // while there is a problem with reading the input
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Enter again!");
    }
    let input= buffer.trim().to_owned();
    if &input == ""{
        None
    }
    else{
        Some(input)
    }
}
fn main(){
    // nothing here lol
}