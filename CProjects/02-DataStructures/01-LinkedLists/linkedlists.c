#include <stdlib.h>
#include <stdio.h>

#include "linkedlists.h"

static SingleNode* create_single_node(double data) {
    SingleNode* new_node = (SingleNode*)malloc(sizeof(SingleNode));
    new_node->data = data;
    // by default when a node is created the next pointer is NULL
    new_node->next = NULL; 

    return new_node;
}

void push_back(SinglyLinkedList* list, double data) {
    SingleNode* new_node = create_single_node(data);

    // if the head of list is NULL, this list is empty
    if(list->head == NULL) {
        list->head = new_node;
    }
    // otherwise traverse to the end and add
    else {
        SingleNode* last_node = list->head;
        while(last_node->next != NULL) {
            last_node = last_node->next;
        }
        last_node->next = new_node;
    }
}

void push_front(SinglyLinkedList* list, double data) {
    SingleNode* new_node = create_single_node(data);
    // if head is pointing to NULL, then we set the next to NULL twice, 
    // but better than checking for NULLness of head
    new_node->next = list->head;
    list->head = new_node;
}

void print_list(SinglyLinkedList* list) {
    SingleNode* current_node = list->head;
    while(current_node != NULL) {
        printf("%.2lf-->", current_node->data);
        current_node = current_node->next;
    }
    printf("NULL\n");
}

void free_list(SinglyLinkedList* list) {
    while(list->head != NULL) {
        SingleNode* tmp = list->head;
        list->head = tmp->next;
        free(tmp);
    }
}

int main() {
    /*SingleNode snode;*/
    /*snode.data = 1.0;*/
    /*snode.next = NULL;*/

    /*DoubleNode dnode;*/
    /*dnode.data = 2.0;*/
    /*dnode.next = NULL;*/
    /*dnode.prev = NULL;*/

    /*printf("%lf\n", snode.data);*/
    /*printf("%lf\n", dnode.data);*/

    SinglyLinkedList list;
    list.head = NULL;
    push_back(&list, 1.0);
    push_back(&list, 2.0);
    push_back(&list, 3.0);
    push_front(&list, 0.0);
    push_front(&list, -1.0);
    print_list(&list);
    free_list(&list);

    return 0;
}
