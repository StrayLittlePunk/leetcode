#include "util/util.h"
#include <assert.h>
#include <string.h>


bool hasCycle(struct ListNode *head);
void test_hasCycle(void);

void deleteNode(struct ListNode *node);
void test_deleteNode(void);

struct ListNode *getIntersectionNode(struct ListNode *headA,
                                     struct ListNode *headB);
void test_getIntersectionNode(void);

struct ListNode *detectCycle(struct ListNode *head);
void test_detectCycle(void);

struct TreeNode *lowestCommonAncestor(struct TreeNode *root, struct TreeNode *p,
                                      struct TreeNode *q);
void testLowestCommonAncestor(void);

struct TreeNode *lowestCommonAncestorII(struct TreeNode *root, struct TreeNode *p,
                                      struct TreeNode *q);
bool lowestCommonAncestorHelper(struct TreeNode *root, struct TreeNode *p,
                                      struct TreeNode *q);
void testLowestCommonAncestorII(void);


void test_serialize_deserialize(void);
struct TreeNode *deserialize(char *data);
struct TreeNode *newNode(int val);



struct TreeNode* inorderSuccessor(struct TreeNode* root, struct TreeNode* p);
void test_inorderSuccessor(void);
void helper(struct TreeNode* root, struct TreeNode* p, bool *sign, struct TreeNode **res);


struct Node *connect_116(struct Node *root);
void test_connect116(void);

struct Node *connect_117(struct Node *root);
void test_connect117(void);


struct NodeGraph *cloneGraph_my(struct NodeGraph *s);
void test_cloneGraph(void);
struct NodeGraph *copy_node(struct NodeGraph *node, struct NodeGraph ***list, int *_len);
struct NodeGraph *cloneGraph(struct NodeGraph *s);

uint32_t reverseBits(uint32_t n);
void test_reverseBits(void);

struct NodeRandom* copyRandomList(struct NodeRandom* head);
void test_copyRandomList(void);

int _read(char* buf, int n);
int read4(char* buf);
void test_read(void);
void test_read_ii(void);
void test_peekingIterator(void);


/* C语言中， 修饰符extern 用在变量或者函数的声明前， 用来说明“此变量/函数”是在
 * 别处定义的，要在此引用 */
extern unsigned int passed;
extern unsigned int failed;
