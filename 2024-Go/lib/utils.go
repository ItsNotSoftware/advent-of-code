package lib

import "fmt"

func MakeMat[T any](sizeL, sizeC int) [][]T {
	mat := make([][]T, sizeL)
	for i := 0; i < sizeL; i++ {
		mat[i] = make([]T, sizeC)
	}
	return mat
}

func PrintMat[T any](mat [][]T) {
	for _, row := range mat {
		fmt.Println(row)
	}
	fmt.Println()
}

func OrBoolMats(dest, other *[][]bool) {
	for i := range *other {
		for j := range (*other)[i] {
			(*dest)[i][j] = (*dest)[i][j] || (*other)[i][j]
		}
	}
}

func InBoundsMat[T any](mat [][]T, l, c int) bool {
	return l >= 0 && l < len(mat) && c >= 0 && c < len(mat[0])
}

func InBoundsArray[T any](arr []T, i int) bool {
	return i >= 0 && i < len(arr)
}

func MatCount[T comparable](mat [][]T, value T) int {
	count := 0
	for _, row := range mat {
		for _, cell := range row {
			if cell == value {
				count++
			}
		}
	}
	return count
}

func ArrayCount[T comparable](arr []T, value T) int {
	count := 0
	for _, cell := range arr {
		if cell == value {
			count++
		}
	}
	return count
}
