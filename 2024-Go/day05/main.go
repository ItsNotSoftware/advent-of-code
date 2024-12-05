package main

import (
	"aoc_2024/lib"
	"fmt"
	"strconv"
	"strings"
)

func parseInput(filename string) (rules [][]int, updates [][]int) {
	lines := lib.ReadLines(filename)
	isSection1 := true

	for _, l := range lines {
		l = strings.TrimSpace(l)
		if l == "" {
			isSection1 = false
			continue
		}

		if isSection1 {
			parts := strings.Split(l, "|")
			n1, _ := strconv.Atoi(strings.TrimSpace(parts[0]))
			n2, _ := strconv.Atoi(strings.TrimSpace(parts[1]))
			rules = append(rules, []int{n1, n2})
		} else {
			parts := strings.Split(l, ",")
			update := make([]int, len(parts))
			for i, val := range parts {
				update[i], _ = strconv.Atoi(strings.TrimSpace(val))
			}
			updates = append(updates, update)
		}
	}

	return rules, updates
}

func getAfter(rules [][]int, p int) []int {
	after := []int{}

	for _, r := range rules {
		if r[0] == p {
			after = append(after, r[1])
		}
	}
	return after
}

func verifyRule(rules [][]int, pages []int, i int) bool {
	target := pages[i]
	after := getAfter(rules, target)

	for _, p := range pages {
		if target == p {
			return true
		}

		for _, v := range after {
			if v == p {
				return false
			}
		}
	}
	return false
}

// Find the first index that fails rules aqnd return it, -1 otherwise
func indexFailsRule(rules [][]int, pages []int) int {
	for i := range pages {
		if !verifyRule(rules, pages, i) {
			return i
		}
	}
	return -1
}

func part1(filename string) int {
	rules, updates := parseInput(filename)
	result := 0

	for _, pages := range updates {
		if indexFailsRule(rules, pages) == -1 {
			result += pages[len(pages)/2]
		}

	}
	return result
}

func part2(filename string) int {
	rules, updates := parseInput(filename)
	result := 0

	for _, pages := range updates {
		shouldSum := false
		for i := indexFailsRule(rules, pages); i != -1; i = indexFailsRule(rules, pages) {
			shouldSum = true
			pages[i], pages[i-1] = pages[i-1], pages[i] // move val to the left
		}

		if shouldSum {
			result += pages[len(pages)/2]
		}
	}

	return result
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
