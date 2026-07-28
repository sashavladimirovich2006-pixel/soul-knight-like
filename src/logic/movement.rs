//! Movement maths shared by the player and, later, by enemies.

/// Turns four directional inputs into a direction vector.
///
/// Opposite inputs cancel out, and diagonals are normalised so that moving
/// diagonally is not faster than moving along a single axis. The result is
/// either a unit vector or exactly zero.
pub fn movement_direction(up: bool, down: bool, left: bool, right: bool) -> (f32, f32) {
    let x = f32::from(right) - f32::from(left);
    let y = f32::from(up) - f32::from(down);
    let length = x.hypot(y);

    if length == 0.0 {
        return (0.0, 0.0);
    }

    (x / length, y / length)
}

#[cfg(test)]
mod tests {
    use super::movement_direction;

    #[test]
    fn no_input_means_no_movement() {
        let direction = movement_direction(false, false, false, false);
        assert_eq!(direction, (0.0, 0.0));
    }

    #[test]
    fn opposite_inputs_cancel_out() {
        let direction = movement_direction(true, true, true, true);
        assert_eq!(direction, (0.0, 0.0));
    }

    #[test]
    fn a_single_input_gives_a_full_speed_axis() {
        let direction = movement_direction(false, false, false, true);
        assert_eq!(direction, (1.0, 0.0));
    }

    #[test]
    fn diagonal_input_is_normalised() {
        let (x, y) = movement_direction(true, false, false, true);
        let length = x.hypot(y);
        assert!((length - 1.0).abs() < 1e-6, "got {length}");
        assert!(x > 0.0, "expected to move right");
        assert!(y > 0.0, "expected to move up");
    }

    #[test]
    fn a_held_axis_still_works_while_the_other_cancels() {
        let direction = movement_direction(true, false, true, true);
        assert_eq!(direction, (0.0, 1.0));
    }
}
