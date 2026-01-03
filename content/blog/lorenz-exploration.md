+++
title = "Nonlinear Dynamics: Numerical Experiments with Rust and Wasm"
date = 2026-01-03
description = "A real-time simulation of the Lorenz Attractor using a 4th-order Runge-Kutta integrator compiled to WebAssembly."
[taxonomies]
tags = ["Physics", "Rust", "Dynamics"]
+++

### Theoretical Framework

The Lorenz system is a simplified mathematical model for atmospheric convection, described by a set of three coupled, nonlinear ordinary differential equations:

$$
\begin{aligned}
\frac{dx}{dt} &= \sigma(y - x) \\
\frac{dy}{dt} &= x(\rho - z) - y \\
\frac{dz}{dt} &= xy - \beta z
\end{aligned}
$$

For the standard parameters $\sigma=10, \rho=28, \text{ and } \beta=8/3$, the system exhibits chaotic behavior, evolving towards a strange attractor with a fractal dimension.

---

### Real-time Numerical Simulation

The visualization below is rendered using a **Rust WebAssembly** engine. The trajectory is computed on-the-fly in your browser's memory, ensuring near-native performance for the iterative numerical integration.

<div style="text-align: center; margin: 2rem 0;">
    <canvas id="lorenz-canvas" width="700" height="500" style="background: #fafafa; border: 1px solid #ddd; cursor: crosshair;"></canvas>
    <p style="font-size: 0.85rem; color: #555; font-family: 'Iosevka', monospace;">Fig 1. Phase space projection on the X-Z plane (Computed via Rust/Wasm)</p>
</div>

<script type="module">
    import init, { LorenzState } from '/wasm/wasm_compute.js';

    async function run() {
        await init();
        
        const canvas = document.getElementById('lorenz-canvas');
        const ctx = canvas.getContext('2d');
        const state = new LorenzState(0.1, 0.0, 0.0);
        
        // Aesthetic configuration: Minimalist laboratory style
        ctx.strokeStyle = 'rgba(20, 20, 20, 0.7)';
        ctx.lineWidth = 0.6;
        
        let lastX = 350 + state.x() * 10;
        let lastZ = 450 - state.z() * 10;

        function render() {
            // Compute multiple steps per frame for smooth trajectory
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

### Implementation Notes

To maintain a high level of technical rigor, this blog post avoids heavy JavaScript frameworks. The computational core is written in **Rust**, utilizing its strict type system and memory safety to handle the floating-point sensitive integration. The resulting binary is compiled to `wasm32-unknown-unknown`, providing a deterministic execution environment that bridges the gap between high-level research and web-based dissemination.