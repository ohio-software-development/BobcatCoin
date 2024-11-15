package main

import (
	"fmt"
)

// main is just a testing site until all functionality is completed
func main() {
	var Blockchain []Block

	genesis := genesisBlock()
	Blockchain = append(Blockchain, genesis)
	
	name1 := "jessie"
	name2 := "james"
	amount := 100

	var transaction Transaction = constructTransaction(name1, name2, float32(amount))
	constructBlock(genesis, transaction)


}