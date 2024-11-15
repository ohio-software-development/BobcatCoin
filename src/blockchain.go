package main

import (
	"fmt"
	"crypto/sha256"
	"encoding/hex"
	"time"
)

type Block struct {
	index uint64
	time string
	transactions []Transaction
	totalTransactions uint16
	prevHash string
	hash string
	nonce int8			// randomly generated number
}

func genesisBlock() Block {
	genesis := Block {
		index: 0,
		time: time.Now().String(),
		transactions: []Transaction{},
		totalTransactions: 0,
		prevHash: "None",
		hash: "",
	}
	genesis.hash = makeHash(genesis)
	return genesis
}

func constructBlock(prevBlock Block, transactions []Transaction) Block {
	var curBlock Block

	curBlock.index = prevBlock.index + 1
	curBlock.time = time.Now().String()
	curBlock.transactions = transactions
	curBlock.totalTransactions = 0;
	curBlock.prevHash = prevBlock.hash
	curBlock.nonce = 0
	curBlock.hash = makeHash(curBlock)

	return curBlock
}

func coinString(block Block) string {
	str := fmt.Sprintf("%d%s%s%s", block.index, block.time, block.transactions, block.prevHash)
	return str
}

func makeHash(block Block) string {
	coinInfo := coinString(block)
	hash := sha256.New()
	hash.Write([]byte(coinInfo))

	return hex.EncodeToString(hash.Sum(nil))
}

// it is standard that hashes start with multiple zeros
func hashCheck(hash string) bool {
	return hash[:4] == "0000"
}

func mineBlock(block Block) {
	/* will code later */
}
