package main

import (
	"fmt"
)

func main() {
	iMap := make(map[string]int)
	iMap["k1"] = 12
	iMap["k2"] = 13
	fmt.Println("iMap", iMap)

	anotherMap := map[string]int{
		"k1": 12,
		"k2": 13,
	}
	fmt.Println("anothermap", anotherMap)
	delete(anotherMap, "k1")
	delete(anotherMap, "k2")
	delete(anotherMap, "k1")
	fmt.Println("anotherMap", anotherMap)

	_, ok := iMap["doesItExist"]
	if ok {
		fmt.Println("Exists!")
	} else {
		fmt.Println("Does NOT exist")
	}

	for key, value := range iMap {
		fmt.Println(key, value)
	}

	aMap := map[string]int{}
	aMap["test1"] = 1
	aMap = nil
	fmt.Println(aMap)
	//aMap["test"] = 1 compile error nil로 정의된 map을 사용하려고 해서
	// 웃긴건 println으로 찍으면 map[]로 나오긴 한다..

}
