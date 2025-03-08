mod addAccount;
mod app;
mod loginProcedure;
use std::io;
fn main() {
    println!("--- Welcome to the stockSim ---");
    let mut input = String::new();
    println!("Do you have account in our application");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    if input.trim() == "no" {
        addAccount::add_account();
    } else {
        loginProcedure::login_procedure();
    }
}
