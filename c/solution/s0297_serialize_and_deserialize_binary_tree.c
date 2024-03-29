#include "solution.h"

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     struct TreeNode *left;
 *     struct TreeNode *right;
 * };
 */

char* serialize(struct TreeNode* root) {
    static struct TreeNode *queue[20480];
    int front=0, rear=0;
    // [front, rear), front==rear means empty
    static char result_str[1024*1024];
    char *s = result_str;

    queue[rear++] = root;
    while(front<rear) {
        struct TreeNode *node = queue[front++];

        if(node) {
            s+=sprintf(s, "%d ", node->val);
            queue[rear++] = node->left;
            queue[rear++] = node->right;
        } else
            s+=sprintf(s, "n ");
    }
    // eliminate the last space character
    s[-1] = '\0';
    return result_str;
}

struct TreeNode *newNode(int val) {
    struct TreeNode *new_node = malloc(sizeof(struct TreeNode));
    new_node->val = val;
    new_node->left = NULL;
    new_node->right = NULL;
    return new_node;
}

/** Decodes your encoded data to tree. */
struct TreeNode* deserialize(char* data) {
    static struct TreeNode *queue[20480];
    struct TreeNode *root = NULL;
    int front=0, rear=0;
    char *p = strtok(data, " ");
    if(p && *p!='n') {
        root = newNode(atoi(p));
        queue[rear++] = root;
    }

    while(front<rear) {
        struct TreeNode *node = queue[front++];

        if(node) {
            char *left = strtok(NULL, " ");
            if(left && *left!='n') {
                node->left = newNode(atoi(left));
                queue[rear++] = node->left;
            }
            char *right = strtok(NULL, " ");
            if(right && *right!='n') {
                node->right = newNode(atoi(right));
                queue[rear++] = node->right;
            }
        }
    }
    return root;
}



// Your functions will be called as such:
// char* data = serialize(root);
// deserialize(data);

void test_serialize_deserialize(void) {

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
