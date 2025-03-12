package main

import (
	"fmt"
	"math"
	"math/rand"
	"os"
	"strconv"
	"time"
)

const MAXN = 2000 // Valor máximo de N

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Uso: go run gauss.go <dimensão_matriz> [semente_aleatória]")
		os.Exit(0)
	}

	N, err := strconv.Atoi(os.Args[1])
	if err != nil || N < 1 || N > MAXN {
		fmt.Printf("N = %d é inválido. Deve ser entre 1 e %d\n", N, MAXN)
		os.Exit(1)
	}

	var seed int64 = time.Now().UnixNano()
	if len(os.Args) >= 3 {
		if s, err := strconv.ParseInt(os.Args[2], 10, 64); err == nil {
			seed = s
		}
	}
	rand.Seed(seed)

	fmt.Printf("\nDimensão da matriz N = %d\n", N)
	fmt.Printf("Semente aleatória = %d\n", seed)

	// Inicializar matrizes
	A := make([][]float64, N)
	for i := range A {
		A[i] = make([]float64, N)
		for j := range A[i] {
			A[i][j] = rand.Float64()
		}
	}

	B := make([]float64, N)
	for i := range B {
		B[i] = rand.Float64()
	}

	X := make([]float64, N)

	printInputs(A, B, N)

	// Medir tempo de execução
	start := time.Now() // Tempo inicial

	// Executar a eliminação gaussiana
	X = gauss(N, A, B)

	elapsed := time.Since(start) // Tempo decorrido

	// Exibir resultados
	printX(X, N)

	// Exibir tempos
	fmt.Printf("\nTempo decorrido = %.3f ms\n", float64(elapsed.Nanoseconds())/1e6)
	fmt.Println("--------------------------------------------")
}

func printInputs(A [][]float64, B []float64, N int) {
	if N >= 10 {
		return
	}
	fmt.Println("\nA =")
	for row := 0; row < N; row++ {
		fmt.Print("\t")
		for col := 0; col < N; col++ {
			fmt.Printf("%5.2f", A[row][col])
			if col < N-1 {
				fmt.Print(", ")
			}
		}
		fmt.Println(";")
	}
	fmt.Println("\nB = [")
	for col := 0; col < N; col++ {
		fmt.Printf("%5.2f", B[col])
		if col < N-1 {
			fmt.Print("; ")
		}
	}
	fmt.Println("]")
}

func printX(X []float64, N int) {
	if N >= 100 {
		return
	}
	fmt.Println("\nX = [")
	for row := 0; row < N; row++ {
		fmt.Printf("%5.2f", X[row])
		if row < N-1 {
			fmt.Print("; ")
		}
	}
	fmt.Println("]")
}

func gauss(N int, A [][]float64, B []float64) []float64 {
	X := make([]float64, N)

	// Eliminação Gaussiana
	for norm := 0; norm < N-1; norm++ {
		for row := norm + 1; row < N; row++ {
			if A[norm][norm] == 0 {
				fmt.Println("Erro: Divisão por zero detectada")
				os.Exit(1)
			}
			multiplier := A[row][norm] / A[norm][norm]
			for col := norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier
			}
			B[row] -= B[norm] * multiplier
		}
	}

	// Substituição regressiva
	for row := N - 1; row >= 0; row-- {
		X[row] = B[row]
		for col := row + 1; col < N; col++ {
			X[row] -= A[row][col] * X[col]
		}
		if math.Abs(A[row][row]) < 1e-10 {
			fmt.Println("Erro: Pivô zero na substituição regressiva")
			os.Exit(1)
		}
		X[row] /= A[row][row]
	}

	return X
}