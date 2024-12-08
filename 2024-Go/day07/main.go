package main

import (
	"aoc_2024/lib"
	"fmt"
	"strconv"
	"strings"
)

type Equation struct {
	result  int64
	numbers []int64
}

func parseInput(filename string) []Equation {
	input := lib.ReadLines(filename)
	equations := make([]Equation, 0, len(input))

	for _, line := range input {
		parts := strings.Split(line, ":")
		result, _ := strconv.ParseInt(strings.TrimSpace(parts[0]), 10, 64)

		// Parse the numbers
		numberParts := strings.Fields(parts[1])
		numbers := make([]int64, len(numberParts))
		for i, numStr := range numberParts {
			numbers[i], _ = strconv.ParseInt(numStr, 10, 64)
		}

		equations = append(equations, Equation{
			result:  result,
			numbers: numbers,
		})
	}

	return equations
}

func verifiesResultP1(eq *Equation, carry int64, i int) bool {
	if i == len(eq.numbers)-1 {
		return carry == eq.result
	}

	i++
	return verifiesResultP1(eq, carry+eq.numbers[i], i) || verifiesResultP1(eq, carry*eq.numbers[i], i)
}

func concat(n1 int64, n2 int64) int64 {
	sn1 := strconv.FormatInt(n1, 10)
	sn2 := strconv.FormatInt(n2, 10)
	sn := sn1 + sn2
	n, _ := strconv.ParseInt(sn, 10, 64)

	return n
}

func verifiesResultP2(eq *Equation, carry int64, i int) bool {
	if i == len(eq.numbers)-1 {
		return carry == eq.result
	}

	i++
	return verifiesResultP2(eq, carry+eq.numbers[i], i) || verifiesResultP2(eq, carry*eq.numbers[i], i) || verifiesResultP2(eq, concat(carry, eq.numbers[i]), i)
}

func part1(filename string) int64 {
	equations := parseInput(filename)
	var sum int64 = 0

	for _, eq := range equations {
		if verifiesResultP1(&eq, eq.numbers[0], 0) {
			sum += eq.result
		}
	}

	return sum
}

func part2(filename string) int64 {
	equations := parseInput(filename)
	var sum int64 = 0

	for _, eq := range equations {
		if verifiesResultP2(&eq, eq.numbers[0], 0) {
			sum += eq.result
		}
	}

	return sum
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
