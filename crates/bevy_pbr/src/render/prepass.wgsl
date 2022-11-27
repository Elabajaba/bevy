#import bevy_pbr::prepass_bindings
#import bevy_pbr::mesh_functions

fn clip_to_uv(clip: vec4<f32>) -> vec2<f32> {
    var uv = clip.xy / clip.w;
    uv = (uv + 1.0) * 0.5;
    uv.y = 1.0 - uv.y;
    return uv;
}

struct Vertex {
    @location(0) position: vec3<f32>,

#ifdef VERTEX_UVS
    @location(1) uv: vec2<f32>,
#endif // VERTEX_UVS

#ifdef PREPASS_NORMALS
    @location(2) normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // PREPASS_NORMALS

#ifdef SKINNED
    @location(4) joint_indices: vec4<u32>,
    @location(5) joint_weights: vec4<f32>,
#endif // SKINNED
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

#ifdef VERTEX_UVS
    @location(0) uv: vec2<f32>,
#endif // VERTEX_UVS

#ifdef PREPASS_NORMALS
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // PREPASS_NORMALS

#ifdef PREPASS_VELOCITIES
    @location(3) world_position: vec4<f32>,
    @location(4) previous_world_position: vec4<f32>,
#endif // PREPASS_VELOCITIES
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
#else // SKINNED
    var model = mesh.model;
#endif // SKINNED

#ifdef PREPASS_VELOCITIES
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.previous_world_position = mesh_position_local_to_world(mesh.previous_model, vec4<f32>(vertex.position, 1.0));
#endif // PREPASS_VELOCITIES

    out.clip_position = mesh_position_local_to_clip(model, vec4(vertex.position, 1.0));

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif // VERTEX_UVS

#ifdef PREPASS_NORMALS
#ifdef SKINNED
    out.world_normal = skin_normals(model, vertex.normal);
#else // SKINNED
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif // SKINNED

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif // VERTEX_TANGENTS
#endif // PREPASS_NORMALS

    return out;
}

struct FragmentInput {
#ifdef PREPASS_NORMALS
    @location(0) world_normal: vec3<f32>,
#ifdef VERTEX_UVS
    @location(1) uv: vec2<f32>,
#endif // VERTEX_UVS
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // PREPASS_NORMALS
#ifdef PREPASS_VELOCITIES
    @location(3) world_position: vec4<f32>,
    @location(4) previous_world_position: vec4<f32>,
#endif // PREPASS_VELOCITIES
}

struct FragmentOutput {
#ifdef PREPASS_NORMALS
    @location(0) normal: vec4<f32>,
#endif // PREPASS_NORMALS
#ifdef PREPASS_VELOCITIES
    @location(#PREPASS_VELOCITY_LOCATION) velocity: vec2<f32>,
#endif // PREPASS_VELOCITIES
}

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    var out: FragmentOutput;

#ifdef PREPASS_NORMALS
    out.normal = vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
#endif // PREPASS_NORMALS

#ifdef PREPASS_VELOCITIES
    let clip_position = view.view_proj * in.world_position;
    let previous_clip_position = previous_view_proj * in.previous_world_position;
    out.velocity = clip_to_uv(clip_position) - clip_to_uv(previous_clip_position);
#endif // PREPASS_VELOCITIES

    return out;
}
