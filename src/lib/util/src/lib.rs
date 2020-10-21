/* SPDX-License-Identifier: GPL-2.0-only */

#[inline]
pub fn round_up_4k(num: usize) -> usize {
    num.checked_add(0xfff).expect("overflow in round_up_4k()") & !0xfff
}

#[inline]
pub fn round_down_4k(num: usize) -> usize {
    num & !0xfff
}

#[cfg(test)]
mod tests {
    use super::round_up_4k;

    #[test]
    fn round_up_4k_test_small() {
        assert_eq!(round_up_4k(0), 0);
        assert_eq!(round_up_4k(1), 4096);
        assert_eq!(round_up_4k(4095), 4096);
        assert_eq!(round_up_4k(4096), 4096);
        assert_eq!(round_up_4k(4097), 8192);
    }

    #[test]
    fn round_up_4k_test_big() {
        assert_eq!(round_up_4k(usize::MAX & !0xFFF), usize::MAX & !0xFFF);
        assert_eq!(round_up_4k(usize::MAX - 4095), usize::MAX & !0xFFF);
        assert_eq!(round_up_4k(usize::MAX - 4096), usize::MAX & !0xFFF);
        assert_eq!(round_up_4k(usize::MAX - 4097), usize::MAX & !0xFFF);
    }

    #[test]
    #[should_panic]
    fn round_up_4k_test_panic_max() {
        round_up_4k(usize::MAX);
    }

    #[test]
    #[should_panic]
    fn round_up_4k_test_panic() {
        round_up_4k((usize::MAX & !0xFFF) + 1);
    }
}
