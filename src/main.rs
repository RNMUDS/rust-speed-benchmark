use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::time::{Duration, Instant};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct BenchmarkResult {
    language: String,
    test_name: String,
    duration_ms: f64,
    operations_per_second: f64,
}

#[derive(Serialize)]
struct BenchmarkResponse {
    results: Vec<BenchmarkResult>,
    timestamp: String,
}

async fn run_rust_benchmarks() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    // Fibonacci calculation benchmark
    let start = Instant::now();
    let _fib = fibonacci(40);
    let duration = start.elapsed();
    results.push(BenchmarkResult {
        language: "Rust".to_string(),
        test_name: "Fibonacci(40)".to_string(),
        duration_ms: duration.as_millis() as f64,
        operations_per_second: 1000.0 / duration.as_millis() as f64,
    });
    
    // Array sorting benchmark
    let start = Instant::now();
    let mut arr: Vec<i32> = (0..100000).rev().collect();
    arr.sort();
    let duration = start.elapsed();
    results.push(BenchmarkResult {
        language: "Rust".to_string(),
        test_name: "Sort 100k integers".to_string(),
        duration_ms: duration.as_millis() as f64,
        operations_per_second: 100000.0 / duration.as_millis() as f64,
    });
    
    // Prime number calculation
    let start = Instant::now();
    let _primes = sieve_of_eratosthenes(100000);
    let duration = start.elapsed();
    results.push(BenchmarkResult {
        language: "Rust".to_string(),
        test_name: "Primes up to 100k".to_string(),
        duration_ms: duration.as_millis() as f64,
        operations_per_second: 100000.0 / duration.as_millis() as f64,
    });
    
    results
}

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    if limit > 0 {
        is_prime[1] = false;
    }
    
    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in ((i * i)..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    
    is_prime.iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

async fn run_other_language_benchmarks() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    // Run Python benchmarks
    if let Ok(output) = Command::new("python3")
        .arg("benchmarks/python/benchmark.py")
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if let Ok(py_results) = serde_json::from_str::<Vec<BenchmarkResult>>(&stdout) {
                results.extend(py_results);
            }
        }
    }
    
    // Run JavaScript benchmarks
    if let Ok(output) = Command::new("node")
        .arg("benchmarks/javascript/benchmark.js")
        .output()
    {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            if let Ok(js_results) = serde_json::from_str::<Vec<BenchmarkResult>>(&stdout) {
                results.extend(js_results);
            }
        }
    }
    
    // Run Java benchmarks
    if let Ok(_) = Command::new("javac")
        .arg("benchmarks/java/Benchmark.java")
        .output()
    {
        if let Ok(output) = Command::new("java")
            .arg("-cp")
            .arg("benchmarks/java")
            .arg("Benchmark")
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                if let Ok(java_results) = serde_json::from_str::<Vec<BenchmarkResult>>(&stdout) {
                    results.extend(java_results);
                }
            }
        }
    }
    
    results
}

async fn run_benchmarks() -> Result<impl warp::Reply, warp::Rejection> {
    let mut all_results = Vec::new();
    
    // Run Rust benchmarks
    let rust_results = run_rust_benchmarks().await;
    all_results.extend(rust_results);
    
    // Run other language benchmarks
    let other_results = run_other_language_benchmarks().await;
    all_results.extend(other_results);
    
    let response = BenchmarkResponse {
        results: all_results,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    
    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);
    
    let static_files = warp::fs::dir("frontend");
    
    let api = warp::path("api")
        .and(warp::path("benchmark"))
        .and(warp::get())
        .and_then(run_benchmarks);
    
    let routes = static_files
        .or(api)
        .with(cors);
    
    println!("Server running on http://localhost:3030");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}