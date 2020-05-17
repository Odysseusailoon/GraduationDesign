use rand;
use rand::Rng;

use crate::parameters::{Task, BANDWIDTH, CURRENT_SLOT, EDGE_COUNT, EDGE_LAMBDA, Q_T, V};
use std::convert::TryInto;
use std::env::temp_dir;
use std::f32::consts::E;

//initial the task distribution, describe how the edges generate task locally;
pub fn initial_task_distribution() -> Vec<Vec<Task>> {
    let mut rng = rand::thread_rng();
    let mut dist_e = Vec::with_capacity(EDGE_COUNT);
    for edge_id in 0..EDGE_COUNT {
        let i = rng.gen_range(0, 10);
        let mut tasks = Vec::with_capacity(i);
        for _ in 0..i {
            //Todo
            let required_productivity: u32 = rng.gen_range(40, 100); //randomly choose one edge-cloud sever to do markov approximation
            let profits: u32 = rng.gen_range(40, 100); //randomly choose one edge-cloud sever to do markov approximation
            let data_size: u32 = rng.gen_range(40, 100); //randomly choose one edge-cloud sever to do markov approximation

            let mut task = Task::new(required_productivity, profits, data_size, edge_id, 0);

            tasks.push(task);
        }
        dist_e.push(tasks);
    }
    dist_e
}

//random dispatch those task to different edges from their locality.
pub unsafe fn task_distribution_after_dispatch(dist: &Vec<Vec<Task>>) -> Vec<Vec<Task>> {
    let mut dist_dispatch: Vec<Vec<Task>> = std::iter::repeat(Vec::with_capacity(10))
        .take(EDGE_COUNT)
        .collect();
    let mut rng = rand::thread_rng();
    for x in dist.iter() {
        for y in x.iter() {
            let i = rng.gen_range(0, EDGE_COUNT);
            dist_dispatch[i].push(*y);
        }
    }
    dist_dispatch
}

pub unsafe fn objective_function(dispatched: &Vec<Vec<Task>>, q_t: u32) -> u32 {
    q_t * latency(dispatched) + V * profits(dispatched)
}

pub unsafe fn latency_t(y: &mut Task, x_i: usize) -> u32 {
    let mut compute_latency = 0u32;
    let mut trans_latency = 0u32;
    compute_latency = y.required_productivity / EDGE_LAMBDA[x_i];
    if x_i == y.edge_id {
        trans_latency = 0;
    } else {
        trans_latency = y.data_size / BANDWIDTH[y.edge_id];
    }
    y.latency_t = compute_latency + trans_latency;
    y.latency_t
}

pub unsafe fn latency(mut dispatched: &Vec<Vec<Task>>) -> u32 {
    // let mut compute_latency = 0u32;
    // let mut trans_latency = 0u32;
    let mut latency = 0u32;
    //let task = Task::new();
    for (x_i, x) in dispatched.iter_mut().enumerate() {
        for y in x.iter_mut() {
            y.latency_t = latency_t(y, x_i);
            latency += y.latency_t;
        }
    }
    latency
}

pub unsafe fn profits(dispatched: &Vec<Vec<Task>>) -> u32 {
    let mut dc_profits = 0;
    for x in dispatched[24].iter() {
        dc_profits += x.profits;
    }
    dc_profits
}

pub unsafe fn greedy_approximation(mut dist_t: Vec<Vec<Task>>, q_t: u32) -> (u32, u32, u32) {
    // let mut rng = rand::thread_rng();
    //let mut stop_flag = false;

    let mut old_val = objective_function(&dist_t, q_t);
    let origin_data_record = old_val;
    // origin_data_record.push(old_val);

    let mut data_record = Vec::new();
    // let mut iteration = 0;
    let mut random_edge = 0;
    // let random_edge = 0;

    let mut new_dispatched = Default::default();
    loop {
        new_dispatched = dist_t.clone();
        // iteration += 1;
        let mut current_edge = dist_t.remove(random_edge);
        new_dispatched[random_edge].clear();
        for task in current_edge.iter_mut() {
            // let new_edge = rng.gen_range(0, EDGE_COUNT);
            // new_dispatched[new_edge].push(*task);
            let mut new_edge: usize = random_edge;
            let mut edge_iter = 0usize;
            let mut tmp_latency_t = 0;

            while edge_iter < EDGE_COUNT {
                edge_iter += 1;
                tmp_latency_t = latency_t(task, edge_iter);
                if tmp_latency_t >= task.latency_t {
                    continue;
                } else {
                    task.latency_t = tmp_latency_t;
                    new_edge = edge_iter;
                }
            }
            new_dispatched[new_edge].push(*task)
        }

        old_val = old_val.max(objective_function(&new_dispatched, q_t));
        //data_record.push(new_val);
        random_edge += 1;
        if random_edge == EDGE_COUNT {
            break;
        } else {
            continue;
        }
    }
    // data_record.push(old_val);
    (latency(&new_dispatched), old_val, origin_data_record)
}

pub unsafe fn markov_approximation(mut dispatched: Vec<Vec<Task>>, q_t: u32) -> (u32, u32, u32) {
    let mut rng = rand::thread_rng();
    let mut stop_flag = false;

    let mut old_val = objective_function(&dispatched, q_t);
    let origin_data_record = old_val;

    let mut data_record = Vec::new();
    let mut iteration = 0;
    let mut new_dispatched = Default::default();

    while !stop_flag {
        iteration += 1;
        new_dispatched = dispatched.clone();
        let random_edge = rng.gen_range(0, EDGE_COUNT); //randomly choose one edge-cloud sever to do markov approximation
        let current_edge = dispatched.remove(random_edge);
        new_dispatched[random_edge].clear();
        for task in current_edge.iter() {
            let new_edge = rng.gen_range(0, EDGE_COUNT);
            new_dispatched[new_edge].push(*task);
        }

        let mut new_val = objective_function(&new_dispatched, q_t); //evaluate the new answer
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

    // data_record.push(old_val);
    (latency(&new_dispatched), old_val, origin_data_record)
}
