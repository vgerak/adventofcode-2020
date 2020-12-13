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

func recFindContainers(color string, rulesContained map[string][]string, containers map[string]bool) {
	if containingBags, ok := rulesContained[color]; ok {
		for _, bagColor := range containingBags {
			containers[bagColor] = true
			recFindContainers(bagColor, rulesContained, containers)
		}
	}
}

func recCountContained(color string, rulesContains map[string][]containedBag, bagCount map[string]int) (count int) {
	count = 1
	if numBags, ok := bagCount[color]; ok {
		return numBags
	}
	if containedBags, ok := rulesContains[color]; ok {
		for _, bag := range containedBags {
			var numBags = recCountContained(bag.Color, rulesContains, bagCount)
			bagCount[bag.Color] = numBags
			count += bag.Num * numBags
		}
	}
	// fmt.Printf("color %q adds %d bags\n", color, count)
	return
}

func main() {
	dat, _ := ioutil.ReadFile("input.txt")
	// dat, _ := ioutil.ReadFile("testinput.txt")
	// dat, _ := ioutil.ReadFile("testinput2.txt")
	var rulesContains = make(map[string][]containedBag)
	var rulesContained = make(map[string][]string)
	for _, rule := range strings.Split(string(dat), "\n") {
		if rule == "" {
			break
		}
		colorPart, containedPart := splitRule(rule, " bags contain ")
		rulesContains[colorPart] = make([]containedBag, 0)
		for _, containedBagStr := range strings.Split(containedPart, ", ") {
			numStr, containedColorPart := splitRule(containedBagStr, " ")
			var containedColor = strings.TrimSpace(strings.TrimRight(containedColorPart, "bags."))
			if num, err := strconv.Atoi(numStr); err == nil {
				var bag = containedBag{Color: containedColor, Num: num}
				rulesContains[colorPart] = append(rulesContains[colorPart], bag)

				if _, ok := rulesContained[containedColor]; ok {
					rulesContained[containedColor] = append(rulesContained[containedColor], colorPart)
				} else {
					rulesContained[containedColor] = make([]string, 1)
					rulesContained[containedColor][0] = colorPart
				}
			}
		}
	}
	var containers = make(map[string]bool)
	recFindContainers("shiny gold", rulesContained, containers)

	var bagCount = make(map[string]int)
	fmt.Println("Result1:", len(containers))
	fmt.Println("Result2:", recCountContained("shiny gold", rulesContains, bagCount)-1)
}
