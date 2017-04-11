#[macro_use]
extern crate bencher;

use bencher::Bencher;

mod nbody_raw;
use nbody_raw::raw_nbody;

mod nbody_metric;
use nbody_metric::metric_nbody;

mod nbody_dimensioned;
use nbody_dimensioned::dimensioned_nbody;

fn nbody_raw_bench(bench: &mut Bencher) {
    bench.iter(|| raw_nbody())
}

fn nbody_metric_bench(bench: &mut Bencher) {
    bench.iter(|| metric_nbody())
}

fn nbody_dimensioned_bench(bench: &mut Bencher) {
    bench.iter(|| dimensioned_nbody())
}

benchmark_group!(benches, nbody_raw_bench, nbody_metric_bench, nbody_dimensioned_bench);
benchmark_main!(benches);
