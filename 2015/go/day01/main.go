package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	currentFloor := part1_GetCurrentFloor()
	fmt.Println(currentFloor)

	position := part2_GetPosition()
	fmt.Println(position)
}

func part1_GetCurrentFloor() int {
	input, _ := os.Open("../../inputs/day01.txt")
	defer input.Close()

	s := bufio.NewScanner(input)
	s.Split(bufio.ScanRunes)

	currentFloor := 0
	for s.Scan() {
		if s.Text() == "(" {
			currentFloor += 1
		} else if s.Text() == ")" {
			currentFloor -= 1
		}
	}

	return currentFloor
}

func part2_GetPosition() int {
	input, _ := os.Open("../../inputs/day01.txt")
	defer input.Close()

	s := bufio.NewScanner(input)
	s.Split(bufio.ScanRunes)

	position := 1
	currentFloor := 0
	for s.Scan() {
		if s.Text() == "(" {
			currentFloor += 1
		} else if s.Text() == ")" {
			currentFloor -= 1
		}

		if currentFloor == -1 {
			break
		}
		position += 1
	}

	return position
}
