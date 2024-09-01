@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform>  mouse_position: vec2<f32>;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let mouse = vec2<i32>(i32(mouse_position.x), i32(mouse_position.y));
    let current_color: vec4<f32> = textureLoad(input, location);
    var blended_color = current_color;

    if location.x == mouse.x && location.y == mouse.y {
        let new_color = vec4<f32>(1.0, 0.21, 0.3, 0.2);
        blended_color = vec4<f32>(
            new_color.rgb * new_color.a + current_color.rgb * (1.0 - new_color.a),
            new_color.a + current_color.a * (1.0 - new_color.a)
        );
    }

    textureStore(output, location, blended_color);
}
