#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <stdint.h>



 //  Definition for singly-linked list.
 struct ListNode {
      int val;
      struct ListNode *next;
 };


//Definition for a binary tree node.
struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

// Definition for a Node.
struct Node {
    int val;
    struct Node *left;
    struct Node *right;
    struct Node *next;
};


 struct NodeGraph {
     int val;
     int numNeighbors;
     struct NodeGraph** neighbors;
 };


struct NodeRandom {
    int val;
    struct Node *next;
    struct Node *random;
};

struct ListNode* to_list(int* vector, int n);
void clean_list(struct ListNode *head);
void inorder_traversal(struct TreeNode *root);
struct TreeNode* to_tree(int *vector, int n);
void clean_tree(struct TreeNode *root);
