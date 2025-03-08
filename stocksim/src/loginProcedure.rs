use rusqlite::{Connection, Result, params};
use std::io;

// Correct module import
use crate::app::mainApp::main_app;

pub fn login_procedure() -> Result<()> {
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter your email for login procedure: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read input");

    println!("Enter password for login procedure: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read input");

    let email = email.trim();
    let password = password.trim();

    // Open database connection (fixed)
    let conn = Connection::open("UsersData.db")?;

    // Check if user exists (fixed)
    let res: Result<i32> = conn.query_row(
        "SELECT id FROM users WHERE email = ?1 AND password = ?2",
        params![email, password],
        |row| row.get(0),
    );

    match res {
        Ok(id) => {
            println!("Login successful! Entering the application...");
            main_app(); // Call the imported function
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No user found. Please check your credentials.");
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
