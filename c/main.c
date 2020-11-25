#include "solution/solution.h"

// count test passed count;
unsigned int passed = 0;
// count test failed count;
unsigned int failed = 0;

int main(int argc, char *argv[]) {
  // suppress warnings
  (void)argc;
  (void)argv;
  printf("Test Started!!\n");

  test_hasCycle();
  test_deleteNode();
  test_getIntersectionNode();
  test_detectCycle();
  testLowestCommonAncestor();
  testLowestCommonAncestorII();
  test_serialize_deserialize();
  test_inorderSuccessor();
  test_connect116();
  test_connect117();
  test_cloneGraph();
  test_read();
  test_read_ii();
  test_peekingIterator();

  // int v[] = {5, 3, 6, 1, 4, 1<<31, 9};
  // struct TreeNode *root = to_tree(v, 7);
  // inorder_traversal(root);
  // printf("\n");
  // int b[] = {5, 3, 6, 1, 4, 1<<31};
  // root = to_tree(b, 6);
  // inorder_traversal(root);
  printf("test result: ok %d passed; %d failed;\n", passed, failed);
  return 0;
}
