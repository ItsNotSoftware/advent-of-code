package main

import "testing"

func TestPart1(t *testing.T) {
	expected := 10092
	result := part1("example.txt")
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 9021
	result := part2("example.txt")
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}
