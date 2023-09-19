use rand::{distributions::Alphanumeric, distributions::DistString};

pub fn generate_random_string() {
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    println!("Random string: {}", string);
}
