package main

import (
	"aoc_2024/lib"
	"fmt"
	"math"
)

func initProblem(filename string) (grid [][]rune, dists [][]int, start, end [2]int) {
	grid = lib.ParseCharMatrix(filename)
	dists = lib.MakeMat[int](len(grid), len(grid[0]))

	for i, line := range grid {
		for j, ch := range line {
			dists[i][j] = -1

			if ch == 'S' {
				start = [2]int{i, j}
			} else if ch == 'E' {
				end = [2]int{i, j}
			}
		}
	}
	return
}

func updateDistances(grid [][]rune, dist [][]int, start [2]int) {
	dist[start[0]][start[1]] = 0
	l, c := start[0], start[1]

	for grid[l][c] != 'E' {
		for _, neighbor := range [4][2]int{{l - 1, c}, {l + 1, c}, {l, c - 1}, {l, c + 1}} {
			nl, nc := neighbor[0], neighbor[1]

			if lib.InBoundsMat(grid, nl, nc) && grid[nl][nc] != '#' && dist[nl][nc] == -1 {
				dist[nl][nc] = dist[l][c] + 1
				l, c = nl, nc
			}
		}
	}
}

func part1(filename string) int {
	count := 0
	grid, dist, start, _ := initProblem(filename)
	updateDistances(grid, dist, start)

	for l, line := range grid {
		for c := range line {
			if grid[l][c] == '#' {
				continue
			}

			for _, neighbor := range [4][2]int{{l + 2, c}, {l + 1, c + 1}, {l, c + 2}, {l - 1, c + 1}} {
				nl, nc := neighbor[0], neighbor[1]

				if !lib.InBoundsMat(grid, nl, nc) {
					continue
				}
				if grid[nl][nc] == '#' {
					continue
				}

				if math.Abs(float64(dist[l][c]-dist[nl][nc])) >= 102 {
					count++
				}
			}

		}
	}

	return count
}

func part2(filename string) int {
	count := 0
	grid, dist, start, _ := initProblem(filename)
	updateDistances(grid, dist, start)

	for l, line := range grid {
		for c := range line {
			if grid[l][c] == '#' {
				continue
			}

			for radius := 2; radius <= 20; radius++ {
				for dl := 0; dl <= radius; dl++ {
					dc := radius - dl
					for _, neighbor := range [4][2]int{{l + dl, c + dc}, {l + dl, c - dc}, {l - dl, c + dc}, {l - dl, c - dc}} {
						nl, nc := neighbor[0], neighbor[1]

						if !lib.InBoundsMat(grid, nl, nc) {
							continue
						}
						if grid[nl][nc] == '#' {
							continue
						}

						if math.Abs(float64(dist[l][c]-dist[nl][nc])) >= float64(100+radius) {
							count++
						}
					}
				}
			}
		}
	}

	return count
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
