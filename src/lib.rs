pub trait Memcmp {
    fn memcmp(self: &Self, b: &Self) -> bool;
}

impl Memcmp for [u8] {
    #[inline(always)]
    fn memcmp(&self, b: &[u8]) -> bool {
       #[allow(improper_ctypes)]
        extern { fn memcmp(s1: *const i8, s2: *const i8, n: uint) -> i32; }
        self.len() == b.len() && unsafe {
            memcmp(self.as_ptr() as *const i8,
                   b.as_ptr() as *const i8,
                   self.len()) == 0
        }
    }
}

mod test {
    extern crate test;
    use  std::iter::repeat;
    use ::Memcmp;


    #[bench]
    fn slice_cmp(b: &mut test::Bencher) {
        let test_val1 : Vec<u8> = repeat('c' as u8).take(1000000).collect();
        let test_val2 : Vec<u8> = test_val1.clone();
        
        b.bytes = test_val1.len() as u64;
        let slice1 = test_val1.as_slice();
        let slice2 = test_val2.as_slice();
        b.iter(|| {
            let s1 = slice1.clone();
            let s2 = slice2.clone();
            return s1==s2
        });
    }

    #[bench]
    fn this_cmp(b: &mut test::Bencher) {
        let test_val1 : Vec<u8> = repeat('c' as u8).take(1000000).collect();
        let test_val2 : Vec<u8> = test_val1.clone();
        
        b.bytes = test_val1.len() as u64;
        let slice1 = test_val1.as_slice();
        let slice2 = test_val2.as_slice();
        b.iter(|| {
            let s1 = slice1.clone();
            let s2 = slice2.clone();
            return s1.memcmp(s2);
        });
    }

}
