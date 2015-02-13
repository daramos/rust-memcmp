#![feature(core)]

#![feature(test)]
extern crate test;

pub trait Memcmp {
    fn memcmp(self: &Self, b: &Self) -> bool;
}



impl Memcmp for [u8] {
    #[inline(always)]
    fn memcmp(&self, b: &[u8]) -> bool {
       #[allow(improper_ctypes)]
        extern { fn memcmp(s1: *const i8, s2: *const i8, n: usize) -> i32; }
        self.len() == b.len() && unsafe {
            memcmp(self.as_ptr() as *const i8,
                   b.as_ptr() as *const i8,
                   self.len()) == 0
        }
    }
}

macro_rules! memcmp_impl {
    ($int_type:ty, $bits:expr) => ( // ex. (u16, 16)
        impl Memcmp for [$int_type] {
            #[inline(always)]
            fn memcmp(&self, b: &[$int_type]) -> bool {
                let bytes = ($bits)/8;
                let u8_len = self.len() * bytes;
                let self_ptr = self.as_ptr() as *const u8;
                let b_ptr = b.as_ptr() as *const u8;
                let self_as_bytes = unsafe {std::slice::from_raw_parts(self_ptr,u8_len)};
                let b_as_bytes = unsafe {std::slice::from_raw_parts(b_ptr,u8_len)};
                return self_as_bytes.memcmp(b_as_bytes);
            }
        }

    );
}
memcmp_impl!(u16,16);
memcmp_impl!(u32,32);
memcmp_impl!(u64,64);

memcmp_impl!(i8,8);
memcmp_impl!(i16,16);
memcmp_impl!(i32,32);
memcmp_impl!(i64,64);



#[cfg(test)]
mod tests {
    
    use  std::iter::repeat;
    use ::Memcmp;
    use ::test;

    macro_rules! test_equals {
        ($int_type:ty, $fn_name:ident) => (
            #[test]
            fn $fn_name() {
                let val1 = vec![1 as $int_type,2 as $int_type,3 as $int_type,4 as $int_type];
                let val2 = val1.clone();
                assert!((&*val1).memcmp(&*val2));
            }
        );
    }
    
    macro_rules! test_not_equals {
        ($int_type:ty, $fn_name:ident) => (
            #[test]
            #[should_fail(expected = "assertion failed")]
            fn $fn_name() {
                let val1 = vec![1 as $int_type,2 as $int_type,3 as $int_type,4 as $int_type];
                let val2 = vec![1 as $int_type,2 as $int_type,2 as $int_type,4 as $int_type];
                assert!((&*val1).memcmp(&*val2));
            }
        );
    }
    
    
    macro_rules! memcmp_bench_slice {
        ($int_type:ty, $bits:expr, $fn_name:ident) => ( // ex. (u16, 16, u16_slice_cmp)
            #[bench]
            fn $fn_name(b: &mut test::Bencher) {
                let num_bytes = ($bits)/8;
                let vec1 = repeat('c' as $int_type).take(10_000).collect::<Vec<_>>();
                let vec2 = vec1.clone();
                
                b.bytes = (vec1.len() * (num_bytes)) as u64;
                b.iter(|| {
                    let (s1, s2) = ( &*vec1, &*vec2 );
                    return s1==s2
                });
            }

        );
    }
    
    macro_rules! memcmp_bench_memcmp {
        ($int_type:ty, $bits:expr, $fn_name:ident) => ( // ex. (u16, 16,u16_memcmp)
            #[bench]
            fn $fn_name(b: &mut test::Bencher) {
                let num_bytes = ($bits)/8;
                let vec1 = repeat('c' as $int_type).take(10_000).collect::<Vec<_>>();
                let vec2 = vec1.clone();
                
                b.bytes = (vec1.len() * (num_bytes)) as u64;
                b.iter(|| {
                    let (s1, s2) = ( &*vec1, &*vec2 );
                    return s1.memcmp(s2)
                });
            }

        );
    }
    test_equals!(u8,u8_eq);
    test_equals!(u16,u16_eq);
    test_equals!(u32,u32_eq);
    test_equals!(u64,u64_eq);
    
    test_equals!(i8,i8_eq);
    test_equals!(i16,i16_eq);
    test_equals!(i32,i32_eq);
    test_equals!(i64,i64_eq);
    
    test_not_equals!(u8,u8_not_eq);
    test_not_equals!(u16,u16_not_eq);
    test_not_equals!(u32,u32_not_eq);
    test_not_equals!(u64,u64_not_eq);
    
    test_not_equals!(i8,i8_not_eq);
    test_not_equals!(i16,i16_not_eq);
    test_not_equals!(i32,i32_not_eq);
    test_not_equals!(i64,i64_not_eq);
    
    
    memcmp_bench_slice!(u8,8,u8_slice_cmp);
    memcmp_bench_slice!(u16,16,u16_slice_cmp);
    memcmp_bench_slice!(u32,32,u32_slice_cmp);
    memcmp_bench_slice!(u64,64,u64_slice_cmp);
    
    memcmp_bench_slice!(i8,8,i8_slice_cmp);
    memcmp_bench_slice!(i16,16,i16_slice_cmp);
    memcmp_bench_slice!(i32,32,i32_slice_cmp);
    memcmp_bench_slice!(i64,64,i64_slice_cmp);
    
    memcmp_bench_memcmp!(u8,8,u8_memcmp);
    memcmp_bench_memcmp!(u16,16,u16_memcmp);
    memcmp_bench_memcmp!(u32,32,u32_memcmp);
    memcmp_bench_memcmp!(u64,64,u64_memcmp);
    
    memcmp_bench_memcmp!(i8,8,i8_memcmp);
    memcmp_bench_memcmp!(i16,16,i16_memcmp);
    memcmp_bench_memcmp!(i32,32,i32_memcmp);
    memcmp_bench_memcmp!(i64,64,i64_memcmp);


}
