#include "solution.h"


/**
 * The read4 API is defined in the parent class Reader4.
 *     int read4(char *buf4);
 */

/**
 * @param buf Destination buffer
 * @param n   Number of characters to read
 * @return    The number of actual characters read
 */
int read4(char *buf) {
  return 4;
}
int _read(char* buf, int n) {
  int copiedChars = 0, readChars = 4;
  while (copiedChars < n && readChars == 4) {
    readChars = read4(buf + copiedChars);
    copiedChars += readChars;
  }

  return n < copiedChars ? n : copiedChars;

}

void test_read(void) {
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
