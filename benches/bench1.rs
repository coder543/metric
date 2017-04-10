#[macro_use]
extern crate bencher;

#[macro_use]
extern crate lazy_static;

use bencher::Bencher;

mod nbody_raw;
use nbody_raw::raw_nbody;

mod nbody_metric;
use nbody_metric::metric_nbody;

fn nbody_raw_bench(bench: &mut Bencher) {
    bench.iter(|| raw_nbody())
}

fn nbody_metric_bench(bench: &mut Bencher) {
    bench.iter(|| metric_nbody())
}

benchmark_group!(benches, nbody_raw_bench, nbody_metric_bench);
benchmark_main!(benches);
