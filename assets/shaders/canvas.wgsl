@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read>;
@group(0) @binding(1) var output: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var<uniform> mouse_position: vec2<f32>;
@group(0) @binding(3) var<uniform> previous_mouse_position: vec2<f32>;

const brush_radius: f32 = 8.0;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let lerp = mix(previous_mouse_position, mouse_position, 0.5);
    let mouse = vec2<i32>(i32(lerp.x), i32(lerp.y));
    let current_color: vec4<f32> = textureLoad(input, location);
    var blended_color = current_color;

    // Calculate the offset from the mouse position
    let offset = vec2<f32>(f32(location.x) - mouse_position.x, f32(location.y) - mouse_position.y);
    let distance = length(offset);

    let alpha = brush_radius - distance;

    // Apply the brush color based on the alpha value
    if alpha > 0.0 {
        let new_color = vec4<f32>(0.0, 0.0, 0.0, alpha);
        blended_color = vec4<f32>(
            new_color.rgb * new_color.a + current_color.rgb * (1.0 - new_color.a),
            new_color.a + current_color.a * (1.0 - new_color.a)
        );
    }

    textureStore(output, location, blended_color);
}
