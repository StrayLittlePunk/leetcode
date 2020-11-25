#include "solution.h"
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

void deleteNode(struct ListNode *node) {
  struct ListNode *tmp = node->next;
  node->next = node->next->next;
  node->val = tmp->val;
  free(tmp);
}

void test_deleteNode(void) {

  int vector1[] = {4, 5, 1, 9};
  int n = 4;
  struct ListNode *head = to_list(vector1, n);
  struct ListNode *node = head->next;
  deleteNode(node);
  int vector1_res[] = {4, 1, 9};
  struct ListNode *p = head;
  int i = 0;
  while (p != NULL) {
    assert(p->val == vector1_res[i]);
    i++;
    p = p->next;
  }
  clean_list(head);

  int vector2[] = {4, 5, 1, 9};
  n = 4;
  head = to_list(vector2, n);
  node = head->next->next;
  deleteNode(node);
  int vector2_res[] = {4, 5, 9};
  p = head;
  i = 0;
  while (p != NULL) {
    assert(p->val == vector2_res[i]);
    i++;
    p = p->next;
  }
  clean_list(head);

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
