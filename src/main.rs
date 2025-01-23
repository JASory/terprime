mod prime_math;
mod factor;
mod ntheory;
mod mapping;

#[cfg(feature="gui")]
mod gui;

#[cfg(feature="gui")]
use gui::build_ui;
#[cfg(feature="gui")]
use gtk4::{
    glib::{self, clone},
    prelude::*,
};

use machine_prime::is_prime_128;
use prime_math::*;
use crate::factor::full_factor;
use crate::ntheory::{gcd,euler_totient};
use crate::mapping::*;

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
 const HP : &str = "-h";
 const AB : &str = "-a";
 const INT_ERROR : &str = "Valid inputs are Natural numbers from 0 to 340,282,366,920,938,463,463,374,607,431,768,211,456";
 const ABOUT : &str = "
   Terprime is an alternate approach to primecounting and listing to 
   Kim Walisch's primesieve.While primesieve aims to be the fastest 
   way to enumerate primes by sieving (and in general),terprime strives
   to be the fastest by primality testing. Terprime is generally slower 
   than primesieve however due to minimal precomputation it is faster 
   for checking (small) intervals of larger integers around 2^128. 
    
   Terprime uses the Machine-Prime library (https://github.com/JASory/machine-prime), 
   and primarily exists to showcase the advantages and drawbacks of individual 
   primality testing vs sieving.  
   
   Terprime 1.2 
   Copyright (C) JASORY
   AGPL 3.0    
 ";
 
 const HELP : &str = "
 Usage: terprime FUNCTION [X] Y
 If one argument is provided then X is set to zero, and the interval 0;Y is analysed
 
 Primality: 
 
    check      Checks an integer for primality, returning Prime or Composite
    count      Counts the number of primes between X and Y
    interval   Lists the primes from X to Y, inclusive
    list       Lists the primes from the X-th to the Y-th, inclusive
    nth        Lists the X-th prime
    next       Lists the next prime greater than X
    prev       Lists the next prime less than X
    write      Writes list of primes from X to Y, writes to \"primes\" file
               in local directory. Much faster than piping stdout to file
    binary     Writes list of primes from X to Y in little-endian binary format, 
               writes to \"primes.bin\" in local directory. Faster than utf8 writing   
               
  Number Theory:
  
    factor     Factors X into primes
    gcd        Greatest common divisor of X,Y
    euler      Euler phi of X
    
    
    -h         This help page 
    -a         About terprime 
 ";



/*
   In: Vector of Strings
   Out: START,STOP,THREADS
*/ 
fn xtrct_args(args: Vec<String>) -> Option<(u128,u128,u128)>{
   match args.len(){
    3 => {
      match args[2].parse::<u128>(){       
       Ok(x) => {return Some((0,x,thread_count() as u128))} ,
       Err(_) => None,
      }
    },
    4 => {
      match (args[2].parse::<u128>(),args[3].parse::<u128>()){
      (Ok(x),Ok(y)) => {let (inf,sup) = fix_sequence(x,y); return Some((inf,sup,thread_count() as u128))}
      _=> None,
      }
    },
   _=> None 
  }
}

// 
fn terminal(flag: bool) -> bool{

      let env_var = std::env::args().collect::<Vec<String>>();
       
       if env_var.len() < 2{
          if flag{
             println!("{}",HELP);
          }
        //  println!("{}",HELP);
          return false;
       }
       
       let op = env_var[1].as_str();
       
       let op_enum = map_string(op);
      
       if op_enum == Op::Invalid{
          println!("{}",HELP);
          return true;
       }
       else{
          match xtrct_args(env_var){
             Some(args) => {
             
              let res = calc_table(op_enum,args.0,args.1,args.2,None);
              
            println!("{}",res.display(false));
            return true;
          }
          None => { println!("{}",INT_ERROR); return true;},
         }
       }
}
/*
fn terminal(flag: bool){
       if flag{
       let env_var = std::env::args().collect::<Vec<String>>();
       
       if env_var.len() < 2{
          println!("{}",HELP);
       }
       
       let op = env_var[1].as_str();
       
       let op_enum = map_string(op);
      
       if op_enum == Op::Invalid{
          println!("{}",HELP);
       }
       else{
       match xtrct_args(env_var){
          Some(args) => {
             
       let res = calc_table(op_enum,args.0,args.1,args.2,None);
              
       println!("{}",res.display(false));
          }
          None => println!("{}",INT_ERROR),
       }
       
       }
      }
      else{
        
        let env_var = std::env::args().collect::<Vec<String>>();
       
       if env_var.len() < 2{
          ()//println!("{}",HELP);
       }
       else{
        let op = env_var[1].as_str();
       
       let op_enum = map_string(op);
      
       if op_enum == Op::Invalid{
          println!("{}",HELP);
       }
       else{
       match xtrct_args(env_var){
          Some(args) => {
             
       let res = calc_table(op_enum,args.0,args.1,args.2,None);
              
       println!("{}",res.display(false));
          }
          None => println!("{}",INT_ERROR),
       } 
       }
      }
}
}
*/

fn main() {
    #[cfg(not(feature="gui"))]
    const ERRORFLAG : bool = true;
    
     #[cfg(feature="gui")]
    const ERRORFLAG : bool = false;
    
    let execflag = terminal(ERRORFLAG);
    
    //let env_var = std::env::args().collect::<Vec<String>>();
    
    if !execflag{
      
    #[cfg(feature="gui")]
    {
        let application = gtk4::Application::builder()
        .application_id("com.github.jasory.terprime")
        .build();

    application.connect_activate(build_ui);
    application.run();
    }
    
    }//println!("Bang!");  
/* 
    let env_var = std::env::args().collect::<Vec<String>>();

   // let mut flag = false;

    if env_var.len() < 2{
     println!("{}",HELP);
    }
    else{
     // Start timer
 //   let start = std::time::Instant::now();

      match env_var[1].as_str(){
       CHECK => {
           if env_var.len() < 3{
             println!("{}",INT_ERROR);
           }
           else{
             match env_var[2].parse::<u128>(){
               Ok(x) => {
                    if is_prime_128(x){
                       println!("Prime");
                    }
                    else{
                      println!("Composite");
                    }
                     }
               Err(_) => println!("{}",INT_ERROR),
             } // end inner match
           } // end inner else
        } //
    COUNT => {
    
       match xtrct_args(env_var){
        Some((inf,sup,tc)) => {
   //            flag = true; 
        println!("{}",parallel_pi(inf,sup,tc));
        },
        None => println!("{}",INT_ERROR), 
       }     
     }
    INTERVAL => {
      
      match xtrct_args(env_var){
        Some((inf,initial_sup,tc)) => {
       //     flag = true; 
         let sup = fix_bound(initial_sup);
           let primelist = parallel_plist(inf,sup,tc);
           for i in primelist.iter(){
             println!("{}",i)
           }   
        },
        None => println!("{}",INT_ERROR), 
      }   
    }
    WRITE => {
         
     match xtrct_args(env_var){
        Some((inf,initial_sup,tc)) => {
         //     flag = true; 
           let sup = fix_bound(initial_sup);
           let mut file = std::io::BufWriter::new(std::fs::File::create("primes").unwrap());
           
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
        },
        None => println!("{}",INT_ERROR), 
      }   
      
    }
    
    BINARY => {

     match xtrct_args(env_var){
        Some((inf,initial_sup,tc)) => {
           //   flag = true; 
           let sup = fix_bound(initial_sup);
           let mut file = std::io::BufWriter::new(std::fs::File::create("primes.bin").unwrap());
           
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
        },
        None => println!("{}",INT_ERROR), 
      }   
      
    }
    
    LIST => {

       match xtrct_args(env_var){ // inner match 
        Some((inf,sup,tc)) => { // match branch
          //  flag = true; 
           let p = primes_interval(inf,sup,tc);
           for i in p{
              println!("{}",i)
           }            
        },
        None => println!("{}",INT_ERROR),        
    }
    },
    NTH => {
      match xtrct_args(env_var){
       Some((_,sup,tc)) =>{
      //  flag = true;
        let p = primes_interval(sup,sup,tc);
        for i in p{
          println!("{}",i)
        }
       }
       None => println!("{}",INT_ERROR),
      }
    },
    NEXT => {
      match xtrct_args(env_var){
       Some((_,sup,_)) =>{
        let k = bounded_prime(sup,1);
        match k{
          Some(x) => println!("{}",x),
          None => println!("Terprime does not support values beyond 2^128"),
        }
       }
       None => println!("{}",INT_ERROR),
      }
    },
    PREV => {
      match xtrct_args(env_var){
       Some((_,sup,_)) =>{
        let k = bounded_prime(sup,u128::MAX);
        match k{
          Some(x) => println!("{}",x),
          None => println!("No prime exists below 2"),
        }
       }
       None => println!("{}",INT_ERROR),
      }
    },
    FACTOR => {
       match xtrct_args(env_var){
       Some((_,sup,_)) =>{
           if sup==0{
              println!("Infinite number of factors");
           }
           if sup == 1{
              println!("1");
           }
           else{
          println!("{}",full_factor(sup).display());
          }
       }
       None => println!("{}",INT_ERROR),
      }
    },
    
    GCD => {
       match xtrct_args(env_var){
        Some((inf,sup,_)) =>{
         println!("{}",gcd(inf,sup));
       }
       None => println!("{}",INT_ERROR),
       }
     },
     EULER => {
    
       match xtrct_args(env_var){
        Some((_,sup,_)) => {
        println!("{}",euler_totient(sup));
        },
        None => println!("{}",INT_ERROR), 
       }     
     },
    HP  => println!("{}",HELP),  
    AB  => println!("{}",ABOUT),
    _=> println!("Select one of the following {{check, count, factor,nth, next, prev, list, interval, write, binary}} as the first argument"),
    } 
 //  if flag{
 //     println!("\nExecuted in {:?}",start.elapsed())
 //  }
}
*/
}
