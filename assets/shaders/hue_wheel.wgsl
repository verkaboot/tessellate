#import bevy_ui::ui_vertex_output::UiVertexOutput

const PI:f32 =  3.14159265358;
const TAU:f32 =  6.28318530718;
const inner_radius = 0.80;
const outer_radius = 0.98;
const edge_smoothness = 0.01;
const outline_thickness = 0.01;

struct Angle {
    percent: f32,
    radius: f32
}

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    let angle = coords_to_angle(mesh.uv);
    var color = rainbow_gradient(angle);
    color = hue_indicator(angle, 4.0, color);
    color = outer_outline(angle, color, vec3<f32>(0.25) * mesh.uv.y + 0.25);
    color = inner_outline(angle, color, vec3<f32>(0.50) * (1 - mesh.uv.y) + 0.15);
    let alpha = outline_alpha(angle);

    return to_linear(vec4<f32>(color, alpha));
}

fn coords_to_angle(input_uv: vec2<f32>) -> Angle {
    // Convert UV coordinates to range [-1, 1]
    let uv = input_uv * 2.0 - vec2<f32>(1.0);

    // Convert to polar coordinates
    let angle = atan2(-uv.y, uv.x);
    let radius = length(uv);

    // Normalize angle to [0, 1]
    return Angle(angle / TAU + 0.5, radius);
}

fn rainbow_gradient(angle: Angle) -> vec3<f32> {
    return 0.5 + cos(TAU * (angle.percent + vec3<f32>(0.00, 0.33, 0.67)));
}

fn hue_indicator(angle: Angle, hue: f32, color: vec3<f32>) -> vec3<f32> {
    let hue_indicator_size = 0.02;
    let hue_indicator_smoothness = 0.003;
    var pos = abs((hue / 360.0) - (1 - angle.percent));

    let mask = 1.0 - (step(pos, hue_indicator_size) + (1 - step(pos, 1.0 - hue_indicator_size)));
    return mix(color, vec3(0.0), mask * 0.5);
}

fn inner_outline(angle: Angle, color: vec3<f32>, outline_color: vec3<f32>) -> vec3<f32> {
    let outline = smoothstep(inner_radius, inner_radius + edge_smoothness, angle.radius * (1 - outline_thickness));

    return  mix(color, outline_color, 1 - outline);
}

fn outer_outline(angle: Angle, color: vec3<f32>, outline_color: vec3<f32>) -> vec3<f32> {
    let outline = smoothstep(outer_radius, outer_radius - edge_smoothness, angle.radius * (1 + outline_thickness));

    return mix(color, outline_color, 1 - outline);
}

fn outline_alpha(angle: Angle) -> f32 {
    return smoothstep(
        outer_radius, outer_radius - edge_smoothness, angle.radius
    ) * (smoothstep(
        inner_radius, inner_radius + edge_smoothness, angle.radius
    ));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
