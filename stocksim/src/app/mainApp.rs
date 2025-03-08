use rusqlite::{Connection, Result, params};

pub fn main_app() -> Result<()> {
    let conn = Connection::open("Stocks.db")?;
    let curr_price: Result<f64> = conn.query_row(
        "SELECT price FROM bv ORDER BY time DESC LIMIT 1",
        [],
        |row| row.get(0),
    );

    match curr_price {
        Ok(price) => {
            println!(
                "Hello, welcome to the trade simulator. For now, there is only one asset called 'bv'."
            );
            println!("The price will update every 10 seconds.");
            println!("Current price is: {}", price);
        }
        Err(e) => {
            eprintln!("Error retrieving the current price: {}", e);
        }
    }

    Ok(())
}
