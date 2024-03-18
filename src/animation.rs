use cgmath::num_traits::{NumOps, Zero};
use cgmath::{Angle, InnerSpace, Quaternion, Rad, Vector3};
use std::f32::consts::PI;
use three_d::Camera;

pub fn ease(t: f32, a: f32) -> f32 {
    t.powf(a) / (t.powf(a) + (1.0 - t).powf(a))
}

///
/// Smooth out an arbitrary value using second order dynamics.
///
/// Based on t3ssel8r's video, this class implements a second order
/// differential equation to smooth out, dampen or otherwise interpolate
/// a discrete value.
///
/// You can use it to make movement look more natural,
/// smooth out rotations or zooming, and even more uses I can't think
/// of right now.
///
pub struct SecondOrderSystem<T> {
    x_prev: T,
    y: T,
    dy: T,
    k1: f32,
    k2: f32,
    k3: f32,
}

///
/// Initial parameters for the second order system.
/// As a rule of thumb:
/// - If you want to change how the system settles after some movement, change **zeta**.
/// - If you want to change the initial response of the system, change **r**.
/// - If you want to speed up or slow down the system as a whole, change **freq**.
///
pub struct SecondOrderSystemParameters {
    /// The natural frequency of the system in cycles per second.
    /// it controls the general speed of the system.
    pub freq: f32,
    /// The damping factor of the system.
    /// It dictates how quickly or slowly a movement 'dies out' after a sudden change.
    /// When `zeta < 1`, the system is *underdamped*, and will oscillate when reaching the
    /// current value, like a spring. When it's greater than 1, the system is *overdamped*
    /// and will not oscillate, instead slowly settling to the value.
    pub zeta: f32,
    /// The response time of the system.
    /// It controls how quickly the system moves after its input moves, or its reaction time.
    /// When `r > 1` the system will overshoot the value, and when `r < 0` it will anticipate
    /// the value, going the opposite direction of the movement for a little bit.
    pub r: f32,
}

impl<T> SecondOrderSystem<T>
where
    T: Sized + NumOps + NumOps<f32, T> + Zero + Copy,
{
    /// Create a new second order system, with the given parameters and initial x value.
    /// For more information on the parameters
    /// and what they do look at [SecondOrderSystemParameters].
    pub fn new(params: SecondOrderSystemParameters, x_initial: T) -> Self {
        let k1 = params.zeta / (PI * params.freq);
        let k2 = 1.0 / ((2.0 * PI * params.freq) * (2.0 * PI * params.freq));
        let k3 = params.r * params.zeta / (2.0 * PI * params.freq);
        SecondOrderSystem {
            x_prev: x_initial,
            y: x_initial,
            dy: T::zero(),
            k1,
            k2,
            k3,
        }
    }

    /// Update the system with a new value. The speed will be approximated using
    /// historical values.
    pub fn update(&mut self, timestep: f32, x: T) {
        let dx = if timestep > 0.0 {
            (x - self.x_prev) / timestep
        } else {
            T::zero()
        };
        self.x_prev = x;
        self.update_with_speed(timestep, x, dx);
    }

    /// Update the system with a new value, specifying its current speed.
    pub fn update_with_speed(&mut self, timestep: f32, x: T, dx: T) {
        self.y = self.y + self.dy * timestep;
        self.dy = (self.dy * self.k2 + (x + dx * self.k3 - self.y) * timestep)
            / (self.k2 + timestep * self.k1);
    }

    /// Get the current state of the system.
    pub fn value(&self) -> T {
        self.y
    }
}

/// Rotate the camera around a given point by two angles (in radians),
/// keeping the camera facing the point.
pub fn rotate_camera_around_target(
    camera: &mut Camera,
    target: Vector3<f32>,
    theta: f32,
    phi: f32,
) {
    let distance = (target - camera.position()).magnitude();
    let dir = (target - camera.position()).normalize();
    let horizontal = dir.cross(*camera.up());
    let vertical = horizontal.cross(dir);
    for i in 0..2 {
        let axis = if i == 0 { vertical } else { horizontal };
        let angle = if i == 0 { theta } else { phi };
        let new_position = rotate_around_axis(*camera.position(), axis, angle);
        camera.set_view(new_position * distance, target, vertical.normalize());
    }
}

/// Rotate a vector around an axis by a given angle in radians.
pub fn rotate_around_axis(vector: Vector3<f32>, axis: Vector3<f32>, angle: f32) -> Vector3<f32> {
    let angle = Rad(angle / 2.0);
    let q = Quaternion::from_sv(angle.cos(), angle.sin() * axis).normalize();
    let p = Quaternion::from_sv(0.0, vector).normalize();
    let rotated = (q * p * q.conjugate()).v;
    rotated
}
