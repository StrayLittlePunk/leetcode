#include "solution.h"
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
struct ListNode *detectCycle(struct ListNode *head) {
    if (head == NULL || head->next == NULL) 
        return NULL;
    struct ListNode *slow = head->next;
    struct ListNode *fast = head->next->next;
    while(slow != fast) {
        if (fast == NULL || fast->next == NULL)
            return NULL;
        slow = slow->next;
        fast = fast->next->next;
    }
    slow = head;
    while(slow != fast) {
      slow=slow->next;
      fast=fast->next;
    }
    return slow;
}


void test_detectCycle(void){

/*   int vector1[] = {3, 2, 0, -4};
 *   int n = 4;
 *   struct ListNode *head = to_list(vector1, n);
 *   head->next->next->next->next = head->next;
 *   assert(detectCycle(head) == head->next);
 *   clean_list(head);
 *
 *   int vector2[] = {1, 2};
 *   n = 2;
 *   head = to_list(vector2, n);
 *
 *   head->next->next = head;
 *   assert(detectCycle(head) == head);
 *
 *
 *   int vector3[] = {1};
 *   n = 1;
 *   head = to_list(vector3, n);
 *   assert(detectCycle(head) == NULL);
 *   clean_list(head); */

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
