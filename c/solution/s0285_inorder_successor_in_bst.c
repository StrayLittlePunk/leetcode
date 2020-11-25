#include "solution.h"
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

// Time best case O(Hp) Hp is number of p height,
// O(H) H is height of tree, in the worst case of no right child
// Space O(log N) worst case, best case O(1)
struct TreeNode *inorderSuccessor(struct TreeNode *root, struct TreeNode *p) {
  // the successor is somewhere lower in the right subtree
  // successor: one step right and then left till you can
  if (root == NULL || p == NULL) {
    return NULL;
  }
  if (p->right != NULL) {
    p = p->right;
    while (p->left != NULL)
      p = p->left;
    return p;
  }
  bool sign = false;
  struct TreeNode *res = NULL;
  helper(root, p, &sign, &res);
  return res;
}

void helper(struct TreeNode *root, struct TreeNode *p, bool *sign,
            struct TreeNode **res) {
  if (root == NULL) {
    return;
  }

  helper(root->left, p, sign, res);
  // do stuff
  if (*sign == true) {
    *sign = false;
    *res = root;
    return;
  }

  if (root == p) {
    *sign = true;
  }

  helper(root->right, p, sign, res);
  return;
}

void test_inorderSuccessor(void) {

  int a[] = {2, 1, 3};

  struct TreeNode *root = to_tree(a, 3);
  struct TreeNode *p = root->left;
  struct TreeNode *ans = inorderSuccessor(root, p);
  assert(ans->val == 2);
  clean_tree(root);

  int b[] = {5, 3, 6, 2, 4, 1 << 30, 1 << 30, 1};
  root = to_tree(b, 8);
  p = root->right;
  ans = inorderSuccessor(root, p);
  assert(ans == NULL);
  clean_tree(root);

  int c[] = {0};
  root = to_tree(c, 1);
  p = root;
  ans = inorderSuccessor(root, p);
  assert(ans == NULL);

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
