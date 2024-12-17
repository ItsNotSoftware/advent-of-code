package main

import (
	"aoc_2024/lib"
	"context"
	"fmt"
	"math"
	"regexp"
	"runtime"
	"strconv"
	"strings"
	"sync"
)

type Opcode int

const (
	ADV Opcode = iota
	BXL
	BST
	JNZ
	BXC
	OUT
	BDV
	CDV
	EXT = -1
)

type Instruction struct {
	opcode  Opcode
	operand int
}

type Program struct {
	registers struct {
		A int
		B int
		C int
	}
	ip      int
	output  string
	program []int
}

func (p Program) print() {
	fmt.Printf("Registers: A=%d, B=%d, C=%d\n", p.registers.A, p.registers.B, p.registers.C)
	fmt.Printf("IP: %d\n", p.ip)
	fmt.Printf("Program: %v\n", p.program)
	fmt.Printf("Output: %s\n\n", p.output)
}

func (p *Program) fetch() (opcode, operandLiteral, operandCombo int) {
	ip := p.ip
	if ip < len(p.program)-1 {
		operand := p.program[ip+1]
		p.ip += 2

		switch operand {
		case 4:
			return p.program[ip], operand, p.registers.A
		case 5:
			return p.program[ip], operand, p.registers.B
		case 6:
			return p.program[ip], operand, p.registers.C
		case 7:
			return p.program[ip], operand, EXT
		default:
			return p.program[ip], operand, operand
		}
	}
	return EXT, 0, 0
}

func (p *Program) decodeAndExecute(opcode, operandLiteral, operandCombo int) {
	switch Opcode(opcode) {
	case ADV:
		p.registers.A /= int(math.Pow(2, float64(operandCombo)))
	case BXL:
		p.registers.B ^= operandLiteral
	case BST:
		p.registers.B = operandCombo % 8
	case JNZ:
		if p.registers.A == 0 {
			return
		}
		p.ip = operandLiteral
	case BXC:
		p.registers.B ^= p.registers.C
	case OUT:
		p.output += strconv.Itoa(operandCombo%8) + ","
	case BDV:
		p.registers.B = p.registers.A / int(math.Pow(2, float64(operandCombo)))
	case CDV:
		p.registers.C = p.registers.A / int(math.Pow(2, float64(operandCombo)))
	}
}

func (p Program) run() string {
	for opcode, operandLiteral, operandCombo := p.fetch(); opcode != EXT; opcode, operandLiteral, operandCombo = p.fetch() {
		p.decodeAndExecute(opcode, operandLiteral, operandCombo)
	}
	return p.output[:len(p.output)-1]
}

func parseInput(filename string) (Program, string) {
	input := lib.ParseFileAsStr(filename)

	r := regexp.MustCompile(`Register ([A-C]): (\d+)`).FindAllStringSubmatch(input, -1)
	p := regexp.MustCompile(`Program:\s*((?:\d+,?)+)`).FindStringSubmatch(input)

	a, _ := strconv.Atoi(r[0][2])
	b, _ := strconv.Atoi(r[1][2])
	c, _ := strconv.Atoi(r[2][2])

	inst := []int{}
	for _, val := range strings.Split(p[1], ",") {
		if num, _ := strconv.Atoi(val); true {
			inst = append(inst, num)
		}
	}

	return Program{
		registers: struct{ A, B, C int }{A: a, B: b, C: c},
		ip:        0,
		output:    "",
		program:   inst,
	}, p[1]
}

func part1(filename string) string {
	program, _ := parseInput(filename)
	output := program.run()
	return output
}

func part2(filename string) string {
	program, expected := parseInput(filename)

	const maxRange = 20_000_000_000
	var result int = -1
	var wg sync.WaitGroup
	var mu sync.Mutex

	numWorkers := runtime.NumCPU()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	worker := func(ctx context.Context, start, end int) {
		defer wg.Done()
		for i := start; i < end; i++ {
			select {
			case <-ctx.Done():
				return
			default:
				p := program
				p.registers.A = i
				if p.run() == expected {
					mu.Lock()
					if result == -1 {
						result = i
						cancel()
					}
					mu.Unlock()
					return
				}
			}
		}
	}

	step := (maxRange + numWorkers - 1) / numWorkers
	for t := 0; t < numWorkers; t++ {
		start := t*step + 1
		end := start + step
		if end > maxRange+1 {
			end = maxRange + 1
		}
		wg.Add(1)
		go worker(ctx, start, end)
	}

	wg.Wait()
	return strconv.Itoa(result)
}

func main() {
	fmt.Println("Part 1:", part1("input.txt"))
	fmt.Println("Part 2:", part2("input.txt"))
}
