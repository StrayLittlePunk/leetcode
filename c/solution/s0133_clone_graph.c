#include "solution.h"

struct NodeGraph *copy_node(struct NodeGraph *node, struct NodeGraph ***list,
                            int *_len) {
  int len = *_len;
  int i;
  if (!node)
    return 0;
  while (node->val >= len) {
    struct NodeGraph **t = (*list);
    (*list) = malloc(len * sizeof(struct NodeGraph *) * 2);
    for (i = 0; i < len; i++)
      (*list)[i] = t[i];
    len *= 2;
    *_len = len;
    free(t);
    for (; i < len; i++)
      (*list)[i] = 0;
  }
  if ((*list)[node->val])
    return (*list)[node->val];

  struct NodeGraph *new_node = malloc(len * sizeof(struct NodeGraph *));
  (*list)[node->val] = new_node;
  new_node->val = node->val;
  new_node->numNeighbors = node->numNeighbors;
  new_node->neighbors =
      malloc(sizeof(struct NodeGraph *) * new_node->numNeighbors);
  for (i = 0; i < new_node->numNeighbors; i++)
    new_node->neighbors[i] = copy_node(node->neighbors[i], list, _len);

  return new_node;
}

struct NodeGraph *cloneGraph(struct NodeGraph *s) {
  int len = 100;
  struct NodeGraph **list = malloc(len * sizeof(struct NodeGraph *));
  for (int i = 0; i < len; i++)
    list[i] = 0;

  return (copy_node(s, &list, &len));
}

// i Don't know wrong `runcode` input : [[]] is fine, but `submit` input: [[]]
// get runtime error

static bool node_vals[101];
static struct NodeGraph *node_arrays[101];
struct NodeGraph *cloneGraph_my(struct NodeGraph *s) {
  if (s == NULL) {
    return NULL;
  }

  if (node_vals[s->val] == true) {
    return node_arrays[s->val];
  }

  struct NodeGraph *ans = malloc(sizeof(struct NodeGraph));

  int neighbors = s->numNeighbors;
  ans->val = s->val;
  ans->numNeighbors = neighbors;
  ans->neighbors = malloc(neighbors * sizeof(struct NodeGraph *));

  // add visited node
  node_vals[s->val] = true;
  node_arrays[s->val] = ans;

  struct NodeGraph **neighbors_array = s->neighbors;
  for (int i = 0; i < neighbors; i++) {
    ans->neighbors[i] = cloneGraph(neighbors_array[i]);
  }

  return ans;
}

void test_cloneGraph(void) {

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
