package day2

import (
	"testing"
)

func TestIsValidPath(t *testing.T) {
	tests := []struct {
		path     []int
		expected bool
	}{
		{[]int{0, 1, 2, 3}, true},
		{[]int{3, 2, 1, 0}, true},
		{[]int{0, 1, 0, 3}, false},
		{[]int{0, 1, 2, 1}, false},
		{[]int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}, true},
		{[]int{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}, true},
		{[]int{0, 1, 2, 3, 2, 1, 0}, false}, // can be only decreasing or increasing
		{[]int{1, 3, 7, 5, 4}, false},       // can't decrease more than 3
		{[]int{1, 7, 3, 5, 4}, false},       // can't decrease more than 3
		{[]int{1, 1, 7, 3, 5, 4}, false},    // can't increase more than 3
	}

	for _, test := range tests {
		result := is_valid_path(test.path)
		if result != test.expected {
			t.Errorf("is_valid_path(%v) = %v; want %v", test.path, result, test.expected)
		}
	}
}
