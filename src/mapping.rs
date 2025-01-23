use crate::{ntheory::*,factor::*,prime_math::*};
use machine_prime::is_prime_128;
use std::io::Write;

 const CHECK : &str = "check";
 const COUNT : &str = "count";
 const EULER : &str = "euler";
 const FACTOR : &str ="factor";
 const LIST : &str = "list";
 const GCD : &str ="gcd";
 const INTERVAL : &str = "interval";
 const NTH : &str = "nth";
 const NEXT : &str ="next";
 const PREV : &str ="prev";
 const WRITE : &str = "write";
 const BINARY : &str = "binary";

#[derive(PartialEq)]
pub(crate) enum Op{
    // Check primality
    Check,
    // Count primes
    Count,
    // List primes by number e.g 0th to 100th
    List,
    // List primes in interval e.g from 0 to 100
    Interval,
    // Nth prime
    Nth,
    // List next prime
    Next,
    // List prev prime
    Prev,
    // Write to utf8 file 
    Write,
    // Write to binary file
    Binary,
    // Factor
    Factor,
    // Euler Totient
    Euler,
    // GCD
    GCD,
    // Not a defined operation
    Invalid, 
 }
 
 pub (crate) enum PrimeResult{
            Prime,
            Composite,
            N(u128),
            Err(String),
            FileWrite,
            PrimeCount(u64),
            Primelist(Vec<u128>),
            Factorisation(Factor),
 }
 
 impl PrimeResult{
 
  pub fn display(&self,dis: bool) -> String{
     match self{
       PrimeResult::Prime => "Prime".to_string(),
       PrimeResult::Composite => "Composite".to_string(),
       PrimeResult::N(x) => x.to_string(),
       PrimeResult::Err(mess) => mess.to_string(),
       PrimeResult::FileWrite => "written to primes".to_string(),
       PrimeResult::PrimeCount(x) => x.to_string(),
       PrimeResult::Primelist(plist) => {
        if dis == true{
         let maxlen = 65535usize;
         let mut total_len = 0usize;
         let mut res = vec![];
         for i in plist.iter(){
            let pstring = i.to_string();
            total_len+=pstring.len()+1;
            if total_len > maxlen{
              break;
            }
            res.push(pstring);    
         }
         let output = res.join(" ");
         return output;
         }
         else{
           let mut output = String::new();
           for i in plist.iter(){
              output+=&(i.to_string()+"\n");
           }
           return output;
         }
       },
       PrimeResult::Factorisation(x) => x.display(),
     }
  }
  
 }

 
 pub(crate) fn map_string(x: &str) -> Op{
     match x{
      CHECK => Op::Check,
      COUNT => Op::Count,
      EULER => Op::Euler,
      FACTOR => Op::Factor,
      LIST => Op::List,
      GCD => Op::GCD,
      INTERVAL => Op::Interval,
      NTH => Op::Nth,
      NEXT => Op::Next,
      PREV => Op::Prev,
      WRITE => Op::Write,
      BINARY => Op::Binary,
      _=> Op::Invalid,
     }
 }
 
 pub (crate) fn calc_table(op: Op, inf: u128, sup: u128, tc: u128, file: Option<std::fs::File>) -> PrimeResult{

   match op { 
   
   Op::Check => {
            if is_prime_128(sup){
              return PrimeResult::Prime
            }
         return PrimeResult::Composite
        },
        
        Op::Count => {
            return PrimeResult::PrimeCount(parallel_pi(inf,sup,tc) as u64)
        },
        
        Op::Interval => {
            let sup = fix_bound(sup);
            return PrimeResult::Primelist(parallel_plist(inf,sup,tc));
        },
        
        Op::Write => {
           let sup = fix_bound(sup);
           let mut file = std::io::BufWriter::new(file.unwrap());
           
           let stride : u128 = 100_000_000*tc;
          
           let steps = (sup-inf)/stride;
           for i in 0..(steps){
              let inner_inf = inf+(stride*i);
              let inner_sup = inf+(stride*(i+1));
              let primelist = parallel_plist(inner_inf,inner_sup,tc);
               
               for i in primelist.iter(){
                 let string = d_string(*i);
                 file.write(string.as_bytes()).unwrap();
               }
           }
           let primelist = parallel_plist(inf+(stride*steps),sup,tc);
           for i in primelist.iter(){
                 let string = d_string(*i);
                 file.write(string.as_bytes()).unwrap();
               }
               PrimeResult::FileWrite
        },
        
        Op::Binary => {
           let sup = fix_bound(sup);
           let mut file = std::io::BufWriter::new(file.unwrap());
           
           let stride : u128 = 100_000_000*tc;
          
           let steps = (sup-inf)/stride;
           for i in 0..(steps){
              let inner_inf = inf+(stride*i);
              let inner_sup = inf+(stride*(i+1));
              let primelist = parallel_plist(inner_inf,inner_sup,tc);
               
               for i in primelist.iter(){
                 file.write(&i.to_le_bytes()).unwrap();
               }
           }
           let primelist = parallel_plist(inf+(stride*steps),sup,tc);
           for i in primelist.iter(){
                 file.write(&i.to_le_bytes()).unwrap();
               }
               PrimeResult::FileWrite
        },
        
        Op::List => {
            return PrimeResult::Primelist(primes_interval(inf,sup,tc))
        },
        
        Op::Next => {
            match bounded_prime(sup,1){
              Some(x) => PrimeResult::N(x),
              None => PrimeResult::Err("Exceeded 2^128".to_string()),
            }
            //return PrimeResult::N(bounded_prime(sup,1))   
        },
        
        Op::Nth => {
            return PrimeResult::N(primes_interval(sup,sup,tc)[0])   
        },
        
        Op::Prev => {
            match bounded_prime(sup,u128::MAX){
              Some(x) => PrimeResult::N(x),
              None => PrimeResult::Err("No prime under 2".to_string()),
            }
            //return PrimeResult::N(bounded_prime(sup,u128::MAX).unwrap())   
        },
        
        Op::Euler => {
           return PrimeResult::N(euler_totient(sup))
        },
        
        Op::Factor => {
           return PrimeResult::Factorisation(full_factor(sup))
        },
        
        Op::GCD => {
          return PrimeResult::N(gcd(inf,sup))
        },
        Op::Invalid =>{
           return PrimeResult::Err("Invalid Operation".to_string())
        },
    }
}

 
