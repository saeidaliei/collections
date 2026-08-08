## Matrix Multiplication

Tiled matrix multiplication. To understand the tiled matrix multiplication 
we take the example of 4x4 matrices:

```math
A = 
\begin{bmatrix}
    a_{00} & a_{01} & \vert & a_{02} & a_{03} \nonumber \\
    a_{10} & a_{11} & \vert & a_{12} & a_{13} \nonumber \\
    \hline
    a_{20} & a_{21} & \vert & a_{22} & a_{23} \nonumber \\
    a_{30} & a_{31} & \vert & a_{32} & a_{33}
\end{bmatrix} = 
\begin{bmatrix}
	A_{00} & A_{01} \nonumber \\
	A_{10} & A_{11}
\end{bmatrix}
```
We can simply show that, $`C_{ij} = A_{ik}B_{kj}`$ , or:

```math
\begin{align}
\begin{bmatrix}
	C_{00} & C_{01} \\
	C_{10} & C_{11}
\end{bmatrix}  = 
\begin{bmatrix}
	A_{00}B_{00} + A_{01}B_{10} & A_{00}B_{01} + A_{01}B_{11} \nonumber \\
	A_{10}B_{00} + A_{11}B_{10} & A_{10}B_{01} + A_{11}B_{11}
\end{bmatrix}
\end{align}
```

Such that for example for $`C_{00}`$ we have:

```math
\begin{align}
c_{00} & = a_{00}b_{00} + a_{01}b_{10} + a_{02}b_{20} + a_{03}b_{30} \nonumber \\
c_{01} & = a_{00}b_{01} + a_{01}b_{11} + a_{02}b_{21} + a_{03}b_{31} \nonumber \\
c_{10} & = a_{10}b_{00} + a_{11}b_{10} + a_{12}b_{20} + a_{13}b_{30} \nonumber \\
c_{11} & = a_{10}b_{01} + a_{11}b_{11} + a_{12}b_{21} + a_{13}b_{31} \nonumber
\end{align}
```

So to compute $`C_{00}`$ we do two multiplication, $`A_{00}B_{00}`$, and $`A_{01}B_{10}`$. 

To write the CUDA kernel for tiled matrix multiplication, we model the computation such that:
- Every thread is responsible for one element of the output matrix $`C`$, so thread(0, 0) computes $`c_{00}`$.
- Every thread block is responsible for one tile of the output matrix $`C`$, 
such that block(0, 0) will load $`A_{00}`$ and $`B_{00}`$ into shared memory, 
do the partial matrix multiplication, then load the next tiles, 
$`A_{01}`$ and $`B_{10}`$ into shared memory and add the partial multiplication result to the previous one. 

Note the indexing, when we load from matrix $`A`$, for each element of one tile of $`A`$, 
we move `N * row` to get to the element's row, `i * TILE_WIDTH` to get to the tile, 
and inside each tile, we move `threadIdx.x` times to get to the element, 
so in total `(N * row) + (i * TILE_WIDTH) + threadIdx.x`, 
which corresponds to the element `t_A[threadIdx.y][threadIdx.x]` of the tile, 
or element `threadIdx.y * TILE_WIDTH + threadIdx.x`. 
For loading columns from matrix $`B`$ to its corresponding tile in shared memory, 
we move `i * TILE_WIDTH + threadIdx.y` downward to get to each element's row, 
and we move `col` to the right to get each element of the column.

For launching the kernel we specify the grid and block dimension such that in each block we have 
`TILE_WIDTH * TILE_WIDTH` threads, `(N / TILE_WIDTH) * (N / TILE_WIDTH)` blocks with possible padding, 
also for convenience we do 2D indexing.

There is a blog post also on tiled matrix multiplication: 
[saliei.io/posts/tiled-matrix-multiplication](https://saliei.io/posts/tiled-matrix-multiplication/).
