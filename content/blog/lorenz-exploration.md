+++
title = "Nonlinear Dynamics: Numerical Experiments with Rust and Wasm"
date = 2026-01-03
description = "A real-time simulation of the Lorenz Attractor using a 4th-order Runge-Kutta integrator compiled to WebAssembly."
[taxonomies]
tags = ["Physics", "Rust", "Dynamics"]
+++

## The Lorenz System

The Lorenz system models atmospheric convection through three coupled nonlinear ODEs:

$$
\begin{aligned}
\frac{dx}{dt} &= \sigma(y - x) \\
\frac{dy}{dt} &= x(\rho - z) - y \\
\frac{dz}{dt} &= xy - \beta z
\end{aligned}
$$

With standard parameters $\sigma=10, \rho=28, \beta=8/3$, the system is chaotic and converges to a strange attractor.

---

## Simulation

This uses Rust compiled to WebAssembly. The integration runs in your browser at near-native speed.

<div style="text-align: center; margin: 2rem 0;">
    <canvas id="lorenz-canvas" width="700" height="500" style="background: #fafafa; border: 1px solid #ddd; cursor: crosshair;"></canvas>
    <p style="font-size: 0.85rem; color: #555; font-family: 'Iosevka', monospace;">X-Z phase space projection</p>
</div>

<script type="module">
    import init, { LorenzState } from '/wasm/wasm_compute.js';

    async function run() {
        await init();
        
        const canvas = document.getElementById('lorenz-canvas');
        const ctx = canvas.getContext('2d');
        const state = new LorenzState(0.1, 0.0, 0.0);
        
        ctx.strokeStyle = 'rgba(20, 20, 20, 0.7)';
        ctx.lineWidth = 0.6;
        
        let lastX = 350 + state.x() * 10;
        let lastZ = 450 - state.z() * 10;

        function render() {
            for(let i = 0; i < 8; i++) { 
                state.next(0.005);
                const nextX = 350 + state.x() * 10;
                const nextZ = 450 - state.z() * 10;

                ctx.beginPath();
                ctx.moveTo(lastX, lastZ);
                ctx.lineTo(nextX, nextZ);
                ctx.stroke();

                lastX = nextX;
                lastZ = nextZ;
            }
            requestAnimationFrame(render);
        }
        
        render();
    }

    run().catch(console.error);
</script>

---

## Notes

The numerics are in Rust, compiled to `wasm32-unknown-unknown`. This is my first test using Wasm in a blog post.