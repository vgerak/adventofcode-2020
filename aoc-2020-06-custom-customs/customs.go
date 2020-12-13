package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	var res1 = 0
	var res2 = 0

	dat, _ := ioutil.ReadFile("input.txt")
	for _, group := range strings.Split(string(dat), "\n\n") {
		var keys1 = make(map[rune]bool)
		var keys2 = make(map[rune]bool)

		for i, answer := range strings.Split(group, "\n") {
			for _, c := range answer {
				keys1[c] = true
				if i == 0 {
					keys2[c] = true
				} else {
					for c := range keys2 {
						if !strings.ContainsRune(answer, c) {
							delete(keys2, c)
						}
					}
				}
			}
		}
		res1 += len(keys1)
		res2 += len(keys2)
	}
	fmt.Println("Result1:", res1)
	fmt.Println("Result2:", res2)
}
