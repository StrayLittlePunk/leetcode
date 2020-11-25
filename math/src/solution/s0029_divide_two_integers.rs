#![allow(unused)]
pub struct Solution {}

use std::i32::{MAX, MIN};

const HALF_MIN: i32 = MIN / 2;
impl Solution {
    //Time O(N) Space O(1)
    //Time Limit Exceeded
    pub fn divide_bruteforce(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        let mut negatives = 2;

        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        let mut quotient = 0;
        while dividend - divisor <= 0 {
            dividend -= divisor;
            quotient -= 1;
        }

        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }
    // Time O(log^2 n) space O(1)
    pub fn divide_better(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        let mut negatives = 2;

        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        let mut quotient = 0;
        /* Once the divisor is bigger than the current dividend,
         * we can't fit any more copies of the divisor into it. */
        while dividend - divisor <= 0 {
            /* We know it'll fit at least once as divivend >= divisor.
             * Note: We use a negative powerOfTwo as it's possible we might have
             * the case divide(INT_MIN, -1). */
            let mut power_two = -1;
            let mut value = divisor;
            /* Check if double the current value is too big. If not, continue doubling.
             * If it is too big, stop doubling and continue with the next step */
            while value >= HALF_MIN && value + value >= dividend {
                value += value;
                power_two += power_two;
            }
            // Remove value so far so that we can continue the process with remainder.
            dividend -= value;

            // We have been able to subtract divisor another powerOfTwo times.
            quotient += power_two;
        }

        /* If there was originally one negative sign, then
         * the quotient remains negative. Otherwise, switch
         * it to positive. */
        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }
    // Time O(log n) space O(log n)
    // Accepted
    pub fn divide_memorize(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        let mut negatives = 2;

        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        let mut doubles = vec![];
        let mut powers_two = vec![];

        /* Nothing too exciting here, we're just making a list of doubles of 1 and
         * the divisor. This is pretty much the same as Approach 2, except we're
         * actually storing the values this time. */
        let mut power_two = -1;
        while dividend <= divisor {
            doubles.push(divisor);
            powers_two.push(power_two);

            if divisor < HALF_MIN {
                break;
            }

            divisor += divisor;
            power_two += power_two;
        }
        let mut quotient = 0;
        /* Go from largest double to smallest, checking if the current double fits.
         * into the remainder of the dividend. */
        for i in (0..doubles.len()).rev() {
            if doubles[i] >= dividend {
                // If it does fit, add the current powerOfTwo to the quotient.
                quotient += powers_two[i];
                // Update dividend to take into account the bit we've now removed.
                dividend -= doubles[i];
            }
        }
        /* If there was originally one negative sign, then
         * the quotient remains negative. Otherwise, switch
         * it to positive. */
        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }

    // Time O(log n) space O(log 1)
    // Accepted
    pub fn divide_constantspace(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        let mut negatives = 2;
        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        /* Nothing too exciting here, we're just making a list of doubles of 1 and
         * the divisor. This is pretty much the same as Approach 2, except we're
         * actually storing the values this time. */
        let mut highest_power_two = -1;
        let mut highest_double = divisor;
        while highest_double >= HALF_MIN && dividend <= highest_double + highest_double {
            highest_double += highest_double;
            highest_power_two += highest_power_two;
        }
        /* In the second loop, we work out which powers of two fit in, by
         * halving highestDouble and highestPowerOfTwo repeatedly.
         * We can do this using bit shifting so that we don't break the
         * rules of the question :-) */

        let mut quotient = 0;
        while dividend <= divisor {
            if dividend <= highest_double {
                quotient += highest_power_two;
                dividend -= highest_double;
            }

            /* We know that these are always even, so no need to worry about the
             * annoying "bit-shift-odd-negative-number" case. */
            highest_double >>= 1;
            highest_power_two >>= 1;
        }

        /* If there was originally one negative sign, then
         * the quotient remains negative. Otherwise, switch
         * it to positive. */
        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }
    // O(log n) O(1)
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        if dividend == MIN && divisor == -1 {
            return MAX;
        }

        if dividend == MIN && divisor == 1 {
            return MIN;
        }

        let mut negatives = 2;
        if dividend > 0 {
            negatives -= 1;
            dividend = -dividend;
        }

        if divisor > 0 {
            negatives -= 1;
            divisor = -divisor;
        }

        /* We want to find the largest doubling of the divisor in the negative 32-bit
         * integer range that could fit into the dividend.
         * Note if it would cause an overflow by being less than HALF_INT_MIN,
         * then we just stop as we know double it would not fit into INT_MIN anyway. */
        let mut max_bit = 0;
        while divisor >= HALF_MIN && divisor + divisor >= dividend {
            max_bit += 1;
            divisor += divisor;
        }

        let mut quotient = 0;

        /* We start from the biggest bit and shift our divisor to the right
         * until we can't shift it any further */

        for bit in (0..max_bit+1).rev() {
            /* If the divisor fits into the dividend, then we should set the current
             * bit to 1. We can do this by subtracting a 1 shifted by the appropriate
             * number of bits. */
            if divisor >= dividend {
                quotient -= (1 << bit);
                /* Remove the current divisor from the dividend, as we've now
                 * considered this part. */
                dividend -= divisor;
            }
            /* Shift the divisor to the right so that it's in the right place
             * for the next positon we're checking at. */

           // We can no longer assume the divisor, that we're right shifting,
           // is always an even number. Therefore, we need to add 1 before doing the right shift.
            divisor = (divisor + 1) >> 1;
        }

        /* If there was originally one negative sign, then
         * the quotient remains negative. Otherwise, switch
         * it to positive. */
        if negatives != 1 {
            quotient = -quotient;
        }

        quotient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_67() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(1, 1), 1);
        assert_eq!(Solution::divide(0, 1), 0);
        assert_eq!(Solution::divide(7, -3), -2);
    }
}
