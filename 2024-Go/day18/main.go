package main

import (
	"aoc_2024/lib"
	"container/heap"
	"fmt"
	"math"
	"strconv"
)

type Node struct {
	x, y   int
	g, h   float64
	parent *Node
}

type PriorityQueue []*Node

func (pq PriorityQueue) Len() int { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].g+pq[i].h < pq[j].g+pq[j].h
}
func (pq PriorityQueue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(*Node))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func heuristic(x1, y1, x2, y2 int) float64 {
	return math.Abs(float64(x1-x2)) + math.Abs(float64(y1-y2))
}

func AStar(graph [][]bool, start, goal [2]int) int {
	rows, cols := len(graph), len(graph[0])
	visited := lib.MakeMat[bool](rows, cols)
	directions := [][2]int{{0, 1}, {1, 0}, {0, -1}, {-1, 0}}
	pq := &PriorityQueue{}
	heap.Init(pq)
	startNode := &Node{start[0], start[1], 0, heuristic(start[0], start[1], goal[0], goal[1]), nil}
	heap.Push(pq, startNode)

	for pq.Len() > 0 {
		current := heap.Pop(pq).(*Node)
		if visited[current.x][current.y] {
			continue
		}
		visited[current.x][current.y] = true
		if current.x == goal[0] && current.y == goal[1] {
			return int(current.g)
		}
		for _, dir := range directions {
			nx, ny := current.x+dir[0], current.y+dir[1]
			if nx >= 0 && ny >= 0 && nx < rows && ny < cols && !graph[nx][ny] && !visited[nx][ny] {
				g := current.g + 1
				h := heuristic(nx, ny, goal[0], goal[1])
				heap.Push(pq, &Node{nx, ny, g, h, current})
			}
		}
	}
	return -1
}

func createGraph(filename string, size, nBytes int, columns, lines []int) [][]bool {
	graph := lib.MakeMat[bool](size, size)
	for i := 0; i < nBytes; i++ {
		l, c := lines[i], columns[i]
		graph[l][c] = true
	}
	return graph
}

func part1(filename string, size, nBytes int) int {
	columns, lines := lib.Parse2Columns(filename, ",")
	graph := createGraph(filename, size, nBytes, columns, lines)
	start := [2]int{0, 0}
	goal := [2]int{size - 1, size - 1}
	return AStar(graph, start, goal)
}

func part2(filename string, size, nBytes int) string {
	columns, lines := lib.Parse2Columns(filename, ",")
	start := [2]int{0, 0}
	goal := [2]int{size - 1, size - 1}

	for i := nBytes; i < 3451; i++ {
		graph := createGraph(filename, size, i, columns, lines)

		if AStar(graph, start, goal) == -1 {
			return strconv.Itoa(columns[i-1]) + "," + strconv.Itoa(lines[i-1])
		}
	}

	return ""
}

func main() {
	fmt.Println("Part 1:", part1("input.txt", 71, 1024))
	fmt.Println("Part 2:", part2("input.txt", 71, 1024))
}
