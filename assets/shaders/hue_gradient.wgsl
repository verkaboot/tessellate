#import bevy_ui::ui_vertex_output::UiVertexOutput

const TAU:f32 =  6.28318530718;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    let a = vec3<f32>(0.5);
    let b = vec3<f32>(0.5);
    let c = vec3<f32>(1.0);
    let d = vec3<f32>(0.0, 0.33, 0.67);

    let color = a + b * cos(TAU * (c * -mesh.uv.y + d));

    return to_linear(vec4<f32>(color, 1.0));
}


fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
