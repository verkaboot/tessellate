@group(0) @binding(0) var input: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var<storage> mouse_positions: array<vec2<f32>, 4>;

const brush_radius: f32 = 12.0;
const brush_color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));

    let alpha = brush_alpha(location, mouse_positions);

    if alpha >= 0.0 {
        let bg: vec4<f32> = textureLoad(input, location);
        var fg = vec4<f32>(brush_color, alpha);
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

fn catmull_rom(p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, p3: vec2<f32>, t: f32) -> vec2<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    return 0.5 * (
        (2.0 * p1) +
        (-p0 + p2) * t +
        (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2 +
        (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3
    );
}

fn brush_alpha(
    location: vec2<i32>,
    mouse_positions: array<vec2<f32>, 4>
) -> f32 {
    let loc = vec2<f32>(f32(location.x), f32(location.y));
    var min_distance = f32(brush_radius);

    let count = 100u;
    for (var i = 0u; i < count; i = i + 1u) {
        let t = f32(i) / f32(count);
        let spline_point = catmull_rom(mouse_positions[0], mouse_positions[1], mouse_positions[2], mouse_positions[3], t);
        let distance = length(loc - spline_point);
        min_distance = min(min_distance, distance);
    }

    let alpha = (brush_radius - min_distance);
    return smoothstep(0.0, 1.0, alpha);
}
