#include "solution.h"

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */
// Time O(N), Space O(N)

struct TreeNode *lowestCommonAncestor(struct TreeNode *root, struct TreeNode *p,
                                      struct TreeNode *q) {

  if (root == NULL) {
    return NULL;
  }
  int rval = root->val;
  int qval = q->val;
  int pval = p->val;
  if (rval > pval && rval > qval) {
    // If both p and q are greater than parent
    return lowestCommonAncestor(root->left, p, q);
  } else if (rval < pval && rval < qval) {
    // If both p and q are lesser than parent
    return lowestCommonAncestor(root->right, p, q);
  } else {
    // We have found the split point, i.e. the LCA node.
    return root;
  }
}

// Time O(N), Space O(1)
struct TreeNode *lowestCommonAncestor_i(struct TreeNode *root,
                                        struct TreeNode *p,
                                        struct TreeNode *q) {

  if (root == NULL) {
    return NULL;
  }
  int qval = q->val;
  int pval = p->val;
  struct TreeNode *node = root;
  // Traverse the tree
  while (node != NULL) {
    // Value of ancestor/parent node.
    int rval = node->val;
    if (rval > pval && rval > qval) {
      // If both p and q are lesser than parent
      node = node->left;
    } else if (rval < pval && rval < qval) {
      // If both p and q are greater than parent
      node = node->right;
    } else {
      // We have found the split point, i.e. the LCA node.
      return node;
    }
  }
  return NULL;
}

void testLowestCommonAncestor(void) {

  int vec1[] = {6, 2, 8, 0, 4, 7, 9, 1 << 31, 1 << 31, 3, 5};
  struct TreeNode *root = to_tree(vec1, 11);
  struct TreeNode *p = malloc(sizeof(struct TreeNode));
  p->val = 2;
  p->left = NULL;
  p->right = NULL;
  struct TreeNode *q = malloc(sizeof(struct TreeNode));
  q->val = 8;
  q->left = NULL;
  q->right = NULL;

  struct TreeNode *res = lowestCommonAncestor(root, p, q);
  assert(res->val == 6);

  q->val = 4;
  res = lowestCommonAncestor(root, p, q);
  assert(res->val == 2);

  res = lowestCommonAncestor_i(root, p, q);
  assert(res->val == 2);

  clean_tree(root);
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
