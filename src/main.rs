
#[inline]
fn get_bit_lsb(x: u64, i: u8) -> u64 {
    (x >> i) & 1
}

#[inline]
fn set_bit_lsb(x: &mut u64, i: u8, bit: u64) {
    debug_assert!(bit == 0 || bit == 1);
    *x = (*x & !(1u64 << i)) | ((bit << i) as u64);

}

fn permute_lsb(input: u64, table: &[u8]) -> u64 {

    assert!(table.len() <= 64);

    let mut out = 0u64;
    for (out_i, &in_i) in table.iter().enumerate() {
        assert!(in_i < 64);
        let b = get_bit_lsb(input, in_i);
        out |= b << out_i;
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block64(pub u64);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key64(pub u64);

impl Key64 {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
}

fn main() {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_identity_8bits() {

        let table: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

        let x = 0b1010_0101u64;
        let y = permute_lsb(x, &table);

        assert_eq!(y, x & 0xFF);
    }

    #[test]
    fn permute_reverse_8bits() {
        // reverse bits: output bit 0 gets input bit 7, ..., output bit 7 gets input bit 0
        let table: [u8; 8] = [7, 6, 5, 4, 3, 2, 1, 0];

        let x = 0b1010_0101u64; // 0xA5
        let y = permute_lsb(x, &table);

        // reversed of 1010_0101 is 1010_0101 (palindrome-ish) -> actually still 0xA5 here
        assert_eq!(y, 0b1010_0101u64);

        // try another value to be sure
        let x2 = 0b0001_0110u64; // 0x16
        let y2 = permute_lsb(x2, &table);
        // reverse of 0001_0110 is 0110_1000 (0x68)
        assert_eq!(y2, 0b0110_1000u64);
    }

    #[test]
    fn permute_single_bit_mapping() {
        // A table that picks bits [2,0,3,1] from the input into output bits [0..3]
        let table: [u8; 4] = [2, 0, 3, 1];

        // Set only input bit 2 => output bit 0 should become 1
        let x = 1u64 << 2;
        let y = permute_lsb(x, &table);
        assert_eq!(y, 1u64 << 0);

        // Set only input bit 0 => output bit 1 should become 1
        let x = 1u64 << 0;
        let y = permute_lsb(x, &table);
        assert_eq!(y, 1u64 << 1);

        // Set only input bit 3 => output bit 2 should become 1
        let x = 1u64 << 3;
        let y = permute_lsb(x, &table);
        assert_eq!(y, 1u64 << 2);

        // Set only input bit 1 => output bit 3 should become 1
        let x = 1u64 << 1;
        let y = permute_lsb(x, &table);
        assert_eq!(y, 1u64 << 3);
    }
}

