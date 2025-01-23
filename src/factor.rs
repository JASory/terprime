use crate::ntheory::gcd;

pub struct Factor{
  fctr: Vec<u128>,
  power: Vec<u32>,
}

impl Factor{
  pub fn new() -> Self{
     Self{fctr: vec![], power: vec![]}
  }
  
  pub fn append_factor(&mut self, factor: u128){
      self.fctr.push(factor);
  }
  
  pub fn append_power(&mut self, power: u32){
      self.power.push(power);
  }
  
  pub fn factors(&self) -> std::slice::Iter<u128>{
      self.fctr.iter()
  }
  
  pub fn powers(&self) -> std::slice::Iter<u32>{
      self.power.iter()
  }
  
  pub fn display(&self) -> String{
      let mut output = String::new();
      if self.power[0] != 1{
        output+=&(self.fctr[0].to_string()+"^"+&self.power[0].to_string());
      }
      else{
        output+=&(self.fctr[0].to_string());
      }
      if self.fctr.len() > 1{
      for i in 1..self.fctr.len(){
         if self.power[i] != 1{
         let pair = self.fctr[i].to_string()+"^"+&self.power[i].to_string();
         output+=&(" * ".to_owned() + &pair);
         }
         else{
         let pair = self.fctr[i].to_string();
         output+= &(" * ".to_owned()+&pair);
         }
      }
      }
      output
  }
}


fn drbg(mut base: u64) -> u64{
     base^=base.wrapping_shr(12);
     base^=base.wrapping_shr(25);
     base^=base.wrapping_shr(27);
     base.wrapping_mul(0x2545F4914F6CDD1D)
}

const fn poly_eval(x: u128, subtrahend: u128, n: u128, npi: u128) -> u128{
     machine_prime::mont_sub_128(machine_prime::mont_sqr_128(x,n,npi),subtrahend,n)
}

fn pollard_brent(base: u128,inv:u128,subtrahend: u128, n: u128) -> Option<u128>{
    let m = 512;
    let mut r = 1;
    let mut q = 1;
    let mut g = 1;
    let mut ys = 1;
    let mut y = base;
    let mut x = y;
    
    for cycle in 1..34{    
      x = y;
      for i in 0..r{
        y = poly_eval(y,subtrahend,n,inv);      
      }
      
      let mut k = 0;
      
      loop{
      
        for i in 0..(m*cycle){
           if i >= r-k{
             break;
           }
         
         y=poly_eval(y,subtrahend,n,inv);
         q=machine_prime::mont_prod_128(q,x.abs_diff(y),n,inv);
         } // end loop

         ys=y;
         g = gcd(q,n);
         k+=m;
         if k >= r || g !=1{
            break;
         }
      }
      
      r<<=1;
      if g != 1{
         break;
      }
      
    }
    
    if g ==n{
       while (g==1){
         ys=poly_eval(ys,subtrahend,n,inv);
         g=gcd(x.abs_diff(ys),n);
      
       }
    }
    if g !=1 && g !=n{
       return Some(g);
    }
    None
}




   fn poly_factor(n: u128) -> u128{
   /*
      Possible improvements fuse calculation of x^2 -1 and x^2+1
      
   */
   
      // Start with the polynomial x^2 -1 
      // This works in most cases and is particularly fast to initialise in Montgomery form
   let inv = machine_prime::mul_inv2_128(n);
   let one = machine_prime::one_mont_128(n);
   let base = machine_prime::two_mont_128(one,n);
   
   match pollard_brent(base,inv,one,n){//pollard_brent(n,inv,base,one){
      Some(factor) => return factor,
      None => {
      // if x^2 -1 failed try x^2+1
      // No particular reason except to reuse some values 
        let coef = machine_prime::to_mont_128(n-1,n);
        match pollard_brent(base,inv,coef,n){//match pollard_brent(n,inv,base,coef){
           Some(factor) => return factor,
           None => {
             // Loop that has a roughly 0.5 probability of factoring each iteration
            let mut param = drbg(n as u64);
              loop{
                 let  rand_base= (param as u128)%(n-3)+3;
                match pollard_brent(rand_base,inv,one,n){
                   Some(factor) => return factor,
                   None => param=drbg(param),
                 }
              }
           }
        }
      }
   }
  }


const PRIME_ARRAY : [u16;66] = [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
   73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
   179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
   283, 293, 307, 311, 313, 1093, 3511];

pub(crate) fn full_factor(mut n: u128) -> Factor{
       let twofactors = n.trailing_zeros();
        n >>= twofactors;

        let mut factors = Factor::new();

        if twofactors > 0 {
            factors.append_factor(2);
            factors.append_power(twofactors);
        }

        for i in PRIME_ARRAY.iter() {
            // strips out small primes
            if n % (*i as u128)== 0 {
                factors.append_factor(*i as u128);
                let mut count = 0u32;
                while n % (*i as u128) == 0 {
                    count += 1;
                    n /= (*i as u128);
                }
                factors.append_power(count);
            }
        }

        if n == 1 {
            return factors;
        }

        if machine_prime::is_prime_wc_128(n) {
            factors.append_factor(n);
            factors.append_power(1);
            return factors;
        }

        while n != 1 {
            let k = poly_factor(n);
            factors.append_factor(k);
            let mut count = 0u32;
            while n % k == 0 {
                count += 1;
                n /= k;
            }
            factors.append_power(count);
            if machine_prime::is_prime_wc_128(n) && n !=1{
            factors.append_factor(n);
            factors.append_power(1);
            return factors;
            }
        }
        factors
    }


