use crate::factor::{full_factor,Factor};
/*
   Number Theoretic functions
*/

pub const fn gcd(mut a: u128, mut b: u128) -> u128{
        if b == 0 {
            return a;
        } else if a == 0 {
            return b;
        }

        let self_two_factor = a.trailing_zeros();
        let other_two_factor = b.trailing_zeros();
        let mut min_two_factor = self_two_factor;
        
        if other_two_factor < self_two_factor{
           min_two_factor=other_two_factor;
        }
        
        a >>= self_two_factor;
        b >>= other_two_factor;
        loop {
            if b > a {
            let interim = b;
                b = a;
                a = interim;

            }
            a -= b;

            if a == 0 {
                return b << min_two_factor;
            }
            a >>= a.trailing_zeros();
        }
}

pub fn euler_totient(x: u128) -> u128{
    if x ==0{
       return 0;
    }
    if x == 1{
      return 1;
    }
    let f = full_factor(x);
    let mut numerator = 1;
    let mut denominator = 1;
    for i in f.factors(){
      numerator*=(*i-1);
      denominator*=i;
    }
    (x/denominator)*numerator
}
