#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

@group(1) @binding(0)
var<uniform> mesh: Mesh;

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::pbr_functions

// NOTE: Bindings must come before functions that use them!
#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,

    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct Time {
    time_since_startup: f32,
};

@group(2) @binding(0)
var<uniform> time: Time;


//
//
fn multQuat(q1: vec4<f32>, q2: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        q1.w * q2.x + q1.x * q2.w + q1.z * q2.y - q1.y * q2.z,
        q1.w * q2.y + q1.y * q2.w + q1.x * q2.z - q1.z * q2.x,
        q1.w * q2.z + q1.z * q2.w + q1.y * q2.x - q1.x * q2.y,
        q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z
    );
}

fn rotate_vector(quat: vec4<f32>, vect: vec3<f32>) -> vec3<f32> {
    var qv: vec4<f32> = multQuat( quat, vec4<f32>(vect, 0.0) );
    return multQuat( qv, vec4(-quat.x, -quat.y, -quat.z, quat.w) ).xyz;
}
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var _tree_sway_speed: f32 = 3.0;
    var _wind_size: f32 = 15.0;
    var _tree_sway_stutter: f32 = 1.5;
    var _tree_sway_stutter_influence: f32 = 0.2;
    var _tree_sway_disp: f32 = 0.3;
    var _leaves_wiggle_speed: f32 = 0.1;
    var _leaves_wiggle_disp: f32 = 0.07;
    var _branches_disp: f32 = 0.3;
    var _wind_dir: vec3<f32> = vec3<f32>(0.5);
    let model_matrix = mat4x4<f32>(
            vertex.model_matrix_0,
            vertex.model_matrix_1,
            vertex.model_matrix_2,
            vertex.model_matrix_3,
        );
// rotation around point: https://answers.unity.com/questions/1751620/rotating-around-a-pivot-point-using-a-quaternion.html
//    var position = rotate_vector(vertex.rotation, vertex.position - vertex.i_pos_scale.xyz) + vertex.i_pos_scale.xyz;
//    var position = vertex.rotation * vertex.position + vertex.i_pos_scale.xyz;
    var position = model_matrix * vec4<f32>(vertex.position, 1.0);
     // Movement and Wiggle
    position.x += (cos(time.time_since_startup * _tree_sway_speed + (position.x/_wind_size) + (sin(time.time_since_startup * _tree_sway_stutter * _tree_sway_speed + (position.x/_wind_size)) * _tree_sway_stutter_influence) ) + 1.0)/2.0 * _tree_sway_disp * _wind_dir.x * (position.y / 10.0) +
    cos(time.time_since_startup * position.x * _leaves_wiggle_speed + (position.x/_wind_size)) * _leaves_wiggle_disp * _wind_dir.x * vertex.i_color.y * 1.0;

    position.z += (cos(time.time_since_startup * _tree_sway_speed + (position.z/_wind_size) + (sin(time.time_since_startup * _tree_sway_stutter * _tree_sway_speed + (position.z/_wind_size)) * _tree_sway_stutter_influence) ) + 1.0)/2.0 * _tree_sway_disp * _wind_dir.z * (position.y / 10.0) +
    cos(time.time_since_startup * position.z * _leaves_wiggle_speed + (position.x/_wind_size)) * _leaves_wiggle_disp * _wind_dir.z * vertex.i_color.y * 1.0;

//    position.y += cos(time.time_since_startup * _tree_sway_speed + (position.z/_wind_size)) * _tree_sway_disp * _wind_dir.y * (position.y / 10.0);

//    //Branches Movement
//    position.y += sin(time.time_since_startup * _tree_sway_speed + _wind_dir.x + (position.z/_wind_size)) * _branches_disp  * vertex.i_color.x * 1.0;

    var position = position.xyz * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;

    var out: VertexOutput;
    out.world_position = mesh.model * vec4<f32>(position, 1.0);
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
    out.color = vertex.i_color;
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec4<f32>,
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let layer = i32(in.world_position.x) & 0x3;

    var pbr_input: PbrInput = pbr_input_new();
    pbr_input.material.base_color = pbr_input.material.base_color * in.color;
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    pbr_input.N = prepare_normal(
        pbr_input.material.flags,
        in.world_normal,
        in.is_front,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    return tone_mapping(pbr(pbr_input));
}