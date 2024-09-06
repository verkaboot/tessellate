@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var<storage> mouse_positions: array<vec2<f32>, 4>;

const brush_radius: f32 = 50.0;

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let alpha = brush_alpha(location, mouse_positions);

    // Apply the brush color based on the alpha value
    if alpha >= 0.0 {
        let bg: vec4<f32> = textureLoad(input, location);
        var fg = vec4<f32>(0.3, 0.0, 0.8, alpha);
        let blend = blend_normal(bg, fg);
        textureStore(input, location, blend);
    }
}

fn blend_normal(bg: vec4<f32>, fg: vec4<f32>) -> vec4<f32> {
    let alpha = fg.a + bg.a * (1 - fg.a);
    return vec4<f32>(
        ((fg.rgb * fg.a) + (bg.rgb * bg.a * (1 - fg.a))) / alpha,
        alpha
    );
}

fn blend_erase(bg: vec4<f32>, fg: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        bg.rgb,
        clamp(bg.a - fg.a, 0.0, 1.0)
    );
}

fn brush_alpha(
    location: vec2<i32>,
    mouse_positions: array<vec2<f32>, 4>
) -> f32 {
    
    // Calculate the vector from the previous mouse position to the current mouse position
    let line_vector = mouse_positions[0] - mouse_positions[1];

    // Get line length with a minimum value to make sure draw happens even on zero length.
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

    return (brush_radius - (distance)) / brush_radius;
}
