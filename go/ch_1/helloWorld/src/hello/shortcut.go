package main

import (
	"fmt"
)

func main() {
	m := 123
	//compile error	m := 234
	m = 234

	i, k := 3, m
	j, k := 1, 2

	fmt.Println(m, i, j, k)

}
