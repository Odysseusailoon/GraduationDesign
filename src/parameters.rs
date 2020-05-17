use rand::distributions::Uniform;
use rand::prelude::*;

//modeling;
pub const SLOT_COUNT: usize = 1;
pub const T_COUNT: usize = 500;
pub const EDGE_COUNT: usize = 25;
pub static mut CURRENT_SLOT: usize = 0;

pub const V: u32 = 50;
//todo
pub const E_MAX: u32 = 2; //can be a var, remember.
pub static mut Q_T: u32 = 0;
//todo
pub const TAO: f32 = 0.1; //parameters for markov approximation's random iteration

pub static mut EDGE_LAMBDA: [u32; EDGE_COUNT] = [0; EDGE_COUNT];
pub static mut BANDWIDTH: [u32; EDGE_COUNT] = [0; EDGE_COUNT];

#[derive(Debug, Clone, Copy, Default)]
pub struct Task {
    pub required_productivity: u32,
    pub profits: u32,
    pub data_size: u32,
    pub edge_id: usize,
    pub latency_t: u32, //use in greedy method;
}

impl Task {
    pub fn new(
        required_productivity: u32,
        profits: u32,
        data_size: u32,
        edge_id: usize,
        latency_t: u32,
    ) -> Self {
        Self {
            required_productivity,
            profits,
            data_size,
            edge_id,
            latency_t,
        }
    }

    /*
    pub unsafe fn edge_lambda(&self) -> u32 {
        EDGE_LAMBDA[self.edge_id]
    }
    */
}

pub unsafe fn init_global_parameters() {
    //EDGE[10] = 1;
    let mut rng = rand::thread_rng();

    let distr_p = Uniform::new_inclusive(40, 100);
    let distr_s = Uniform::new_inclusive(20, 40);
    let distr_data = Uniform::new_inclusive(20, 40);
    let distr_band = Uniform::new_inclusive(20, 40);
    let distr_profits = Uniform::new_inclusive(50, 100);

    for x in EDGE_LAMBDA.iter_mut() {
        *x = rng.sample(distr_s);
    }
    for x in BANDWIDTH.iter_mut() {
        *x = rng.sample(distr_b);
    }

    //
    // REQUIRED_PRODUCTIVITY[1000] = 500;
    // REQUIRED_PRODUCTIVITY[3000] = 300;
    // REQUIRED_PRODUCTIVITY[6000] = 400;
    EDGE_LAMBDA[24] = 400000;
    BANDWIDTH[24] = 1;
}
