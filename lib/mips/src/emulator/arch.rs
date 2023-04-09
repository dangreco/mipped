#[non_exhaustive]
pub struct Register;

#[rustfmt::skip]
impl Register {
  /* Zero */
  pub const ZERO: usize = 0;  

  /* Assembler */
  pub const AT: usize = 1;  

  /* Results */
  pub const V0: usize = 2;  
  pub const V1: usize = 3;  

  /* Args */
  pub const A0: usize = 4;  
  pub const A1: usize = 5;  
  pub const A2: usize = 6;  
  pub const A3: usize = 7;  

  /* Temporaries */
  pub const T0: usize = 8;  
  pub const T1: usize = 9;  
  pub const T2: usize = 10;  
  pub const T3: usize = 11;  
  pub const T4: usize = 12;  
  pub const T5: usize = 13;  
  pub const T6: usize = 14;  
  pub const T7: usize = 15;  

  /* Saved */
  pub const S0: usize = 16;  
  pub const S1: usize = 17;  
  pub const S2: usize = 18;  
  pub const S3: usize = 19;  
  pub const S4: usize = 20;  
  pub const S5: usize = 21;  
  pub const S6: usize = 22;  
  pub const S7: usize = 23;  

  /* Temporaries */
  pub const T8: usize = 24;
  pub const T9: usize = 25;

  /* Kernel */
  pub const K0: usize = 26;
  pub const K1: usize = 27;

  /* Global pointer */
  pub const GP: usize = 28;
  
  /* Stack pointer */
  pub const SP: usize = 29;

  /* Frame poitner */
  pub const FP: usize = 30;

  /* Return address */
  pub const RA: usize = 31;
}
