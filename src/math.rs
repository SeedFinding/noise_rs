#![allow(dead_code)]

use std::cmp;

use sha2::{Digest, Sha256};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha() {
        assert_eq!(sha2long(1551515151585454), 4053242177535254290)
    }
}

pub fn lfloor(x: f64) -> i64 {
    let int_x: i64 = x as i64;
    if x < (int_x as f64) { int_x - 1i64 } else { int_x }
}

pub fn modf(x: f64) -> (f64, f64) {
    // https://code.woboq.org/userspace/glibc/sysdeps/ieee754/dbl-64/wordsize-64/s_modf.c.html for proper
    let i: f64 = x.floor();
    (i, x - i)
}


pub fn floor(x: f64) -> i32 {
    let int_x: i32 = x as i32;
    if x < (int_x as f64) { int_x - 1i32 } else { int_x }
}


pub fn smooth_step_fast(x: f64) -> f64 {
    // this is sadly incorrect due to lost of precision sniff
    let x_3: f64 = x * x * x;
    let x_4: f64 = x_3 * x;
    10.0f64 * x_3 - 15.0f64 * x_4 + 6.0f64 * x_4 * x
}
pub fn smooth_step(x: f64) -> f64 {
    x * x * x * (x * (x * 6.0f64 - 15.0f64) + 10.0f64)
}

pub fn min(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b) {
        None => { a }
        Some(order) => {
            match order {
                cmp::Ordering::Less => { a }
                _ => b
            }
        }
    }
}

pub fn sqr(d: f64) -> f64 {
    d * d
}

pub fn wrap(x: f64) -> f64 {
    return x - (lfloor(x / 3.3554432E7 + 0.5) as f64) * 3.3554432E7;
}

pub fn dot(g: [i32; 3], d: f64, d2: f64, d3: f64) -> f64 {
    (g[0 as usize]) as f64 * d + (g[1 as usize]) as f64 * d2 + (g[2 as usize]) as f64 * d3
}

pub fn grad(hash: u8, x: f64, y: f64, z: f64) -> f64 {
    return match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => 0f64, // never happens
    };
}


pub fn lerp3(sx: f64, sy: f64, sz: f64, x1: f64, x2: f64, x3: f64, x4: f64, x5: f64, x6: f64, x7: f64, x8: f64) -> f64 {
    lerp1(sz, lerp2(sx, sy, x1, x2, x3, x4), lerp2(sx, sy, x5, x6, x7, x8))
}

pub fn lerp2(x1: f64, x2: f64, x3: f64, x4: f64, x5: f64, x6: f64) -> f64 {
    lerp1(x2, lerp1(x1, x3, x4), lerp1(x1, x5, x6))
}

pub fn lerp1(t: f64, v0: f64, v1: f64) -> f64 {
    // Imprecise method, which does not guarantee v = v1 when t = 1, due to floating-point arithmetic error. This method is monotonic
    // This form may be used when the hardware has a native fused multiply-add instruction.
    v0 + (v1 - v0) * t
}

pub fn lerp1_bis(t: f64, v0: f64, v1: f64) -> f64 {
    // Precise method, which guarantees v = v1 when t = 1. This method is monotonic only when v0 * v1 < 0. Lerping between same values might not produce the same value
    (1.0f64 - t) * v0 + t * v1
}

pub fn abs(a: f32) -> f32 {
    if a <= 0.0f32 { -a } else { a }
}

pub fn clamp(f1: f32, f2: f32, f3: f32) -> f32 {
    if f1 < f2 {
        return f2;
    }
    if f1 > f3 {
        return f3;
    }
    return f1;
}

pub fn floor_mod(x: i64, y: i64) -> i64 {
    let mut modulo: i64 = x % y;
    // if the signs are different and modulo not zero, adjust result
    if (x ^ y) < 0 && modulo != 0 {
        modulo += y;
    }
    modulo
}

pub fn sqrt(f: f32) -> f32 {
    return f.sqrt();
}

pub fn max(a: f32, b: f32) -> f32 {
    if a >= b { a } else { b }
}

pub fn sha2long(mut seed: u64) -> u64 {
    let mut bytes: [u8; 8] = [0; 8];
    for i in 0..8 {
        bytes[i] = (seed & 255) as u8;
        seed >>= 8;
    }
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    let mut ret_val: u64 = (result[0] & 0xFF) as u64;
    for i in 1..cmp::min(8, result.len()) {
        ret_val |= (((result[i] & 0xFF) as u64).wrapping_shl((i << 3) as u32)) as u64;
    }
    ret_val
}


