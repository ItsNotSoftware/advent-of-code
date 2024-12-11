package main

import (
	"aoc_2024/lib"
	"fmt"
	"slices"
	"strconv"
	"strings"
)

func parseInput(filename string) []int64 {
	input := strings.Split(strings.ReplaceAll(lib.ParseFileAsStr(filename), "\n", ""), " ")
	stones := make([]int64, 0, 550000)

	for _, part := range input {
		n, _ := strconv.ParseInt(part, 10, 64)
		stones = append(stones, n)
	}
	return stones
}

func part1(filename string) int {
	stones := parseInput(filename)

	for blinkCount := 0; blinkCount < 25; blinkCount++ {
		i := 0
		for i < len(stones) {
			s_str := strconv.FormatInt(stones[i], 10)

			if stones[i] == 0 {
				stones[i] = 1
			} else if len(s_str)%2 == 0 {
				mid := len(s_str) / 2
				n1, _ := strconv.ParseInt(s_str[:mid], 10, 64)
				n2, _ := strconv.ParseInt(s_str[mid:], 10, 64)

				stones[i] = n1
				stones = slices.Insert(stones, i+1, n2)
				i++ // skip new added val
			} else {
				stones[i] *= 2024
			}
			i++
		}
	}

	return len(stones)
}

func part2(filename string) int {
	return -1
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
