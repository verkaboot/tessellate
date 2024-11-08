#import bevy_ui::ui_vertex_output::UiVertexOutput

const TAU:f32 =  6.28318530718;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    // Convert UV coordinates to range [-1, 1]
    let uv = mesh.uv * 2.0 - vec2<f32>(1.0);

    // Convert to polar coordinates
    let angle = atan2(-uv.y, uv.x);
    let radius = length(uv);

    // Normalize angle to [0, 1]
    let normalized_angle = angle / TAU + 0.5;

    // Create the rainbow color based on the angle
    var color = 0.5 + 0.5 * cos(TAU * (normalized_angle + vec3<f32>(0.0, 0.33, 0.67)));

    // Define the inner and outer radius for the ring
    let inner_radius = 0.80;
    let outer_radius = 0.98;
    let edge_smoothness = 0.01;
    let outline_thickness = 0.01;
    let outer_outline_color = vec3<f32>(0.25) * mesh.uv.y + 0.25;
    let inner_outline_color = vec3<f32>(0.50) * (1 - mesh.uv.y) + 0.15;

    // Set outline influence
    let outer_outline = smoothstep(outer_radius, outer_radius - edge_smoothness, radius * (1 + outline_thickness));

    let inner_outline = smoothstep(inner_radius, inner_radius + edge_smoothness, radius * (1 - outline_thickness));

    // Mix in Outline Color
    color = mix(color, outer_outline_color, 1 - outer_outline);
    color = mix(color, inner_outline_color, 1 - inner_outline);

    // Apply alpha based on radius to create a ring with anti-aliased edges
    var alpha = smoothstep(
        outer_radius, outer_radius - edge_smoothness, radius
    ) * (smoothstep(
        inner_radius, inner_radius + edge_smoothness, radius
    ));

    return to_linear(vec4<f32>(color, alpha));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
