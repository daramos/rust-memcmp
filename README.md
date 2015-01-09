# rust-memcmp
Optimized comparisons for u8 slices. 
Use memcmp to compare u8 slices efficiently.

###Baseline PartialEq comparison:
####test test::slice_cmp ... bench:   2002387 ns/iter (+/- 25872) = 499 MB/s
```rust
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
```
###Using this crate:
###test test::memcmp_cmp  ... bench:     55611 ns/iter (+/- 10529) = 17982 MB/s
```rust
extern crate memcmp;
use memcmp::Memcmp;
#[bench]
fn memcmp_cmp(b: &mut test::Bencher) {
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
```
