package main

import "testing"

func TestPart1(t *testing.T) {
	expected := "4,6,3,5,6,3,5,2,1,0"
	result := part1("example.txt")
	if result != expected {
		t.Errorf("part1() = %v; want %v", result, expected)
	}
}
