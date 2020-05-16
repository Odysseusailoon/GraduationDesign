use rand::distributions::Uniform;
use rand::prelude::*;

//modeling;
pub const SLOT_COUNT: usize = 1;
pub const T_COUNT: usize = 500;
pub const EDGE_COUNT: usize = 25;
pub const TASK_COUNT: usize = 10000;
pub static mut CURRENT_SLOT: usize = 0;

pub const V: u32 = 50;
pub const E_MAX: u32 = 2; //can be a var, remember.
pub static mut Q_T: u32 = 0;
pub const TAO: f32 = 0.1; //parameters for markov approximation's random iteration

///computation delay parameters;
pub static mut EDGE_LAMBDA: [u32; EDGE_COUNT] = [0; EDGE_COUNT];
pub static mut REQUIRED_PRODUCTIVITY: [u32; TASK_COUNT] = [0; TASK_COUNT];
pub static mut PROFITS: [u32; TASK_COUNT] = [0; TASK_COUNT];
///trans delay parameters;
pub static mut DATA_SIZE: [u32; TASK_COUNT] = [0; TASK_COUNT];
pub static mut BANDWIDTH: [u32; EDGE_COUNT] = [0; EDGE_COUNT];

pub unsafe fn init_global_parameters() {
    //EDGE[10] = 1;
    let mut rng = rand::thread_rng();

    let distr_p = Uniform::new_inclusive(40, 100);
    let distr_s = Uniform::new_inclusive(20, 40);
    let distr_data = Uniform::new_inclusive(20, 40);
    let distr_band = Uniform::new_inclusive(20, 40);
    let distr_profits = Uniform::new_inclusive(50, 100);

    for x in REQUIRED_PRODUCTIVITY.iter_mut() {
        *x = rng.sample(distr_p);
    }
    for x in EDGE_LAMBDA.iter_mut() {
        *x = rng.sample(distr_s);
    }
    for x in DATA_SIZE.iter_mut() {
        *x = rng.sample(distr_data);
    }
    for x in PROFITS.iter_mut() {
        *x = rng.sample(distr_profits);
    }

    REQUIRED_PRODUCTIVITY[1000] = 500;
    REQUIRED_PRODUCTIVITY[3000] = 300;
    REQUIRED_PRODUCTIVITY[6000] = 400;
    EDGE_LAMBDA[24] = 400000;
    BANDWIDTH[24] = 1;
}
