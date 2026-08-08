#include <stdlib.h>

#include "bt.h"

Node* new_node(int data) {
    Node* node = (Node*)malloc(sizeof(Node));
    node->data = data;
    node->left = NULL;
    node->right = NULL;
    
    return node;
}
