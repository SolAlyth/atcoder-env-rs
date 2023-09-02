pub const MOD: u128 = 998244353;
pub fn mpow(b: u128, p: u128) -> u128 {if p<=1{b.pow(p as u32)%MOD} else {let sqr=mpow(b.pow(2)%MOD,p/2);if p%2==0{sqr}else{sqr*b%MOD}}}
