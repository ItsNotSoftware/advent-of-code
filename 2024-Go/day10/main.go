package main

import (
	"aoc_2024/lib"
	"fmt"
)

var DIRS = [4][2]int{
	{-1, 0},
	{0, -1},
	{1, 0},
	{0, 1},
}

func exploreTrailHead(input [][]int, l, c int, part1 bool) int {
	score := 0

	if input[l][c] == 9 {
		if part1 {
			input[l][c] = -9
		}

		return 1
	}

	for _, d := range DIRS {
		ll := l + d[0]
		cc := c + d[1]

		if lib.InBoundsMat(input, ll, cc) {
			if input[ll][cc] == input[l][c]+1 {
				score += exploreTrailHead(input, ll, cc, part1)
			}

		}
	}

	return score
}

func clearInput(input [][]int) {
	for i := range input {
		for j := range input[i] {
			if input[i][j] == -9 {
				input[i][j] = 9
			}
		}
	}
}

func part1(filename string) int {
	input := lib.ParseMatrix(filename, false)
	score := 0

	for i, line := range input {
		for j, v := range line {
			if v == 0 {
				score += exploreTrailHead(input, i, j, true)
				clearInput(input)
			}
		}
	}

	return score
}

func part2(filename string) int {
	input := lib.ParseMatrix(filename, false)
	score := 0

	for i, line := range input {
		for j, v := range line {
			if v == 0 {
				score += exploreTrailHead(input, i, j, false)
			}
		}
	}

	return score
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
