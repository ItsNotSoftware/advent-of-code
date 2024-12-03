package main

import (
	"aoc_2024/lib"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

func part1(filename string) int {
	result := 0
	input := lib.ParseFileAsStr(filename)

	pattern := `mul\((\d+),(\d+)\)`
	re := regexp.MustCompile(pattern)

	matches := re.FindAllStringSubmatch(input, -1)

	for _, m := range matches {
		n1, _ := strconv.Atoi(m[1])
		n2, _ := strconv.Atoi(m[2])

		result += n1 * n2
	}

	return result
}

func part2(filename string) int {
	result := 0
	input := lib.ParseFileAsStr(filename)

	pattern := `mul\((\d+),(\d+)\)`
	re := regexp.MustCompile(pattern)

	tokens := strings.Split(input, "do")

	for _, t := range tokens {
		if !strings.HasPrefix(t, "n't") {
			matches := re.FindAllStringSubmatch(t, -1)

			for _, m := range matches {
				n1, _ := strconv.Atoi(m[1])
				n2, _ := strconv.Atoi(m[2])

				result += n1 * n2
			}
		}
	}

	return result
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
