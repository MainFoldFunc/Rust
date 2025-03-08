package routes

import (
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3" // Correct import for SQLite3
	"log"
	"net/http"
)

// AddBlockchain creates a table with the given name.
func AddBlockchain(w http.ResponseWriter, r *http.Request, name string) {
	// Open SQLite database
	db, err := sql.Open("sqlite3", "./stocks.db")
	if err != nil {
		log.Fatal("Error while opening a database: ", err)
	}
	defer db.Close()

	// Sanitize the table name (important to avoid SQL injection)
	// Here, we'll assume the table name is safe for simplicity
	// In a production application, you should validate the table name properly
	createTable := fmt.Sprintf(`
		CREATE TABLE IF NOT EXISTS %s (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			price REAL NOT NULL,
			time DATETIME DEFAULT CURRENT_TIMESTAMP
		);`, name)

	// Execute the table creation query
	_, err = db.Exec(createTable)
	if err != nil {
		log.Fatal("Error while adding a table to database: ", err)
	}

	// Respond to the client and print confirmation
	fmt.Fprintf(w, "Table '%s' added successfully.", name)
	fmt.Printf("Table '%s' added.\n", name)
}
