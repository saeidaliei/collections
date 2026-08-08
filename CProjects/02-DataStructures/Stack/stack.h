#ifndef STACK_H
#define STACK_H 1

#define MAX_STACK_ARRAY_SIZE 32
#define TOP_INDEX -1

// an array based stack
struct stack_array_struct {
    size_t top;
    int data[MAX_STACK_ARRAY_SIZE];
} stack_array_default = {TOP_INDEX}; // default index for the top is TOP_INDEX

typedef struct stack_array_struct stack;

#endif // STACK_H
