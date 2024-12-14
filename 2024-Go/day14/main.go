package main

import (
	"aoc_2024/lib"
	"fmt"
	"regexp"
	"strconv"
)

var width int = 0
var height int = 0

type Robot struct {
	p [2]int
	v [2]int
}

func (r *Robot) move(t int) (int, int) {
	r.p[0] = (r.p[0] + r.v[0]*t) % width
	r.p[1] = (r.p[1] + r.v[1]*t) % height

	if r.p[0] < 0 {
		r.p[0] = width + r.p[0]
	}
	if r.p[1] < 0 {
		r.p[1] = height + r.p[1]
	}

	return r.p[0], r.p[1]
}

func parseInput(filename string) []Robot {
	input := lib.ParseFileAsStr(filename)

	pattern := `p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)`
	re := regexp.MustCompile(pattern)
	matches := re.FindAllStringSubmatch(input, -1)

	robots := []Robot{}
	for _, match := range matches {
		px, _ := strconv.Atoi(match[1])
		py, _ := strconv.Atoi(match[2])
		vx, _ := strconv.Atoi(match[3])
		vy, _ := strconv.Atoi(match[4])

		robots = append(robots, Robot{
			p: [2]int{px, py},
			v: [2]int{vx, vy},
		})
	}
	return robots
}

func printPart2(pic [][]bool) {
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			if pic[i][j] {
				fmt.Printf("# ")
			} else {
				fmt.Printf("  ")
			}
		}
		fmt.Println()
	}
}

func calculateVariance(robots []Robot) float64 {
	n := len(robots)

	var sumX, sumY float64
	for _, robot := range robots {
		sumX += float64(robot.p[0])
		sumY += float64(robot.p[1])
	}
	meanX := sumX / float64(n)
	meanY := sumY / float64(n)

	var sumSquaredDiffX, sumSquaredDiffY float64
	for _, robot := range robots {
		diffX := float64(robot.p[0]) - meanX
		diffY := float64(robot.p[1]) - meanY
		sumSquaredDiffX += diffX * diffX
		sumSquaredDiffY += diffY * diffY
	}

	vx := sumSquaredDiffX / float64(n)
	vy := sumSquaredDiffY / float64(n)
	return (vx + vy) / 2
}

func part1(filename string, w, h int) int {
	width = w
	height = h
	robots := parseInput(filename)

	midX := width / 2
	midY := height / 2
	q1, q2, q3, q4 := 0, 0, 0, 0

	for _, r := range robots {
		px, py := r.move(100)

		// Increase quadrant count based on final pos
		if px < midX && py < midY {
			q1++
		} else if px > midX && py < midY {
			q2++
		} else if px < midX && py > midY {
			q3++
		} else if px > midX && py > midY {
			q4++
		}
	}
	return q1 * q2 * q3 * q4
}

func part2(filename string, w, h int) (int, [][]bool) {
	width = w
	height = h
	robots := parseInput(filename)

	for t := 1; t <= 10000; t++ {
		pic := lib.MakeMat[bool](height, width)
		for i := range robots {
			px, py := robots[i].move(1)
			pic[py][px] = true
		}
		variance := calculateVariance(robots)

		if variance < 500 {
			return t, pic
		}
	}
	return -1, lib.MakeMat[bool](height, width)
}

func main() {
	result, pic := part2("input.txt", 101, 103)

	printPart2(pic)
	fmt.Println("=========================================================================")
	fmt.Println("Part 1:", part1("input.txt", 101, 103))
	fmt.Println("Part 2:", result)
}
