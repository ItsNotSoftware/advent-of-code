package main

import "testing"

func TestPart1(t *testing.T) {
	expected := 22
	result := part1("example.txt", 7, 12)
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := "6,1"
	result := part2("example.txt", 7, 12)
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}
