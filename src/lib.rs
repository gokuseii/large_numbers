#[derive(PartialEq, PartialOrd)]
struct BigInt(Vec<u32>);

impl BigInt {
    fn new(value: &str) -> Self {
        let mut n = BigInt::empty();
        n.set_hex(value);
        n
    }

    fn empty() -> Self {
        Self(Vec::new())
    }

    fn set_hex(&mut self, hex: &str) {
        self.0 = hex.chars().filter_map(|ch| ch.to_digit(16)).collect();
    }

    fn get_hex(&self) -> String {
        self.0.iter().map(|digit| format!("{:x}", digit)).collect()
    }

    fn xor(&self, int: &BigInt) -> BigInt {
        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());
        let result_digits = pad_a.iter().zip(&pad_b).map(|(&a, &b)| a ^ b).collect();
        BigInt(result_digits)
    }

    fn inv(&self) -> BigInt {
        let result_digits = self.0.iter().map(|digit| !digit ).collect();
        BigInt(result_digits)
    }

    fn or(&self, int: &BigInt) -> BigInt {
        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());
        let result_digits = pad_a.iter().zip(&pad_b).map(|(&a, &b)| a | b).collect();
        BigInt(result_digits)
    }

    fn and(&self, int: &BigInt) -> BigInt {
        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());
        let result_digits = pad_a.iter().zip(&pad_b).map(|(&a, &b)| a & b).collect();
        BigInt(result_digits)
    }

    fn shift_r(&self, bits: usize) -> BigInt {
        let mut vec = self.0.clone();
        let shift_amount = bits % 32;
        let carry_bits = 32 - shift_amount;

        for i in (1..vec.len()).rev() {
            vec[i] = (vec[i] >> shift_amount) | (vec[i-1] << carry_bits);
        }

        vec[0] >>= shift_amount;

        BigInt(vec)
    }

    fn shift_l(&self, n: u32) -> BigInt {
        let mut shifted_digits = Vec::new();
        let mut carry = 0;
        let bits = n % 32;

        for &digit in self.0.iter().rev() {
            let shifted_digit = (digit << bits) | carry;
            shifted_digits.push(shifted_digit);
            carry = digit >> (32 - bits);
        }

        if carry > 0 {
            shifted_digits.push(carry);
        }

        shifted_digits.reverse();
        BigInt(shifted_digits)
    }

    fn pad(&self, a: Vec<u32>, b: Vec<u32>) -> (Vec<u32>, Vec<u32>) {
        let (mut pad_a, mut pad_b) = (a.clone(), b.clone());
        if a.len() > b.len() {
            while pad_b.len() != a.len() {
                pad_b.insert(0, 0);
            }
        }
        if a.len() < b.len() {
            while pad_a.len() != b.len() {
                pad_a.insert(0, 0);
            }
        }
        (pad_a, pad_b)
    }

    fn add(&self, int: &BigInt) -> BigInt {
        let (mut carry, mut result_digits) = (0, Vec::new());

        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());
        for (a, b) in pad_a.iter().rev().zip(pad_b.iter().rev()) {
            let sum = a + b + carry;
            let digit = sum % 0x10;
            carry = sum / 0x10;
            result_digits.push(digit);
        }

        if carry > 0 {
            result_digits.push(carry);
        }

        result_digits.reverse();
        BigInt(result_digits)
    }

    fn sub(&self, int: &BigInt) -> BigInt {
        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());
        let mut result_digits = Vec::new();
        let mut borrow = false;

        for (a, b) in pad_a.iter().rev().zip(pad_b.iter().rev()) {
            let a = if borrow {
                if *a == 0 {
                    borrow = true;
                    0xF
                } else {
                    borrow = false;
                    a - 1
                }
            } else {
                *a
            };
            let diff = if a < *b {
                borrow = true;
                0x10 + a - b
            } else {
                a - b
            };
            result_digits.push(diff);
        }

        if borrow {
            panic!("Attempted to subtract a larger number from a smaller number");
        }

        result_digits.reverse();
        BigInt(result_digits)
    }

    fn mul(&self, int: &BigInt) -> BigInt {
        let mut result = BigInt::empty();
        let (pad_a, pad_b) = self.pad(self.0.clone(), int.0.clone());

        for (i, &a) in pad_a.iter().enumerate().rev() {
            let mut carry = 0;
            let mut temp_result = vec![0; pad_a.len() - i - 1];

            for &b in pad_b.iter().rev() {
                let product = a * b + carry;
                let digit = product % 0x10;
                carry = product / 0x10;
                temp_result.push(digit);
            }

            if carry > 0 {
                temp_result.push(carry);
            }

            temp_result.reverse();
            let temp_result_bigint = BigInt(temp_result);

            result = result.add(&temp_result_bigint);
        }
        result
    }

    fn mod_by(&self, modulo: &BigInt) -> BigInt {
        let mut result = self.sub(modulo);

        let zero = BigInt::empty();
        while result >= *modulo || result < zero {
            if result >= *modulo {
                result = result.sub(modulo);
            } else {
                result = result.add(modulo);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;

    #[test]
    fn xor() {
        let hex_a = "51bf608414ad5726a3c1bec098f77b1b54ffb2787f8d528a74c1d7fde6470ea4";
        let number_a = BigInt::new(hex_a);

        let hex_b = "403db8ad88a3932a0b7e8189aed9eeffb8121dfac05c3512fdb396dd73f6331c";
        let number_b = BigInt::new(hex_b);

        let result = number_a.xor(&number_b);
        assert_eq!(
            "1182d8299c0ec40ca8bf3f49362e95e4ecedaf82bfd167988972412095b13db8",
            result.get_hex()
        )
    }

    #[test]
    fn add() {
        let hex_a = "36f028580bb02cc8272a9a020f4200e346e276ae664e45ee80745574e2f5ab80";
        let number_a = BigInt::new(hex_a);

        let hex_b = "70983d692f648185febe6d6fa607630ae68649f7e6fc45b94680096c06e4fadb";
        let number_b = BigInt::new(hex_b);

        let result = number_a.add(&number_b);
        assert_eq!(
            "a78865c13b14ae4e25e90771b54963ee2d68c0a64d4a8ba7c6f45ee0e9daa65b",
            result.get_hex()
        )
    }

    #[test]
    fn sub() {
        let hex_a = "33ced2c76b26cae94e162c4c0d2c0ff7c13094b0185a3c122e732d5ba77efebc";
        let number_a = BigInt::new(hex_a);

        let hex_b = "22e962951cb6cd2ce279ab0e2095825c141d48ef3ca9dabf253e38760b57fe03";
        let number_b = BigInt::new(hex_b);

        let result = number_a.sub(&number_b);
        assert_eq!(
            "10e570324e6ffdbc6b9c813dec968d9bad134bc0dbb061530934f4e59c2700b9",
            result.get_hex()
        )
    }

    #[test]
    fn mul() {
        let hex_a = "7d7deab2affa38154326e96d350deee1";
        let number_a = BigInt::new(hex_a);

        let hex_b = "97f92a75b3faf8939e8e98b96476fd22";
        let number_b = BigInt::new(hex_b);

        let result = number_a.mul(&number_b);
        assert_eq!(
            "4a7f69b908e167eb0dc9af7bbaa5456039c38359e4de4f169ca10c44d0a416e2",
            result.get_hex()
        )
    }

    #[test]
    fn modulus() {
        let hex_a = "abcdef";
        let number_a = BigInt::new(hex_a);

        let hex_b = "123456";
        let number_b = BigInt::new(hex_b);

        let result = number_a.mod_by(&number_b);
        assert_eq!(
            "07f6e9",
            result.get_hex()
        )
    }

    #[test]
    fn test_and() {
        let number_a = BigInt::new("ff");
        let number_b = BigInt::new("0f");
        let result = number_a.and(&number_b);
        assert_eq!(
            "0f",
            result.get_hex()
        );
    }

    #[test]
    fn test_or() {
        let number_a = BigInt::new("f0");
        let number_b = BigInt::new("0f");
        let result = number_a.or(&number_b);
        assert_eq!("ff", result.get_hex());
    }

    #[test]
    fn test_shift_l() {
        let number_a = BigInt::new("f");
        let result = number_a.shift_l(4);
        assert_eq!("f0", result.get_hex());
    }
}