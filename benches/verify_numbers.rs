#![feature(test)]

extern crate mynumber;
extern crate test;

#[bench]
fn bench_verify_individual_number(b: &mut test::Bencher) {
    b.iter(|| {
        let number = "123456789018";
        assert!(mynumber::individual::verify(number).is_ok());
    });
}

#[bench]
fn bench_verify_corporate_number(b: &mut test::Bencher) {
    b.iter(|| {
        let number = "9234567890123";
        assert!(mynumber::corporate::verify(number).is_ok());
    });
}
