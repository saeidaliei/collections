CUDA-Aware MPI is an MPI implementation that allows GPU buffers (e.g., GPU memory allocated with cudaMalloc) 
to be used directly in MPI calls. However, CUDA-Aware MPI by itself does not specify whether data is staged 
through CPU memory or passed directly from GPU to GPU. That's where GPUDirect comes in!

GPUDirect can enhance CUDA-Aware MPI by allowing data transfers directly between GPUs 
on the same node (peer-to-peer) or directly between GPUs on different nodes (RDMA support) 
without the need to stage data through CPU memory.

- [https://github.com/olcf-tutorials/MPI_ping_pong](MPI-Ping-Pong)
