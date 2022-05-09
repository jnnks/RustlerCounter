
use rustler::*;
use std::{sync::Mutex, thread::sleep, time::Duration};

struct CounterObject {
    pub count: Mutex<u64>
}

fn load(env: Env, _: Term) -> bool{
    rustler::resource!(CounterObject, env);
    true
}

#[rustler::nif]
fn new(count: u64) -> Result<ResourceArc<CounterObject>, Error> {
    let counter = ResourceArc::new(CounterObject {
        count: Mutex::new(count)
    });

    Ok(counter)
}

#[rustler::nif]
fn bump(env: Env, counter: ResourceArc<CounterObject>) -> Result<Term, Error> {
    let mut count = counter.count.lock().unwrap();
    *count += 1;

    sleep(Duration::from_millis(500));

    Ok(count.encode(env))
}

rustler::init!("Elixir.RustlerCounter.Native", [new, bump], load = load);
