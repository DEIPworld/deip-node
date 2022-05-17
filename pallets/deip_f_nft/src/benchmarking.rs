#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::benchmarks_instance_pallet;
use frame_support::{assert_err, assert_ok, ensure};

use crate::{Config, Pallet};

benchmarks_instance_pallet! {
    sort_vector {
        let x in 1 .. 10000;
        let mut m = Vec::<u32>::new();
        for i in (0..x).rev() {
            m.push(i);
        }
    }: {
        m.sort();
    } verify {
        ensure!(m[0] == 0, "You forgot to sort!")
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test)
}
