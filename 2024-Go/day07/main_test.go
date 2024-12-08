package main

import "testing"

func TestPart1(t *testing.T) {
	var expected int64 = 3749
	result := part1("example.txt")
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	var expected int64 = 11387
	result := part2("example.txt")
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}
