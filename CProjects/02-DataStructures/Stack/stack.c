#include <stdlib.h>
#include <stdio.h>
#include <limits.h>

#include "stack.h"


int is_full(stack* st) {
    if(st->top == MAX_STACK_ARRAY_SIZE-1 ) {
        return 1;
    } else {
        return 0;
    }
}

int is_empty(stack* st) {
    if(st->top == TOP_INDEX) {
        return 1;
    }
    else {
        return 0;
    }
}

stack* push(stack* st, int val) {
    if(!is_full(st)) {
        st->top = st->top + 1;
        st->data[st->top] = val;
    }
    else {
        fprintf(stderr, "Error: stack is full!\n");
    }
    return st;
} 

int pop(stack* st) {
    int val = INT_MAX;
    if(is_empty(st)) {
        fprintf(stderr, "Error: stack is emtpy!\n");
    }
    else {
        val = st->data[st->top];
        st->top = st->top - 1;
    }
    return val;
}

int peek(stack* st) {
    return st->data[st->top];
}

int main(int argc, char* argv[]) {
    stack st = stack_array_default;

    push(&st, 10);
    push(&st, 11);
    push(&st, 12);
    push(&st, 13);
    push(&st, 14);
    push(&st, 15);

    int val1 = pop(&st);
    printf("%d\n", val1);
    int val2 = peek(&st);
    printf("%d\n", val2);
    int val3 = pop(&st);
    printf("%d\n", val3);

    return 0;
}
