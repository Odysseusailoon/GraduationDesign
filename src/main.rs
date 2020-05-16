mod parameters;
mod simulation;

fn main() {
    // unsafe { parameters::init_global_parameters(); }
    println!("{:#?}", simulation::initial_task_distribution());
}
