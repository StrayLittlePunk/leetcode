#include "solution.h"
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

struct ListNode *getIntersectionNode(struct ListNode *headA,
                                     struct ListNode *headB) {

  struct ListNode *pA = headA;
  struct ListNode *pB = headB;

  // equal distance so can handle null testcase 
  // 因为交换起点， 所以都走了两次，路程相等，可以处理 NULL case
  while (pA != pB) {
    pA = pA == NULL ? headB : pA->next;
    pB = pB == NULL ? headA : pB->next;
  }
  return pA;
}

void test_getIntersectionNode(void) {

  int vector1[] = {0, 9, 1, 2, 4};
  int vector2[] = {3};
  int n = 5;
  struct ListNode *l1 = to_list(vector1, n);
  struct ListNode *l2 = to_list(vector2, 1);
  l2->next = l1->next->next->next;
  assert(getIntersectionNode(l1, l2) == l2->next);
  clean_list(l1);
  clean_list(l2);
 
  /* int vector3[] = {2, 6, 4};
   * int vector4[] = {1, 5};
   * n = 3;
   * l1 = to_list(vector3, n);
   * l2 = to_list(vector4, 2);
   * assert(getIntersectionNode(l1, l2) == NULL);
   * clean_list(l1);
   * clean_list(l2); */

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
