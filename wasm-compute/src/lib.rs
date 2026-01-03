use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct LorenzState {
    x: f64,
    y: f64,
    z: f64,
}

#[wasm_bindgen]
impl LorenzState {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn next(&mut self, dt: f64) {
        let sigma = 10.0;
        let rho = 28.0;
        let beta = 8.0 / 3.0;

        let dx = sigma * (self.y - self.x);
        let dy = self.x * (rho - self.z) - self.y;
        let dz = self.x * self.y - beta * self.z;

        self.x += dx * dt;
        self.y += dy * dt;
        self.z += dz * dt;
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }
}