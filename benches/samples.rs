use bounced::samples;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline]
pub fn ceiling_division_by_branch(dividend: usize, divisor: usize) -> usize {
	if dividend > 0 {
		1 + (dividend - 1) / divisor
	} else {
		0
	}
}

#[inline]
pub fn ceiling_division_by_cond(dividend: usize, divisor: usize) -> usize {
	(dividend > 0) as usize * (1 + (dividend - 1) / divisor)
}

#[inline]
pub fn ceiling_division_by_mod(dividend: usize, divisor: usize) -> usize {
	let is_rem = dividend % divisor > 0;
	dividend / divisor + is_rem as usize
}

fn benchmark_branch(c: &mut Criterion) {
	c.bench_function("branch", |b| {
		b.iter(|| ceiling_division_by_branch(black_box(10500), black_box(1000)))
	});
	c.bench_function("cond", |b| {
		b.iter(|| ceiling_division_by_cond(black_box(10500), black_box(1000)))
	});
	c.bench_function("samples", |b| {
		b.iter(|| samples(black_box(500), black_box(21)))
	});
}

fn benchmark_mod(c: &mut Criterion) {
	c.bench_function("mod", |b| {
		b.iter(|| ceiling_division_by_mod(black_box(10500), black_box(1000)))
	});
}

criterion_group!(benches, benchmark_mod, benchmark_branch);
criterion_main!(benches);
