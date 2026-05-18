use crate::Vec2;

pub fn euler_step(pos: Vec2, vel: Vec2, acc: Vec2, dt: f64) -> (Vec2, Vec2) {
    let new_vel = vel + acc * dt;

    let new_pos = pos + vel * dt;

    (new_pos, new_vel)
}

pub fn symplectic_euler_step(pos: Vec2, vel: Vec2, acc: Vec2, dt: f64) -> (Vec2, Vec2) {
    let new_vel = vel + acc * dt;

    let new_pos = pos + new_vel * dt;

    (new_pos, new_vel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_step() {
        let pos = Vec2::new(0.0, 0.0);
        let vel = Vec2::new(1.0, 0.0);
        let acc = Vec2::new(2.0, 0.0);

        let dt = 1.0;

        let (new_pos, new_vel) = euler_step(pos, vel, acc, dt);

        assert_eq!(new_vel, Vec2::new(3.0, 0.0));

        assert_eq!(new_pos, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_symplectic_euler_step() {
        let pos = Vec2::new(0.0, 0.0);
        let vel = Vec2::new(1.0, 0.0);
        let acc = Vec2::new(2.0, 0.0);

        let dt = 1.0;

        let (new_pos, new_vel) = symplectic_euler_step(pos, vel, acc, dt);

        assert_eq!(new_vel, Vec2::new(3.0, 0.0));

        assert_eq!(new_pos, Vec2::new(3.0, 0.0));
    }
}
