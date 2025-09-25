# 🌄 Raytracer in Rust

A modular, CPU-based raytracer written in [Rust](https://www.rust-lang.org/). This project simulates how rays of light interact with 3D objects and materials to render a simple scene from scratch.


---

## Features

-  Custom camera system (position, direction, orientation vectors)
-  Basic light source with color and position
-  Hittable trait for generic objects like spheres and planes
-  Surface normal handling with front/back face detection
-  Material system using trait-based design
-  Written with math libraries (`nalgebra`, `palette`, `glam`) for numerical stability

---

##  Folder Structure

raytracer-rust/
│
├── camera.rs # Defines the Camera struct and orientation logic
├── hittable.rs # Defines the Hittable trait and HitRecord struct
├── light.rs # Light struct with color and position, implements Source trait
├── src/ # Main entry point, ray, material, object, and rendering logic
├── LICENSE # MIT License
