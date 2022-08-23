mod android;

use rand::prelude::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

pub fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.par_iter().map(|x| x * x).sum()
}

const INIT_STEPS: u32 = 100;
const INIT_DELTA: f64 = 0.001;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

// Support float vector addition
impl std::ops::Add for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Scalar multiplication
impl std::ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, c: f64) -> Self::Output {
        Vec2D {
            x: self.x * c,
            y: self.y * c,
        }
    }
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    fn distance_sqr(lhs: Self, rhs: Self) -> f64 {
        f64::powi(lhs.x - rhs.x, 2) + f64::powi(lhs.y - rhs.y, 2)
    }
    fn distance(lhs: Self, rhs: Self) -> f64 {
        f64::sqrt(Vec2D::distance_sqr(lhs, rhs))
    }

    pub fn magnitude(v: Self) -> f64 {
        f64::sqrt(f64::powi(v.x, 2) + f64::powi(v.y, 2))
    }

    pub fn zero() -> Vec2D {
        Vec2D { x: 0.0, y: 0.0 }
    }

    pub fn convert_coords(
        from_coords: Vec2D,
        to_width: u32,
        to_height: u32,
        from_width: u32,
        from_height: u32,
    ) -> Vec2D {
        let x = from_coords.x / from_width as f64 * to_width as f64;
        let y = from_coords.y / from_height as f64 * to_height as f64;
        Vec2D::new(x, y)
    }
}

impl Default for Vec2D {
    fn default() -> Vec2D {
        Vec2D { x: 0.0, y: 0.0 }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r, g, b }
    }
    fn white() -> Rgb {
        Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    fn black() -> Rgb {
        Rgb { r: 0, g: 0, b: 0 }
    }

    fn red() -> Rgb {
        Rgb { r: 255, g: 0, b: 0 }
    }
    fn green() -> Rgb {
        Rgb { r: 0, g: 255, b: 0 }
    }
    fn blue() -> Rgb {
        Rgb { r: 0, g: 0, b: 255 }
    }
    pub fn size_of() -> usize {
        std::mem::size_of::<Rgb>()
    }
    pub fn mix_pastel(mix: Rgb) -> Rgb {
        let red = mix.r as f64;
        let green = mix.g as f64;
        let blue = mix.b as f64;
        return Rgb {
            r: ((red + 255.0) / 2.0) as u8,
            g: ((green + 255.0) / 2.0) as u8,
            b: ((blue + 255.0) / 2.0) as u8,
        };
    }

    pub fn mix_blacken(mix: Rgb) -> Rgb {
        let red = mix.r as f64;
        let green = mix.g as f64;
        let blue = mix.b as f64;
        return Rgb {
            r: ((red + 0.0) / 2.0) as u8,
            g: ((green + 0.0) / 2.0) as u8,
            b: ((blue + 0.0) / 2.0) as u8,
        };
    }
    // Generate a random pastel colour
    pub fn random_pastel() -> Rgb {
        let (r, g, b): (u8, u8, u8) = (random(), random(), random());
        Rgb::mix_pastel(Rgb { r, g, b })
    }
}

#[repr(C)]
pub struct Universe {
    width: u32,
    height: u32,
    /// a vector of size width*height, where cells[i]
    /// represents the index of the magnet in the magents vector, or -1 if indeterminate
    pendulums: Vec<Pendulum>,
    /// a vector of size > 1 where
    magnets: Vec<Magnet>,
    emitters: Vec<Emitter>,
    /// the max iterations for computing the magnet associated with a pendulum
    max_iters: u32,
    nums: Vec<f64>,
    steps: u32,
    delta: f64,
}

impl Universe {
    pub fn new(width: u32, height: u32, max_iters: u32, steps: u32) -> Universe {
        // create some magnets
        let magnets = vec![
            // Magnet::new(
            //     Vec2D::new(width as f64 / 3.0, height as f64 / 2.0),
            //     100.0,
            //     2.0,
            // ),
            // Magnet::new(
            //     Vec2D::new(width as f64 / 3.0 * 2.0, height as f64 / 2.0),
            //     100.0,
            //     2.0,
            // ),
            // Magnet::new(
            //     Vec2D::new(width as f64 / 2.0, height as f64 / 3.0),
            //     100.0,
            //     2.0,
            // ),
            // Magnet::new(
            //     Vec2D::new(width as f64 / 2.0, (height as f64) / 3.0 * 2.0),
            //     100.0,
            //     2.0,
            // ),
        ];
        let pendulums = vec![];

        Universe {
            width,
            height,
            pendulums,
            magnets,
            emitters: vec![],
            max_iters,
            nums: vec![3.14; 1],
            steps: steps,
            delta: INIT_DELTA,
        }
    }
    /// Computes the tension force induced on a pendulum
    fn compute_tension_force(pendulum: &Pendulum, width: u32, height: u32) -> Vec2D {
        Vec2D {
            x: pendulum.k * (width as f64 / 2.0 - pendulum.pos.x),
            y: pendulum.k * (height as f64 / 2.0 - pendulum.pos.y),
        }
    }
    /// Computes the magnetic force induced on a pendulum by a magnet
    fn compute_magnetic_force(magnet: &Magnet, pendulum: &Pendulum) -> Vec2D {
        let m_coeff: f64 = Vec2D::distance(magnet.pos, pendulum.pos);
        Vec2D {
            x: magnet.strength * (magnet.pos.x - pendulum.pos.x) / f64::powi(m_coeff, 2),
            y: magnet.strength * (magnet.pos.y - pendulum.pos.y) / f64::powi(m_coeff, 2),
        }
    }

    pub fn add_magnet(&mut self, magnet: Magnet) {
        self.magnets.push(magnet);
    }
    pub fn add_nums(&mut self, n: f64) {
        self.nums.push(n as f64)
    }

    pub fn create_magnet(&mut self, x: f64, y: f64, strength: f64, radius: f64) {
        self.magnets
            .push(Magnet::new(Vec2D::new(x, y), strength, radius));
    }

    pub fn create_magnet_with_rgb(
        &mut self,
        x: f64,
        y: f64,
        strength: f64,
        radius: f64,
        r: u8,
        g: u8,
        b: u8,
    ) {
        self.magnets.push(Magnet::new_with_rgb(
            Vec2D::new(x, y),
            strength,
            radius,
            r,
            g,
            b,
        ))
    }

    pub fn create_pendulum(&mut self, x: f64, y: f64, tension: f64, friction: f64, mass: f64) {
        self.pendulums
            .push(Pendulum::new(Vec2D::new(x, y), tension, friction, mass));
    }

    pub fn create_emitter(
        &mut self,
        x: f64,
        y: f64,
        interval: u32,
        tension: f64,
        friction: f64,
        mass: f64,
    ) {
        self.emitters
            .push(Emitter::new(x, y, interval, tension, friction, mass));
    }

    pub fn get_num(self, i: usize) -> f64 {
        self.nums[i]
    }

    pub fn clear_all(&mut self) {
        self.clear_magnets();
        self.clear_emitters();
        self.clear_pendulums();
    }

    pub fn clear_magnets(&mut self) {
        self.magnets.clear();
    }

    pub fn clear_and_spawn_random_magnets(&mut self, n: u32) {
        self.magnets.clear();
        
        for i in 0..n {
            let range_x = Uniform::from(0.0..self.width as f64);
            let range_y = Uniform::from(0.0..self.height as f64);
            let mut rng = rand::thread_rng();
            // TODO: Replace with constants
            self.create_magnet(range_x.sample(&mut rng), 
            range_y.sample(&mut rng), 
            0.1_f64, 2.0_f64)
        }
    }

    pub fn add_pendulum(&mut self, pendulum: Pendulum) {
        self.pendulums.push(pendulum);
    }

    pub fn clear_pendulums(&mut self) {
        self.pendulums.clear();
    }

    pub fn clear_emitters(&mut self) {
        self.emitters.clear();
    }

    pub fn spawn_random_emitters(&mut self, n: u32, tension: f64, friction: f64, mass: f64) {
        self.emitters.clear();
        let range_x = Uniform::from(0.0..self.width as f64);
        let range_y = Uniform::from(0.0..self.height as f64);
        let mut rng = rand::thread_rng();
        // TODO: Replace with constants
        let rand_interval = Uniform::<u32>::from(50..150);
        (0..n).into_iter().for_each(|_| {
            self.emitters.push(Emitter::new(
                range_x.sample(&mut rng),
                range_y.sample(&mut rng),
                rand_interval.sample(&mut rng),
                tension,
                friction,
                mass,
            ));
        })
    }

    pub fn tick(&mut self) {
        // Emit pendulums from the emitters, if any
        for emitter in self.emitters.iter_mut() {
            emitter.tick();
        }
        let mut emits = vec![];

        for emitter in self.emitters.iter() {
            if emitter.clock == 0 {
                emits.push((
                    emitter.pos.x,
                    emitter.pos.y,
                    emitter.tension,
                    emitter.friction,
                    emitter.mass,
                ));
            }
        }
        for emit in emits {
            self.create_pendulum(emit.0, emit.1, emit.2, emit.3, emit.4)
        }

        // Move the pendulums
        for pendulum in self.pendulums.iter_mut() {
            pendulum.tick(
                &self.magnets,
                self.width,
                self.height,
                self.steps,
                self.delta,
            );
        }

        // Delete any pendulums that are stationary
        self.pendulums.retain(|p| !p.is_stationary);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pendulums(&self) -> *const Pendulum {
        self.pendulums.as_ptr()
    }

    pub fn magnets(&self) -> *const Magnet {
        self.magnets.as_ptr()
    }

    pub fn nums(&self) -> *const f64 {
        self.nums.as_ptr()
    }

    pub fn pendulums_len(&self) -> u32 {
        self.pendulums.len() as u32
    }

    pub fn magnets_len(&self) -> u32 {
        self.magnets.len() as u32
    }

    // Serialize the magnets vec into bytes
    pub fn magnets_to_u8(&self) -> Vec<u8> {
        let ret: Vec<u8> = self.magnets.iter()
            .flat_map(|m|{
                let m_bytes = vec![
                    m.pos.x.to_be_bytes(),
                    m.pos.y.to_be_bytes(),
                    m.strength.to_be_bytes(),
                    m.radius.to_be_bytes(),
                    
                ];
                // Have to create a separate vec since f64 -> [u8;8] but u8 -> [u8;1]
                let rgb_bytes = vec![
                    m.color.r.to_be_bytes(),
                    m.color.g.to_be_bytes(),
                    m.color.b.to_be_bytes(),
                ];
                let mut ret: Vec<u8> = m_bytes.into_iter().flatten().collect();
                let ret_rgb : Vec<u8> = rgb_bytes.into_iter().flatten().collect();
                ret.extend(ret_rgb);
                ret
            }
            )
        .collect::<Vec<u8>>();
        ret
    }

    pub fn pendulums_to_u8(&self) -> Vec<u8> {
        let ret: Vec<u8> = self.pendulums.iter()
            .flat_map(|m|{
                let m_bytes = vec![
                    m.pos.x.to_be_bytes(),
                    m.pos.y.to_be_bytes(),
                    m.vel.x.to_be_bytes(),
                    m.vel.y.to_be_bytes(),
                    m.acc.x.to_be_bytes(),
                    m.acc.y.to_be_bytes(),
                    m.mass.to_be_bytes(),
                ];
                m_bytes.into_iter().flatten()
            }   
            )
        .collect::<Vec<u8>>();
        ret
    }

    pub fn set_steps(&mut self, new_steps: u32) {
        self.steps = new_steps;
    }

    pub fn set_delta(&mut self, new_delta: f64) {
        self.delta = new_delta;
    }
}

impl Universe {
    pub fn get_pendulums(&self) -> &[Pendulum] {
        &self.pendulums
    }

    pub fn get_magnets(&self) -> &[Magnet] {
        &self.magnets
    }

    pub fn get_width(&self) -> &u32 {
        &self.width
    }

    pub fn get_height(&self) -> &u32 {
        &self.height
    }
}

/*
pos: 16 bytes
strength: 8 bytes
color: 3 bytes
padding
total: 32 bytes
*/
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Magnet {
    strength: f64,
    pos: Vec2D,
    radius: f64,
    color: Rgb, // rgb
}

impl Magnet {
    pub fn new(pos: Vec2D, strength: f64, radius: f64) -> Magnet {
        Magnet {
            pos,
            strength,
            radius,
            color: Rgb::random_pastel(),
        }
    }

    pub fn new_with_rgb(pos: Vec2D, strength: f64, radius: f64, r: u8, g: u8, b: u8) -> Magnet {
        Magnet {
            pos,
            strength,
            radius,
            color: Rgb::new(r, g, b),
        }
    }

    pub fn size_of() -> u32 {
        mem::size_of::<Magnet>() as u32
    }
}

#[repr(C)]
pub struct Pendulum {
    pos: Vec2D,
    vel: Vec2D,
    acc: Vec2D,
    mass: f64,
    // tension force
    f_tension: Vec2D,
    // tension coefficient
    k: f64,
    // friction coefficient
    friction: f64,
    // magmetic force
    f_magnetic: Vec2D,
    // net force
    f_net: Vec2D,

    // The magnet that it attracts to at pos_start
    is_stationary: bool,
    magnet_color: Rgb,
}

impl Pendulum {
    pub fn new(pos: Vec2D, k: f64, friction: f64, mass: f64) -> Pendulum {
        Pendulum {
            pos,
            vel: Vec2D::zero(),
            acc: Vec2D::zero(),
            mass,
            f_tension: Vec2D::zero(),
            f_magnetic: Vec2D::zero(),
            f_net: Vec2D::zero(),
            k,
            friction,
            is_stationary: false,
            magnet_color: Rgb::black(),
        }
    }

    pub fn size_of() -> usize {
        mem::size_of::<Pendulum>()
    }

    pub fn pos(&self) -> Vec2D {
        self.pos
    }
    // Perform euler integration to determine the next position
    fn tick(&mut self, magnets: &Vec<Magnet>, width: u32, height: u32, steps: u32, delta: f64) {
        // If the pendulum is marked stationary, don't move
        if self.is_stationary {
            return;
        }

        // Update tension force (Hooke's law: F_t = kx where x is distance from center)
        self.f_tension = Universe::compute_tension_force(self, width, height);

        // Compute the net magnetic force exerted by all magnets
        self.f_magnetic = Vec2D { x: 0.0, y: 0.0 };

        for magnet in magnets.iter() {
            // Check if the pendulum is pulled into the magnet
            if Vec2D::distance(magnet.pos, self.pos) > magnet.radius {
                self.f_magnetic = self.f_magnetic + Universe::compute_magnetic_force(magnet, self);

                // Pendulum is pulled into magnet
            } else {
                self.pos = magnet.pos;
                self.is_stationary = true;
                self.vel = Vec2D::zero();
                self.acc = Vec2D::zero();
                // Record the magnet's colour
                self.magnet_color = magnet.color.clone();
                return;
            }
        }
        // Fnet = F_t + F_m
        self.f_net = self.f_tension + self.f_magnetic;

        // Calculate acceleration (Fnet = ma) ==> a = Fnet/m
        // Apply friction proportional to previous velocity
        self.acc = Vec2D {
            x: (self.f_net.x + self.f_magnetic.x - self.vel.x * self.friction) / self.mass,
            y: (self.f_net.y + self.f_magnetic.y - self.vel.y * self.friction) / self.mass,
        };

        // Euler integration
        for _ in 0..steps {
            self.vel = self.vel + self.acc * delta;
            self.pos = self.pos + self.vel * delta;
        }
    }
}

pub struct Emitter {
    pos: Vec2D,
    interval: u32,
    // if clock is zero, then the universe will spawn
    // a pendulum at that setting
    pub clock: u32,
    pub tension: f64,
    pub friction: f64,
    pub mass: f64,
}

impl Emitter {
    pub fn new(x: f64, y: f64, interval: u32, tension: f64, friction: f64, mass: f64) -> Emitter {
        Emitter {
            pos: Vec2D::new(x, y),
            interval,
            clock: 0,
            tension,
            friction,
            mass,
        }
    }
    pub fn tick(&mut self) {
        self.clock += 1;
        self.clock %= self.interval;
    }
}

pub struct FractalGenerator {
    image_width: usize,
    image_height: usize,
}

pub fn generate_fractal(
    image_width: usize,
    image_height: usize,
    universe: &Universe,
    k: f64,
    friction: f64,
    mass: f64,
) -> Vec<u8> {
    FractalGenerator::new(image_width, image_height).generate(universe, k, friction, mass)
}

pub fn generate_rand() -> Vec<u8> {
    (0..512 * 512)
        .into_par_iter()
        .map(|i: usize| {
            // console::log_1(&"hello".into());
            128
        })
        .collect::<Vec<u8>>()
}

impl FractalGenerator {
    pub fn new(image_width: usize, image_height: usize) -> FractalGenerator {
        // Create a copy universe with only the magnets

        FractalGenerator {
            image_width,
            image_height,
        }
    }

    pub fn get_width(&self) -> usize {
        self.image_width
    }

    pub fn get_height(&self) -> usize {
        self.image_height
    }

    // Generate a fractal image by placing a pendulum at each point of the canvas
    // And iterate the pendulum until it exceeds max_iters or reaches a magnet
    pub fn generate(&self, universe: &Universe, k: f64, friction: f64, mass: f64) -> Vec<u8> {
        // Create a hash map that stores the maximum number of iterations recorded for each magnet
        let mut max_iters_per_magnet = HashMap::<Rgb, u32>::new();

        // console::log_1(&"Hello using web-sys".into());
        // Create an auxiliary vector that stores the number of iterations that was taken for each pendulum
        let mut pendulum_iters = vec![0; self.image_width * self.image_height];

        // // First pass: Loop through the canvas and run a pendulum, and store info
        FractalGenerator::iter_all(
            self.image_width as u32,
            self.image_height as u32,
            universe,
            k,
            friction,
            mass,
        )
        .collect::<Vec<u8>>()
        .clone()

        // console_error_panic_hook::set_once();
        // (0..self.image_width*self.image_height).into_par_iter().map(|i: usize|{
        //     127
        // }).collect::<Vec<u8>>()
    }
}

impl FractalGenerator {
    // iter_row iterates through a row of self.image_data and computes `get_iterations` for each (x,y)
    pub fn iter_row<'a>(
        image_width: u32,
        image_height: u32,
        i: u32,
        universe: &'a Universe,
        k: f64,
        friction: f64,
        mass: f64,
    ) -> impl Iterator<Item = (u32, Rgb)> + '_ {
        let res = (0..(image_width)).map(move |j| {
            FractalGenerator::get_iterations(
                image_width,
                image_height,
                i as u32,
                j as u32,
                universe,
                k,
                friction,
                mass,
            )
        });

        res
    }
    // Single-threaded implementation
    #[cfg(feature = "rayon")]
    pub fn iter_all<'a>(
        image_width: u32,
        image_height: u32,
        universe: &'a Universe,
        k: f64,
        friction: f64,
        mass: f64,
    ) -> impl ParallelIterator<Item = u8> + '_ {
        let mut max_iters_map: HashMap<Rgb, u32> = HashMap::new();

        let iters_colors: Vec<(u32, Rgb)> = (0..image_height)
            .into_par_iter()
            .flat_map_iter(move |i| {
                FractalGenerator::iter_row(
                    image_width,
                    image_height,
                    i as u32,
                    universe,
                    k,
                    friction,
                    mass,
                )
            })
            .collect();

        // // Method 1: Get max iters per magnet
        // iters_colors.iter().for_each(|(iters, color)| {
        //     if max_iters_map.contains_key(color) {
        //         let old_val = *(max_iters_map.get(color).unwrap());
        //         max_iters_map.insert(*color, u32::max(*iters, old_val));
        //     } else {
        //         max_iters_map.insert(*color, *iters);
        //     }
        // });

        // Method 2: Get max iters overall (faster)
        let max_iters: u32 = iters_colors
            .par_iter()
            .cloned()
            .reduce_with(|a, b| (u32::max(a.0, b.0), a.1))
            .unwrap()
            .0;

        iters_colors
            .into_par_iter()
            .flat_map(move |(iters, color)| {
                // let max_iters = *(max_iters_map.get(&color).unwrap());
                // let max_iters = 10;
                vec![
                    (color.r as f64 * (1.0 - (iters as f64) / (max_iters as f64))) as u8,
                    (color.g as f64 * (1.0 - (iters as f64) / (max_iters as f64))) as u8,
                    (color.b as f64 * (1.0 - (iters as f64) / (max_iters as f64))) as u8,
                    255,
                ]
            })
    }

    pub fn get_iterations(
        image_width: u32,
        image_height: u32,
        i: u32,
        j: u32,
        universe: &Universe,
        k: f64,
        friction: f64,
        mass: f64,
    ) -> (u32, Rgb) {
        let pendulum_pos = Vec2D::convert_coords(
            Vec2D::new(j as f64, i as f64),
            universe.width,
            universe.height,
            image_width as u32,
            image_height as u32,
        );
        // Create a pendulum
        let mut p = Pendulum::new(pendulum_pos, k, friction, mass);
        // Loop until max iters
        let mut iter = 0;
        while iter < universe.max_iters {
            p.tick(
                &universe.magnets,
                universe.width,
                universe.height,
                universe.steps,
                universe.delta,
            );
            // If p reached a magnet, do early break
            if p.is_stationary {
                break;
            }
            iter += 1;
        }
        let mut rgb = p.magnet_color.clone();
        // // If pendulum did not attract to a magnet, assign it a default colour
        if !p.is_stationary {
            rgb = Rgb::black();
        }

        (iter, rgb)
    }
}
