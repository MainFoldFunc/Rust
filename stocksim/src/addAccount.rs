use rusqlite::{Connection, Result, params};
use std::io;

use crate::loginProcedure;

pub fn add_account() -> Result<()> {
    let mut email_u = String::new();
    let mut password_u = String::new();

    println!("Enter your email: ");
    io::stdin()
        .read_line(&mut email_u)
        .expect("Failed to read a line");

    println!("Enter your password: ");
    io::stdin()
        .read_line(&mut password_u)
        .expect("Failed to read a line");

    // Trim the input to remove newlines
    let email_u = email_u.trim();
    let password_u = password_u.trim();

    // Open the database connection (fixed)
    let conn = Connection::open("UsersData.db")?;

    // Create table if it doesn't exist (fixed SQL)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    // Insert user into the database
    conn.execute(
        "INSERT INTO users (email, password) VALUES (?1, ?2)",
        params![email_u, password_u],
    )?;

    println!("Added user {} to database", email_u);
    println!("Redirecting you to the login screen");
    loginProcedure::login_procedure();
    Ok(())
}
