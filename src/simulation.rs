use rand;
use rand::Rng;

use crate::parameters::{
    BANDWIDTH, CURRENT_SLOT, DATA_SIZE, EDGE_COUNT, EDGE_LAMBDA, PROFITS, Q_T,
    REQUIRED_PRODUCTIVITY, TASK_COUNT, T_COUNT, V,
};
use std::convert::TryInto;
use std::f32::consts::E;

//initial the task distribution, describe how the edges generate task locally;
pub fn initial_task_distribution() -> Vec<Vec<Vec<usize>>> {
    let mut rng = rand::thread_rng();
    let mut dist_t = Vec::with_capacity(T_COUNT);
    for _ in 0..T_COUNT {
        let mut dist_e = Vec::with_capacity(EDGE_COUNT);
        for _ in 0..EDGE_COUNT {
            let i = rng.gen_range(0, 10);
            let mut tasks = Vec::with_capacity(i);
            for _ in 0..i {
                tasks.push(rng.gen_range(0, TASK_COUNT));
            }
            dist_e.push(tasks);
        }
        dist_t.push(dist_e);
    }
    dist_t
}

//random dispatch those task to different edges from their locality.
pub unsafe fn task_distribution_after_dispatch(dist: &Vec<Vec<Vec<usize>>>) -> Vec<Vec<usize>> {
    let mut dist_dispatch: Vec<Vec<usize>> = std::iter::repeat(Vec::with_capacity(10))
        .take(EDGE_COUNT)
        .collect();
    let mut rng = rand::thread_rng();
    for x in &dist[CURRENT_SLOT] {
        for y in x.iter() {
            let i = rng.gen_range(0, EDGE_COUNT);
            dist_dispatch[i].push(*y);
        }
    }
    dist_dispatch
}

pub unsafe fn objective_function(dispatched: &Vec<Vec<usize>>) -> u32 {
    Q_T * latency(dispatched) + V * profits(dispatched)
}

pub unsafe fn latency(dispatched: &Vec<Vec<usize>>) -> u32 {
    let mut compute_latency = 0u32;
    let mut trans_latency = 0u32;
    for (x_i, x) in dispatched.iter().enumerate() {
        for y in x.iter() {
            compute_latency += REQUIRED_PRODUCTIVITY[*y] / EDGE_LAMBDA[x_i];
            trans_latency += DATA_SIZE[*y] / BANDWIDTH[x_i];
        }
    }
    compute_latency + trans_latency
}

pub unsafe fn profits(dispatched: &Vec<Vec<usize>>) -> u32 {
    let mut dc_profits = 0;
    for x in dispatched[24].iter() {
        dc_profits += PROFITS[*x];
    }
    dc_profits
}

pub unsafe fn markov_approximation(q_t: u32, t: u32, dispatched: &Vec<Vec<usize>>) {
    let mut rng = rand::thread_rng();
    let mut stop_flag = false;
    let mut old_val = objective_function(dispatched);
    // let mut data_record = Vec::new();
    let mut iteration = 0;

    while !stop_flag {
        iteration += 1;
        let mut new_dispatched = dispatched.clone();
        let random_edge = rng.gen_range(0, EDGE_COUNT); //randomly choose one edge-cloud sever to do markov approximation
        let current_edge = &dispatched[random_edge];
        for task in current_edge {
            let new_edge = rng.gen_range(0, EDGE_COUNT);
            unimplemented!()
        }

        let mut new_val = objective_function(&new_dispatched); //evaluate the new answer
        if iteration < 300 {
            //the probability of accepting present shuffled configuration
            let mumu = 1.0 / (1.0 + E.powi((new_val - old_val).try_into().unwrap()));
            let tmp: f32 = rng.gen();
            if mumu < tmp {
                old_val = new_val;
            }
        } else {
            stop_flag = true;
        }
    }
}
