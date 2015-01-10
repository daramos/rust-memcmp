# rust-memcmp
[![Build status](https://api.travis-ci.org/daramos/rust-memcmp.png)](https://travis-ci.org/daramos/rust-memcmp)
Optimized comparisons for integer slices. 
Use memcmp to compare integer slices efficiently.

Workaround for [Rust issue 16913](https://github.com/rust-lang/rust/issues/16913).

###Baseline PartialEq comparison:
####test test::u8_slice_cmp  ... bench:      2201 ns/iter (+/- 113) = 454 MB/s
```rust
#[bench]
fn slice_cmp(b: &mut test::Bencher) {
    let vec1 = repeat(b'c').take(10_000).collect::<Vec<_>>();
    let vec2 = vec1.clone();
    
    b.bytes = vec1.len() as u64;
    b.iter(|| {
        let (s1, s2) = ( &*vec1, &*vec2 );
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
    let vec1 = repeat(b'c').take(10_000).collect::<Vec<_>>();
    let vec2 = vec1.clone();
    
    b.bytes = vec1.len() as u64;
    b.iter(|| {
        let (s1, s2) = ( &*vec1, &*vec2 );
        return s1.memcmp(s2)
    });
}
```
