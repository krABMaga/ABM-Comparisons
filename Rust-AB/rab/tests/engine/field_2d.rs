#[cfg(test)]

static mut _COUNT: u128 = 0;
static _STEP: u128 = 10;
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
static NUM_AGENT: u32 = 10;

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
use {
    crate::model::flockers::{bird::*, state::*},
    rand::Rng,
    rust_ab::engine::location::Real2D,
    rust_ab::engine::schedule::Schedule,
    rust_ab::engine::state::State,
};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
pub fn field_2d_single_step() {
    let mut state = Flocker::new((WIDTH, HEIGHT), NUM_AGENT);
    let mut schedule: Schedule = Schedule::new();

    state.init(&mut schedule);
    schedule.step(&mut state);

    let v = *(state.field1.nagents.borrow());
    assert_eq!(NUM_AGENT as usize, v);

    let vec = state
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.0, y: 5.0 }, 10.0);
    assert_eq!(NUM_AGENT as usize, vec.len());

    let vec = state
        .field1
        .get_neighbors_within_relax_distance(Real2D { x: 5.0, y: 5.0 }, 10.0);
    assert_eq!(NUM_AGENT as usize, vec.len());
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn field_2d_neighbors() {
    let mut state = Flocker::new((WIDTH, HEIGHT), 2);

    let last_d = Real2D { x: 0.0, y: 0.0 };

    let mut bird1 = Bird::new(1, Real2D { x: 0.0, y: 0.0 }, last_d);
    let bird2 = Bird::new(2, Real2D { x: 0.0, y: 0.0 }, last_d);

    state.field1.set_object_location(bird1, bird1.pos);
    state.field1.set_object_location(bird2, bird2.pos);

    state.update(0);

    let v = *(state.field1.nagents.borrow());
    assert_eq!(2, v);

    let vec = state
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.0, y: 5.0 }, 1.0);
    assert_eq!(0, vec.len());
    let vec = state
        .field1
        .get_neighbors_within_relax_distance(Real2D { x: 5.0, y: 5.0 }, 1.0);
    assert_eq!(0, vec.len());

    let mut rng = rand::thread_rng();

    let fly: f32 = rng.gen_range(5..10) as f32;

    bird1.pos = Real2D {
        x: bird1.pos.x + fly,
        y: bird1.pos.y + fly,
    };

    state.field1.set_object_location(bird1, bird1.pos);
    state.field1.set_object_location(bird2, bird2.pos);

    state.update(0);

    let vec = state.field1.get_neighbors_within_distance(bird1.pos, 1.0);
    assert_eq!(1, vec.len());
    assert!(vec.contains(&bird1));
    let vec = state.field1.get_neighbors_within_distance(bird2.pos, 1.0);
    assert_eq!(1, vec.len());
    assert!(vec.contains(&bird2));

    let vec = state
        .field1
        .get_neighbors_within_distance(Real2D { x: 5.0, y: 5.0 }, 10.0);
    assert_eq!(2, vec.len());
    assert!(vec.contains(&bird1));
    assert!(vec.contains(&bird2));

    let vec = state
        .field1
        .get_neighbors_within_relax_distance(Real2D { x: 5.0, y: 5.0 }, 10.0);
    assert_eq!(2, vec.len());
    assert!(vec.contains(&bird1));
    assert!(vec.contains(&bird2));
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn field_2d_gets() {
    let mut state = Flocker::new((WIDTH, HEIGHT), 2);

    let last_d = Real2D { x: 0.0, y: 0.0 };

    let bird1 = Bird::new(1, Real2D { x: 0.0, y: 0.0 }, last_d);
    let bird2 = Bird::new(2, Real2D { x: 5.0, y: 5.0 }, last_d);
    let bird3 = Bird::new(3, Real2D { x: 5.0, y: 5.0 }, last_d);

    state.field1.set_object_location(bird1, bird1.pos);
    state.field1.set_object_location(bird2, bird2.pos);
    state.field1.set_object_location(bird3, bird3.pos);

    state.update(0);

    let v = *(state.field1.nagents.borrow());
    assert_eq!(3, v);

    let birds = state.field1.get_objects(Real2D { x: 5.0, y: 5.0 });
    assert_eq!(2, birds.len());
    assert!(birds.contains(&bird2));
    assert!(birds.contains(&bird3));

    let no_birds = state.field1.get_objects(Real2D { x: 10.0, y: 0.0 });
    assert_eq!(0, no_birds.len());

    let mut num_birds = state
        .field1
        .num_objects_at_location(Real2D { x: 5.0, y: 5.0 });
    assert_eq!(2, num_birds);

    num_birds = state
        .field1
        .num_objects_at_location(Real2D { x: 0.0, y: 0.0 });
    assert_eq!(1, num_birds);
}
