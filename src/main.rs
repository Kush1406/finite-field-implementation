#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(test)]

use std::collections::HashMap;

trait Add {
    fn add(&self, x: Self) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Self) -> u64;
}

trait Reduce {
    fn reduce(&self) -> u64;
}

struct ModularArithmetic {
    value: u64,
    modulus: u64,
    reduce_f: Box<dyn Fn(u64, u64) -> u64>,
}

impl Add for ModularArithmetic {
    fn add(&self, x:Self) -> u64 {
        // We only need at most 2 words, since size will at most double
        const N: usize = 2;

        let mut carry = 0;
        let mut ws: [u64; N + 1] = [0; N + 1];
        let mut us: [u64; N] = [0; N];
        let mut vs: [u64; N] = [0; N];

        us[0] = (&self.value << 32) >> 32;
        vs[0] = (x.value << 32) >> 32;
        
        us[1] = &self.value >> 32;
        vs[1] = x.value >> 32;

        for i in 0..N {

            let b = if i = 0 { 1 << 32 } else { u64::MAX }            
            ws[i] = (us[i] + vs[i] + carry).rem_euclid(b);
            carry = (us[i] + vs[i] + carry) / b;
        }
        ws[N] = carry;


        let mut result:u64 = 0;
        for i in 0..N {
            let mut byte_shift: u64 = 1 << (i * 32);
            let mut product: u64 = (&self.reduce_f)(ws[i], self.modulus) * (&self.reduce_f)(byte_shift, self.modulus);
            result += (&self.reduce_f)(product, self.modulus);
        }

        return result;

    }
}

impl Multiply for ModularArithmetic {
    fn multiply(&self, x: Self) -> u64 {
        const N:usize = 8;

        let mut ws: [u64; N * 2] = [0; N * 2];
        let mut us: [u64; N] = [0; N];
        let mut vs: [u64; N] = [0; N];

        for i in 0..N {
            us[i] = &self.value >> (8 - i);
            vs[i] = x.value >> (8 - i);

            let base: u64 = 2;
            us[i] = us[i] & (base.power(8) - 1);
            vs[i] = us[i] & (base.power(8) - 1);
        }

        let mut carry: u64 = 0;
        for i in 0..N {
            carry = 0;

            for j in 0..N {
                if us[i] == 0 {
                    w[i+N] = 0
                } else {
                    for j in 0..N {
                        let t: u64 = us[i] * vs[j] + ws[i+j] + carry;
                        ws[i+j] = t.rem_euclid(self.modulus);
                        carry = t / self.modulus;
                    }

                    ws[i+N] = carry;
                }
            }
        }
        


        // TODO: Implement this correctly.
        // Reduce:
        // This step iterates through each word. Without using modulo, these values might exceed what a u64 can handle.
        // We need to ensure none of them overflow, as the final result must fit within u64 due to the modulo constraint.

        // Montgomery's method approaches this differently, which might require special handling.
        // It keeps values in a different form and can switch back to classical form if needed,
        // but this frequent conversion could negate performance gains.

        // Modulo of a series is the sum of their individual moduli.
        // Each word's value in the array is multiplied by a power of 2.
        // The modulus of that product can be represented as (a*b mod m) = (a mod m) * (b mod m) mod m.

        // A concern is potential overflow of b (the power of 2) beyond u64.
        // To address this, we can decompose b as 2**n = 2**(x+y) where n = x + y, = 2**x * 2**y.
        // We then apply the product rule for modulus again using this breakdown.

        // TODO: Is this the best straightforward approach?
        for i in 0..N {

            // TODO
            let byte_shift: u64 = 1 << (i * 32);
            let product: u64 = (&self.reduce_f)(ws[i], self.modulus) * (&self.reduce_f)(byte_shift, self.modulus);
            ws[i] = ((&self.reduce_f)(product, self.modulus));
        }
        return 0;
    }
}


//////////////////REDUCTION FUNCTIONS/////////////////
fn reduce(value: u64, modulus: u64) -> u64 {
    println!("{}", value);
    let q: u64 = value / modulus;
    return value - (q * modulus);
}

fn mostly_reduce(value: u64, modulus: u64) -> u64 {
    let q: u64 = value / modulus;
    return value - (q * modulus);
}