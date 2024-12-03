package main

import "testing"

func TestPart1(t *testing.T) {
	expected := 161
	result := part1("example1.txt")
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}

func TestPart2(t *testing.T) {
	expected := 48
	result := part2("example2.txt")
	if result != expected {
		t.Errorf("part2() = %v; want %v", result, expected)
	}
}
