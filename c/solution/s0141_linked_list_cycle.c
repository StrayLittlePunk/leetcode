#include "solution.h"
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */
bool hasCycle(struct ListNode *head) {
    if (head == NULL || head->next == NULL) 
        return false;
    struct ListNode *slow = head;
    struct ListNode *fast = head->next;
    while(slow != fast) {
        if (fast == NULL || fast->next == NULL)
            return false;
        slow = slow->next;
        fast = fast->next->next;
    }
    return true;
}


void test_hasCycle(void){

  int vector1[] = {3, 2, 0, -4};
  int n = 4;
  struct ListNode *head = to_list(vector1, n); 
  head->next->next->next->next = head->next;
  assert(hasCycle(head) == true);
  clean_list(head);

  int vector2[] = {1, 2};
  n = 2;
  head = to_list(vector2, n);

  head->next->next = head;
  assert(hasCycle(head) == true);


  int vector3[] = {1};
  n = 1;
  head = to_list(vector3, n);
  assert(hasCycle(head) == false);
  clean_list(head);

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
