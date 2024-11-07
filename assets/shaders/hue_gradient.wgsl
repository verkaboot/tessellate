#import bevy_ui::ui_vertex_output::UiVertexOutput

const TAU:f32 =  6.28318530718;

// @fragment
// fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
//     // Convert UV coordinates to range [-1, 1]
//     let uv = mesh.uv * 2.0 - vec2<f32>(1.0);

//     // Convert to polar coordinates
//     let angle = atan2(uv.y, uv.x);
//     let radius = length(uv);

//     // Normalize angle to [0, 1]
//     let normalized_angle = angle / TAU + 0.5;

//     // Create the rainbow color based on the angle
//     let color = 0.5 + 0.5 * cos(TAU * (normalized_angle + vec3<f32>(0.0, 0.33, 0.67)));

//     // Apply alpha based on radius to create a circular gradient
//     let alpha = step(radius, 1.0);

//     return to_linear(vec4<f32>(color, alpha));
// }

// fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
//     let cutoff = step(nonlinear, vec4<f32>(0.04045));
//     let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
//     let lower = nonlinear / vec4<f32>(12.92);
//     return mix(higher, lower, cutoff);
// }

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
    let color = 0.5 + 0.5 * cos(TAU * (normalized_angle + vec3<f32>(0.0, 0.33, 0.67)));

    // Define the inner and outer radius for the ring
    let inner_radius = 0.8;
    let outer_radius = 1.0;

    // Apply alpha based on radius to create a ring
    var alpha: f32;
    if (radius < outer_radius && radius > inner_radius) {
        alpha = 1.0;
    } else {
        alpha = 0.0;
    };

    return to_linear(vec4<f32>(color, alpha));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
