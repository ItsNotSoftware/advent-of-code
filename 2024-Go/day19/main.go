package main

import (
	"aoc_2024/lib"
	"fmt"
	"strings"
)

func parseInput(filename string) ([]string, []string) {
	input := lib.ParseFileAsStr(filename)
	parts := strings.Split(input, "\n\n")
	patterns := strings.Split(parts[0], ", ")
	designs := strings.Split(parts[1], "\n")

	return patterns, designs[:len(designs)-1]
}

func countMatches(design string, patterns []string) int {
	n := len(design)
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 1; i <= n; i++ {
		for _, p := range patterns {
			if i >= len(p) && strings.HasPrefix(design[i-len(p):i], p) {
				dp[i] += dp[i-len(p)]
			}
		}
	}

	return dp[n]
}

func part1(filename string) int {
	patterns, designs := parseInput(filename)
	count := 0

	for _, d := range designs {
		if countMatches(d, patterns) > 0 {
			count++
		}
	}

	return count
}

func part2(filename string) int {
	patterns, designs := parseInput(filename)
	count := 0

	for _, d := range designs {
		count += countMatches(d, patterns)
	}

	return count
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
