package main

import (
	"aoc_2024/lib"
	"fmt"
)

const WALL rune = '#'
const BOX rune = 'O'
const ROBOT rune = '@'
const EMPTY rune = '.'

const BIG_BOX_LEFT rune = '['
const BIG_BOX_RIGHT rune = ']'

var mvMap map[rune][2]int = map[rune][2]int{
	'<': [2]int{0, -1},
	'^': [2]int{-1, 0},
	'>': [2]int{0, 1},
	'v': [2]int{1, 0},
}

func parseInput(filename string) (warehouse [][]rune, moves string, l, c int) {
	input := lib.ReadLines(filename)
	isMoves := false

	for i, line := range input {
		if line == "" {
			isMoves = true
			continue
		}

		if !isMoves {
			buff := []rune{}
			for j, ch := range line {
				if ch == '@' {
					l, c = i, j
				}

				buff = append(buff, ch)
			}
			warehouse = append(warehouse, buff)
		} else {
			moves += line
		}
	}
	return
}

func expandWarehouse(warehouse [][]rune) ([][]rune, int, int) {
	expandedWarehouse := lib.MakeMat[rune](len(warehouse), len(warehouse[0])*2)
	l := 0
	c := 0

	for i, line := range warehouse {
		for j, ch := range line {
			switch ch {
			case WALL:
				expandedWarehouse[i][j*2] = WALL
				expandedWarehouse[i][j*2+1] = WALL

			case BOX:
				expandedWarehouse[i][j*2] = BIG_BOX_LEFT
				expandedWarehouse[i][j*2+1] = BIG_BOX_RIGHT

			case EMPTY:
				expandedWarehouse[i][j*2] = EMPTY
				expandedWarehouse[i][j*2+1] = EMPTY

			case ROBOT:
				expandedWarehouse[i][j*2] = ROBOT
				expandedWarehouse[i][j*2+1] = EMPTY
				l, c = i, j*2
			}
		}
	}
	return expandedWarehouse, l, c
}

func moveBoxesP1(warehouse [][]rune, mv [2]int, l, c int) {
	targetL := l + mv[0]
	targetC := c + mv[1]
	target := &warehouse[targetL][targetC]

	// Try to move next box in the way
	if *target == BOX || *target == BIG_BOX_LEFT || *target == BIG_BOX_RIGHT {
		moveBoxesP1(warehouse, mv, targetL, targetC)
	}

	// Move current box if possible
	if *target == EMPTY {
		*target = warehouse[l][c]
		warehouse[l][c] = EMPTY
	}
}

func moveBoxesP2(warehouse [][]rune, mv [2]int, l, c int) {
	targetL := l + mv[0]
	targetCLeft := c + mv[1]
	targetCRight := c + mv[1]

	// Horizotal move
	if mv[0] == 0 {
		moveBoxesP1(warehouse, mv, l, c)
	} else {
		// Check if l c correspond to left or right side of the box
		if warehouse[l][c] == BIG_BOX_LEFT {
			targetCRight++
		} else {
			targetCLeft--
		}

		targetLeft := &warehouse[targetL][targetCLeft]
		targetRight := &warehouse[targetL][targetCRight]

		// left side touching a box
		if *targetLeft == BIG_BOX_LEFT || *targetLeft == BIG_BOX_RIGHT {
			moveBoxesP2(warehouse, mv, targetL, targetCLeft)
		}

		// Right side touching a box
		if *targetRight == BIG_BOX_LEFT || *targetRight == BIG_BOX_RIGHT {
			moveBoxesP2(warehouse, mv, targetL, targetCRight)
		}

		// Move current box if possible
		if *targetLeft == EMPTY && *targetRight == EMPTY {
			warehouse[l][targetCLeft] = EMPTY
			warehouse[l][targetCRight] = EMPTY
			*targetLeft = BIG_BOX_LEFT
			*targetRight = BIG_BOX_RIGHT
			// lib.PrintRuneMat(warehouse)
		}
	}
}

func moveRobot(warehouse [][]rune, moves string, robotL, robotC int) {
	for _, ch := range moves {
		mv := mvMap[ch]
		targetL := robotL + mv[0]
		targetC := robotC + mv[1]

		target := &warehouse[targetL][targetC]

		// Attempt to move box according to part1 or part2
		if *target == BOX {
			moveBoxesP1(warehouse, mv, targetL, targetC)
		} else if *target == BIG_BOX_LEFT || *target == BIG_BOX_RIGHT {
			moveBoxesP2(warehouse, mv, targetL, targetC)
		}

		// Move robot if target is empty
		if *target == EMPTY || *target == ROBOT {
			*target = ROBOT
			warehouse[robotL][robotC] = EMPTY
			robotL, robotC = targetL, targetC
		}
	}
}

func part1(filename string) int {
	warehouse, moves, l, c := parseInput(filename)
	result := 0
	moveRobot(warehouse, moves, l, c)

	for i, line := range warehouse {
		for j, ch := range line {
			if ch == BOX {
				result += 100*i + j
			}
		}
	}
	return result
}

func part2(filename string) int {
	warehouse, moves, _, _ := parseInput(filename)
	warehouse, l, c := expandWarehouse(warehouse)
	result := 0
	moveRobot(warehouse, moves, l, c)

	for i, line := range warehouse {
		for j, ch := range line {
			if ch == BIG_BOX_LEFT {
				result += 100*i + j
			}
		}
	}
	return result
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
