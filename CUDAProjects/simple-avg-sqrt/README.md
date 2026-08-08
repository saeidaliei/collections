The root MPI process creates random numbers. Using MPI_Scatter we scatter them 
for all the processes, then deletes it, since its not needed anymore.
Each MPI process, allocates GPU buffers for input and output data buffers 
on the GPU, each of size per GPU process, launches the GPU kernel, which 
calculates the square root of the numbers in the array, copies the result back
to the host, and frees the buffers.
