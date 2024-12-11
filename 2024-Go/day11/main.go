package main

import (
	"aoc_2024/lib"
	"fmt"
	"strconv"
	"strings"
)

type Stones struct {
	countMap map[int64]int64
}

func (s *Stones) removeAll(stoneNr int64) {
	s.countMap[stoneNr] = 0
}

func (s *Stones) remove(stoneNr int64, countDecrement int64) {
	s.countMap[stoneNr] -= countDecrement
}

func (s *Stones) add(stoneNr int64, countIncrement int64) {
	if val, exists := s.countMap[stoneNr]; exists {
		s.countMap[stoneNr] = val + countIncrement
	} else {
		s.countMap[stoneNr] = countIncrement
	}
}

func (s *Stones) clone() *Stones {
	clonedCounts := make(map[int64]int64, len(s.countMap))
	for key, value := range s.countMap {
		clonedCounts[key] = value
	}
	return &Stones{countMap: clonedCounts}
}

func (s Stones) count() int {
	sum := 0
	for _, val := range s.countMap {
		sum += int(val)
	}
	return sum
}

func parseInput(filename string) Stones {
	input := strings.Split(strings.ReplaceAll(lib.ParseFileAsStr(filename), "\n", ""), " ")
	stones := Stones{countMap: make(map[int64]int64)}

	for _, part := range input {
		n, _ := strconv.ParseInt(part, 10, 64)
		stones.add(n, 1)
	}
	return stones
}

func solve(filename string, nBlinks int) int {
	stones := parseInput(filename)

	for blinkCount := 0; blinkCount < nBlinks; blinkCount++ {
		newStones := stones.clone()

		for stoneNr, count := range stones.countMap {
			if count == 0 || stoneNr == 0 {
				continue
			}
			s_str := strconv.FormatInt(stoneNr, 10)

			if len(s_str)%2 == 0 {
				mid := len(s_str) / 2
				n1, _ := strconv.ParseInt(s_str[:mid], 10, 64)
				n2, _ := strconv.ParseInt(s_str[mid:], 10, 64)

				newStones.add(n1, count)
				newStones.add(n2, count)
				newStones.remove(stoneNr, count)
			} else {
				newStones.add(stoneNr*2024, count)
				newStones.remove(stoneNr, count)
			}
		}
		// transform 0s in 1s
		newStones.add(1, stones.countMap[0])
		newStones.remove(0, stones.countMap[0])

		stones = *newStones
	}

	return stones.count()
}

func part1(filename string) int {
	return solve(filename, 25)
}

func part2(filename string) int {
	return solve(filename, 75)
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
