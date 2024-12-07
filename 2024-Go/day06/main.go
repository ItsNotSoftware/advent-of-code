package main

import (
	"aoc_2024/lib"
	"fmt"
)

const GUARD rune = '^'
const OBSTACLE rune = '#'
const VISITED rune = 'X'

var DIRECTIONS = [][2]int{
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
}

type Guard struct {
	currDir int
	l       int
	c       int
	steps   int
}

func initGuard(input [][]rune) Guard {
	guard := Guard{
		currDir: 0,
		l:       0,
		c:       0,
		steps:   1,
	}

	for i, line := range input {
		for j, ch := range line {
			if ch == GUARD {
				guard.l = i
				guard.c = j
				input[i][j] = VISITED
				return guard
			}
		}
	}
	return guard
}

func (g Guard) getForwardDir() [2]int {
	return DIRECTIONS[g.currDir]
}

func (g *Guard) turn() {
	g.currDir = (g.currDir + 1) % len(DIRECTIONS)
}

// Returns false when dest is met
func (g *Guard) patrol(input [][]rune) bool {
	l := g.l + g.getForwardDir()[0]
	c := g.c + g.getForwardDir()[1]

	// Turn in case of obstacle
	if input[l][c] == OBSTACLE {
		g.turn()
		return true
	}

	// Move forward
	g.l = l
	g.c = c

	// Check if pos was previosly visited
	if input[g.l][g.c] != VISITED {
		g.steps++
		input[g.l][g.c] = VISITED
	}

	// false if end was reached true othewise
	return l != 0 && l != len(input)-1 && c != 0 && c != len(input[0])-1
}

func part1(filename string) int {
	input := lib.ParseCharMatrix(filename)
	guard := initGuard(input)

	for guard.patrol(input) {
	}

	return guard.steps
}

func part2(filename string) int {
	return -1
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
