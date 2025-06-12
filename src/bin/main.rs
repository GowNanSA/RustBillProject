// interactive bill manager project 
use std::io; 
// user input function 
fn user_input () -> Option<String>{
    // loop input until done 
    let mut buffer = String::new();
    while io::stdin().read_line(&mut bufer).is_err(){
        println!("Enter again!");
    }
    let input = buffer.trim().to_owned;
    if &input = ""{
        None
    }
    else{
        Some(input)
    }
}

// main menu 
fn main() {
   
}
