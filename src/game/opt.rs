pub struct Mask {
  width: usize,

  mask: Vec<u128>,
}

pub struct PlayerAllowedInfo {
  occupied_masks: [u32; 20],
  corners_masks: [[u32; 20]; 4],
}


// completely unverified
fn compress_u128_to_u32(a: u128, mask: u64) -> u32 {
  (_pext_u64(a as u64, mask) | ((_pext_u64((a >> 64) as u64, mask) << 16)) as u32
}

impl PlayerAllowedInfo {
  pub fn new(mask: &Mask) {
  // 20 bits in each thingy, "is this square occupied?"
  // higher 12 bits are considered occupied and so are 1
  let mut occupied_masks = [0u32; 20];
  // "is this square occupied by color i?"
  let mut color_masks = [[0u32; 20]; 4];
  // "is this square adjacent to a color i?"
  let mut corners_masks = [[0u32; 20]; 4];

  for i in 0..20 {
    // pretend everything out of bounds is occupied
    let mut occupied_mask = !((1u32 << 20) - 1);
    for j in 0..4 {
      // Broadcasts 0x1, 0x2, 0x4 or 0x8 to each group of 4 bits
      let splatted = 0x1111111111111111u64 << (j as u64);

      // extract the squares used by the player denoted by 1 << j;
      color_masks[j] = compress_u128_to_u32(mask.mask[i], splatted);

      // accumulate the occupied mask
      occupied_mask |= color_masks[j];
    }

    occupied_masks[i] = occupied_mask;
  }

  for j in 0..4 {
    // compute corner masks from color masks ... TODO
  }

      PlayerAllowedInfo {
         occupied_masks,
     corners_masks,
      }
  }

/// Format for shape: shape should be justified to the "top right" corner. e.g.
/// shape =
///  [ 0b000...0010,
///    0b000...0111,
///    0b000...0010,
///    0b000...0010
///    0b000...0000,
/// ... ]
/// shape_width = 3 and shape_height = 4
/// This method needs adjustment for one case which is where the shape is a vertical 5x1.
/// But it's not too bad
pub unsafe fn do_shape(&self, shape: [u32; 8], player: usize,
  shape_width: u32, shape_height: u32) -> Vec<(u8, u8)> {

  use std::arch::x86_64::*;  //smth like this I forget

  // NB: Each __m256i stores exactly 8 rows because it is 256 bits wide.
  // 256 / 32 = 8.

  // Get a checker for rows [offset, offset + 7].
  // If we have a "piece" stored with the same format in an __m256i, then
  // none(piece & occupied) & some(piece & corners) means that the piece fits in those 8 rows
  let get_checker = |offset: usize| -> (__m256i /* occupied */, __m256i /* corners */) {
    (
      _mm256_loadu_si256(self.occupied_masks.as_ptr().wrapping_add(offset) as *const __m256i),
      _mm256_loadu_si256(self.corner_masks[player].as_ptr().wrapping_add(offset) as *const __m256i),
    )
  };

  // load the shape
  let shape = _mm256_loadu_si256(shape.as_ptr() as *const __m256i);

  // Take the shape and shift it one bit to the left
  // Visually:
  // shape =
  //  [ 0b000...0010,
  //    0b000...0111,
  //    0b000...0010,
  //    0b000...0010
  //    0b000...0000,
  // ... ]
  // new_shape =
  //  [ 0b000...0100,
  //    0b000...1110,
  //    0b000...0100,
  //    0b000...0100
  //    0b000...0000,
  // ... ]
  let shift_shape_left = |shape: __m256i| -> __m256i {
    _mm256_slli_epi32(shape, 1)   // shift all 32 bit elements left
  };

  // Take the shape and shift it one row down
  // Visually:
  // shape =
  //  [ 0b000...0100,
  //    0b000...1110,
  //    0b000...0100,
  //    0b000...0100
  //    0b000...0000,
  // ... ]
  // new_shape =
  //  [ 0b000...0000,
  //    0b000...0100,
  //    0b000...1110,
  //    0b000...0100,
  //    0b000...0100
  //    0b000...0000,
  // ... ]
  let shift_shape_down = #[inline(always)] |shape: __m256i| -> __m256i {
    // cycles 32 bit elements using the shuffle (untested whether this is the actual one)
    let shifted = _mm256_permutevar8x32_epi32(shape, _mm256_set_epi32(0, 7, 6, 5, 4, 3, 2, 1));
    // andnot(a,b) = ~a & b. Only keep the lower seven rows.
    _mm256_andnot_si256(_mm256_set_epi32(-1i32, 0, 0, 0, 0, 0, 0, 0), shifted)
  };

  // Check whether the given shape can fit
  let check = #[inline(always)] |checker: (__m256i, __m256i), shape: __m256i| {
    // testz computes the bitwise AND and returns 1 iff the result is zero
    (_mm256_testz_si256(checker.0, shape) as bool) && (!_mm256_testz_si256(checker.1, shape))
  };

  let mut results = Vec::new::<(u8 /* y */, u8 /* x */)>();
  
  match shape_height {
    1,2,3,4 => {
      // Check overlapping spans 0-7, 4-11, 8-15, and 12-19 all at once.
      // In a sense we are scanning four copies of the same shape across the whole thing.
      // For shapes of height <= 4 this gives us complete coverage
      let check0to7 = get_checker(0);
      let check4to11 = get_checker(4);
      let check8to15 = get_checker(8);
      let check12to19 = get_checker(12);

      // First iteration:
      //  [ 0b000...0100,
      //    0b000...1110,
      //    0b000...0100,
      //    0b000...0100
      //    0b000...0000,
      // ... ]
      // Second iteration:
      //  [ 0b000...0000,
      //    0b000...0100,
      //    0b000...1110,
      //    0b000...0100,
      //    0b000...0100
      //    0b000...0000,
      // ... ]
      let mut y_shifted_shape = shape;

      for y_offset in 0..4 {
        // gets shifted left over and over
        let mut shape = y_shifted_shape;

        for x_offset in 0..20 /* conservative. could do less dep. on width */ {

          // I'd expect the overhead to be dominated by .push, so a tweaked approach
          // to this innermost loop might be warranted, but unclear
          if check(check0to7, shape) {
            // succeeds if this translated version of the shape fits in rows 0-7
            results.push((y_offset, x_offset));
          }
          if check(check4to11, shape) {
            // succeeds if this translated version of the shape fits in rows 4-11
            results.push((y_offset + 4, x_offset));
          }
          if check(check8to15, shape) {
            // succeeds if this translated version of the shape fits in rows 8-15
            results.push((y_offset + 8, x_offset));
          }
          if check(check12to19, shape) {
            // succeeds if this translated version of the shape fits in rows 12-19
            results.push((y_offset + 12, x_offset));
          }

          shape = shift_shape_left(shape);
        }

        y_shifted_shape = shift_shape_down(y_shifted_shape);
      }
    },
    _ => unimplemented!()
  }

  results
}
}

fn main() {
  println!("Hello, world!");
}