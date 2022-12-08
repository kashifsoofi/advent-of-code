package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	part1Answer := part1()
	fmt.Println(part1Answer)

	part2Answer := part2()
	fmt.Println(part2Answer)
}

type input = [][]int

func load_input() input {
	file, _ := os.Open("../../inputs/day08.txt")
	defer file.Close()

	s := bufio.NewScanner(file)
	s.Split(bufio.ScanLines)

	input := [][]int{}
	for s.Scan() {
		line := s.Text()
		row := []int{}
		for _, r := range line {
			str := string(r)
			i, _ := strconv.Atoi(str)
			row = append(row, int(i))
		}
		input = append(input, row)
	}

	return input
}

func isVisible(input input, i, j int) bool {
	if i == 0 || j == 0 || i == len(input)-1 || j == len(input)-1 {
		return true
	}

	// visble from left
	visible := true
	for y := j - 1; y >= 0; y-- {
		if input[i][y] >= input[i][j] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	// visible from right
	visible = true
	for y := j + 1; y < len(input); y++ {
		if input[i][y] >= input[i][j] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	// visible from top
	visible = true
	for x := i - 1; x >= 0; x-- {
		if input[x][j] >= input[i][j] {
			visible = false
			break
		}
	}
	if visible {
		return visible
	}

	// visible from bottom
	visible = true
	for x := i + 1; x < len(input); x++ {
		if input[x][j] >= input[i][j] {
			visible = false
			break
		}
	}

	return visible
}

func part1() int {
	input := load_input()
	visibleCount := 0
	for i := 0; i < len(input); i++ {
		for j := 0; j < len(input); j++ {
			visible := isVisible(input, i, j)
			if visible {
				visibleCount++
			}
		}
	}
	return visibleCount
}

func getScenicScore(input input, i, j int) int {
	left := 0
	for y := j - 1; y >= 0; y-- {
		left++
		if input[i][y] >= input[i][j] {
			break
		}
	}

	right := 0
	for y := j + 1; y < len(input); y++ {
		right++
		if input[i][y] >= input[i][j] {
			break
		}
	}

	top := 0
	for x := i - 1; x >= 0; x-- {
		top++
		if input[x][j] >= input[i][j] {
			break
		}
	}

	bottom := 0
	for x := i + 1; x < len(input); x++ {
		bottom++
		if input[x][j] >= input[i][j] {
			break
		}
	}

	return top * bottom * left * right
}

func part2() int {
	input := load_input()
	max := 0
	for i := 0; i < len(input); i++ {
		for j := 0; j < len(input); j++ {
			scenicScore := getScenicScore(input, i, j)
			if scenicScore > max {
				max = scenicScore
			}
		}
	}
	return max
}
