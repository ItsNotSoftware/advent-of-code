package main

import (
	"aoc_2024/lib"
	"fmt"
	"math"
	"regexp"
	"strconv"
)

type Problem struct {
	A [2][2]float64
	b [2]float64
}

func det(A [2][2]float64) float64 {
	return A[0][0]*A[1][1] - A[0][1]*A[1][0]
}

func getAi(A [2][2]float64, b [2]float64, col int) [2][2]float64 {
	clonedMatrix := A
	clonedMatrix[0][col] = b[0]
	clonedMatrix[1][col] = b[1]
	return clonedMatrix
}

func parseInput(filename string, delta float64) []Problem {
	input := lib.ParseFileAsStr(filename)

	pattern := `Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)`
	re := regexp.MustCompile(pattern)

	matches := re.FindAllStringSubmatch(input, -1)

	var problems []Problem

	for _, match := range matches {
		if len(match) != 7 {
			continue
		}

		ax, _ := strconv.ParseFloat(match[1], 64)
		ay, _ := strconv.ParseFloat(match[2], 64)
		bx, _ := strconv.ParseFloat(match[3], 64)
		by, _ := strconv.ParseFloat(match[4], 64)
		px, _ := strconv.ParseFloat(match[5], 64)
		py, _ := strconv.ParseFloat(match[6], 64)

		problems = append(problems, Problem{
			A: [2][2]float64{{ax, bx}, {ay, by}},
			b: [2]float64{px + delta, py + delta},
		})
	}
	return problems
}

/*
* minimize c.T @ x
* s.t.
*	    A@x = b
 */
func (p Problem) solve() float64 {
	determinant := det(p.A)
	if determinant == 0 {
		return math.Inf(1) // No solution exists
	}

	// Cramer's rule
	nA := det(getAi(p.A, p.b, 0)) / determinant
	nB := det(getAi(p.A, p.b, 1)) / determinant

	if nA < 0 || nB < 0 || math.Mod(nA, 1) != 0 || math.Mod(nB, 1) != 0 {
		return math.Inf(1) // Invalid solution
	}

	cost := 3*nA + 1*nB
	return cost
}

func part1(filename string) int {
	problems := parseInput(filename, 0)
	cost := float64(0)

	for _, p := range problems {
		c := p.solve()
		if c != math.Inf(1) {
			cost += c
		}
	}
	return int(cost)
}

func part2(filename string) int {
	problems := parseInput(filename, 10000000000000)
	cost := float64(0)

	for _, p := range problems {
		c := p.solve()
		if c != math.Inf(1) {
			cost += c
		}
	}
	return int(cost)
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
