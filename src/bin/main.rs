// interactive bill manager project 



use std::io;
use std::collections::HashMap;

use crate::menu::view_bills; 
#[derive(Debug)]
pub struct Bill{
    name:String,
    amount: f64
}

pub struct Bills{
    inner: HashMap<String, Bill>, // hashmap to be able to change
}

// all the functions are here 
impl Bills{
    fn new() -> Self{
        Self{
            inner:HashMap::new() // make it a hashmap
        }
        
    }
    // add bill
    fn add(&mut self, bill: Bill){
        // move the bill into vector
        self.inner.insert(bill.name.to_string(), bill);

    }

    // get all the needed bills borrowed bills 
    fn get_all(&self) -> Vec<&Bill>{
        // borrowed bills 
        self.inner.values().collect() 
        //iterate and collect the bills, takes the values  
    }
}
// user input function 
fn user_input () -> Option<String>{
    // loop input until done 
    let mut buffer = String::new();
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
    
    // takes out of hashmap 
    fn remove(&mut self, name: &str) -> bool{
        self.inner.remove(name).is_some(); // remove if it is found some bill 
    }
}

// main menu 
enum Menu{
    Add,
    View,
    Remove
}

// module for menu 
mod menu{
    use crate::{Bills, Bill, user_input, convert_bill};
    // public function
    pub fn add_bill(bills: &mut Bills){
        println!("Bill name: ");
        let name =  match user_input(){
            Some(input) => input, 
            None => return,
        };

        let amount = match convert_bill(){
            Some(amount) => amount,
            None => return, 
        }; 

        let bill: Bill = Bill{name: name, amount : amount};
        bills.add(bill);
        println!("bill add complete "); 
        
    }

    // note &mut bills lets you change the bills 
    pub fn remove_bill(bills: &mut Bills){
        for bill in bills.get_all(){
            println!("{:?}", bill); 
        }
        println!("Enter name to remove: "); 

        let name = match user_input(){
            Some(name) => name,
            None => return,
        };

        if bills.remove(&name){
            println!("bill removed");

        }
        else{
            println!("Bruh bill not found"); 
        }
    }

    // &bills lets you read bills 
    pub fn view_bills(bills: &Bills){
        for bill in bills.get_all(){
            // get all gives us all the bills collected
            println!("{:?}", bill);
        }
    }
    
}
// convert string to float 
fn convert_bill() -> Option<f64>{
    println!("Amount: ");
    loop{
        let input = match user_input(){
            Some(input) => input,
            None => return None, 

        };
        // if input case return 
        if &input == ""{
            return None; 
        }

        let parsed_input:Result<f64, _> = input.parse(); // split the input
        match parsed_input{
            Ok(amount) => return Some(amount),
            Err(_)=>println!("Enter a number"),
        }
    }

}
// implement the menu 
impl Menu{
    // user input is borrowed string 
    fn from_str(input: &str) -> Option<Menu>{
        // input from user is borrowed string 
        match input{
            "1" => Some(Self::Add),
            "2" => Some(Self::View),
            "3" => Some(Self::Remove),
            _ => None, 
        }
    }

    fn show_menu(){
        println!("\nMENU FOR BILLS");
        println!("1 - add \n2 - view \n3- remove \nEnter your choice: ");

    }
}
fn main() {
    // loop and actions 
    // bills 
    let mut bills = Bills::new();
    loop{
        // display
        Menu::show_menu();
        let input = user_input().expect("none");
        match Menu::from_str(input.as_str()){
            // choice
            Some(Menu::Add) =>menu::add_bill(&mut bills),
            Some(Menu::View) => menu::view_bills(&bills),
            Some(Menu::Remove) => menu::remove_bill(&mut bills),
            None => return, 
        }        

    }
}
