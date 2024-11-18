#import bevy_ui::ui_vertex_output::UiVertexOutput

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    let angle = uv_to_angle(mesh.uv);

    // Create the rainbow color based on the angle
    var color = 0.5 + cos(TAU * (angle.percent + vec3<f32>(0.00, 0.33, 0.67)));

    color = hue_indicator(angle.percent, color);

    // Define the inner and outer radius for the ring
    let inner_radius = 0.80;
    let outer_radius = 0.98;
    let edge_smoothness = 0.01;
    let outline_thickness = 0.01;
    let outer_outline_color = vec3<f32>(0.25) * mesh.uv.y + 0.25;
    let inner_outline_color = vec3<f32>(0.50) * (1 - mesh.uv.y) + 0.15;

    // Set outline influence
    let outer_outline = smoothstep(outer_radius, outer_radius - edge_smoothness, angle.radius * (1 + outline_thickness));

    let inner_outline = smoothstep(inner_radius, inner_radius + edge_smoothness, angle.radius * (1 - outline_thickness));

    // Mix in Outline Color
    color = mix(color, outer_outline_color, 1 - outer_outline);
    color = mix(color, inner_outline_color, 1 - inner_outline);

    // Apply alpha based on radius to create a ring with anti-aliased edges
    var alpha = smoothstep(
        outer_radius, outer_radius - edge_smoothness, angle.radius
    ) * (smoothstep(
        inner_radius, inner_radius + edge_smoothness, angle.radius
    ));

    return to_linear(vec4<f32>(color, alpha));
}

const PI:f32 =  3.14159265358;
const TAU:f32 =  6.28318530718;

struct Angle {
    percent: f32,
    radius: f32
}

fn uv_to_angle(input_uv: vec2<f32>) -> Angle {
    // Convert UV coordinates to range [-1, 1]
    let uv = input_uv * 2.0 - vec2<f32>(1.0);

    // Convert to polar coordinates
    let angle = atan2(-uv.y, uv.x);
    let radius = length(uv);

    // Normalize angle to [0, 1]
    return Angle (angle / TAU + 0.5, radius);
}

fn hue_indicator(angle: f32, color: vec3<f32>) -> vec3<f32> {
    let hue_indicator_size = 0.02;
    let hue_indicator_smoothness = 0.003;
    let hue = 90.0;
    let x = abs((hue / 360.0) - (1 - angle));
    // if x > hue_indicator_size && x < (1 - hue_indicator_size) {
        // color = mix(color, vec3(0.0), step(x, 0.025));
    // }

    let ci_black = smoothstep(x, x + hue_indicator_smoothness, hue_indicator_size) - smoothstep(x, x + hue_indicator_smoothness, hue_indicator_size - 0.005);
    return mix(color, vec3(0.0), ci_black);
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
