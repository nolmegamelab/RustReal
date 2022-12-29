mod learn_vulkan;
mod learn_wgpu;

fn main() {
    pollster::block_on(learn_wgpu::run());
}
