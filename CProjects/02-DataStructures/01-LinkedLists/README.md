### Linked Lists

Consists of nodes, each node has a data field and the address for the next node, 
in case of doubly linked list, also has reference to the previous node. The first 
node is the head and the last one is the tail which points to `NULL`. In circular 
linked lists the tail node points to the head node, which can also be doubly linked. 


- The memory size can be adjusted at runtime, there is no need for redundant memory allocation.
unlike arrays where the size is fixed, the size of linked list can grow or shrink.
- Insertion and deletion are simpler than arrays, where there is shifting of data involved, 
but for lineked lists, the addresses needed to be updated. But for arrays there is no need 
for allocation of extra space for next/previous pointer.
- Searching in a linked lists involves traversing the data, unlike arrays there is no direct
access to elements by index.
