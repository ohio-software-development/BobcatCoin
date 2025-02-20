package main

import (
	"fmt"
	"time"
)

type Transaction struct {
	sender string
	reciever string
	amount float32
	time string
}

type Transaction_List struct {
	transactions []Transaction
	total uint16
}

func (t Transaction) print() {
	fmt.Printf("Sender: %s Reciever: %s Amount: %f", t.sender, t.reciever, t.amount)
}

func (list *Transaction_List) add(t Transaction) {
	list.transactions = append(list.transactions, t)
	list.total++
}