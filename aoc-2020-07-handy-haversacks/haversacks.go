package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type containedBag struct {
	Color string
	Num   int
}

func splitRule(s, sep string) (string, string) {
	x := strings.SplitN(s, sep, 2)
	return x[0], x[1]
}

func recFindContainers(color string, rules map[string][]containedBag, containers map[string]bool) {
	if containingBags, ok := rules[color]; ok {
		// fmt.Println("FOUND", color, ":", containingBags)
		for _, bag := range containingBags {
			containers[bag.Color] = true
			recFindContainers(bag.Color, rules, containers)
		}
	}
}

func main() {
	dat, _ := ioutil.ReadFile("input.txt")
	// dat, _ := ioutil.ReadFile("input.txt")
	var rules = make(map[string][]containedBag)
	for _, rule := range strings.Split(string(dat), "\n") {
		if rule == "" {
			break
		}
		colorPart, containedPart := splitRule(rule, " bags contain ")
		for _, containedBagStr := range strings.Split(containedPart, ", ") {
			numStr, containedColorPart := splitRule(containedBagStr, " ")
			var containedColor = strings.TrimSpace(strings.TrimRight(containedColorPart, "bags."))
			if num, err := strconv.Atoi(numStr); err == nil {
				var bag = containedBag{Color: colorPart, Num: num}
				if _, ok := rules[containedColor]; ok {
					rules[containedColor] = append(rules[containedColor], bag)

				} else {
					rules[containedColor] = make([]containedBag, 1)
					rules[containedColor][0] = bag
				}
			}
		}
	}

	var containers = make(map[string]bool)
	recFindContainers("shiny gold", rules, containers)

	fmt.Println("Result1:", len(containers))
	// fmt.Println("Result2:", res2)
}
