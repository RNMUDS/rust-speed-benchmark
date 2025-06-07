#!/usr/bin/env node

function fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

function sieveOfEratosthenes(limit) {
    const isPrime = new Array(limit + 1).fill(true);
    isPrime[0] = isPrime[1] = false;
    
    for (let i = 2; i <= Math.sqrt(limit); i++) {
        if (isPrime[i]) {
            for (let j = i * i; j <= limit; j += i) {
                isPrime[j] = false;
            }
        }
    }
    
    return isPrime.map((prime, index) => prime ? index : null).filter(x => x !== null);
}

function main() {
    const results = [];
    
    // Fibonacci benchmark
    let startTime = Date.now();
    fibonacci(40);
    let duration = Date.now() - startTime;
    results.push({
        language: "JavaScript",
        test_name: "Fibonacci(40)",
        duration_ms: duration,
        operations_per_second: 1000.0 / duration
    });
    
    // Array sorting benchmark
    startTime = Date.now();
    const arr = Array.from({length: 100000}, (_, i) => 100000 - i);
    arr.sort((a, b) => a - b);
    duration = Date.now() - startTime;
    results.push({
        language: "JavaScript",
        test_name: "Sort 100k integers",
        duration_ms: duration,
        operations_per_second: 100000.0 / duration
    });
    
    // Prime numbers benchmark
    startTime = Date.now();
    sieveOfEratosthenes(100000);
    duration = Date.now() - startTime;
    results.push({
        language: "JavaScript",
        test_name: "Primes up to 100k",
        duration_ms: duration,
        operations_per_second: 100000.0 / duration
    });
    
    console.log(JSON.stringify(results));
}

main();