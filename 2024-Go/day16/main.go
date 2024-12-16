package main

import (
	"aoc_2024/lib"
	"container/heap"
	"fmt"
)

type State struct {
	Cost      int
	Position  [2]int
	Direction int
}

type PriorityQueue []State

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].Cost < pq[j].Cost
}

func (pq *PriorityQueue) Swap(i, j int) {
	(*pq)[i], (*pq)[j] = (*pq)[j], (*pq)[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(State))
}

func (pq *PriorityQueue) Pop() interface{} {
	n := len(*pq)
	item := (*pq)[n-1]
	*pq = (*pq)[:n-1]
	return item
}

func parseInput(maze [][]rune) ([2]int, [2]int) {
	var start, end [2]int
	for y, row := range maze {
		for x, cell := range row {
			switch cell {
			case 'S':
				start = [2]int{x, y}
			case 'E':
				end = [2]int{x, y}
			}
		}
	}
	return start, end
}

func neighbors(pos [2]int, direction int, maze [][]rune) []State {
	directions := [][2]int{
		{0, -1},
		{1, 0},
		{0, 1},
		{-1, 0},
	}

	x, y := pos[0], pos[1]
	var result []State

	// Move forward
	dx, dy := directions[direction][0], directions[direction][1]
	nx, ny := x+dx, y+dy
	if ny >= 0 && ny < len(maze) && nx >= 0 && nx < len(maze[0]) && maze[ny][nx] != '#' {
		result = append(result, State{
			Cost:      1,
			Position:  [2]int{nx, ny},
			Direction: direction,
		})
	}

	// Rotate clockwise
	result = append(result, State{
		Cost:      1000,
		Position:  pos,
		Direction: (direction + 1) % 4,
	})

	// Rotate counterclockwise
	result = append(result, State{
		Cost:      1000,
		Position:  pos,
		Direction: (direction + 3) % 4, // Equivalent to (direction - 1 + 4) % 4
	})

	return result
}

func aStar(maze [][]rune, start, end [2]int) int {
	pq := &PriorityQueue{}
	heap.Init(pq)
	heap.Push(pq, State{
		Cost:      0,
		Position:  start,
		Direction: 1,
	})

	visited := make(map[[3]int]bool)

	for pq.Len() > 0 {
		current := heap.Pop(pq).(State)
		if visited[[3]int{current.Position[0], current.Position[1], current.Direction}] {
			continue
		}
		visited[[3]int{current.Position[0], current.Position[1], current.Direction}] = true

		if current.Position == end {
			return current.Cost
		}

		for _, neighbor := range neighbors(current.Position, current.Direction, maze) {
			if !visited[[3]int{neighbor.Position[0], neighbor.Position[1], neighbor.Direction}] {
				heap.Push(pq, State{
					Cost:      current.Cost + neighbor.Cost,
					Position:  neighbor.Position,
					Direction: neighbor.Direction,
				})
			}
		}
	}
	return -1
}

func part1(filename string) int {
	maze := lib.ParseCharMatrix(filename)
	start, end := parseInput(maze)
	return aStar(maze, start, end)
}

func part2(filename string) int {
	return -1
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
