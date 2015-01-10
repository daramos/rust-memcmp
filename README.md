# rust-memcmp
Optimized comparisons for integer slices. 
Use memcmp to compare integer slices efficiently.

Workaround for [Rust issue 16913](https://github.com/rust-lang/rust/issues/16913).

###Baseline PartialEq comparison:
####test test::u8_slice_cmp  ... bench:      2201 ns/iter (+/- 113) = 454 MB/s
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
###test test::u8_memcmp     ... bench:        33 ns/iter (+/- 2) = 30303 MB/s
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
