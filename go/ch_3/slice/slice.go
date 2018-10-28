package main

import (
	"fmt"
)

func main() {
	// 크기를 지정하면 배열, 지정안하면 슬라이스
	aSliceLiteral := []int{1, 2, 3, 4, 5}
	fmt.Println(aSliceLiteral)

	// 기본 크기를 가진 빈 슬라이스 생성, 0으로 초기화
	integer := make([]int, 20)

	for i := 0; i < len(integer); i++ {
		fmt.Println(integer[i])
	}

	// 슬라이스를 비운다
	aSliceLiteral = nil

	integer = append(integer, -5000)
	// 2번째에서 3번째 사이의 슬라이스를 새로운 슬라이스로 생성
	//s2 := integer[1:3]

	s1 := make([]int, 5)
	reSlice := s1[1:3]
	fmt.Println(s1)
	fmt.Println(reSlice)

	reSlice[0] = -100
	reSlice[1] = 123456
	// 슬라이스로 새로운 슬라이스를 만들었을때 같은 메모리 영역을 참조함
	fmt.Println(s1)
	fmt.Println(reSlice)
}
