package main

import (
	"fmt"
	"os"
	"strconv"
)

func fib(num int64) int64 {
	if (num <= 2) {
		return 1
	}
	values := make(chan int64)
	go func() {
		values <- fib(num - 1)
	}()
	go func() {
		values <- fib(num - 2)
	}()
	return (<-values) + (<-values)
}

func main() {
	num, _ := strconv.ParseInt(os.Args[1], 0, 64)
	fmt.Println("fib(", num, ") = ", fib(num))
}
