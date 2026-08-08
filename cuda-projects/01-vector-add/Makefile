NVCC := nvcc
ARCH := -arch=sm_89
OPT_FLAGS := -O3 -use_fast_math
WARN_FLAGS := --compiler-options -Wall,-Wextra
DEBUG_FALGS := -g -G -lineinfo
REGISTER_FLAGS := -Xptxas -v
MULTIARCH_FLAGS := -gencode arch=compute_70,code=sm_70 -gencode arch=compute_80,code=sm_80

# CUDA_PATH := /usr/local/cuda
# INCLUDE := -I$(CUDA_PATH)/include
# LIBS := -L$(CUDA_PATH)/lib64 -lcudart

SRC := vector_add.cu
EXE := vec.x

all: $(EXE)

$(EXE): $(SRC)
	$(NVCC) $(OPT_FLAGS) $(ARCH) $(WARN_FLAGS) $< -o $@

debug: $(SRC)
	$(NVCC) $(DEBUG_FLAGS) $(ARCH) $(WARN_FLAGS) $< -o $(EXE)

registers: $(SRC)
	$(NVCC) $(REGISTER_FLAGS) $< -o $(EXE)

multiarch: $(SRC)
	$(NVCC) $(OPT_FLAGS) $(MULTIARCH_FLAGS) $(WARN_FLAGS) $< -o $(EXE)

help:
	@echo "Available targets:"
	@echo "  vec.x       make default executable (default)"
	@echo "  debug       make debug executable"
	@echo "  registers   make register usage info version"
	@echo "  multiarch   make multi-architecture version"
	@echo "  clean       remove executable and object files"
	@echo "  help        print this help message"

clean:
	rm -f *.o $(EXE)

.PHONY: help clean
