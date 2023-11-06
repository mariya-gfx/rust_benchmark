#![feature(test)]

extern crate test;

static ARR: [i32; 10000] = [0; 10000];

fn iterate_array_for_loop(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &value in arr.iter() {
        sum += value;
    }
    sum
}

fn iterate_array_for_each(arr: &[i32]) -> i32 {
    let mut sum = 0;
    arr.iter().for_each(|&value| sum += value);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_iterate_array_for_loop() {
        let result = iterate_array_for_loop(&ARR);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_iterate_array_for_each() {
        let result = iterate_array_for_each(&ARR);
        assert_eq!(result, 0);
    }

    #[bench]
    fn bench_iterate_array_for_loop(b: &mut Bencher) {
        b.iter(|| iterate_array_for_loop(&ARR));
    }

    #[bench]
    fn bench_iterate_array_for_each(b: &mut Bencher) {
        b.iter(|| iterate_array_for_each(&ARR));
    }
}
