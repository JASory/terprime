use machine_prime::is_prime;
use std::io::Write;


 const CHECK : &str = "check";
 const COUNT : &str = "count";
 const LIST : &str = "list";
 const WRITE : &str = "write";
 const INTERVAL : &str = "interval";
 const HP : &str = "-h";
 const AB : &str = "-a";
 const INT_ERROR : &str = "Valid inputs are Natural numbers from 0 to 18,446,744,073,709,551,616";
 const ABOUT : &str = "
   Terprime is an alternate approach to primecounting and listing to 
   Kim Walisch's primesieve.While primesieve aims to be the fastest 
   way to enumerate primes by sieving (and in general),terprime strives
   to be the fastest by primality testing. Terprime is generally slower 
   than primesieve however due to minimal precomputation it is faster 
   for checking (small) intervals of larger integers around 2^64.
    
   Terprime uses the Machine-Prime library (https://github.com/JASory/machine-prime), 
   and primarily exists to showcase the advantages and drawbacks of individual 
   primality testing vs sieving.  
   
   Terprime 1.0 
   Copyright (C) JASORY
   AGPL 3.0     
 ";
 
 const HELP : &str = "
 Usage: terprime OPTION [START] STOP
 Check, count and list primes less than 2^64
 
 Options: 
 
    check      Checks an integer for primality, returning a boolean TRUE or FALSE
    count      Counts the number of primes between START and STOP
    interval   Lists the primes between START and STOP 
    write      Writes list of primes between START and STOP, writes to \"primes\" file
               in local directory. Much faster than piping stdout to file  
    -h         This help page 
    -a         About terprime 
 ";

 fn thread_count() -> usize{
  match std::thread::available_parallelism(){
   Ok(x) => usize::from(x),
   Err(_) => 1usize,
  }
}

fn d_string(x: u64) -> String{
  x.to_string()+"\n"
}

/*
  In: Two 64-bit unsigned integers
  
  Out: Two 64-bit unsigned integers in the order of lo,hi where lo <= hi
*/

fn fix_sequence(pos_inf:u64, pos_sup: u64) -> (u64,u64){
   (std::cmp::min(pos_inf,pos_sup),std::cmp::max(pos_inf,pos_sup))
}



fn thread_pi(inf: u64, sup: u64) -> u64{
       let mut count = 0u64;
        for i in inf..sup{
          if is_prime(i){
            count+=1;
          }
        }
       count
     }


/*
      
*/


fn pi(inf: u64, sup: u64, tc: u64) -> u64{
   
 let (start, stop) = fix_sequence(inf,sup);
 let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<u64>> = Vec::new();

 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    threads.push( 
      std::thread::spawn( move || { 
        thread_pi(thread_start,thread_stop)
} ));
  } // end for loop

   // Last interval to account for any integer division flooring
  threads.push(
    std::thread::spawn( move || { 
     thread_pi(start+(tc-1)*stride,stop)
}));
  
  let mut total = 0u64;
  
  for handle in threads{
     total+=handle.join().unwrap();
  }
  total
    
}


fn plist(inf: u64, sup: u64, tc: u64) -> Vec<u64>{

    let thread_plist = |inf: u64, sup: u64| -> Vec<u64>{
       let mut veccy = vec![];
        for i in inf..sup{
          if is_prime(i){
            veccy.push(i);
          }
        }
       return veccy;
     };
   
 let (start, stop) = fix_sequence(inf,sup);
 let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<Vec<u64>>> = Vec::new();

 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    threads.push( 
      std::thread::spawn( move || { 
        thread_plist(thread_start,thread_stop)
} ));
  } // end for loop

   // Last interval to account for any integer division flooring
  threads.push(
    std::thread::spawn( move || { 
     thread_plist(start+(tc-1)*stride,stop)
}));
  
  let mut total = vec![];
  
  for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
    
}



/*
   In: Vector of Strings
   Out: START,STOP,THREADS
*/ 
fn xtrct_args(args: Vec<String>) -> Option<(u64,u64,u64)>{
   match args.len(){
    3 => {
      match args[2].parse::<u64>(){
       Ok(x) => {return Some((0,x,thread_count() as u64))} ,
       Err(_) => None,
      }
    },
    4 => {
      match (args[2].parse::<u64>(),args[3].parse::<u64>()){
      (Ok(x),Ok(y)) => {let (inf,sup) = fix_sequence(x,y); return Some((inf,sup,thread_count() as u64))}
      _=> None,
      }
    },
   _=> None 
  }
}



fn main() {
    
    let env_var = std::env::args().collect::<Vec<String>>();
    
    if env_var.len() < 2{
     println!("{}",HELP);
    }
    else{
      match env_var[1].as_str(){
       CHECK => {
           if env_var.len() < 3{
             println!("{}",INT_ERROR);
           }
           else{
             match env_var[2].parse::<u64>(){
               Ok(x) => println!("{}",is_prime(x)),
               Err(_) => println!("{}",INT_ERROR),
             } // end inner match
           } // end inner else
        } //
    COUNT => {
       match xtrct_args(env_var){
        Some((inf,sup,tc)) => println!("{}",pi(inf,sup,tc)),
        None => println!("{}",INT_ERROR), 
       }     
     }
    INTERVAL => {

      match xtrct_args(env_var){
        Some((inf,sup,tc)) => {
           let primelist = plist(inf,sup,tc);
           for i in primelist.iter(){
             println!("{}",i)
           }   
        },
        None => println!("{}",INT_ERROR), 
      }   
    }
    WRITE => {
    
     match xtrct_args(env_var){
        Some((inf,sup,tc)) => {
        
           let mut file = std::io::BufWriter::new(std::fs::File::create("primes").unwrap());
           
           let stride : u64 = 100_000_000*tc;
          
           let steps = (sup-inf)/stride;
           for i in 0..(steps){
              let inner_inf = inf+(stride*i);
              let inner_sup = inf+(stride*(i+1));
              let primelist = plist(inner_inf,inner_sup,tc);
               
               for i in primelist.iter(){
                 let string = d_string(*i);
                 file.write(string.as_bytes()).unwrap();
               }
           }
           let primelist = plist(inf+(stride*steps),sup,tc);
           for i in primelist.iter(){
                 let string = d_string(*i);
                 file.write(string.as_bytes()).unwrap();
               }
        },
        None => println!("{}",INT_ERROR), 
      }   
      
    }
    HP  => println!("{}",HELP),  
    AB  => println!("{}",ABOUT),
    _=> println!("Select either check,count,list or write as the first argument"),
    }
}

//println!("{:?}",stop) 

}
