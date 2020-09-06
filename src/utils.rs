use rand::{thread_rng, Rng};

pub fn get_rnd_port() -> u16 {
  let mut rng = thread_rng();
  let port: u16 = rng.gen_range(2000, 60000);
  port
}