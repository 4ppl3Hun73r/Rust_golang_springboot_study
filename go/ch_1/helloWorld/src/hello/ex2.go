package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	var f *os.File
	f = os.Stdin
	defer f.Close()

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		input := scanner.Text()
		if input == "STOP" {
			os.Exit(0)
		}
		n, err := strconv.ParseInt(input, 0, 64)
		if err == nil {
			fmt.Println(">", n)
		}
	}
}
