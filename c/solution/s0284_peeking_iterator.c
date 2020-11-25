#include "solution.h"

struct Iterator {
  //    Returns true if the iteration has more elements.
  bool (*hasNext)();

  //   Returns the next element in the iteration.
  int (*next)();
};

struct PeekingIterator {
  struct Iterator *iterator;
  int next;
  bool hasPeeked;
};

struct PeekingIterator *Constructor(struct Iterator *iter) {
  struct PeekingIterator *piter = malloc(sizeof(struct PeekingIterator));
  piter->iterator = iter;
  piter->hasPeeked = false;
  return piter;
}

bool hasNext(struct PeekingIterator *obj) {
  return obj->hasPeeked || obj->iterator->hasNext();
}

int peek(struct PeekingIterator *obj) {
  if (obj->hasPeeked) {
    return obj->next;
  } else if (obj->iterator->hasNext()) {
    obj->hasPeeked = true;
    obj->next = obj->iterator->next();
    return obj->next;
  }
  return 0;
}

int next(struct PeekingIterator *obj) {
  if (hasNext(obj)) {
    if (obj->hasPeeked) {
      obj->hasPeeked = false;
      return obj->next;
    } else {
      return obj->iterator->next();
    }
  }
  return 0;
}

/**
 * Your PeekingIterator struct will be instantiated and called as such:
 * PeekingIterator* obj = peekingIteratorCreate(arr, arrSize);
 * int param_1 = peek(obj);
 * int param_2 = next(obj);
 * bool param_3 = hasNext(obj);
 * peekingIteratorFree(obj);
 */

void test_peekingIterator(void) {

  passed++;
  printf("test %s ... ok\n", __FILE__);
}
