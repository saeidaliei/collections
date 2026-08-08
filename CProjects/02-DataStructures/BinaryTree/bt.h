#ifndef BT_H
#define BT_H 1

typedef struct Node {
    int data;
    struct Node* left;
    struct Node* right;
} Node;

Node* new_node(int data);

#endif // BT_H
