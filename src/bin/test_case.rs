use barevec::BitVec;
use rayon::prelude::*;
use std::time::Instant;
use tracing::{ info, Level };
use tracing_subscriber;

fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Test parameters
    let size = 1_000_000;
    let iterations = 10;

    // Typical vector implementation
    let mut typical_vec = vec![0; size];
    let start = Instant::now();
    for _ in 0..iterations {
        typical_vec.par_iter_mut().for_each(|x| {
            *x += 1;
        });
    }
    let duration = start.elapsed();
    info!("Typical vector implementation took: {:?}", duration);
    println!("Typical vector implementation took: {:?}", duration);

    // SIMD-accelerated bit vector implementation
    let mut simd_vec = BitVec::with_capacity(size);
    simd_vec.extend_from_slice(&vec![0; size]);
    let start = Instant::now();
    for _ in 0..iterations {
        simd_vec.par_iter_mut().for_each(|x| {
            *x += 1;
        });
    }
    let duration = start.elapsed();
    info!("SIMD-accelerated bit vector implementation took: {:?}", duration);
    println!("SIMD-accelerated bit vector implementation took: {:?}", duration);

    // Memory usage comparison
    let typical_vec_memory =
        std::mem::size_of_val(&typical_vec) + typical_vec.capacity() * std::mem::size_of::<i32>();
    let simd_vec_memory =
        std::mem::size_of_val(&simd_vec) + simd_vec.capacity() * std::mem::size_of::<i32>();
    info!("Typical vector memory usage: {} bytes", typical_vec_memory);
    println!("Typical vector memory usage: {} bytes", typical_vec_memory);
    info!("SIMD-accelerated bit vector memory usage: {} bytes", simd_vec_memory);
    println!("SIMD-accelerated bit vector memory usage: {} bytes", simd_vec_memory);
}
