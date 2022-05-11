extern crate priority_queue;

use crate::engine::{agent::Agent, agentimpl::AgentImpl, priority::Priority, state::State};

use cfg_if::cfg_if;
use priority_queue::PriorityQueue;
use std::fmt;

cfg_if! {
    if #[cfg(feature ="parallel")]{
        use crossbeam::thread;
        use std::sync::{Arc,Mutex};
        use clap::{App, Arg};
        use lazy_static::*;

    }
}

cfg_if! {
    if #[cfg(feature ="parallel")]{
        lazy_static! {
            pub static ref THREAD_NUM: usize = {
                let matches = App::new("Rust-AB")
                    .arg(Arg::with_name("bench").long("bench"))
                    .arg(
                        Arg::with_name("num_thread")
                            .help("sets the number of threads to use")
                            .takes_value(true)
                            .long("nt"),
                    )
                    .get_matches();
                let n = match matches.value_of("num_thread") {
                    Some(nt) => match nt.parse::<usize>() {
                        Ok(ris) => ris,
                        Err(_) => {
                            eprintln!("error: --nt value is not an integer");
                            num_cpus::get()
                        }
                    },
                    _ => 1,
                };
                n
            };
        }
    }
}
cfg_if! {
    if #[cfg(feature ="parallel")] {
        pub struct Schedule {
            pub step: usize,
            pub time: f32,
            pub events: Arc<Mutex<PriorityQueue<AgentImpl, Priority>>>,
            pub thread_num:usize,
            pub agent_ids_counting: Arc<Mutex<u32>>,
        }

        #[derive(Clone)]
        pub struct Pair {
            agentimpl: AgentImpl,
            priority: Priority,
        }

        impl Pair {
            #[allow(dead_code)]
            fn new(agent: AgentImpl, the_priority: Priority) -> Pair {
                Pair {
                    agentimpl: agent,
                    priority: the_priority
                }
            }
        }

        impl fmt::Display for Pair {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "agent: {} priority: {}", self.agentimpl, self.priority)
            }
        }

        impl Schedule {
            pub fn new() -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num: *THREAD_NUM,
                    agent_ids_counting: Arc::new(Mutex::new(0)),
                }
            }

            pub fn with_threads(thread_num: usize) -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: Arc::new(Mutex::new(PriorityQueue::new())),
                    thread_num,
                    agent_ids_counting: Arc::new(Mutex::new(0)),
                }
            }

            pub fn schedule_once(&mut self, agent: AgentImpl, the_time:f32, the_ordering:i32) {
                self.events.lock().unwrap().push(
                    agent,
                    Priority {
                        time: the_time,
                        ordering: the_ordering,
                    },
                );
            }

            pub fn schedule_repeating(&mut self, agent: Box<dyn Agent>, the_time:f32, the_ordering:i32) {
                let mut agent_ids_counting = self.agent_ids_counting.lock().unwrap();
                let mut a = AgentImpl::new(agent, *agent_ids_counting);
                *agent_ids_counting +=1;
                a.repeating = true;
                let pr = Priority::new(the_time, the_ordering);
                self.events.lock().unwrap().push(a, pr);
            }

            

            pub fn get_all_events(&self) -> Vec<Box<dyn Agent>>{
                let mut tor: Vec<Box<dyn Agent>> = Vec::new();
                for e in self.events.lock().unwrap().iter(){
                    tor.push(e.0.agent.clone());
                }
                tor
            }

            pub fn step(&mut self, state: &mut dyn State) {

                let thread_num = self.thread_num;
                let thread_division = (self.events.lock().unwrap().len() as f64 / thread_num as f64).ceil() as usize;
                let mut state = Arc::new(Mutex::new(state));

                if self.step == 0{
                    Arc::get_mut(&mut state).unwrap().lock().unwrap().update(self.step.clone() as u64);
                }

                Arc::get_mut(&mut state).unwrap().lock().unwrap().before_step(self);

                if self.events.lock().unwrap().is_empty() {
                    println!("No agent in the queue to schedule. Terminating.");
                    //TODO check if we need to exit on 0 agents or we have to continue until new agents are spawned
                    std::process::exit(0);
                }

                let mut cevents: Vec<Vec<Pair>> = vec![Vec::with_capacity(thread_division); thread_num];

                match self.events.lock().unwrap().peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop")
                }

                let mut i = 0;
                loop {
                    if self.events.lock().unwrap().is_empty() {
                        break;
                    }

                    let item = self.events.lock().unwrap().pop();
                    match item {
                        Some(item) => {
                            let (agent, priority) = item;
                            let index = match thread_num{
                                0 => 0,
                                _ => i%thread_num
                            };
                            cevents[index].push(Pair::new(agent, priority));
                            i+=1;
                        },
                        None => panic!("no item"),
                    }
                }

                let _result = thread::scope( |scope| {
                    for _tid in 0..thread_num {
                        let events = Arc::clone(&self.events);
                        let state = Arc::clone(&state);

                        let mut batch = cevents.pop().unwrap();

                        scope.spawn(move |_| {

                            for item in batch.iter_mut(){
                                // take the lock from the state
                                let mut state = state.lock().unwrap();
                                let state = state.as_state_mut();

                                // compute the agent
                                item.agentimpl.agent.before_step(state);
                                item.agentimpl.agent.step(state);
                                item.agentimpl.agent.after_step(state);

                                // after computation check if repeating and not stopped
                                if item.agentimpl.repeating && !item.agentimpl.agent.is_stopped(state) {
                                    // take the lock from the queue
                                    let mut q = events.lock().unwrap();
                                    // schedule_once transposition
                                    q.push(
                                        item.agentimpl.clone(),
                                        Priority {
                                            time: item.priority.time + 1.0,
                                            ordering: item.priority.ordering,
                                        },
                                    );

                                    // continue on the next item
                                    continue;
                                }
                            }
                        });
                    }
                });

                Arc::get_mut(&mut state).unwrap().lock().unwrap().after_step(self);
                self.step += 1;
                Arc::get_mut(&mut state).unwrap().lock().unwrap().update(self.step.clone() as u64);
            }
        }
    }
    // SEQUENTIAL IF
    else{
        pub struct Schedule{
            pub step: u64,
            pub time: f32,
            pub events: PriorityQueue<AgentImpl,Priority>,
            pub agent_ids_counting: u32,
        }

        #[derive(Clone)]
        pub struct Pair{
            agentimpl: AgentImpl,
            priority: Priority,
        }

        impl Pair {
            fn new(agent: AgentImpl, the_priority: Priority) -> Pair {
                Pair {
                    agentimpl: agent,
                    priority: the_priority
                }
            }
        }

        impl fmt::Display for Pair {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "agent: {} priority: {}", self.agentimpl, self.priority)
            }
        }

        impl Default for Schedule {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Schedule {
            pub fn new() -> Schedule {
                Schedule {
                    step: 0,
                    time: 0.0,
                    events: PriorityQueue::new(),
                    agent_ids_counting: 0,
                }
            }

            pub fn schedule_once(&mut self, agent: AgentImpl,the_time:f32, the_ordering:i32) {
                self.events.push(agent, Priority{time: the_time, ordering: the_ordering});
            }

            // return false if the insertion in the priority queue fails
            pub fn schedule_repeating(&mut self, agent: Box<dyn Agent>, the_time:f32, the_ordering:i32) -> bool {
                let mut a = AgentImpl::new(agent, self.agent_ids_counting);
                self.agent_ids_counting +=1;
                a.repeating = true;

                let pr = Priority::new(the_time, the_ordering);
                let opt = self.events.push(a, pr);
                match opt {
                    Some(_) => false,
                    None => true,
                }
            }

            // return false if the insertion in the priority queue fails
            pub fn dequeue(&mut self, agent: Box<dyn Agent>, my_id: u32) -> bool {
                let a = AgentImpl::new(agent, my_id);
                let removed = self.events.remove(&a);
                match removed {
                    //some if finded and removed
                    Some(_) => {

                        // println!("Agent {} -- {} removed from the queue",a, my_id);
                        true
                    },
                    None => false,
                }
            }

            pub fn get_all_events(&self) -> Vec<Box<dyn Agent>>{
                let mut tor: Vec<Box<dyn Agent>> = Vec::new();
                for e in self.events.iter(){
                    tor.push(e.0.agent.clone());
                }
                tor
            }

            pub fn step(&mut self, state: &mut dyn State){

                if self.step == 0{
                    state.update(self.step);
                }

                state.before_step(self);

                let events = &mut self.events;

                if events.is_empty() {
                    println!("No agent in the queue to schedule. Terminating.");
                    //TODO check if we need to exit on 0 agents or we have to continue until new agents are spawned
                    std::process::exit(0);
                }

                let mut cevents: Vec<Pair> = Vec::new();

                match events.peek() {
                    Some(item) => {
                        let (_agent, priority) = item;
                        self.time = priority.time;
                    },
                    None => panic!("Agent not found - out loop"),
                }

                loop {
                    if events.is_empty() {
                        break;
                    }

                    match events.pop() {
                        Some(item) => {
                            let (agent, priority) = item;
                            if priority.time > self.time {
                                break;
                            }
                            cevents.push(Pair::new(agent, priority));
                        },
                        None => panic!("Agent not found - inside loop"),
                    }
                }

                for mut item in cevents.into_iter() {

                    item.agentimpl.agent.before_step(state);
                    item.agentimpl.agent.step(state);
                    item.agentimpl.agent.after_step(state);

                    if item.agentimpl.repeating && !item.agentimpl.agent.is_stopped(state) {
                        self.schedule_once(
                            item.agentimpl,
                            item.priority.time + 1.0,
                            item.priority.ordering,
                        );
                    }
                }

                state.after_step(self);
                self.step += 1;
                state.update(self.step);

                //println!("------------ Step {} --------------", self.step);
            }
        }
    }
}

// A struct used to specify schedule options to pass to an agent's clone when an agent reproduces.
pub struct ScheduleOptions {
    pub ordering: i32,
    pub repeating: bool,
}
