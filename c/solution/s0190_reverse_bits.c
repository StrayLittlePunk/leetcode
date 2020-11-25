#include "solution.h"

// Time O(1) Space O(1)
uint32_t reverseBits(uint32_t n) {
  //  00000010100101000001111010011100
  //  00111001011110000010100101000000

  n = (n >> 16) | (n << 16);
  n = ((n & 0xff00ff00) >> 8) | ((n & 0x00ff00ff) << 8);
  n = ((n & 0xf0f0f0f0) >> 4) | ((n & 0x0f0f0f0f) << 4);
  n = ((n & 0xcccccccc) >> 2) | ((n & 0x33333333) << 2);
  n = ((n & 0xaaaaaaaa) >> 1) | ((n & 0x55555555) << 1);
  return n;
}

void test_reverseBits(void) {
  assert(reverseBits(0b00000010100101000001111010011100) ==
         0b00111001011110000010100101000000);
  assert(reverseBits(0b11111111111111111111111111111101) ==
         0b10111111111111111111111111111111);
  passed++;
  printf("test %s ... ok\n", __FILE__);
}
