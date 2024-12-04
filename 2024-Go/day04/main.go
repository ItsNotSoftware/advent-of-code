package main

import (
	"aoc_2024/lib"
	"fmt"
)

func nextChar(c rune) rune {
	mapping := map[rune]rune{
		'X': 'M',
		'M': 'A',
		'A': 'S',
	}

	return mapping[c]
}

func findChar(input [][]rune, c rune) (int, int) {
	for i, row := range input {
		for j, cell := range row {
			if cell == c {
				return i, j
			}
		}
	}
	return -1, -1
}

func outOfBounds(input [][]rune, i int, j int) bool {
	if i < 0 || j < 0 || i >= len(input) || j >= len(input[0]) {
		return true
	}

	return false
}

func checkSeq(input [][]rune, i int, j int, di int, dj int) bool {
	if outOfBounds(input, i, j) {
		return false
	}

	oldChar := input[i][j]
	i += di
	j += dj

	if outOfBounds(input, i, j) {
		return false
	}

	newChar := input[i][j]

	// Found XMAS
	if newChar == 'S' && oldChar == 'A' {
		return true
	}

	// Check if newChar is the following letter
	if newChar == nextChar(oldChar) {
		return checkSeq(input, i, j, di, dj)
	}

	return false
}

func part1(filename string) int {
	input := lib.ParseCharMatrix(filename)
	dirs := [][2]int{{-1, 0}, {0, -1}, {1, 0}, {0, 1}, {-1, -1}, {-1, 1}, {1, -1}, {1, 1}}
	nSeqs := 0

	for i, j := findChar(input, 'X'); i != -1; i, j = findChar(input, 'X') {
		for _, d := range dirs {
			if checkSeq(input, i, j, d[0], d[1]) {
				nSeqs++
			}
		}

		input[i][j] = 0
	}

	return nSeqs
}

func part2(filename string) int {
	input := lib.ParseCharMatrix(filename)
	nSeqs := 0
	dirs := [][2]int{{-1, -1}, {-1, 1}, {1, -1}, {1, 1}}

	for i, j := findChar(input, 'A'); i != -1; i, j = findChar(input, 'A') {
		diag := 0

		for _, d := range dirs {
			if checkSeq(input, i+d[0], j+d[1], -d[0], -d[1]) {
				diag++
			}

			if diag == 2 {
				nSeqs++
				break
			}
		}

		input[i][j] = 0
	}

	return nSeqs
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
