@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform>  mouse_position: vec2<f32>;

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    textureStore(output, location, color);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let mouse = vec2<i32>(i32(mouse_position.x), i32(mouse_position.y));
    let current_color: vec4<f32> = textureLoad(input, location);

    if location.x == mouse.x && location.y == mouse.y {
        let color = vec4<f32>(1.0, 1.0, 1.0, 1.0) + current_color;
        textureStore(output, location, color);
    }
}
