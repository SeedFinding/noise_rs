#![allow(dead_code)]

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


