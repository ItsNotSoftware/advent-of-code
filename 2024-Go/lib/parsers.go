package lib

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Parse2Columns(filename string, separator string) ([]int, []int) {
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
		parts := strings.Split(line, separator)
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

func ParseMatrix(filePath string) [][]int {
	var matrix [][]int

	file, err := os.Open(filePath)
	if err != nil {
		panic(err) // Crash if the file cannot be opened
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		stringValues := strings.Fields(line)
		var row []int
		for _, str := range stringValues {
			val, err := strconv.Atoi(str)
			if err != nil {
				panic(err) // Crash if a value cannot be converted to an integer
			}
			row = append(row, val)
		}
		matrix = append(matrix, row)
	}

	if err := scanner.Err(); err != nil {
		panic(err) // Crash if an error occurred during scanning
	}

	return matrix
}

func ParseFileAsStr(filePath string) string {
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var content strings.Builder
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		content.WriteString(scanner.Text())
		content.WriteString("\n")
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return content.String()
}

func ParseCharMatrix(filePath string) [][]rune {
	var matrix [][]rune

	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		matrix = append(matrix, []rune(line))
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return matrix
}

func ReadLines(filePath string) []string {
	file, err := os.Open(filePath)
	if err != nil {
		return nil
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil
	}

	return lines
}
