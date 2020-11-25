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

static struct TreeNode *ans = NULL;
struct TreeNode *lowestCommonAncestorII(struct TreeNode *root,
                                        struct TreeNode *p,
                                        struct TreeNode *q) {

  lowestCommonAncestorHelper(root, p, q);
  return ans;
}

bool lowestCommonAncestorHelper(struct TreeNode *root, struct TreeNode *p,
                                struct TreeNode *q) {
  if (root == NULL) {
    return false;
  }

  int left = lowestCommonAncestorHelper(root->left, p, q) ? 1 : 0;
  int right = lowestCommonAncestorHelper(root->right, p, q) ? 1 : 0;

  int mid = (root == p || root == q) ? 1 : 0;

  if (mid + left + right >= 2) {
    ans = root;
  }

  return (mid + left + right > 0);
}


void testLowestCommonAncestorII(void) {

  int vec1[] = {3, 5, 1, 6, 2, 0, 8, 1 << 31, 1 << 31, 7, 4};
  struct TreeNode *root = to_tree(vec1, 11);
  struct TreeNode *p = root->left;
  struct TreeNode *q = root->right;

  struct TreeNode *res = lowestCommonAncestorII(root, p, q);
   assert(res->val == 3);

  q = root->left->right->right;
  res = lowestCommonAncestorII(root, p, q);
   assert(res->val == 5);

  clean_tree(root);
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
