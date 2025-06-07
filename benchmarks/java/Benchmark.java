import java.util.*;
import java.util.stream.IntStream;
import com.google.gson.Gson;

class BenchmarkResult {
    String language;
    String test_name;
    double duration_ms;
    double operations_per_second;
    
    BenchmarkResult(String language, String testName, double durationMs, double opsPerSec) {
        this.language = language;
        this.test_name = testName;
        this.duration_ms = durationMs;
        this.operations_per_second = opsPerSec;
    }
}

public class Benchmark {
    public static long fibonacci(int n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    
    public static List<Integer> sieveOfEratosthenes(int limit) {
        boolean[] isPrime = new boolean[limit + 1];
        Arrays.fill(isPrime, true);
        isPrime[0] = isPrime[1] = false;
        
        for (int i = 2; i <= Math.sqrt(limit); i++) {
            if (isPrime[i]) {
                for (int j = i * i; j <= limit; j += i) {
                    isPrime[j] = false;
                }
            }
        }
        
        List<Integer> primes = new ArrayList<>();
        for (int i = 0; i <= limit; i++) {
            if (isPrime[i]) primes.add(i);
        }
        return primes;
    }
    
    public static void main(String[] args) {
        List<BenchmarkResult> results = new ArrayList<>();
        
        // Fibonacci benchmark
        long startTime = System.currentTimeMillis();
        fibonacci(40);
        double duration = System.currentTimeMillis() - startTime;
        results.add(new BenchmarkResult("Java", "Fibonacci(40)", duration, 1000.0 / duration));
        
        // Array sorting benchmark
        startTime = System.currentTimeMillis();
        int[] arr = IntStream.range(0, 100000).map(i -> 100000 - i).toArray();
        Arrays.sort(arr);
        duration = System.currentTimeMillis() - startTime;
        results.add(new BenchmarkResult("Java", "Sort 100k integers", duration, 100000.0 / duration));
        
        // Prime numbers benchmark
        startTime = System.currentTimeMillis();
        sieveOfEratosthenes(100000);
        duration = System.currentTimeMillis() - startTime;
        results.add(new BenchmarkResult("Java", "Primes up to 100k", duration, 100000.0 / duration));
        
        // Simple JSON output (without Gson dependency for simplicity)
        System.out.print("[");
        for (int i = 0; i < results.size(); i++) {
            BenchmarkResult r = results.get(i);
            System.out.printf("{\"language\":\"%s\",\"test_name\":\"%s\",\"duration_ms\":%.2f,\"operations_per_second\":%.2f}",
                r.language, r.test_name, r.duration_ms, r.operations_per_second);
            if (i < results.size() - 1) System.out.print(",");
        }
        System.out.println("]");
    }
}