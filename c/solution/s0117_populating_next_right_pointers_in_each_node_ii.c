#include "solution.h"

// Time O(N) Space O(1)

struct Node *prev;
struct Node *leftmost;

void process_child(struct Node *child) {

  if (child != NULL) {
    // If the "prev" pointer is alread set i.e. if we
    // already found atleast one node on the next level,
    // setup its next pointer
    if (prev != NULL) {
      prev->next = child;

    } else {
      // Else it means child node is the first node
      // we have encountered on the next level, so, we
      // set the leftmost pointer
      leftmost = child;
    }

    prev = child;
  }
}

struct Node *connect_117(struct Node *root) {

  if (root == NULL) {
    return root;
  }

  // The root node is the only node on the first level
  // and hence its the leftmost node for that level
  leftmost = root;

  // Variable to keep track of leading node on the "current" level
  struct Node *curr = leftmost;

  // We have no idea about the structure of the tree,
  // so, we keep going until we do find the last level.
  // the nodes on the last level won't have any children
  while (leftmost != NULL) {

    // "prev" tracks the latest node on the "next" level
    // while "curr" tracks the latest node on the current
    // level.
    prev = NULL;
    curr = leftmost;

    // We reset so that we can re-assign it to the leftmost
    // node of the next level. Also, if there isn't one, this
    // would help break us out of the outermost loop.
    leftmost = NULL;

    // Iterate on the nodes in the current level using
    // the next pointers already established.
    while (curr != NULL) {

      // Process both the children and update the prev
      // and leftmost pointers as necessary.
      process_child(curr->left);
      process_child(curr->right);

      // Move onto the next node.
      curr = curr->next;
    }
  }

  return root;
}

void test_connect117(void) {
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
