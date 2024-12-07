package main

import (
	"aoc_2024/lib"
	"fmt"
)

const GUARD rune = '^'

func initializeGuardPosition(grid [][]rune, n, m int) (int, int) {
	for i := 0; i < n; i++ {
		for j := 0; j < m; j++ {
			if grid[i][j] == '^' {
				return i, j
			}
		}
	}
	return -1, -1
}

func collectSeen(grid [][]rune, ii, jj int, n, m int, dd [][]int) map[[2]int]struct{} {
	dir := 0
	seen := make(map[[2]int]struct{})
	i, j := ii, jj

	for {
		seen[[2]int{i, j}] = struct{}{}

		nextI := i + dd[dir][0]
		nextJ := j + dd[dir][1]

		if nextI < 0 || nextI >= n || nextJ < 0 || nextJ >= m {
			break
		}

		if grid[nextI][nextJ] == '#' {
			dir = (dir + 1) % 4
		} else {
			i, j = nextI, nextJ
		}
	}

	return seen
}

func willLoop(grid [][]rune, ii, jj, oi, oj int, n, m int, dd [][]int) bool {
	if grid[oi][oj] == '#' {
		return false
	}

	grid[oi][oj] = '#'
	i, j := ii, jj
	dir := 0
	seen := make(map[[3]int]struct{})

	for {
		if _, exists := seen[[3]int{i, j, dir}]; exists {
			grid[oi][oj] = '.'
			return true
		}
		seen[[3]int{i, j, dir}] = struct{}{}

		nextI := i + dd[dir][0]
		nextJ := j + dd[dir][1]

		if nextI < 0 || nextI >= n || nextJ < 0 || nextJ >= m {
			grid[oi][oj] = '.'
			return false
		}

		if grid[nextI][nextJ] == '#' {
			dir = (dir + 1) % 4
		} else {
			i, j = nextI, nextJ
		}
	}
}

func part1(filename string) int {
	grid := lib.ParseCharMatrix(filename)

	n := len(grid)
	m := len(grid[0])

	var i, j int
	found := false
	for i = 0; i < n; i++ {
		for j = 0; j < m; j++ {
			if grid[i][j] == '^' {
				found = true
				break
			}
		}
		if found {
			break
		}
	}

	dir := 0
	dd := [][]int{
		{-1, 0}, // up
		{0, 1},  // right
		{1, 0},  // down
		{0, -1}, // left
	}

	// Track visited cells
	seen := make(map[[2]int]bool)

	for {
		seen[[2]int{i, j}] = true

		nextI := i + dd[dir][0]
		nextJ := j + dd[dir][1]

		if nextI < 0 || nextI >= n || nextJ < 0 || nextJ >= m {
			break
		}

		if grid[nextI][nextJ] == '#' {
			dir = (dir + 1) % 4
		} else {
			i, j = nextI, nextJ
		}
	}

	return len(seen)
}

func part2(filename string) int {
	grid := lib.ParseCharMatrix(filename)
	n := len(grid)
	m := len(grid[0])

	ii, jj := initializeGuardPosition(grid, n, m)
	if ii == -1 || jj == -1 {
		panic("Guard position not found")
	}

	dd := [][]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	ogSeen := collectSeen(grid, ii, jj, n, m, dd)
	ans := 0

	for key := range ogSeen {
		oi, oj := key[0], key[1]

		// Skip the guard position
		if oi == ii && oj == jj {
			continue
		}

		if willLoop(grid, ii, jj, oi, oj, n, m, dd) {
			ans++
		}
	}

	return ans
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
