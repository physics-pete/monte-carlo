use std::fmt::Display;
use rand::{distributions::{Distribution, Standard}, Rng};


#[derive(Copy, Clone)]
enum Spin {
    Up,
    Down
}

impl Display for Spin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spin::Up => write!(f, "↑"),
            Spin::Down => write!(f, "↓"),
        }
    }
}

impl From<&Spin> for f32 {
    fn from(value: &Spin) -> Self {
        match value {
            Spin::Up => 1f32,
            Spin::Down => -1f32
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    k: i16,
    l: i16,
    m: i16,
    spin: Spin
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.k, self.l, self.m, self.spin)
    }
}

enum ChangeChoice {
    K,
    L,
    M,
    Spin
}

impl Distribution<ChangeChoice> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> ChangeChoice {
        match rng.gen_range(1..=4) {
            1 => ChangeChoice::K,
            2 => ChangeChoice::L,
            3 => ChangeChoice::M,
            4 => ChangeChoice::Spin,
            _ => ChangeChoice::K
        }
    }
}

impl State {
    pub fn new_up(k:i16, l:i16, m:i16) -> State{
       State { k, l, m, spin: Spin::Up }
    }

    pub fn new_down(k:i16, l:i16, m:i16) -> State{
       State { k, l, m, spin: Spin::Down }
    }

    pub fn get_energy(&self) -> f32 {
        f32::from(self.k).powi(2) + f32::from(self.l).powi(2) + f32::from(self.m).powi(2)
    }

    fn generate_new_state(&self) -> State {
        let mut rng = rand::thread_rng();
        let choice: ChangeChoice = rand::random();

        match choice {
            ChangeChoice::K => State { k: rng.gen_range(1..10), l: self.l, m: self.m, spin: self.spin },
            ChangeChoice::L => State { k: self.k, l: rng.gen_range(1..10), m: self.m, spin: self.spin },
            ChangeChoice::M => State { k: self.k, l: self.l, m: rng.gen_range(1..10), spin: self.spin },
            ChangeChoice::Spin => State { 
                k: rng.gen_range(1..10), 
                l: self.l, 
                m: self.m, 
                spin: match rng.gen_range(1..=2) {
                    1 => Spin::Up,
                    2 => Spin::Down,
                    _ => Spin::Up
                }
            },
        }
    }
}

trait Hamiltonian {
    fn apply_state_state(&self, state: &State) -> f32;
}

struct KondoEffect {
    coupling_strength: f32
}

impl Hamiltonian for KondoEffect {
    fn apply_state_state(&self, state: &State) -> f32 {
        state.get_energy() + f32::from(&state.spin) * self.coupling_strength
    }
}

fn main() {

    let mut s = State::new_up(1, 1, 1);
    let k = KondoEffect { coupling_strength: 0.5f32 };
    let mut beta = 0.0f32;

    let mut old_e = k.apply_state_state(&s);
    
    let mut states: Vec<State> = vec![];
    states.push(s.clone());

    let mut rng = rand::thread_rng();

    for _ in 1..100 {
        let new_state = s.generate_new_state();
        let new_e = k.apply_state_state(&new_state);
        
        beta += 0.01f32;

        let prop = (beta * (old_e - new_e)).exp();

        let p: f32 = rng.gen_range(0.0..1.0);

        if p < prop {
            old_e = new_e;
            s = new_state;
            states.push(s.clone());
        }
        
        println!("{}", s);

    }

}
