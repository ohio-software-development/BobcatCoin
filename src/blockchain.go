package main

import (
	"crypto/sha256"
	"encoding/hex"
	"time"
)

type Block struct {
	index 		uint32		// can increase to 64-bit if chain gets bigger
	time 		string
	data 		string		// data about transactions
	prevHash 	string
	hash 		string
}

func genesisBlock() Block {
	genesis := Block {
		index:		0,
		time:		time.Now().String(),
		data: 		"GenesisBlock (First Block)",
		prevHash:   "None",
		hash:		"",
	}
	genesis.hash = makeHash(genesis)
	return genesis
}

func makeBlock(prevBlock Block, data string) Block {
	var curBlock Block

	curBlock.index = prevBlock.index + 1
	curBlock.time = time.Now().String()
	curBlock.data = data
	curBlock.prevHash = prevBlock.hash
	curBlock.hash = makeHash(curBlock)

	return curBlock
}

func makeHash(block Block) string {
	coinInfo := string(block.index) + block.time + block.data + block.prevHash
	hash := sha256.New()
	hash.Write([]byte(coinInfo))

	return hex.EncodeToString(hash.Sum(nil))
}
