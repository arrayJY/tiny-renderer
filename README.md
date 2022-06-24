# Tiny Renderer   ![](https://img.shields.io/static/v1?label=Linux&message=passing&color=success) ![](https://img.shields.io/static/v1?label=MacOS&message=passing&color=success) ![](https://img.shields.io/static/v1?label=Windows&message=passing&color=success)
A toy software renderer for learning Computer Graphics written in Rust.

## Features
- Homogeneous clipping
- Back-face culling
- Perspective correct interpolation
- Depth testing
- Phong shading
- Orbital camera controls
- Physicallly-Based-Rendering(Cook-Torrance)

## Usage
Tiny renderer is available in **Windows, Linux and MacOSX** now.

### Launch
You can run with `cargo`.
```
cargo run --release [shader] [path]
cargo run --release pbr [path] [spp]
```
`shader` is the name of fragment shader.

Available shaders:
- `z`:  Depth shader
- `color`: Color shader.
- `texture`: Texture mapping shader.
- `phong-color`: Color shader with Phong shading.
- `phong-texture`: Texture shader with Phong shading.

`path` is the the path **without extension** to `.gltf/.obj/.mtl` and texture image(`.jpg/.png`) file. 
It means they should be in same directory. PBR will try to load `.gltf` and others will try to load `.obj`.

For example, you can run just like:
```
cargo run --release color static/cube
```
It will run the renderer with `static/cube.obj`.

There are some simple models in `static` directory.
- `cube`
- `cone`
- `multi_models`
- `earth`
- `earth_good`
- `spot`
- `cornell-box`

Don't forget `--release` flag because of its poor performance.

### Controls
- Rotate camera: `W/S/A/D`
- Rotate light: `I/K/J/L`
- Zoom camera: `↑/↓`

## Screenshots

|                          screenshot                          |                    command                    |
| :----------------------------------------------------------: | :-------------------------------------------: |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_earth.gif) |   `cargo run --release texture static/earth`    |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_cube.gif) |       `cargo run --release z static/cube`       |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_spot.gif) | `cargo run --release phong-texture static/spot` |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_path_tracing1.png) | `cargo run --release pbr static/cornell-box 512` |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_path_tracing2.png) | `cargo run --release pbr static/silver-mirror 512` |
| ![](https://raw.githubusercontent.com/arrayJY/tiny-renderer/master/static/screenshot_path_tracing3.png) | `cargo run --release pbr static/bunny 512` |


## License
[MIT](https://github.com/arrayJY/tiny-renderer/blob/master/LICENSE)
