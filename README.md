# Terprime
Counting, checking, and listing primes

## About 
Terminal prime is an alternate approach to Kim Walisch's [primesieve](https://github.com/kimwalisch/primesieve). While primesieve performs calculations using sieving, terprime checks each individual integer for primality. This means that it can be faster than primesieve for small intervals (around 100 million) of relatively large integers, as it does not need to perform all the computations that are used in sieving large integers. 

Terprime is also faster than primesieve's command-line program when it comes to writing data files, as primesieve relies on piping output to files using the terminal, while terprime's `write` option directly writes to files. As the writing time consumes the majority of time in both programs, terprime is infact faster for total output of primes. 

In general however, terprime is considerably slower than primesieve, taking approximately 35,096 core-years to enumerate all primes from 0 to 2^64. 
