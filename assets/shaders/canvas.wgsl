@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var<storage> mouse_positions: array<vec2<f32>, 4>;

const brush_radius: f32 = 4.0;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let current_color: vec4<f32> = textureLoad(input, location);
    var blended_color = current_color;

    // Calculate the vector from the previous mouse position to the current mouse position
    let line_vector = mouse_positions[0] - mouse_positions[1];
    let line_length = max(length(line_vector), 0.0001);
    let line_direction = line_vector / line_length;

    // Calculate the vector from the previous mouse position to the current texture location
    let location_vector = vec2<f32>(f32(location.x), f32(location.y)) - mouse_positions[1];

    // Project the location vector onto the line direction to find the nearest point on the line segment
    let projection_length = dot(location_vector, line_direction);
    let clamped_projection_length = clamp(projection_length, 0.0, line_length);
    let nearest_point = mouse_positions[1] + line_direction * clamped_projection_length;

    // Calculate the distance from the current texture location to the nearest point on the line segment
    let distance = length(vec2<f32>(f32(location.x), f32(location.y)) - nearest_point);

    let alpha = brush_radius - distance;

    // Apply the brush color based on the alpha value
    if alpha > 0.0 {
        let new_color = vec4<f32>(0.0, 0.0, 0.0, alpha);
        blended_color = vec4<f32>(
            new_color.rgb * new_color.a + current_color.rgb * (1.0 - new_color.a),
            new_color.a + current_color.a * (1.0 - new_color.a)
        );
    }

    textureStore(input, location, blended_color);
}
