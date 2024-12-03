package main

import "testing"

func TestPart1(t *testing.T) {
	expected := 2
	result := part1("example.txt")
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 4
	result := part2("example.txt")
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}