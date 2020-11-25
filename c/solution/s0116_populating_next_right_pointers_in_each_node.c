#include "solution.h"

// Time O(N) Space O(1)
struct Node *connect_116(struct Node *root) {

  if (root == NULL) {
    return root;
  }

  // Start with the root node. There are no next pointers
  // that need to be set up on the first level

  struct Node *leftmost = root;

  // once we reach the final level, we are done
  while (leftmost->left != NULL) {

    // Iterate the "linked list" starting from the head
    // node and using the next pointers, establish the
    // corresponding links for the next level
    struct Node *head = leftmost;

    while (head != NULL) {
      // CONNECTION 1
      head->left->next = head->right;

      // CONNECTION 2
      if (head->next != NULL) {
        head->right->next = head->next->left;
      }

      // progress along the list (nodes on the current level)
      head = head->next;
    }

    leftmost = leftmost->left;
  }

  return root;
}

void test_connect116(void) {
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
