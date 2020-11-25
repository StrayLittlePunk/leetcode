#include "util.h"

struct ListNode *to_list(int *vector, int n) {
  struct ListNode *node = malloc(sizeof(struct ListNode));
  node->val = vector[0];
  node->next = NULL;

  struct ListNode *head = node;
  struct ListNode *prev = head;

  /* int vector3[] = {2, 6, 4};
   * int vector4[] = {1, 5};
   * n = 3;
   * l1 = to_list(vector3, n);
   * l2 = to_list(vector4, 2);
   * assert(getIntersectionNode(l1, l2) == NULL);
   * clean_list(l1);
   * clean_list(l2); */
  // todo  连续调用两次tolist， 后面的内存指针跟前面一样，从而覆盖前面的内存，
  // debug时候发现在下面for循环时候malloc 返回的值为NULL，但是未赋值给node
  //难以理解 ， 写完操作系统后回来看看C语言怎么弄成这样的，反汇编可执行程序后
  // %rax 值被引用， 因为malloc，由jemalloc库负责动态内存分配，之前做csapp 的
  // 实验研究过 jemalloc源码，看懂一小部分，复杂性过高，等回来再一同解决
  //
  for (int i = 1; i < n; i++) {
    node = malloc(sizeof(struct ListNode));
    if (node == NULL) {
      node = malloc(sizeof(struct ListNode));
    }

    node->val = vector[i];
    node->next = NULL;
    prev->next = node;
    prev = node;
    node = NULL;
  }

  return head;
}

struct TreeNode *to_tree(int *vector, int n) {
  if (n < 1) {
    return NULL;
  }

  struct TreeNode *root = malloc(sizeof(struct TreeNode));
  if (root == NULL) {
    return NULL;
  }
  root->val = vector[0];
  root->left = NULL;
  root->right = NULL;
  if(n == 1) {
    return root;
  }


  struct TreeNode *queue[n];
  int queue_back_count = 0;
  int queue_front_count = 0;
  queue[queue_back_count % n] = root;

  struct TreeNode *p;
  int i;
  for (i = 1; i < n; i += 2) {

    struct TreeNode *l = NULL;
    struct TreeNode *r = NULL;
    p = queue[queue_front_count % n];
    queue_front_count++;
    // NULL NODE
    if (vector[i] != 1 << 30) {
      l = malloc(sizeof(struct TreeNode));
      l->val = vector[i];
      l->left = NULL;
      l->right = NULL;
      queue_back_count++;
      queue[queue_back_count % n] = l;
    }
    if (i + 1 != n && vector[i + 1] != 1 << 30) {
      r = malloc(sizeof(struct TreeNode));
      r->val = vector[i + 1];
      r->left = NULL;
      r->right = NULL;
      queue_back_count++;
      queue[queue_back_count % n] = r;
    }

    p->left = l;
    p->right = r;
  }

  return root;
}

void clean_tree(struct TreeNode *root) {
  if (root != NULL) {
    clean_tree(root->left);
    clean_tree(root->right);
    free(root);
  }
}

void inorder_traversal(struct TreeNode *root) {
  if(root == NULL) {
    return;
  } else {
     inorder_traversal(root->left);
     printf("%d\n", root->val);
     inorder_traversal(root->right);
  }
}

void clean_list(struct ListNode *head) {

  if (head == NULL)
    return;

  else if (head->next == NULL) {
    free(head);
    return;
  }

  struct ListNode *slow = head->next;
  struct ListNode *fast = head->next->next;

  while (slow != fast) {
    if (fast == NULL || fast->next == NULL)
      break;
    slow = slow->next;
    fast = fast->next->next;
  }
  if (fast == NULL || fast->next == NULL) {

    struct ListNode *tmp;
    while (head != NULL) {
      tmp = head;
      head = head->next;
      free(tmp);
    }
  } else {
    // has cycle
    slow = head;
    while (slow != fast) {
      slow = slow->next;
      fast = fast->next;
    }

    struct ListNode *tmp;
    while (head != slow) {
      tmp = head;
      head = head->next;
      tmp->val = 0;
      tmp->next = NULL;
      free(tmp);
    }
    head = head->next;
    while (head != slow) {
      tmp = head;
      head = head->next;
      tmp->val = 0;
      tmp->next = NULL;
      free(tmp);
    }
    free(slow);
  }
}
