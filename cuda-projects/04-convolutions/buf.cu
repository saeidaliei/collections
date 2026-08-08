#include <cuda_runtime.h>
#include <stdio.h>
#include <stdlib.h>

// Basic 2D convolution kernel
__global__ void conv2d_basic(const float* input,      // Input feature map
                            const float* kernel,       // Convolution kernel
                            float* output,            // Output feature map
                            const int batch_size,     // Number of images in batch
                            const int in_channels,    // Number of input channels
                            const int out_channels,   // Number of output channels
                            const int in_height,      // Input height
                            const int in_width,       // Input width
                            const int kernel_size)    // Kernel size (assuming square kernel)
{
    // Calculate output dimensions
    const int out_height = in_height - kernel_size + 1;
    const int out_width = in_width - kernel_size + 1;
    
    // Calculate global thread position
    const int x = blockIdx.x * blockDim.x + threadIdx.x;  // Output width position
    const int y = blockIdx.y * blockDim.y + threadIdx.y;  // Output height position
    const int k = blockIdx.z;  // Output channel index
    
    // Boundary check
    if (x >= out_width || y >= out_height || k >= out_channels) return;
    
    // For each batch
    for (int b = 0; b < batch_size; b++) {
        float sum = 0.0f;
        
        // For each input channel
        for (int c = 0; c < in_channels; c++) {
            // Convolution operation
            for (int ky = 0; ky < kernel_size; ky++) {
                for (int kx = 0; kx < kernel_size; kx++) {
                    // Input value
                    int in_x = x + kx;
                    int in_y = y + ky;
                    float in_val = input[
                        b * (in_channels * in_height * in_width) +
                        c * (in_height * in_width) +
                        in_y * in_width +
                        in_x
                    ];
                    
                    // Kernel value
                    float kernel_val = kernel[
                        k * (in_channels * kernel_size * kernel_size) +
                        c * (kernel_size * kernel_size) +
                        ky * kernel_size +
                        kx
                    ];
                    
                    sum += in_val * kernel_val;
                }
            }
        }
        
        // Write output
        output[
            b * (out_channels * out_height * out_width) +
            k * (out_height * out_width) +
            y * out_width +
            x
        ] = sum;
    }
}

// Helper function to check for CUDA errors
#define CHECK_CUDA_ERROR(val) check_cuda((val), #val, __FILE__, __LINE__)
template<typename T>
void check_cuda(T err, const char* const func, const char* const file, const int line)
{
    if (err != cudaSuccess)
    {
        fprintf(stderr, "CUDA Runtime Error at: %s:%d\n", file, line);
        fprintf(stderr, "%s %s\n", cudaGetErrorString(err), func);
        exit(1);
    }
}

// CPU reference implementation for verification
void conv2d_cpu_reference(const float* input,
                         const float* kernel,
                         float* output,
                         const int batch_size,
                         const int in_channels,
                         const int out_channels,
                         const int in_height,
                         const int in_width,
                         const int kernel_size)
{
    const int out_height = in_height - kernel_size + 1;
    const int out_width = in_width - kernel_size + 1;
    
    for (int b = 0; b < batch_size; b++) {
        for (int k = 0; k < out_channels; k++) {
            for (int y = 0; y < out_height; y++) {
                for (int x = 0; x < out_width; x++) {
                    float sum = 0.0f;
                    for (int c = 0; c < in_channels; c++) {
                        for (int ky = 0; ky < kernel_size; ky++) {
                            for (int kx = 0; kx < kernel_size; kx++) {
                                int in_x = x + kx;
                                int in_y = y + ky;
                                sum += input[
                                    b * (in_channels * in_height * in_width) +
                                    c * (in_height * in_width) +
                                    in_y * in_width +
                                    in_x
                                ] * kernel[
                                    k * (in_channels * kernel_size * kernel_size) +
                                    c * (kernel_size * kernel_size) +
                                    ky * kernel_size +
                                    kx
                                ];
                            }
                        }
                    }
                    output[
                        b * (out_channels * out_height * out_width) +
                        k * (out_height * out_width) +
                        y * out_width +
                        x
                    ] = sum;
                }
            }
        }
    }
}

int main()
{
    // Test parameters
    const int batch_size = 1;
    const int in_channels = 3;
    const int out_channels = 64;
    const int in_height = 224;
    const int in_width = 224;
    const int kernel_size = 3;
    
    // Calculate output dimensions
    const int out_height = in_height - kernel_size + 1;
    const int out_width = in_width - kernel_size + 1;
    
    // Allocate host memory
    const size_t input_size = batch_size * in_channels * in_height * in_width * sizeof(float);
    const size_t kernel_size_bytes = out_channels * in_channels * kernel_size * kernel_size * sizeof(float);
    const size_t output_size = batch_size * out_channels * out_height * out_width * sizeof(float);
    
    float* h_input = (float*)malloc(input_size);
    float* h_kernel = (float*)malloc(kernel_size_bytes);
    float* h_output = (float*)malloc(output_size);
    float* h_ref_output = (float*)malloc(output_size);
    
    // Initialize input and kernel with random values
    for (size_t i = 0; i < input_size/sizeof(float); i++) {
        h_input[i] = static_cast<float>(rand()) / RAND_MAX;
    }
    for (size_t i = 0; i < kernel_size_bytes/sizeof(float); i++) {
        h_kernel[i] = static_cast<float>(rand()) / RAND_MAX;
    }
    
    // Allocate device memory
    float *d_input, *d_kernel, *d_output;
    CHECK_CUDA_ERROR(cudaMalloc(&d_input, input_size));
    CHECK_CUDA_ERROR(cudaMalloc(&d_kernel, kernel_size_bytes));
    CHECK_CUDA_ERROR(cudaMalloc(&d_output, output_size));
    
    // Copy data to device
    CHECK_CUDA_ERROR(cudaMemcpy(d_input, h_input, input_size, cudaMemcpyHostToDevice));
    CHECK_CUDA_ERROR(cudaMemcpy(d_kernel, h_kernel, kernel_size_bytes, cudaMemcpyHostToDevice));
    
    // Set up grid and block dimensions
    dim3 block_dim(16, 16, 1);
    dim3 grid_dim(
        (out_width + block_dim.x - 1) / block_dim.x,
        (out_height + block_dim.y - 1) / block_dim.y,
        out_channels
    );
    
    // Create CUDA events for timing
    cudaEvent_t start, stop;
    CHECK_CUDA_ERROR(cudaEventCreate(&start));
    CHECK_CUDA_ERROR(cudaEventCreate(&stop));
    
    // Launch kernel and measure time
    CHECK_CUDA_ERROR(cudaEventRecord(start));
    conv2d_basic<<<grid_dim, block_dim>>>(
        d_input, d_kernel, d_output,
        batch_size, in_channels, out_channels,
        in_height, in_width, kernel_size
    );
    CHECK_CUDA_ERROR(cudaEventRecord(stop));
    
    // Copy result back to host
    CHECK_CUDA_ERROR(cudaMemcpy(h_output, d_output, output_size, cudaMemcpyDeviceToHost));
    
    // Calculate elapsed time
    float milliseconds = 0;
    CHECK_CUDA_ERROR(cudaEventElapsedTime(&milliseconds, start, stop));
    printf("Kernel execution time: %f ms\n", milliseconds);
    
    // Compute reference result
    conv2d_cpu_reference(
        h_input, h_kernel, h_ref_output,
        batch_size, in_channels, out_channels,
        in_height, in_width, kernel_size
    );
    
    // Verify results
    bool correct = true;
    for (size_t i = 0; i < output_size/sizeof(float); i++) {
        if (fabs(h_output[i] - h_ref_output[i]) > 1e-5) {
            printf("Error at index %zu: GPU = %f, CPU = %f\n", 
                   i, h_output[i], h_ref_output[i]);
            correct = false;
            break;
        }
    }
    
    if (correct) {
        printf("Test passed!\n");
    } else {
        printf("Test failed!\n");
    }
    
    // Cleanup
    free(h_input);
    free(h_kernel);
    free(h_output);
    free(h_ref_output);
    cudaFree(d_input);
    cudaFree(d_kernel);
    cudaFree(d_output);
    
    return 0;
}
