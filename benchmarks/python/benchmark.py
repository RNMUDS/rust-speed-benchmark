#!/usr/bin/env python3
import time
import json
import sys

def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

def sieve_of_eratosthenes(limit):
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False
    
    return [i for i, prime in enumerate(is_prime) if prime]

def main():
    results = []
    
    # Fibonacci benchmark
    start_time = time.time()
    fibonacci(40)
    duration = (time.time() - start_time) * 1000
    results.append({
        "language": "Python",
        "test_name": "Fibonacci(40)",
        "duration_ms": duration,
        "operations_per_second": 1000.0 / duration
    })
    
    # Array sorting benchmark
    start_time = time.time()
    arr = list(range(100000, 0, -1))
    arr.sort()
    duration = (time.time() - start_time) * 1000
    results.append({
        "language": "Python",
        "test_name": "Sort 100k integers",
        "duration_ms": duration,
        "operations_per_second": 100000.0 / duration
    })
    
    # Prime numbers benchmark
    start_time = time.time()
    sieve_of_eratosthenes(100000)
    duration = (time.time() - start_time) * 1000
    results.append({
        "language": "Python",
        "test_name": "Primes up to 100k",
        "duration_ms": duration,
        "operations_per_second": 100000.0 / duration
    })
    
    print(json.dumps(results))

if __name__ == "__main__":
    main()