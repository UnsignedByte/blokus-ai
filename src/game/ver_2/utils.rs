use std::arch::x86_64::*;

#[inline]
pub unsafe fn ymm(a: [u32; 8]) -> __m256i {
    _mm256_loadu_si256(a.as_ptr() as *const __m256i)
}

/// Rotates the ymms down 1
#[inline]
pub unsafe fn rotate_down_1(ymm: __m256i) -> __m256i {
    // This is safe because we only care about pieces that have height <= 4
    _mm256_permutevar8x32_epi32(ymm, _mm256_set_epi32(6, 5, 4, 3, 2, 1, 0, 7))
}

/// Shift ymms to the left
#[inline]
pub unsafe fn shift_left_1(ymm: __m256i) -> __m256i {
    _mm256_slli_epi32::<1>(ymm)
}

#[cfg(test)]
mod tests {
    use crate::game::ver_2::utils::*;
    use std::fmt::Write;

    fn ymm_str(ymm: __m256i) -> String {
        let rows = unsafe { std::mem::transmute::<__m256i, [u32; 8]>(ymm) };

        rows.iter().fold(String::new(), |mut output, v| {
            let _ = writeln!(output, "{v:032b}");
            output
        })
    }

    unsafe fn ymm_eq(a: __m256i, b: __m256i) -> bool {
        // Fills mask with 1 if a and b are equal
        let mask = _mm256_cmpeq_epi32(a, b);
        // Take the most significant bit from each 8 bit chunk
        let ret = _mm256_movemask_epi8(mask);
        ret == -1
    }

    #[test]
    fn rotate_down_test() {
        let ymm1 = unsafe { ymm([0, 1, 2, 3, 4, 5, 6, 7]) };
        let expected = unsafe { ymm([7, 0, 1, 2, 3, 4, 5, 6]) };
        let rotated = unsafe { rotate_down_1(ymm1) };

        println!("{}", ymm_str(rotated));
        println!("{}", ymm_str(expected));
        assert!(unsafe { ymm_eq(rotated, expected) })
    }
}
