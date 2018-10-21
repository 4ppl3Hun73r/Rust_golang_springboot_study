package main

import "fmt"
import "runtime"
import "strconv"
import "strings"

func main() {
	fmt.Println(runtime.Compiler)
	fmt.Println(runtime.GOARCH)
	fmt.Println(runtime.Version())
	fmt.Println(runtime.NumCPU())
	fmt.Println(runtime.NumGoroutine())

	myVersion := runtime.Version()
	major := strings.Split(myVersion, ".")[0][2]
	minor := strings.Split(myVersion, ".")[1]
	m1, _ := strconv.Atoi(string(major))
	m2, _ := strconv.Atoi(minor)

	if m1 == 1 && m2 < 8 {
		fmt.Println("Need Go version 1.8 or higher!")
		return
	}

	fmt.Println("You are using Go versino 1.8 or higher!")

}
