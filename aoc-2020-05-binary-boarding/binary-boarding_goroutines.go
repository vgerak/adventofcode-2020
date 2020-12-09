package main

import (
	"fmt"
	"io/ioutil"
	"strings"
	"sync"
)

func binsearch(low int, high int, s string) int {
	// fmt.Println(low, high, s)
	if s == "" {
		return low
	}
	if s[0] == '1' {
		return binsearch(low+(high-low)/2+1, high, s[1:])
	}
	return binsearch(low, low+(high-low)/2, s[1:])
}

func worker(pass string, wg *sync.WaitGroup, seatIDs chan int, bitmap *[127*8 + 7]bool) {
	defer wg.Done()
	if pass != "" {
		var s = strings.ReplaceAll(strings.ReplaceAll(pass, "B", "1"), "R", "1")
		var row = binsearch(0, 127, s[:7])
		var col = binsearch(0, 7, s[7:])
		var seatID = row*8 + col
		bitmap[seatID] = true
		seatIDs <- seatID
	}
}

func main() {
	var maxID = 0
	var bitmap [127*8 + 7]bool
	seatIDs := make(chan int)
	wg := &sync.WaitGroup{}
	dat, _ := ioutil.ReadFile("input.txt")
	for _, pass := range strings.Split(string(dat), "\n") {
		// fmt.Println("Starting worker: ", i)
		wg.Add(1)
		pass := pass
		go worker(pass, wg, seatIDs, &bitmap)
	}

	// fmt.Println("Waiting for workers...")
	go func() {
		wg.Wait()
		close(seatIDs)
	}()

	for seatID := range seatIDs {
		if seatID > maxID {
			maxID = seatID
		}
	}
	fmt.Println("Result1:", maxID)
	for i, n := range bitmap {
		if i > 0 && !n && bitmap[i-1] && bitmap[i+1] {
			fmt.Println("Result 2", i)
			return
		}
	}
}
