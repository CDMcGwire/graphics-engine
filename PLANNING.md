# Components Needed

* Logging system
* Window Initialization
  * winit https://crates.io/crates/winit
* Maths
  * nalgebra https://crates.io/crates/glam
  * glam https://crates.io/crates/nalgebra
  * palette https://crates.io/crates/palette
* Renderer
  * Vulkan API
    * Ash; Vulkano; WebGPU?
  * DirectX 12 API
  * ? kajiya GI (experimental)
  * Shader Precomp
  * Vector Graphics
    * femtovg https://crates.io/crates/femtovg
    * lyon https://github.com/nical/lyon
  * Text Rendering
    * ab-glyph https://crates.io/crates/ab_glyph
  * IMGUI
    * egui library maybe
  * Game UI - Iced https://crates.io/crates/iced
  * Images
    * image https://crates.io/crates/image
* 3D Coordinate System
* Physics
  * Rapier phyiscs https://crates.io/crates/rapier2d https://crates.io/crates/rapier3d
* Animation
  * in-betweening functions
    * pareen https://crates.io/crates/pareen
  * Keyframing https://crates.io/crates/keyframe
* Audio
  * ? cpal https://crates.io/crates/cpal
  * ? kira https://crates.io/crates/kira
* Asset Loader
  * gltf https://crates.io/crates/gltf
  * tobj https://crates.io/crates/tobj
* Input
  * gilrs https://crates.io/crates/gilrs
* ECS
* Event Queue
* Worker Queue Multithreading
* Memory Pooling
  * ? blink-alloc arena-allocator https://github.com/zakarumych/blink-alloc
* Configuration Management
  * ? Cvars (Configuration variables) https://crates.io/crates/cvars
* Networking
  * Quinn QUIC messaging https://crates.io/crates/quinn
* Other Stuff
  * Noise Generator https://crates.io/crates/noise