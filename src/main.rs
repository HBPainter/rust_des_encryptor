
#[inline]
fn get_bit_msb(input: u64, pos_1_based: u8) -> u64 {
    debug_assert!((1..=64).contains(&pos_1_based));
    let shift = (64 - pos_1_based) as u32;
    (input >> shift) & 1
}

fn permute_msb(input: u64, table: &[u8]) -> u64 {
    let mut out: u64 = 0;
    for &pos in table {
        out = (out << 1) | get_bit_msb(input, pos);
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Block64(pub u64);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key64(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Key56(u64);

const PC1_MSB: [u8; 56] = [57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26,
    18, 10, 2,	59, 51, 43, 35,	27, 19,	11,	3, 60, 52, 44, 36, 63, 55, 47, 39,
    31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29, 21, 13,
    5, 	28, 20, 12, 4 ];

//TODO Permutation table found on the internet make it dynamic in the future

impl Key64 {
    pub fn has_valid_odd_parity(self) -> bool {
        for i in 0..8 {
            let byte = ((self.0 >> (8 * i)) & 0xFF) as u8;
            if byte.count_ones() % 2 == 0 {
                return false;
            }
        }
        true
    }

    pub fn pc1(self) -> Key56 {
        let k56 = permute_msb(self.0, &PC1_MSB);

        Key56(k56)
    }
}

fn main() {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_msb_identity_64bits() {
        const ID64: [u8; 64] = [
            1, 2, 3, 4, 5, 6, 7, 8,
            9, 10, 11, 12, 13, 14, 15, 16,
            17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
            33, 34, 35, 36, 37, 38, 39, 40,
            41, 42, 43, 44, 45, 46, 47, 48,
            49, 50, 51, 52, 53, 54, 55, 56,
            57, 58, 59, 60, 61, 62, 63, 64,
        ];

        let x = 0x0123_4567_89AB_CDEFu64;
        let y = permute_msb(x, &ID64);

        assert_eq!(y, x);
    }
}

