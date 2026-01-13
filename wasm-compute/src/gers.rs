use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GersState {
    h: [f64; 2],
    t: f64,
}

#[wasm_bindgen]
impl GersState {
    #[wasm_bindgen(constructor)]
    pub fn new(h1: f64, h2: f64) -> Self {
        Self {
            h: [h1, h2],
            t: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.t += dt;
        
        // 这里的 alpha 和 beta 可以是随时间变化的干预强度
        let alpha = self.t.sin();
        let beta = (self.t * 0.5).cos() * 0.1;

        // 简化的 2D 指数映射实现
        let s = beta.exp();
        let cos_a = alpha.cos();
        let sin_a = alpha.sin();

        // 计算群作用矩阵
        let m11 = s * cos_a;
        let m12 = -s * sin_a;
        let m21 = s * sin_a;
        let m22 = s * cos_a;

        // 应用公式 h' = h * exp(...)
        let new_h1 = self.h[0] * m11 + self.h[1] * m21;
        let new_h2 = self.h[0] * m12 + self.h[1] * m22;

        self.h = [new_h1, new_h2];
    }

    pub fn h1(&self) -> f64 { self.h[0] }
    pub fn h2(&self) -> f64 { self.h[1] }
}