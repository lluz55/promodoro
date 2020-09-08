#[cfg(not(debug_assertions))]
use rand::{thread_rng, Rng};

#[cfg(not(debug_assertions))]
pub fn get_rnd_port() -> u16 {
  let mut rng = thread_rng();
  let port: u16 = rng.gen_range(2000, 60000);
  port
}