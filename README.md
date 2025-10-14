# FerrousGL

<a href="">Documentation</a><br>
<a href="#current-features">Features</a><br>
<a href="">Website</a>

**FerrousGL** is a high performance and simple-to-use Rust library for OpenGL rendering. It offers straightforward ways to handle a window, rendering, shaders, textures and meshes. This makes the development fast and clean.

It aims to support all commonly know OpenGL functionalities, such as the previously mentioned as well as more complicated things like custom render targets, other shader types, different buffer types and instanced rendering.

To use FerrousGL in your projects, just add it into your `Cargo.toml` like this:

```toml
[dependencies]
ferrousgl = "0.1.0"
```

## Examples

<img src="examples/screenshots/instanced_grass.png" alt="Instanced Grass Example" width="300">

## Current Features
- Nice and easy to implement Instanced Rendering
- Fragment, Vertex, Compute, Geometry (untested) Shaders
- Window with configuration and selectable OpenGL versions
- Mesh creation with a mesh config for simple meshes and support for primitives
- More on the way