use cgmath::{InnerSpace, MetricSpace};
use cgmath::num_traits::abs;
use three_d::{Camera, Event, MouseButton, Vec2, Vec3, Zero};
use crate::animation::{rotate_camera_around_target, SecondOrderSystem, SecondOrderSystemParameters};

pub struct SmoothOrbitControl {
    target: Vec3,
    orbit_decay_rate: f32,
    sensitivity: f32,
    scroll_sensitivity: f32,
    max_orbit_speed: f32,
    rotation_speed: Vec2,
    mouse_pressed: bool,
    min_zoom: f32,
    max_zoom: f32,
    hard_zoom: f32,
    soft_zoom: SecondOrderSystem<f32>,
    curr_zoom: f32,
}

pub struct SmoothOrbitControlSettings {
    pub orbit_decay_rate: f32,
    pub sensitivity: f32,
    pub scroll_sensitivity: f32,
    pub max_orbit_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub scroll_params: SecondOrderSystemParameters,
}

impl Default for SmoothOrbitControlSettings {
    fn default() -> Self {
        SmoothOrbitControlSettings {
            orbit_decay_rate: 0.25,
            sensitivity: 1.6,
            scroll_sensitivity: 1.5,
            max_orbit_speed: 1.2,
            min_zoom: 2.0,
            max_zoom: 15.0,
            scroll_params: SecondOrderSystemParameters {
                freq: 2.0,
                zeta: 0.95,
                r: 1.1
            }
        }
    }
}

impl SmoothOrbitControl {
    pub fn new(target: Vec3, camera: &Camera, settings: SmoothOrbitControlSettings) -> Self {
        let dist = camera.position().distance(target);
        Self {
            target,
            rotation_speed: Vec2::zero(),
            mouse_pressed: false,
            orbit_decay_rate: settings.orbit_decay_rate,
            sensitivity: settings.sensitivity,
            scroll_sensitivity: settings.scroll_sensitivity,
            max_orbit_speed: settings.max_orbit_speed,
            min_zoom: settings.min_zoom,
            max_zoom: settings.max_zoom,
            hard_zoom: dist,
            soft_zoom: SecondOrderSystem::new(settings.scroll_params, dist),
            curr_zoom: dist,
        }
    }

    pub fn handle_events(
        &mut self,
        camera: &mut Camera,
        events: &mut [Event],
        delta_time: f32
    ) -> bool {
        let mut change = false;
        let mut mouse_delta: Option<Vec2> = None;
        let mut scroll_delta: Option<Vec2> = None;
        for event in events.iter_mut() {
            match event {
                Event::MouseMotion {
                    button: Some(MouseButton::Left),
                    delta: (x, y),
                    handled, ..
                } if *handled == false => {
                    mouse_delta = Some(Vec2::new(*x, *y));
                    *handled = true;
                    change = true;
                }
                Event::MousePress {
                    button: MouseButton::Left,
                    handled, ..
                } if *handled == false => {
                    self.mouse_pressed = true;
                    *handled = true;
                    change = true;
                }
                Event::MouseRelease {
                    button: MouseButton::Left,
                    handled, ..
                } if *handled == false => {
                    self.mouse_pressed = false;
                    *handled = true;
                }
                Event::MouseWheel {
                    delta: (x, y),
                    handled, ..
                } if *handled == false => {
                    // self.handle_action(camera, event);
                    *handled = true;
                    scroll_delta = Some(Vec2::new(*x, *y));
                    change = true;
                }
                _ => {}
            }
        }
        self.frame(camera, mouse_delta, scroll_delta, delta_time);
        change
    }

    fn frame(
        &mut self,
        camera: &mut Camera,
        mouse_delta: Option<Vec2>,
        scroll_delta: Option<Vec2>,
        delta_time: f32
    ) {
        if delta_time > 0.0 {
            let delta_time = delta_time / 1000.0;
            let scroll_delta = (
                scroll_delta
                .unwrap_or(Vec2::zero())
                * self.scroll_sensitivity / 20.0
            ).y.clamp(-1.8, 1.8);
            self.hard_zoom = (self.hard_zoom - scroll_delta)
                .clamp(self.min_zoom, self.max_zoom);
            self.soft_zoom.update_with_speed(
                delta_time,
                self.hard_zoom,
                -scroll_delta/delta_time
            );
        }

        if let Some(delta) = mouse_delta {
            self.rotation_speed += -delta/10.0 * self.sensitivity/100.0;
            let max = self.max_orbit_speed/10.0;
            self.rotation_speed.x = self.rotation_speed.x.clamp(-max, max);
            self.rotation_speed.y = self.rotation_speed.y.clamp(-max, max);
        }

        if self.rotation_speed.magnitude() > 0.001 {
            self.rotation_speed *= 1.0 - if self.mouse_pressed { 0.2 } else { self.orbit_decay_rate/10.0 };
            rotate_camera_around_target(
                camera,
                self.target,
                self.rotation_speed.x,
                self.rotation_speed.y
            );
        }

        let zoom_delta = self.soft_zoom.value() - self.curr_zoom;
        if abs(zoom_delta) > 0.005 {
            let direction = (camera.position() - self.target).normalize();
            let mut distance = self.soft_zoom.value();
            // https://www.desmos.com/calculator/a98lnplr2z
            if distance < self.min_zoom {
                // Zoom is soft capped at min_zoom, hard capped to min_zoom-1.5
                // Ofc if min_zoom < 1.5 this will break
                let x0 = 10.0/3.0 + self.min_zoom;
                distance = -1.0 / (0.2*(distance - x0)) + self.min_zoom - 1.5;
            }
            let new_pos = self.target + direction * distance;
            let up = *camera.up();
            camera.set_view(new_pos, self.target, up);
        }
        self.curr_zoom = self.soft_zoom.value();
    }
}
