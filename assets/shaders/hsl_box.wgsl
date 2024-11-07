
#import bevy_ui::ui_vertex_output::UiVertexOutput

const TAU:f32 =  6.28318530718;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    var alpha = 0.0;
    var color = vec3<f32>(0.0);

    let square_margin = 0.0;
    let is_in_square = mesh.uv.x > square_margin && mesh.uv.x < (1 - square_margin) && mesh.uv.y > square_margin && mesh.uv.y < (1 - square_margin);
    if is_in_square {
        alpha = 1.0;
    }

    return to_linear(vec4<f32>(color, alpha));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
