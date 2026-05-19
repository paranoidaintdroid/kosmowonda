use kosmo_math::Vec2;
use rayon::prelude::*;

//const G: f64 = 6.674e-11;
const G: f64 = 0.05;

const SOFTENING: f64 = 1e-2;

#[derive(Debug, Clone)]
pub struct Body {
    pub mass: f64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Body {
    pub fn new(mass: f64, position: Vec2, velocity: Vec2) -> Self {
        Self {
            mass,
            position,
            velocity,
            acceleration: Vec2::new(0.0, 0.0),
        }
    }
}

pub fn gravitational_force(a: &Body, b: &Body) -> Vec2 {
    let (distance, direction) = a.position.distance_and_direction(b.position);

    let softened_distance_squared = distance * distance + SOFTENING * SOFTENING;

    let force_magnitude = G * a.mass * b.mass / softened_distance_squared;

    direction * force_magnitude
}
pub fn accumulate_forces(bodies: &mut Vec<Body>) {
    for body in bodies.iter_mut() {
        body.acceleration = Vec2::new(0.0, 0.0);
    }

    let n = bodies.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let force = gravitational_force(&bodies[i], &bodies[j]);
            bodies[i].acceleration = bodies[i].acceleration + force / bodies[i].mass;
            bodies[j].acceleration = bodies[j].acceleration - force / bodies[j].mass;
        }
    }
}

pub fn accumulate_forces_parallel(bodies: &mut Vec<Body>) {
    let temp_bodies = bodies.clone();

    let accelerations: Vec<Vec2> = (0..temp_bodies.len())
        .into_par_iter()
        .map(|i| {
            let mut acc = Vec2::new(0.0, 0.0);

            for j in 0..temp_bodies.len() {
                if i == j {
                    continue;
                }

                let force = gravitational_force(&temp_bodies[i], &temp_bodies[j]);

                acc = acc + force / temp_bodies[i].mass;
            }

            acc
        })
        .collect();

    for (body, acc) in bodies.iter_mut().zip(accelerations) {
        body.acceleration = acc;
    }
}

const THRESHOLD: usize = 100;

pub fn accumulate_forces_adaptive(
    bodies: &mut Vec<Body>,
) {
    if bodies.len()
        < THRESHOLD
    {
        accumulate_forces(bodies);
    } else {
        accumulate_forces_parallel(
            bodies,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TINY: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < TINY
    }

    #[test]
    fn test_equal_and_opposite_accelerations() {
        let mut bodies = vec![
            Body::new(10.0, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            Body::new(10.0, Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0)),
        ];

        accumulate_forces(&mut bodies);

        let a_acc = bodies[0].acceleration;
        let b_acc = bodies[1].acceleration;

        assert!(approx_eq(a_acc.x, -b_acc.x));
        assert!(approx_eq(a_acc.y, -b_acc.y));
    }

    #[test]
    fn test_acceleration_resets_before_accumulation() {
        let mut bodies = vec![
            Body::new(10.0, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            Body::new(10.0, Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0)),
        ];

        bodies[0].acceleration = Vec2::new(999.0, 999.0);

        accumulate_forces(&mut bodies);

        let expected_force = gravitational_force(&bodies[0], &bodies[1]);

        let expected_acc = expected_force / bodies[0].mass;

        assert!(approx_eq(bodies[0].acceleration.x, expected_acc.x));

        assert!(approx_eq(bodies[0].acceleration.y, expected_acc.y));
    }
}
