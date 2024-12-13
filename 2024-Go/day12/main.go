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

const EMPTY int = 0
const PLANT int = 1
const FENCE int = 2

func getRegionPrice1(input [][]rune, l, c int) int {
	area := 0
	perimeter := 0
	plant := input[l][c]
	visited := lib.MakeMat[bool](len(input), len(input[0]))

	stack := lib.Stack[[2]int]{}
	stack.Push([2]int{l, c})
	visited[l][c] = true

	for pair, notEmpty := stack.Pop(); notEmpty; pair, notEmpty = stack.Pop() {
		l, c = pair[0], pair[1]

		if lib.InBoundsMat(input, l, c) {
			if input[l][c] == plant {
				area++
				input[l][c] = 0
			} else {
				perimeter++
				continue
			}
		} else {
			perimeter++
			continue
		}

		for _, dir := range DIRS {
			ll := l + dir[0]
			cc := c + dir[1]

			if lib.InBoundsMat(input, ll, cc) {
				if visited[ll][cc] {
					continue
				}
				if input[ll][cc] == plant {
					visited[ll][cc] = true
				}
			}
			stack.Push([2]int{l + dir[0], c + dir[1]})
		}
	}
	return perimeter * area
}

func findFence(simpMap [][]int) (int, int) {
	for i := range simpMap {
		for j := range simpMap[i] {
			if simpMap[i][j] == FENCE {
				return i, j
			}
		}
	}
	return -1, -1
}

func countFenceTurns(mapInput [][]int) int {
	visited := lib.MakeMat[bool](len(mapInput), len(mapInput[0]))
	turns := 0

	// Helper function to find the next fence segment
	findNext := func(l, c, prevDir int) (int, int, int, bool) {
		for d, dir := range DIRS {
			if d == (prevDir+2)%4 { // Skip opposite direction
				continue
			}
			ll, cc := l+dir[0], c+dir[1]
			if lib.InBoundsMat(mapInput, ll, cc) && mapInput[ll][cc] == FENCE && !visited[ll][cc] {
				return ll, cc, d, true
			}
		}
		return -1, -1, -1, false
	}

	// Find the starting point
	startRow, startCol := findFence(mapInput)
	if startRow == -1 && startCol == -1 {
		return 0 // No fences found
	}

	// Initialize traversal
	l, c := startRow, startCol
	visited[l][c] = true
	direction := -1

	for {
		ll, cc, newDir, found := findNext(l, c, direction)
		if !found {
			break
		}
		if direction != -1 && direction != newDir {
			turns++
		}
		visited[ll][cc] = true
		l, c = ll, cc
		direction = newDir
		if l == startRow && c == startCol {
			break // Completed the loop
		}
	}

	return turns
}

func getRegionPrice2(input [][]rune, l, c int) int {
	area := 0
	plant := input[l][c]
	visited := lib.MakeMat[bool](len(input), len(input[0]))
	simpMap := lib.MakeMat[int](len(input)+2, len(input[0])+2)

	stack := lib.Stack[[2]int]{}
	stack.Push([2]int{l, c})
	visited[l][c] = true

	for pair, notEmpty := stack.Pop(); notEmpty; pair, notEmpty = stack.Pop() {
		l, c = pair[0], pair[1]

		if lib.InBoundsMat(input, l, c) {
			if input[l][c] == plant {
				area++
				input[l][c] = 0
				simpMap[l+1][c+1] = PLANT
			} else {
				simpMap[l+1][c+1] = FENCE
				continue
			}
		} else {
			simpMap[l+1][c+1] = FENCE
			continue
		}

		for _, dir := range DIRS {
			ll := l + dir[0]
			cc := c + dir[1]

			if lib.InBoundsMat(input, ll, cc) {
				if visited[ll][cc] {
					continue
				}
				if input[ll][cc] == plant {
					visited[ll][cc] = true
				}
			}
			stack.Push([2]int{l + dir[0], c + dir[1]})
		}
	}

	turns := countFenceTurns(simpMap)
	fmt.Printf("Fence groups[%c]: %d\n", plant, turns)
	lib.PrintMat(simpMap)

	return area * turns
}

func part1(filename string) int {
	input := lib.ParseCharMatrix(filename)
	totPrice := 0

	for i := range input {
		for j := range input[i] {
			if input[i][j] != 0 {
				totPrice += getRegionPrice1(input, i, j)
			}
		}
	}
	return totPrice
}

func part2(filename string) int {
	input := lib.ParseCharMatrix(filename)
	totPrice := 0

	for i := range input {
		for j := range input[i] {
			if input[i][j] != 0 {
				totPrice += getRegionPrice2(input, i, j)
			}
		}
	}
	return totPrice
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
