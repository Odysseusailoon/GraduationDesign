use rand::Rng;

mod parameters;
mod simulation;

fn main() {
    //let mut _data_record = Vec::new();
    // unsafe { parameters::init_global_parameters(); }
    //for x in 0..T_COUNT {}
    //println!("{:#?}", simulation::initial_task_distribution());
    let mut rng = rand::thread_rng();
    let tmp: f32 = rng.gen();
    println!("{}", tmp);
}
