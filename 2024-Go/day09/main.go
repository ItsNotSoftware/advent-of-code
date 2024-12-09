package main

import (
	"aoc_2024/lib"
	"fmt"
	"strconv"
)

const FREE int64 = -1

type Range struct {
	value int64
	idx_s int64
	idx_e int64 // Not included
}

func (r Range) size() int64 {
	return r.idx_e - r.idx_s
}

func initRangesSlice(input string) []Range {
	ranges := []Range{}
	var curr_i int64 = 0
	var curr_v int64 = 0

	for i, ch := range input {
		v, _ := strconv.ParseInt(string(ch), 10, 64)
		if i%2 == 0 {
			ranges = append(ranges, Range{idx_s: curr_i, idx_e: curr_i + v, value: curr_v})
			curr_v++
		} else {
			ranges = append(ranges, Range{idx_s: curr_i, idx_e: curr_i + v, value: FREE})
		}
		curr_i += v
	}

	return ranges
}

func findFreeSlot(ranges []Range) int {
	for i := range ranges {
		if ranges[i].value == FREE {
			return i
		}
	}
	return -1
}

func findRightmostRange(ranges []Range) int {
	for i := len(ranges) - 1; i >= 0; i-- {
		if ranges[i].value != FREE {
			return i
		}
	}
	return -1
}

func insertSorted(ranges []Range, new Range) []Range {
	ranges = append(ranges, new)

	for i := len(ranges) - 1; i > 0; i-- {
		if ranges[i].idx_s < ranges[i-1].idx_s {
			ranges[i], ranges[i-1] = ranges[i-1], ranges[i]
		}
	}
	return ranges
}

func compactFiles(ranges []Range) []Range {
	for {
		freeSlot_i := findFreeSlot(ranges)
		r_i := findRightmostRange(ranges)
		freeSlot := &ranges[freeSlot_i]
		r := &ranges[r_i]

		if freeSlot_i > r_i {
			break
		}

		if freeSlot.size() == r.size() { // Free slot is the same size as the populated
			*freeSlot, *r = *r, *freeSlot
		} else if freeSlot.size() > r.size() { // Free slot is bigger
			newRange := Range{idx_s: freeSlot.idx_s + r.size(), idx_e: freeSlot.idx_e, value: FREE}
			freeSlot.idx_e = freeSlot.idx_s + r.size()
			freeSlot.value = r.value
			r.value = FREE
			ranges = insertSorted(ranges, newRange)
		} else { // Free slot is smaller
			freeSlot.value = r.value
			r.idx_e = r.idx_e - freeSlot.size()
		}

	}
	return ranges
}

func printRanges(ranges []Range) {
	for _, r := range ranges {
		for i := 0; i < int(r.size()); i++ {
			if r.value == FREE {
				fmt.Print(".")
			} else {
				fmt.Print(r.value)
			}
		}
	}
	fmt.Println()
}

func moveFilesDescending(ranges []Range) []Range {
	for fileID := int64(len(ranges) - 1); fileID >= 0; fileID-- {
		for i := 0; i < len(ranges); i++ {
			for j := 0; j < len(ranges); j++ {
				if ranges[j].value == FREE && ranges[j].size() >= ranges[i].size() {
					freeSlot := &ranges[j]
					file := &ranges[i]

					if freeSlot.size() > file.size() {
						newFreeSlot := Range{
							idx_s: freeSlot.idx_s + file.size(),
							idx_e: freeSlot.idx_e,
							value: FREE,
						}
						freeSlot.idx_e = freeSlot.idx_s + file.size()
						ranges = insertSorted(ranges, newFreeSlot)
					}

					freeSlot.value = file.value
					file.value = FREE

					file.idx_s = file.idx_e
					file.idx_e = file.idx_s
					break
				}
			}
		}
	}
	return ranges
}

func part1(filename string) int64 {
	input := lib.ParseFileAsStr(filename)
	ranges := initRangesSlice(input)
	ranges = compactFiles(ranges)
	var result int64 = 0
	var idx int64 = 0

	for _, r := range ranges {
		if r.value == FREE {
			break
		}

		for i := r.idx_s; i < r.idx_e; i++ {
			result += idx * r.value
			idx++
		}
	}

	return result - 1
}

func part2(filename string) int64 {
	input := lib.ParseFileAsStr(filename)
	ranges := initRangesSlice(input)
	ranges = moveFilesDescending(ranges)
	var result int64 = 0
	var idx int64 = 0

	for _, r := range ranges {
		if r.value == FREE {
			continue
		}

		for i := r.idx_s; i < r.idx_e; i++ {
			result += idx * r.value
			idx++
		}
	}

	return result
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
