package main

import (
	"aoc_2024/lib"
	"fmt"
)

func findAntinodesForAntennaP1(input [][]rune, l, c int) [][]bool {
	freq := input[l][c]
	antinodes := lib.MakeMat[bool](len(input), len(input[0]))

	for i, line := range input {
		for j, f := range line {
			if i == l && j == c {
				continue
			}

			if f == freq {
				antinodeL := (l - i) + l
				antinodeC := (c - j) + c

				if lib.InBoundsMat(antinodes, antinodeL, antinodeC) {
					antinodes[antinodeL][antinodeC] = true
				}

			}
		}
	}
	return antinodes
}

func findAntinodesForAntennaP2(input [][]rune, l, c int) [][]bool {
	freq := input[l][c]
	antinodes := lib.MakeMat[bool](len(input), len(input[0]))

	for i, line := range input {
		for j, f := range line {
			if i == l && j == c {
				continue
			}

			if f == freq {
				dL := (l - i)
				dC := (c - j)

				for k := 1; k < len(input); k++ {
					if lib.InBoundsMat(input, k*dL+l, k*dC+c) {
						antinodes[k*dL+l][k*dC+c] = true
					}
				}
			}
		}
	}
	return antinodes
}

func part1(filename string) int {
	input := lib.ParseCharMatrix(filename)
	antinodes := lib.MakeMat[bool](len(input), len(input[0]))

	for i, line := range input {
		for j, f := range line {
			if f != '.' {
				newAntinodes := findAntinodesForAntennaP1(input, i, j)
				lib.OrBoolMats(&antinodes, &newAntinodes)
			}
		}
	}

	return lib.MatCount(antinodes, true)
}

func part2(filename string) int {
	input := lib.ParseCharMatrix(filename)
	antinodes := lib.MakeMat[bool](len(input), len(input[0]))

	for i, line := range input {
		for j, f := range line {
			if f != '.' {
				newAntinodes := findAntinodesForAntennaP2(input, i, j)
				lib.OrBoolMats(&antinodes, &newAntinodes)
			}
		}
	}

	count := 0
	for i := range antinodes {
		for j := range antinodes[i] {
			if antinodes[i][j] || input[i][j] != '.' {
				count++
			}
		}
	}

	return count
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
