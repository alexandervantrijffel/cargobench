use cargobenchlib::Library;
use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};

async fn heap(levels: u32) {
    let library = Box::new(Library::new_sample_library());
    process_heap(library, levels);
}

#[inline]
fn process_heap(library: Box<Library>, levels: u32) -> Box<Library> {
    match levels {
        0 => library,
        x => return process_heap(library, x - 1),
    }
}

async fn stack(levels: u32) {
    let library = Library::new_sample_library();
    process_stack(library, levels);
}

#[inline]
fn process_stack(library: Library, levels: u32) -> Library {
    match levels {
        0 => library,
        x => return process_stack(library, x - 1),
    }
}

async fn stack_with_clone(levels: u32) {
    let library = Library::new_sample_library();
    process_stack_with_clone(library, levels);
}

#[inline]
fn process_stack_with_clone(library: Library, levels: u32) -> Library {
    match levels {
        0 => library,
        x => return process_stack_with_clone(library.clone(), x - 1),
    }
}

async fn stack_with_ref(levels: u32) {
    let library = Library::new_sample_library();
    process_stack_with_ref(&library, levels);
}

#[inline]
fn process_stack_with_ref(library: &Library, levels: u32) -> &Library {
    match levels {
        0 => library,
        x => return process_stack_with_ref(library, x - 1),
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let levels = 10;

    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("mygroup");
    group.significance_level(0.1).sample_size(1000);
    group.bench_with_input(
        BenchmarkId::new("stack_unboxed_with_clone", levels),
        &levels,
        |b, &s| {
            b.to_async(&rt).iter(|| stack_with_clone(s));
        },
    );
    group.bench_with_input(BenchmarkId::new("heap_boxed", levels), &levels, |b, &s| {
        b.to_async(&rt).iter(|| heap(s));
    });
    group.bench_with_input(BenchmarkId::new("stack_ref", levels), &levels, |b, &s| {
        b.to_async(&rt).iter(|| stack_with_ref(s));
    });
    group.bench_with_input(
        BenchmarkId::new("stack_unboxed", levels),
        &levels,
        |b, &s| {
            b.to_async(&rt).iter(|| stack(s));
        },
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
