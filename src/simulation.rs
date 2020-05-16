use rand;
use rand::Rng;

use crate::parameters::{
    Task, BANDWIDTH, CURRENT_SLOT, DATA_SIZE, EDGE_COUNT, EDGE_LAMBDA, PROFITS, Q_T,
    REQUIRED_PRODUCTIVITY, TASK_COUNT, T_COUNT, V,
};
use std::convert::TryInto;
use std::f32::consts::E;

//initial the task distribution, describe how the edges generate task locally;
pub fn initial_task_distribution() -> Vec<Vec<Vec<Task>>> {
    let mut rng = rand::thread_rng();
    let mut dist_t = Vec::with_capacity(T_COUNT);
    for _ in 0..T_COUNT {
        let mut dist_e = Vec::with_capacity(EDGE_COUNT);
        for edge_id in 0..EDGE_COUNT {
            let i = rng.gen_range(0, 10);
            let mut tasks = Vec::with_capacity(i);
            for _ in 0..i {
                //Todo
                let required_productivity:u32 = rng.gen_range(40,100); //randomly choose one edge-cloud sever to do markov approximation
                let profits:u32 = rng.gen_range(40,100); //randomly choose one edge-cloud sever to do markov approximation
                let data_size:u32 = rng.gen_range(40,100); //randomly choose one edge-cloud sever to do markov approximation

                let task = Task::new(required_productivity,profits,data_size,edge_id);
                tasks.push(task));
            }
            dist_e.push(tasks);
        }
        dist_t.push(dist_e);
    }
    dist_t
}

//random dispatch those task to different edges from their locality.
pub unsafe fn task_distribution_after_dispatch(dist: &Vec<Vec<Vec<Task>>>) -> Vec<Vec<Task>> {
    let mut dist_dispatch: Vec<Vec<Task>> = std::iter::repeat(Vec::with_capacity(10))
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

pub unsafe fn objective_function(dispatched: &Vec<Vec<Task>>) -> u32 {
    Q_T * latency(dispatched) + V * profits(dispatched)
}

pub unsafe fn latency(dispatched: &Vec<Vec<Task>>) -> u32 {
    let mut compute_latency = 0u32;
    let mut trans_latency = 0u32;
    let task = Task::new();
    for (x_i, x) in dispatched.iter().enumerate() {
        for y in x.iter() {
            compute_latency += REQUIRED_PRODUCTIVITY[*y] / EDGE_LAMBDA[x_i];
            trans_latency += DATA_SIZE[*y] / BANDWIDTH[x_i];
        }
    }
    compute_latency + trans_latency
}

pub unsafe fn profits(dispatched: &Vec<Vec<Task>>) -> u32 {
    let mut dc_profits = 0;
    for x in dispatched[24].iter() {
        dc_profits += PROFITS[*x];
    }
    dc_profits
}

pub unsafe fn markov_approximation(q_t: u32, t: u32, mut dispatched: Vec<Vec<Task>>) {
    let mut rng = rand::thread_rng();
    let mut stop_flag = false;
    let mut old_val = objective_function(&dispatched);
    let mut data_record = Vec::new();
    let mut iteration = 0;

    while !stop_flag {
        iteration += 1;
        let mut new_dispatched = dispatched.clone();
        let random_edge = rng.gen_range(0, EDGE_COUNT); //randomly choose one edge-cloud sever to do markov approximation
        let current_edge = dispatched.remove(random_edge);
        new_dispatched[random_edge].clear();
        for task in current_edge.iter() {
            let new_edge = rng.gen_range(0, EDGE_COUNT);
            new_dispatched[new_edge].push(*task);
        }

        let mut new_val = objective_function(&new_dispatched); //evaluate the new answer
        if iteration < 300 {
            //the probability of accepting present shuffled configuration
            let mumu = 1.0 / (1.0 + E.powi((new_val - old_val).try_into().unwrap()));
            let tmp: f32 = rng.gen();
            if mumu < tmp {
                old_val = new_val;
                dispatched = new_dispatched;
            }
        } else {
            stop_flag = true;
        }
    }

    data_record.push(old_val);





}
