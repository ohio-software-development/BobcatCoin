package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"time"
)

// note that cryptocurrency does not actually exist within code
// it's just a list, or a ledger of how much money people have sent and recieved
type Transaction struct {
	hash string
	sender string
	reciever string
	amount float32
	time string
}

func constructTransaction(sender string, reciever string, amount float32) Transaction {
    var curTransaction Transaction

    curTransaction.sender = sender
    curTransaction.reciever = reciever
    curTransaction.amount = amount
    curTransaction.time = time.Now().String()
    curTransaction.hash = makeTransactionHash(curTransaction)

    return curTransaction
}

func transactionString(trans Transaction) string {
	str := fmt.Sprintf("%s%s%s%f%s", trans.hash, trans.sender, trans.reciever, trans.amount, trans.time)
	return str
}

func makeTransactionHash(transaction Transaction) string {
	transInfo := transactionString(transaction)
	hash := sha256.New()
	hash.Write([]byte(transInfo))

	return hex.EncodeToString(hash.Sum(nil))
}
