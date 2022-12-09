package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	part1Answer := part1()
	fmt.Println(part1Answer)

	part2Answer := part2()
	fmt.Println(part2Answer)
}

type move struct {
	direction string
	steps     int
}

type input = []move

func load_input() input {
	file, _ := os.Open("../../inputs/day09.txt")
	defer file.Close()

	s := bufio.NewScanner(file)
	s.Split(bufio.ScanLines)

	input := []move{}
	for s.Scan() {
		line := s.Text()
		direction, stepsStr, _ := strings.Cut(line, " ")
		steps, _ := strconv.Atoi(stepsStr)
		input = append(input, move{
			direction: direction,
			steps:     steps,
		})
	}

	return input
}

type point struct {
	x int
	y int
}

func adjustTail(tail, head point) point {
	newTail := tail
	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{-2, 1}, point{-1, 2}, point{0, 2}, point{1, 2}, point{2, 1}:
		newTail.y++
	}

	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{2, -1}, point{1, -2}, point{0, -2}, point{-1, -2}, point{-2, -1}:
		newTail.y--
	}

	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{1, 2}, point{2, 1}, point{2, 0}, point{2, -1}, point{1, -2}:
		newTail.x++
	}

	switch (point{head.x - tail.x, head.y - tail.y}) {
	case point{-1, -2}, point{-2, -1}, point{-2, 0}, point{-2, 1}, point{-1, 2}:
		newTail.x--
	}

	return newTail
}

func part1() int {
	input := load_input()

	head, tail := point{x: 0, y: 0}, point{x: 0, y: 0}
	visited := map[point]bool{}
	visited[tail] = true
	for _, m := range input {
		for i := 0; i < m.steps; i++ {
			switch m.direction {
			case "U":
				head.y++
			case "R":
				head.x++
			case "D":
				head.y--
			case "L":
				head.x--
			}

			tail = adjustTail(tail, head)
			visited[tail] = true
		}
	}

	return len(visited)
}

func part2() int {
	return 0
}
