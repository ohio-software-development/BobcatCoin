package main

import (
	"fmt"
)

func main() {
	var Blockchain []Block

	genesis := genesisBlock()
	Blockchain = append(Blockchain, genesis)
	curBlock := genesis

	for i := 0; i < 10; i++ {
		newBlock := makeBlock(curBlock, "")			// "" will be later be replaced by transaction data
		Blockchain = append(Blockchain, newBlock)
		curBlock = newBlock
		fmt.Println(newBlock)
	}

}