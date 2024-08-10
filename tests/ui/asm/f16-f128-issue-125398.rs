#![feature(f16, f128)]
//@ run-rustfix
//@ check-pass
#![crate_type = "lib"]

#[inline(never)]
pub fn f32_to_f16(a: f32) -> f16 {
    a as f16
}

#[inline(never)]
pub fn f32_to_f16_asm(a: f32) -> f16 {
    let ret: f16;
    unsafe {
        core::arch::asm!(
                "fcvt {r:x}, {i:x}",
                i = in(reg) a,
                r = out(reg) ret,
        );
    }

    ret
}

#[inline(never)]
pub fn f64_to_f32(a: f64) -> f32 {
    a as f32
}

#[inline(never)]
pub fn f64_to_f32_asm(a: f64) -> f32 {
    let ret: f32;
    unsafe {
        core::arch::asm!(
                "fcvt {r:x}, {i:x}",
                i = in(reg) a,
                r = out(reg) ret,
        );
    };

    ret
}

#[inline(never)]
pub fn f128_to_f64(a: f128) -> f64 {
    a as f64
}

#[inline(never)]
pub fn f128_to_f64_asm(a: f128) -> f64 {
    let ret: f64;
    unsafe {
        core::arch::asm!("vcvtpd2ps {0}, {1}",
                        in(xmm_reg) a,
                        out(xmm_reg) ret);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn f32_to_f16_test() {
        assert_eq!(f32_to_f16_asm(3.14159), 0x3243F3 as f16, "error");
    }

    #[test]
    pub fn f64_to_f32_test() {
        assert_eq!(f64_to_f32_asm(3.14159), 0x3243F3 as f32, "error");
    }

    #[test]
    pub fn f128_to_f64_test() {
        assert_eq!(f128_to_f64_asm(3.14159), 0x3243F3 as f64, "error");
    }
}
