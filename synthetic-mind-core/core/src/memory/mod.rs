```rust
/// Онтологическая модель памяти: сфера и тор.
#[derive(Clone, Copy, Debug)]
pub enum MemoryTopology {
    Sphere(f32),  // объём памяти
    Torus(f32, f32), // toroidal: (радиус, толщина)
}

impl MemoryTopology {
    pub const fn sphere(radius: f32) -> Self {
        Self::Sphere(radius)
    }

    pub const fn torus(major: f32, minor: f32) -> Self {
        Self::Torus(major, minor)
    }
}
```