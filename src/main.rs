mod algebra;
mod pipeline;
mod window;

#[cfg(test)]
mod test;

use window::Window;
fn main() {
    Window::new().run()

}
