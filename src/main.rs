// Based on: https://github.com/hunar4321/particle-life
// See also: https://particle-life.com/about
// Author: Gabriel Abrahams
// Date: 7 September 2022

use std::ops::Sub;
use std::ops::Mul;
use std::ops::Add;
use rand::Rng;

use speedy2d::Window;

use speedy2d::color::Color;
// use speedy2d::error::BacktraceError;
// use speedy2d::error::ErrorMessage;
// use speedy2d::time::Stopwatch;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vector2 as speedyVec2;

struct MyWindowHandler {
    green: Vec<Particle>,
    red: Vec<Particle>,
    yellow: Vec<Particle>,
    // timer: Result<Stopwatch, BacktraceError<ErrorMessage>>
}

const WINDOW_X: u32 = 2000;
const WINDOW_Y: u32 = 1500;
const N_PARTICLES: u32 = 5000;
const RADIUS: f32 = 5.0;

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        // let dt = match &self.timer {
        //     Ok(timer) => timer.secs_elapsed() as f32 * 100.0,
        //     Err(_) => 0.0,
        // };
        let dt = 0.5;
        
        let y = self.yellow.clone();
        let r = self.red.clone();
        let g = self.green.clone();

        apply_rule(&mut self.red, &r, -0.1, dt);
        apply_rule(&mut self.red, &g, -0.34, dt);
        apply_rule(&mut self.red, &y, 0.0, dt);

        apply_rule(&mut self.green, &r, -0.17, dt);
        apply_rule(&mut self.green, &g, -0.32, dt);
        apply_rule(&mut self.green, &y, 0.34, dt);

        apply_rule(&mut self.yellow, &g, -0.2, dt);
        apply_rule(&mut self.yellow, &r, 0.0, dt);
        apply_rule(&mut self.yellow, &y, 0.15, dt);

        graphics.clear_screen(Color::from_rgb(0.09, 0.0, 0.09));

        for particle in &self.green {
            graphics.draw_circle(speedyVec2{x: particle.pos.x, y: particle.pos.y}, RADIUS, Color::GREEN)
        }
        for particle in &self.red {
            graphics.draw_circle(speedyVec2{x: particle.pos.x, y: particle.pos.y}, RADIUS, Color::RED)
        }
        for particle in &self.yellow {
            graphics.draw_circle(speedyVec2{x: particle.pos.x, y: particle.pos.y}, RADIUS, Color::YELLOW)
        }
        // Request that we draw another frame once this one has finished
        helper.request_redraw();

        // self.timer = Stopwatch::new();
    }

}

fn main() {
    let window = Window::new_centered("Title", (WINDOW_X, WINDOW_Y)).unwrap();
    window.run_loop(MyWindowHandler{
        green: Particle::create(N_PARTICLES),
        red: Particle::create(N_PARTICLES),
        yellow: Particle::create(N_PARTICLES),
        // timer: Stopwatch::new()
    });
    
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vector2 {
    x: f32,
    y: f32
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Vector2 {
    fn scale(self, factor: f32) -> Self {
        Vector2 {
            x: self.x * factor,
            y: self.y * factor
        }
    }

    fn zero() -> Self {
        Vector2 {
            x: 0.0,
            y: 0.0
        }
    }

    fn rand(xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> Self {
        let mut rng = rand::thread_rng();
        Vector2 {
            x: rng.gen_range(xmin..xmax),
            y: rng.gen_range(ymin..ymax)
        }
    }       
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Particle {
    pos: Vector2,
    vel: Vector2
}

impl Particle {
    fn create(n_particles:u32) -> Vec<Particle> {
        let mut particles: Vec<Particle> = Vec::new();
        for _ in 0..n_particles {
            particles.push(Particle { pos: Vector2::rand(0.0, WINDOW_X as f32, 0.0, WINDOW_Y as f32), vel: Vector2::zero() });
        }
        particles
    }
}

use rayon::prelude::*;
fn apply_rule(particles1: &mut Vec<Particle>, particles2: &Vec<Particle>, g: f32, dt: f32) {
    particles1.par_iter_mut().map(
        |a| {
            let mut force: Vector2 = Vector2::zero();
            for b in particles2 {
                let df = a.pos - b.pos;
                let d = (df * df).sqrt();
                if d > RADIUS*2.0 && d < 80.0 {
                    force = force + df.scale(g / d);
                    //force = force + df.scale(-1.0 / d);
                } else if 0.0 < d && d <= RADIUS*2.0 {
                    force = force + df.scale(2.0 / d.powi(2));
                }
            };
            a.vel = a.vel.scale(0.5) + force.scale(dt);
            a.pos = a.pos + a.vel.scale(dt);
            if a.pos.x <= 0.0 || a.pos.x >= WINDOW_X as f32 {a.vel.x *= -1.0}
            if a.pos.y <= 0.0 || a.pos.y >= WINDOW_Y as f32 {a.vel.y *= -1.0}
        }
    ).collect::<()>();
    
    // for i in 0..particles1.len() {
    //     let mut force: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    //     if let Some(a) = particles1.get_mut(i) {
    //     for j in 0..particles2.len() {
    //         if let Some(b) = particles2.get(j) {
    //             let df = a.pos - b.pos;
    //             let d = (df * df).sqrt();
    //             if d > 0.0 && d < 80.0 {
    //                 force = force + df.scale(g / d);
    //             }
    //         };};
    //     a.vel = (a.vel + force).scale(0.5);
    //     a.pos = a.pos + a.vel;
    //     if a.pos.x <= 0.0 || a.pos.x >= WINDOW_X as f32 {a.vel.x *= -1.0}
    //     if a.pos.y <= 0.0 || a.pos.y >= WINDOW_Y as f32 {a.vel.y *= -1.0}
    //     };
    // }
}