use rand::{rngs::ThreadRng, Rng};
use std::time::Duration;

#[derive(Clone, Debug, Copy)]
struct worker {
    pub working: Option<Duration>,
    pub wait: Duration,
}

pub enum Event {
    In,
    Out,
}

pub enum State {
    S0,
    S1,
    S2,
    S3,
}

fn main() {
    const n: usize = 3;
    let workers = [worker {
        working: None,
        wait: Duration::from_secs(0),
    }; n];

    let mut current_state = State::S0;

    println!("{:?}", workers);
    let mut trng = rand::thread_rng();
    let k = std::time::Duration::from_secs(((15.0 + trng.gen_range(0.0..5.0)) * 60.0) as u64);

    let t = std::time::Duration::from_secs(((16.0 + trng.gen_range(0.0..4.0)) * 60.0) as u64);
    println!("k = {k:?}");

    let end = std::time::Duration::from_secs(8 * 60 * 60);
    println!("end = {end:?}");
    let count = end.as_secs_f64() / k.as_secs_f64();
    println!("count {count:?}");

    let mut time = Duration::from_secs(0);

    let mut timeline: Vec<Duration>;

    while time < end {

    }
}

fn timeline_gen(trng: &mut ThreadRng) {}

trait EventState {
    fn next(&self, event: Event) -> Self;
}

impl EventState for State {
    fn next(&self, event: Event) -> State {
        match self {
            State::S0 => match event {
                Event::In => State::S1,
                Event::Out => panic!("ALO Unreacheble!"),
            },
            State::S1 => match event {
                Event::In => State::S2,
                Event::Out => State::S0,
            },
            State::S2 => match event {
                Event::In => State::S3,
                Event::Out => State::S1,
            },
            State::S3 => match event {
                Event::In => State::S3,
                Event::Out => State::S2,
            },
        }
    }
}
