package main

import (
	"fmt"
)

type Digit int
type Power2 int

const PI = 3.1415926

const (
	C1 = "C1C1C1"
	C2 = "C2C2C2"
	C3 = "C3C3C3"
)

func main() {
	const s1 = 123
	var v1 float32 = s1 * 12
	fmt.Println(v1)
	fmt.Println(PI)

	const (
		Zero Digit = iota // 상수 생성자 (연속으로 값을 정의하게 해줌))
		One
		Two
		Three
		Four
	)
	fmt.Println(One)
	fmt.Println(Two)

	const (
		p2_0 Power2 = 1 << iota // 자동으로 값이 증가하는데 _ 부분으로 불필요한 정의가 없게 값 세팅
		_
		p2_2
		_
		p2_4
		_
		p2_6
	)

	fmt.Println("2^0", p2_0)
	fmt.Println("2^2", p2_2)
	fmt.Println("2^4", p2_4)
	fmt.Println("2^6", p2_6)
}
