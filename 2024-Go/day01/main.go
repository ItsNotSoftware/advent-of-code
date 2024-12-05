package main

import (
	"aoc_2024/lib"
	"fmt"
	"math"
	"sort"
)

func part1(filename string) int {
	list1, list2 := lib.Parse2Columns(filename, "   ")
	distSum := 0

	sort.Ints(list1)
	sort.Ints(list2)

	for i := 0; i < len(list1); i++ {
		distSum += int(math.Abs(float64((list1[i] - list2[i]))))
	}

	return distSum
}

func part2(filename string) int {
	list1, list2 := lib.Parse2Columns(filename, "   ")
	similarityScore := 0

	sort.Ints(list1)
	sort.Ints(list2)

	for _, v1 := range list1 {
		n := 0

		for _, v2 := range list2 {
			if v1 == v2 {
				n++
			}
		}

		similarityScore += n * v1
	}

	return similarityScore
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
