# 🦀 Rust Speed Benchmark

A web application that demonstrates Rust's performance by comparing it with Python, JavaScript, and Java across various computational tasks.

## Features

- **Real-time Benchmarking**: Run performance tests comparing Rust, Python, JavaScript, and Java
- **Interactive Dashboard**: Visual charts and tables showing execution times and operations per second
- **Multiple Test Cases**: 
  - Fibonacci calculation (recursive)
  - Array sorting (100k integers)
  - Prime number generation (Sieve of Eratosthenes)

## Prerequisites

Make sure you have the following installed:
- [Rust](https://rustup.rs/) (latest stable)
- [Python 3](https://www.python.org/downloads/)
- [Node.js](https://nodejs.org/)
- [Java JDK](https://openjdk.org/)

## Quick Start

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd rust-speed-benchmark
   ```

2. **Build and run the Rust server**
   ```bash
   cargo run
   ```

3. **Open your browser**
   Navigate to `http://localhost:3030`

4. **Run benchmarks**
   Click the "🚀 Run Benchmarks" button to compare performance across all languages

## Project Structure

```
rust-speed-benchmark/
├── src/
│   └── main.rs              # Rust web server and benchmark implementations
├── benchmarks/
│   ├── rust/                # Rust benchmark implementations (in main.rs)
│   ├── python/
│   │   └── benchmark.py     # Python benchmark implementations
│   ├── javascript/
│   │   └── benchmark.js     # JavaScript benchmark implementations
│   └── java/
│       └── Benchmark.java   # Java benchmark implementations
├── frontend/
│   └── index.html           # Web dashboard
├── Cargo.toml               # Rust dependencies
└── README.md
```

## Benchmark Tests

### 1. Fibonacci Calculation
Recursive calculation of the 40th Fibonacci number - tests function call overhead and recursion performance.

### 2. Array Sorting
Sorting 100,000 integers in reverse order - tests memory management and sorting algorithm efficiency.

### 3. Prime Number Generation
Generating all prime numbers up to 100,000 using the Sieve of Eratosthenes - tests loop performance and memory allocation.

## Expected Results

Typically, you'll see:
- **Rust**: Fastest execution times, excellent memory efficiency
- **Java**: Close to Rust performance after JVM warmup
- **JavaScript (Node.js)**: Good performance thanks to V8 optimization
- **Python**: Slower due to interpreted nature, but still respectable

## API Endpoints

- `GET /api/benchmark` - Runs all benchmarks and returns JSON results
- `GET /` - Serves the web dashboard

## Development

To modify benchmarks:

1. **Rust**: Edit `src/main.rs`
2. **Python**: Edit `benchmarks/python/benchmark.py`
3. **JavaScript**: Edit `benchmarks/javascript/benchmark.js`
4. **Java**: Edit `benchmarks/java/Benchmark.java`

All benchmark scripts output JSON in the same format for consistency.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with `cargo run`
5. Submit a pull request

## License

MIT License - feel free to use this project for learning and demonstration purposes!