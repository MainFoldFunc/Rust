package main

import (
	"ServerForStockSim/routes"
	"fmt"
	"log"
	"net/http"
)

func main() {
	fmt.Println("Server starting at port 8080")
	err := http.ListenAndServe(":8080", nil)
	http.HandleFunc("/add_blockchain", func(w http.ResponseWriter, r *http.Request) {
		// Call AddBlockchain with the table name as part of the request
		routes.AddBlockchain(w, r, "bv")
	})

	if err != nil {
		log.Fatal("Error while server", err)
	}
}
