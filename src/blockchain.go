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
	prev_hash *string	// cryptographic pointer
	hash string
	nonce int8			// "number used once"
}

func genesis_block() *Block {
	genesis := &Block {
		index: 0,
		time: time.Now().String(),
		prev_hash: nil,
		hash: "",
	}

	genesis.set_hash()
	return genesis
}

func construct_block(prev_block *Block) *Block {
	cur_block := &Block {
		index: prev_block.index + 1,
		time: time.Now().String(),
		prev_hash: &prev_block.hash,
		nonce: 0,
		hash: "",
	}

	cur_block.set_hash()
	return cur_block
}

func (b Block) print() {
	fmt.Printf(``)

}

func (b Block) get_string() string {
	str := fmt.Sprintf("%d%s%s%s", b.index, b.time, b.prev_hash)
	return str
}

func (b *Block) set_hash() {
	coinInfo := b.get_string()
	hash := sha256.New()
	hash.Write([]byte(coinInfo))

	b.hash = hex.EncodeToString(hash.Sum(nil))
}

// it is standard that hashes start with multiple zeros
func (b  Block) hash_check() bool {
	return b.hash[:4] == "0000"
}

func mine_block (block *Block) {
	/* will code later */
}
