# Tiny Renderer
A toy software renderer for learning Computer Graphics written in Rust.

## Usage
Tiny renderer is only available in **Windows** now.

### Launch
You can run with `cargo`.
```
cargo run --release [shader] [path]
```
`shader` is the name of fragment shader.

Available shaders:
- `z`:  Depth shader
- `color`: Color shader.
- `texture`: Texture mapping shader.
- `phong-color`: Color shader with Phong shading.
- `phong-texture`: Texture shader with Phong shading.

`path` is the the path **without extension** to `.obj/.mtl` and texture image(`.jpg/.png`) file. 
It means they should be in same directory.

For example, you can run just like:
```
cargo run --release color static/cube
```
It will run the renderer with `static/cube.obj` and `static/cube.mtl`.

There are some simple models in `static` directory.
- `cube`
- `cone`
- `multi_models`
- `earth`
- `earth_good`
- `spot`

Don't forget `--release` flag because of its poor performance.

### Controls
- Rotate camera: `W/S/A/D`
- Zoom camera: `↑/↓`

## License
[MIT](https://github.com/arrayJY/tiny-renderer/blob/master/LICENSE)
