package main

import "testing"

func TestPart1(t *testing.T) {
	expected := 12
	result := part1("example.txt", 11, 7)
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 1
	result, _ := part2("example.txt", 11, 7)
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}
