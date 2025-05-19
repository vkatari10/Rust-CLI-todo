use std::io;

fn main() {

        println!("Enter something.");

        let mut ui = String::new();
        
        io::stdin().read_line(&mut ui).expect("Something went wrong");
 
        if ui.trim() == "hello" {
            println!("hello");
        }

        println!();

} // main



