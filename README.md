Use `test.sh`

## Sample Output

```
    Finished release [optimized] target(s) in 0.42s
Testing with input: 1
fib(1) = 1
0.00user 0.00system 0:00.00elapsed 42%CPU (0avgtext+0avgdata 2656maxresident)k
936inputs+0outputs (2major+144minor)pagefaults 0swaps

Testing with input: 2
fib(2) = 1
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2884maxresident)k
0inputs+0outputs (0major+152minor)pagefaults 0swaps

Testing with input: 4
fib(4) = 3
0.00user 0.00system 0:00.00elapsed 75%CPU (0avgtext+0avgdata 2972maxresident)k
0inputs+0outputs (0major+172minor)pagefaults 0swaps

Testing with input: 8
fib(8) = 21
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2956maxresident)k
0inputs+0outputs (0major+166minor)pagefaults 0swaps

Testing with input: 16
fib(16) = 987
0.00user 0.00system 0:00.00elapsed 240%CPU (0avgtext+0avgdata 3224maxresident)k
0inputs+0outputs (0major+279minor)pagefaults 0swaps

Testing with input: 32
fib(32) = 2178309
14.38user 7.99system 0:07.40elapsed 302%CPU (0avgtext+0avgdata 837860maxresident)k
0inputs+0outputs (0major+209403minor)pagefaults 0swaps

Testing with input: 36
fib(36) = 14930352
103.19user 61.33system 0:52.71elapsed 312%CPU (0avgtext+0avgdata 5500684maxresident)k
0inputs+8outputs (0major+1375193minor)pagefaults 0swaps
```

## Same Go Output

```
Testing with input: 1
fib( 1 ) =  1
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 1752maxresident)k
0inputs+0outputs (0major+188minor)pagefaults 0swaps

Testing with input: 2
fib( 2 ) =  1
0.00user 0.00system 0:00.00elapsed 50%CPU (0avgtext+0avgdata 1700maxresident)k
0inputs+0outputs (0major+184minor)pagefaults 0swaps

Testing with input: 4
fib( 4 ) =  3
0.00user 0.00system 0:00.00elapsed 50%CPU (0avgtext+0avgdata 1804maxresident)k
0inputs+0outputs (0major+186minor)pagefaults 0swaps

Testing with input: 8
fib( 8 ) =  21
0.00user 0.00system 0:00.00elapsed 200%CPU (0avgtext+0avgdata 1928maxresident)k
0inputs+0outputs (0major+230minor)pagefaults 0swaps

Testing with input: 16
fib( 16 ) =  987
0.00user 0.00system 0:00.00elapsed 160%CPU (0avgtext+0avgdata 3000maxresident)k
0inputs+0outputs (0major+504minor)pagefaults 0swaps

Testing with input: 32
fib( 32 ) =  2178309
7.46user 0.48system 0:02.36elapsed 336%CPU (0avgtext+0avgdata 674876maxresident)k
0inputs+0outputs (0major+173787minor)pagefaults 0swaps

Testing with input: 36
fib( 36 ) =  14930352
53.04user 2.56system 0:16.72elapsed 332%CPU (0avgtext+0avgdata 3713892maxresident)k
0inputs+8outputs (0major+945427minor)pagefaults 0swaps
```


## Rayon output

```
    Finished release [optimized] target(s) in 10.49s
Testing with input: 1
fib(1) = 1
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2340maxresident)k
0inputs+0outputs (0major+103minor)pagefaults 0swaps

Testing with input: 2
fib(2) = 1
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2280maxresident)k
0inputs+0outputs (0major+101minor)pagefaults 0swaps

Testing with input: 4
fib(4) = 3
0.00user 0.00system 0:00.00elapsed 66%CPU (0avgtext+0avgdata 2640maxresident)k
0inputs+0outputs (0major+132minor)pagefaults 0swaps

Testing with input: 8
fib(8) = 21
0.00user 0.00system 0:00.00elapsed 66%CPU (0avgtext+0avgdata 2548maxresident)k
0inputs+0outputs (0major+129minor)pagefaults 0swaps

Testing with input: 16
fib(16) = 987
0.00user 0.00system 0:00.00elapsed 33%CPU (0avgtext+0avgdata 2556maxresident)k
0inputs+0outputs (0major+128minor)pagefaults 0swaps

Testing with input: 32
fib(32) = 2178309
0.10user 0.00system 0:00.04elapsed 265%CPU (0avgtext+0avgdata 2596maxresident)k
0inputs+0outputs (0major+139minor)pagefaults 0swaps

Testing with input: 36
fib(36) = 14930352
0.71user 0.00system 0:00.19elapsed 365%CPU (0avgtext+0avgdata 2572maxresident)k
0inputs+0outputs (0major+139minor)pagefaults 0swaps
```
