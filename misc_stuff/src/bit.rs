#[inline]
pub fn zero_bit(n: u32, i: u32) -> u32 {
   n & (!(1 << i)) 
}

#[inline]
pub fn one_bit(n: u32, i: u32) -> u32 {
   n | (1 << i) 
}

#[inline]
pub fn swap_bit(n: u32, i: u32) -> u32 {
    n ^ (1 << i)
}
