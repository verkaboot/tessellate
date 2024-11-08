#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> hsv: vec3<f32>;

@fragment
fn fragment(mesh: UiVertexOutput) -> @location(0) vec4<f32> {
    let white = vec3<f32>(1.0);
    let black = vec3<f32>(0.0);
    let hue = vec3<f32>(1.0, 0.0, 1.0);
    let border_size = 0.009;
    let border_color = vec3<f32>(0.15) * mesh.uv.y + 0.28;
    let x = mix(-border_size, 1.0 + border_size, mesh.uv.x);
    let y = mix(-border_size, 1.0 + border_size, mesh.uv.y);

    var color = mix(white, hue, x) * mix(white, black, y);

    let in_border = mesh.uv.x < border_size || mesh.uv.x > (1 - border_size)
    || mesh.uv.y < border_size || mesh.uv.y > (1 - border_size);

    if in_border {
        color = border_color;
    }

    // Selected Color Indicator
    let ci_white_size = 0.05;
    let ci_black_size = 0.06;
    let ci_line_smoothing = 0.01;
    let ci_line_thickness = 0.01;
    let ci_pos = length(hsv.yz - mesh.uv);
    let ci_white =
        smoothstep(ci_white_size, ci_white_size - ci_line_smoothing, ci_pos)
        - smoothstep(ci_white_size, ci_white_size - ci_line_smoothing, ci_pos + ci_line_thickness);
    let ci_black =
        smoothstep(ci_black_size, ci_black_size - ci_line_smoothing, ci_pos)
        - smoothstep(ci_black_size, ci_black_size - ci_line_smoothing, ci_pos + ci_line_thickness);

    color = mix(color, white, ci_white);
    color = mix(color, black, ci_black);
    
    return to_linear(vec4<f32>(color, 1.0));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}
