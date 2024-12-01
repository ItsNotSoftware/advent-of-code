package lib

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Parse2Columns(filename string) ([]int, []int) {
	file, err := os.Open(filename)
	if err != nil {
		fmt.Printf("Error opening file: %v\n", err)
		return []int{}, []int{}
	}
	defer file.Close()

	var col1 []int
	var col2 []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		parts := strings.Fields(line)
		if len(parts) != 2 {
			fmt.Printf("Invalid format in line: %q\n", line)
			continue
		}

		val1, err := strconv.Atoi(parts[0])
		if err != nil {
			fmt.Printf("Invalid number in column 1: %q\n", parts[0])
			continue
		}
		val2, err := strconv.Atoi(parts[1])
		if err != nil {
			fmt.Printf("Invalid number in column 2: %q\n", parts[1])
			continue
		}

		col1 = append(col1, val1)
		col2 = append(col2, val2)
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return []int{}, []int{}
	}

	return col1, col2
}
