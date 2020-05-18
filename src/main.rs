use crate::parameters::{init_global_parameters, E_MAX, Q_T, TIME_COUNT};
use crate::plot::plot_two_lines;
use crate::simulation::{
    greedy_approximation, initial_task_distribution, markov_approximation,
    task_distribution_after_dispatch,
};

mod parameters;
mod plot;
mod simulation;

// fn multi_return() -> (i32, u32) {
//     (1, 2)
// }

fn main() {
    unsafe {
        init_global_parameters();
        work()
    };
}

unsafe fn work() {
    // let (x, y) = multi_return();

    //let mut _data_record = Vec::new();
    // unsafe { parameters::init_global_parameters(); }
    //for x in 0..T_COUNT {}
    //println!("{:#?}", simulation::initial_task_distribution());
    // let mut rng = rand::thread_rng();
    // let tmp: f32 = rng.gen();
    // println!("{}", tmp);
    let mut origin_data_greedy = Vec::with_capacity(TIME_COUNT);
    let mut origin_data_markov = Vec::with_capacity(TIME_COUNT);
    let mut new_data_greedy = Vec::with_capacity(TIME_COUNT);
    let mut new_data_markov = Vec::with_capacity(TIME_COUNT);
    let mut dist_t = Default::default();
    let mut dist_dispatch = Default::default();
    let mut latency_data_greedy = Vec::with_capacity(TIME_COUNT);
    let mut latency_data_markov = Vec::with_capacity(TIME_COUNT);
    let mut q_t_greedy_queue = Vec::with_capacity(TIME_COUNT);
    let mut q_t_markov_queue = Vec::with_capacity(TIME_COUNT);

    let mut q_t_greedy = 0u32;
    let mut q_t_markov = 0u32;

    for x in 0..TIME_COUNT {
        dist_t = initial_task_distribution();
        dist_dispatch = task_distribution_after_dispatch(&dist_t);
        // let mut current_latency_greedy = Default::default();
        // let mut current_latency_markov = Default::default();

        let (current_latency_greedy, current_new_data, current_old_data) =
            greedy_approximation(dist_t, q_t_greedy);
        latency_data_greedy.push(current_latency_greedy);
        new_data_greedy.push(current_new_data);
        origin_data_greedy.push(current_old_data);
        q_t_greedy = q_t_greedy + current_latency_greedy - E_MAX;
        q_t_greedy = q_t_greedy.max(0);
        q_t_greedy_queue.push(q_t_greedy);

        let (current_latency_markov, current_new_data, current_old_data) =
            markov_approximation(dist_dispatch, q_t_markov);
        latency_data_markov.push(current_latency_markov);
        new_data_markov.push(current_new_data);
        origin_data_markov.push(current_old_data);
        q_t_markov = q_t_markov + current_latency_markov - E_MAX;
        q_t_markov = q_t_markov.max(0);
        q_t_markov_queue.push(q_t_markov);
    }

    plot_two_lines(q_t_markov_queue, q_t_greedy_queue, "#BFFF81", "#FF856A");

    // let v: Vec<_> = q_t_markov_queue.iter().enumerate().collect();
}
