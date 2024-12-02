#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <day_number>"
  exit 1
fi

DAY=$(printf "%02d" "$1")

DIR_NAME="day${DAY}"
mkdir "$DIR_NAME"

cat <<EOL > "${DIR_NAME}/main.go"
package main

import ("fmt")

func part1(filename string) int {
    return -1 
}

func part2(filename string) int {
    return -1
}

func main() {
    fmt.Println("Part 1:", part1("input.txt"))
    fmt.Println("Part 2:", part2("input.txt"))
}
EOL

cat <<EOL > "${DIR_NAME}/main_test.go"
package main

import "testing"

func TestPart1(t *testing.T) {
    expected := -1
    result := part1("example.txt")
    if result != expected {
        t.Errorf("part1() = %v; want %v", result, expected)
    }
}

func TestPart2(t *testing.T) {
    expected := -1
    result := part2("example.txt")
    if result != expected {
        t.Errorf("part2() = %v; want %v", result, expected)
    }
}
EOL

touch "${DIR_NAME}/input.txt"
touch "${DIR_NAME}/example.txt"
