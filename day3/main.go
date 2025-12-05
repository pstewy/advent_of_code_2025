package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// part 2
func main() {
	raw, err := os.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	count := 0
	for _, line := range strings.Split(string(raw), "\n") {
		count += findLargestTwelveDigitNumber(line)
	}
	fmt.Println(count)
}

func findLargestTwelveDigitNumber(line string) int {
	s := stack{inner: []int{}}
	removalsLeft := len(line) - 12
	for _, numStr := range line {
		num, _ := strconv.Atoi(string(numStr))
		for len(s.inner) != 0 {
			if removalsLeft == 0 || s.Peak() >= num {
				break
			}
			s.Pop()
			removalsLeft--
		}
		s.Push(num)
	}
	for removalsLeft > 0 {
		s.Pop()
		removalsLeft--
	}
	workingNumber := ""
	for _, v := range s.inner {
		workingNumber += strconv.Itoa(v)
	}
	finalNum, _ := strconv.Atoi(workingNumber)
	return finalNum
}

type stack struct {
	inner []int
}

func (s *stack) Push(v int) {
	s.inner = append(s.inner, v)
}

func (s *stack) Pop() int {
	if len(s.inner) == 0 {
		panic("pop empty")
	}
	n := len(s.inner)
	val := s.inner[n-1]
	s.inner = s.inner[:n-1]
	return val
}

func (s *stack) Peak() int {
	if len(s.inner) == 0 {
		panic("peak empty")
	}
	return s.inner[len(s.inner)-1]
}
