//! Regression coverage for generated bodies without awaited-argument bindings.

#![deny(unused_braces)]

use rstest::{fixture, rstest};

#[derive(Default)]
struct World {
    counter: usize,
}

#[fixture]
fn world() -> World { World::default() }

#[rstest]
fn injects_default_world(world: World) {
    assert_eq!(world.counter, 0);
}

#[test]
fn resolves_default_world() {
    assert_eq!(world::default().counter, 0);
}

#[rstest]
#[case(0)]
fn injects_parameterized_world(world: World, #[case] expected: usize) {
    assert_eq!(world.counter, expected);
}

#[rstest]
fn injects_matrix_world(world: World, #[values(0, 1)] increment: usize) {
    assert_eq!(world.counter + increment, increment);
}
