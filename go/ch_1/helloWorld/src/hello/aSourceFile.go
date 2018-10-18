package main

import (
	"fmt"
	// compile error 사용하지 않는 import "os"
	_ "os" // no error
	// compile error 다운로드 필요 "github.com/mactsouk/go/simpleGitHub"
)

func main() {
	fmt.Println("This is a sample Go Program!")

	v1 := "123"
	v2 := 123
	v3 := "Have a nice day\n"
	v4 := "abc"

	fmt.Print(v1, v2, v3, v4)
	fmt.Println()
	fmt.Println(v1, v2, v3, v4)
	fmt.Print(v1, " ", v2, " ", v3, " ", v4, " ", "\n")
	fmt.Printf("%s%d %s %s\n", v1, v2, v3, v4)
}

/* comiple error
func errorFunc
{

}
*/
