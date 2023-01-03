#include <stdio.h>
#include <stdlib.h>

struct Node {
    int value; 
    struct Node* next; 
};

// Note: this may look like a constant based on naming convention, but without
// `const`, it's actually declared as a global variable
const int kNumElements = 20;

struct Node* make_list() {
    // Make a list with one element
    struct Node* front = malloc(sizeof(struct Node));
    front->value = 0;
    front->next = NULL;

    struct Node* end = front;
    // Tack on 19 elements to the end of the list
    for (int i = 1; i < kNumElements; i++) {
        // Allocate a node and add it to the end of the list
        end->next = malloc(sizeof(struct Node));
        // Now, this new node is the end of the list
        end = end->next;
        // Initialize the new node
        end->value = i;
        end->next = NULL;
    }

    return front;
}

void swap_tenth_node(struct Node* list) {
    // Go to the 10th node
    struct Node* curr = list;
    // should be i < 9
    for (int i = 0; i < 10; i++) {
        curr = curr->next;
    }

    // Replace the next node
    struct Node* nextNext = curr->next->next;
    curr->next = malloc(sizeof(struct Node));
    curr->next->next = nextNext;
    curr->next->value = 100;

    // right one
    // struct Node* next = curr->next;
    // struct Node* nextNext = curr->next->next;
    // curr->next = malloc(sizeof(struct Node));
    // curr->next->next = nextNext;
    // curr->next->value = 100;
    // free(next);
}

/**
 * Program is going to create a linked list with 20 nodes. Then, it will
 * replace the 10th node with a different one, and finally print/free the list.
 */
int main() {
    struct Node* list = make_list();

    // Swap a node
    swap_tenth_node(list);

    /* Print and free everything */
    struct Node* curr = list;
    // should be curr != NULL
    while (curr->next != NULL) {
        printf("%d\n", curr->value);
        struct Node* next = curr->next;  
        free(curr);
        curr = next;
    }
}
