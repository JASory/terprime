# Terprime
Counting, checking, and listing primes under 2^64

## About 
Terminal prime is an alternate approach to Kim Walisch's [primesieve](https://github.com/kimwalisch/primesieve). While primesieve performs calculations using sieving, terprime checks each individual integer for primality. This means that it can be faster than primesieve for small intervals (around 100 million) of relatively large integers, as it does not need to perform all the computations that are used in sieving large integers.  

In general however, terprime is considerably slower than primesieve, taking approximately 35,096 core-years to enumerate all primes from 0 to 2^64. However both are fast enough that outside of number-theory research they can produce more primes per second (> 400k) than a typical application would ever need. 
