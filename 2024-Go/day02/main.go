package main

import (
	"aoc_2024/lib"
	"fmt"
)

func cloneAndRemoveI(src []int, i int) []int {
	new := append([]int{}, src[:i]...)
	new = append(new, src[i+1:]...)

	return new
}

func abs(val int) int {
	if val < 0 {
		val *= -1
	}

	return val
}

func getSign(val int) int {
	if val < 0 {
		return -1
	} else {
		return 1
	}
}

func isSafe(report []int) bool {
	diff := report[1] - report[0]
	sign := getSign(diff)

	for i := 0; i < len(report)-1; i++ {
		diff = report[i+1] - report[i]
		notSafe := !(abs(diff) >= 1 && abs(diff) <= 3 && getSign(diff) == sign)

		if notSafe {
			return false
		}
	}

	return true
}

func part1(filename string) int {
	reports := lib.ParseMatrix(filename)
	nSafeReports := 0

	for _, r := range reports {
		if isSafe(r) {
			nSafeReports++
		}
	}

	return nSafeReports
}

func part2(filename string) int {
	reports := lib.ParseMatrix(filename)
	nSafeReports := 0

	for _, r := range reports {
		safe := false

		for i := 0; i < len(r); i++ {
			testReport := cloneAndRemoveI(r, i)

			if isSafe(testReport) {
				safe = true
				break
			}
		}

		if safe {
			nSafeReports++
		}
	}

	return nSafeReports
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
