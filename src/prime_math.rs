use machine_prime::is_prime_128;


pub(crate) fn thread_count() -> usize{
  match std::thread::available_parallelism(){
   Ok(x) => usize::from(x),
   Err(_) => 1usize,
  }
}

pub fn d_string(x: u128) -> String{
  x.to_string()+"\n"
}

/*
   Christian Axler's approximation of nth prime
*/

fn nth_core(x: u128, param: f64) -> u128{
    let float_x = x as f64;
    let log = float_x.ln();
    let double_log = log.ln();
    
    let last_term = (double_log.powi(2)-6f64*double_log + param)/(2f64*log.powi(2));

    (float_x*(log + double_log - 1f64 + (double_log - 2f64)/log - last_term)) as u128
    
}

fn nth_est_upper(x: u128) -> u128{
   if x < 3{
     return 6
  }
   if x > 46254380{
     return nth_core(x,10.667)
   }
   if x > 3467{
    return nth_core(x,0.0)
   }
  return (((x as f64)*(x as f64).ln()) as u128)<<2
}

fn nth_est_lower(x: u128) -> u128{
   nth_core(x,11.321)
}

/*
  In: Two 64-bit unsigned integers
  Out: Two 64-bit unsigned integers in the order of lo,hi where lo <= hi
*/

pub(crate) fn fix_sequence(pos_inf:u128, pos_sup: u128) -> (u128,u128){
   (std::cmp::min(pos_inf,pos_sup),std::cmp::max(pos_inf,pos_sup))
}


pub(crate) fn bounded_prime(mut p: u128, stride: u128) -> Option<u128>{
   loop{
     p= p.wrapping_add(stride);
     
     if p == 0 || p == u128::MAX{
        return None
     }
     
     
     if is_prime_128(p){
        return Some(p)
     }
   }
}
/*
  Description: Function converts to exclusive range limited to the last 64-bit prime
   In: 64-bit integer, x
  Out: x+1, x < 2^64-58
*/


pub(crate) fn fix_bound(x: u128) -> u128{
   if x > (u128::MAX-57){
      return u128::MAX-57
   }
  x+1  
}

/*
    
*/

fn pi(inf: u128, sup: u128) -> u64{
       let mut count = 0u64;
        for i in inf..sup{
          if is_prime_128(i){
            count+=1;
          }
        }
       return count;
     }

/*
      
*/


pub(crate) fn parallel_pi(inf: u128, sup: u128, tc: u128) -> u128{
   
 let (start, stop) = fix_sequence(inf,sup);
 let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<u64>> = Vec::new();

 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    threads.push( 
      std::thread::spawn( move || { 
        pi(thread_start,thread_stop)
} ));
  } // end for loop

   // Last interval to account for any integer division flooring
  threads.push(
    std::thread::spawn( move || { 
     pi(start+(tc-1)*stride,stop+1)
}));
  
  let mut total = 0u128;
  
  for handle in threads{
     total+=handle.join().unwrap() as u128;
  }
  total
    
}



 fn plist(inf: u128, sup: u128) -> Vec<u128>{
       let mut veccy = vec![];
        for i in inf..sup{
          if is_prime_128(i){
            veccy.push(i);
          }
        }
       return veccy
 }


pub(crate) fn parallel_plist(inf: u128, sup: u128, tc: u128) -> Vec<u128>{
   
 let (start, stop) = fix_sequence(inf,sup);
 let stride = (stop-start)/tc;
 let mut threads : Vec<std::thread::JoinHandle::<Vec<u128>>> = Vec::new();

 for i in 0..(tc-1){
    let thread_start = start+i*stride;
    let thread_stop = start+stride*(i+1);
    threads.push( 
      std::thread::spawn( move || { 
        plist(thread_start,thread_stop)
} ));
  } // end for loop

   // Last interval to account for any integer division flooring
  threads.push(
    std::thread::spawn( move || { 
     plist(start+(tc-1)*stride,stop)
}));
  
  let mut total = vec![];
  
  for handle in threads{
     total.extend_from_slice(&handle.join().unwrap()[..]);
  }
  total
    
}



pub(crate) fn primes_interval(inf:u128,sup: u128,tc: u128) -> Vec<u128>{

         if inf == 0{  // if branch

          let bound = nth_est_upper(sup);
          
          let primelist = parallel_plist(inf,bound,tc);

           return primelist[..(sup as usize)].iter().cloned().collect::<Vec<u128>>() 
         }  // end if
           else {

           let mut low_est =  nth_est_lower(inf);
           let mut low_pi = parallel_pi(0u128,low_est,tc);
      
           while low_pi < inf{
              low_est+=1;
              if is_prime_128(low_est){
                 low_pi+=1;
              } // end if
           } // end while
      
           let high_est = nth_est_upper(sup);
           let plist = parallel_plist(low_est,high_est,tc);
           let interval = (sup-inf) as usize+1;

           return plist[..interval].iter().cloned().collect::<Vec<u128>>() 
          } // end branching

}

