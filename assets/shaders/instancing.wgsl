#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;
    var out: VertexOutput;
     var model = mesh.model;
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);

    return out;
}

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_types
#import bevy_pbr::pbr_functions

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var pbr_input: PbrInput = pbr_input_new();
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.N = prepare_normal(
        pbr_input.material.flags,
        in.world_normal,
    #ifdef VERTEX_TANGENTS
    #ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
    #endif
    #endif
    #ifdef VERTEX_UVS
        in.uv,
    #endif
        in.is_front,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    return tone_mapping(pbr(pbr_input));
}
