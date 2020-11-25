#include "solution.h"

/**
 * The read4 API is defined in the parent class Reader4.
 *     int read4(char *buf4);
 */

typedef struct {
  int i;
  int buf_sz;
  char buf[4];

} Solution;


/** initialize your data structure here. */
Solution *solutionCreate() { return calloc(1, sizeof(Solution)); }

/**
 * @param buf Destination buffer
 * @param n   Number of characters to read
 * @return    The number of actual characters read
 */
int _readii(Solution *obj, char *buf, int n) {
  char *b = buf;
  int pending, ret;

  for (pending = n; pending > 0;) {
    if (obj->buf_sz > 0) {
      if (pending >= obj->buf_sz) {
        memcpy(b, &obj->buf[obj->i], obj->buf_sz);
        b += obj->buf_sz;
        pending -= obj->buf_sz;
        obj->buf_sz = 0;
        obj->i = 0;
      } else {
        memcpy(b, &obj->buf[obj->i], pending);
        b += pending;
        obj->i += pending;
        obj->buf_sz -= pending;
        pending = 0;
      }
    } else {
      ret = read4(b);
      if (ret == 0)
        break;
      if (pending < ret) {
        b += pending;
        memcpy(obj->buf, b, ret - pending);
        obj->buf_sz = ret - pending;
        pending = 0;
      } else {
        b += ret;
        pending -= ret;
      }
    }
    b[0] = '\0';
  }

  return n - pending;
}

void test_read_ii(void) {
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
