package main

import (
	"bufio"
	"fmt"
	"math"
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

type nodeType int

const (
	nodeTypeDir nodeType = iota
	nodeTypeFile
)

type node struct {
	nodeType nodeType
	name     string
	size     uint64
	children map[string]*node
	parent   *node
}

func newNode(nodeType nodeType, name string) *node {
	return newNodeWithParent(nodeType, name, nil)
}

func newNodeWithParent(nodeType nodeType, name string, parent *node) *node {
	return newNodeWithSizeAndParent(nodeType, name, 0, parent)
}

func newNodeWithSizeAndParent(nodeType nodeType, name string, size uint64, parent *node) *node {
	return &node{
		nodeType: nodeType,
		name:     name,
		size:     size,
		children: map[string]*node{},
		parent:   parent,
	}
}

type input struct {
	root    *node
	allDirs []*node
}

func load_input() input {
	file, _ := os.Open("../../inputs/day07.txt")
	defer file.Close()

	s := bufio.NewScanner(file)
	s.Split(bufio.ScanLines)

	allDirs := []*node{}
	root := newNode(nodeTypeDir, "/")
	allDirs = append(allDirs, root)
	current := root
	for s.Scan() {
		line := s.Text()
		if strings.HasPrefix(line, "$ cd ") {
			dirName := strings.TrimPrefix(line, "$ cd ")
			if dirName == "/" {
				current = root
			} else if dirName == ".." {
				current = current.parent
			} else {
				current = current.children[dirName]
			}
		} else if strings.HasPrefix(line, "$ ls") {
		} else if strings.HasPrefix(line, "dir ") {
			dirName := strings.TrimPrefix(line, "dir ")
			node := newNodeWithParent(nodeTypeDir, dirName, current)
			current.children[dirName] = node
			allDirs = append(allDirs, node)
		} else {
			sizeStr, filename, _ := strings.Cut(line, " ")
			size, _ := strconv.ParseUint(sizeStr, 10, 64)
			node := newNodeWithSizeAndParent(nodeTypeFile, filename, size, current)
			current.children[filename] = node
		}
	}

	return input{
		root:    root,
		allDirs: allDirs,
	}
}

func updateSize(node *node) {
	var size uint64
	for _, child := range node.children {
		if child.nodeType == nodeTypeDir && len(child.children) > 0 {
			updateSize(child)
		}
		size += child.size
	}
	node.size = size
}

func part1() uint64 {
	input := load_input()
	updateSize(input.root)

	var totalSize uint64 = 0
	for _, dir := range input.allDirs {
		if dir.size <= 100000 {
			totalSize += dir.size
		}
	}
	return totalSize
}

func part2() uint64 {
	input := load_input()
	updateSize(input.root)

	totalUsed := input.root.size
	unused := uint64(70000000) - totalUsed
	toDelete := uint64(30000000) - unused

	candidates := []*node{}
	for _, dir := range input.allDirs {
		if dir.size >= toDelete {
			candidates = append(candidates, dir)
		}
	}

	var smallest uint64 = math.MaxUint64
	for _, c := range candidates {
		if c.size < smallest {
			smallest = c.size
		}
	}

	return smallest
}
