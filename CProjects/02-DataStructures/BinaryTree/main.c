#include <stdlib.h>
#include <stdio.h>

#include "bt.h"

int main(int argc, char* argv[]) {
    Node* a = new_node(1);
    Node* b = new_node(2);
    Node* c = new_node(3);
    Node* d = new_node(4);
    Node* e = new_node(5);
    Node* f = new_node(6);
    Node* g = new_node(7);

    a->left = b;
    a->right = c;
    b->left = d;
    b->right = e;
    c->left = f;
    c->right = g;
    
    printf("a->data: %d\n", a->data);
    printf("a->left->data: %d\n", a->left->data);

    free(a);
    free(b);
    free(c);
    free(d);
    free(e);
    free(f);
    free(g);
    return 0;
}
