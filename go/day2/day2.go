package day2

import (
	"strconv"
	"strings"

	"github.com/s3ig8u/adventofcode-2024/utils"
)

func is_valid_path(values []int) bool {
	if len(values) <= 1 {
		return true
	}

	directionOfIncrease := 1 // 1 for increase, -1 for decrease
	if values[0] > values[1] {
		directionOfIncrease = -1
	}

	for i := 0; i < len(values)-1; i++ {
		diff := values[i+1] - values[i]
		abs := diff
		if abs < 0 {
			abs = -abs
		}

		if directionOfIncrease == 1 && diff < 0 {
			return false
		}
		if directionOfIncrease == -1 && diff > 0 {
			return false
		}
		if abs < 1 || abs > 3 {
			return false
		}
	}

	return true
}

func day2_part1(content string) int {
	lines := strings.Split(content, utils.NEW_LINE)
	answer := 0
	for _, line := range lines {
		parts := strings.Fields(line)
		values := make([]int, 0, len(parts))

		for _, part := range parts {
			val, _ := strconv.Atoi(part)
			values = append(values, val)
		}

		if is_valid_path(values) {
			answer++
		}
	}
	return answer
}
func find_valid_paths(values []int, idx int, path []int) bool {
	if idx >= len(values) {
		// we can only remove 0 or 1 element from total length
		if len(path) >= len(values)-1 {
			return is_valid_path(path)
		}
		return false 
	}

	// Try including current element
	path = append(path, values[idx])
	if find_valid_paths(values, idx+1, path) {
		return true // found valid path! no need to try other options
	}

	// backtrack to the previous state and try skipping the current element
	path = path[:len(path)-1]
	return find_valid_paths(values, idx+1, path)
}

func day2_part2(content string) int {
	lines := strings.Split(content, utils.NEW_LINE)
	answer := 0
	for _, line := range lines {
		parts := strings.Fields(line)
		values := make([]int, 0, len(parts))

		for _, part := range parts {
			val, _ := strconv.Atoi(part)
			values = append(values, val)
		}

		if is_valid_path(values) {
			answer++
			continue // already valid, no need to find another path! 
		}

		if find_valid_paths(values, 0, []int{}) {
			answer++
		}
	}
	return answer
}

func Day2(content string) (int, int) {
	return day2_part1(content), day2_part2(content)
}
