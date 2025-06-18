// interactive bill manager project 
use std::io; 
// user input function 
fn user_input () -> Option<String>{
    // loop input until done 
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
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
enum Menu{
    Add,
    View
}

// implement the menu 
impl Menu{
    // user input is borrowed string 
    fn str(input: &str) -> Option<Menu>{
        // input from user is borrowed string 
        match input{
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            _ => None, 
        }
    }

    fn showMenu(){
        println!("\n MENU FOR BILLS");
        println!("1 - add \n 2 - view \n Enter your choice: ");

    }
}
fn main() {
    // loop and actions 
    // bills 
    loop{
        // display

        // user input choice 

    }
}
