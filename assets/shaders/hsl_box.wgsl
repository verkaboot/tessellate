
#import bevy_ui::ui_vertex_output::UiVertexOutput

const TAU:f32 =  6.28318530718;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    let white = vec3<f32>(1.0);
    let black = vec3<f32>(0.0);
    let hue = vec3<f32>(0.0, 0.0, 1.0);
    let color = mix(white, hue, mesh.uv.x) * mix(white, black, mesh.uv.y);

    return to_linear(vec4<f32>(color, 1.0));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
