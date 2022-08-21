use bevy::prelude::*;
const STALK_LENGTH: f32 = 1.0;
const STALK_WIDTH: f32 = 0.01;
const GRAIN_LENGTH: f32 = 0.3;
const GRAIN_WIDTH: f32 = 0.03;

pub fn get_mesh() -> Mesh {
    #[cfg_attr(rustfmt, rustfmt_skip)]
        let indices = bevy::render::mesh::Indices::U32(vec![
        // Grain top
        0,1,2,
        2,3,0,

        // Grain sides
        5,4,12,
        12,13,5,

        7,6,14,
        14,15,7,

        9,8,16,
        16,17,9,

        11,10,18,
        18,19,11,


        // Stalk sides
        21,20,28,
        28,29,21,

        23,22,30,
        30,31,23,

        25,24,32,
        32,33,25,

        27,26,34,
        34,35,27,

    ]);



    #[cfg_attr(rustfmt, rustfmt_skip)]
    let vertices = [
        // top grain
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [0.0, 1.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [0.0, 1.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [0.0, 1.0, 0.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [0.0, 1.0, 0.0]),

        // sides same vertex, different normal for flat shading
        // top grain
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [0.0, 0.0, 1.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [0.0, 0.0, 1.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [1.0, 0.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [1.0, 0.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [0.0, 0.0, -1.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [0.0, 0.0, -1.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, -GRAIN_WIDTH], [-1.0, 0.0, 0.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH + GRAIN_LENGTH, GRAIN_WIDTH], [-1.0, 0.0, 0.0]),

        // bottom grain
        ([-GRAIN_WIDTH, STALK_LENGTH, GRAIN_WIDTH], [0.0, 0.0, 1.0]),
        ([GRAIN_WIDTH, STALK_LENGTH, GRAIN_WIDTH], [0.0, 0.0, 1.0]),
        ([GRAIN_WIDTH, STALK_LENGTH, GRAIN_WIDTH], [1.0, 0.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH, -GRAIN_WIDTH], [1.0, 0.0, 0.0]),
        ([GRAIN_WIDTH, STALK_LENGTH, -GRAIN_WIDTH], [0.0, 0.0, -1.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH, -GRAIN_WIDTH], [0.0, 0.0, -1.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH, -GRAIN_WIDTH], [-1.0, 0.0, 0.0]),
        ([-GRAIN_WIDTH, STALK_LENGTH, GRAIN_WIDTH], [-1.0, 0.0, 0.0]),

        // sides stalk
        // top stalk
        ([-STALK_WIDTH, STALK_LENGTH , STALK_WIDTH], [0.0, 0.0, 1.0]),
        ([STALK_WIDTH, STALK_LENGTH, STALK_WIDTH], [0.0, 0.0, 1.0]),
        ([STALK_WIDTH, STALK_LENGTH, STALK_WIDTH], [1.0, 0.0, 0.0]),
        ([STALK_WIDTH, STALK_LENGTH, -STALK_WIDTH], [1.0, 0.0, 0.0]),
        ([STALK_WIDTH, STALK_LENGTH, -STALK_WIDTH], [0.0, 0.0, -1.0]),
        ([-STALK_WIDTH, STALK_LENGTH, -STALK_WIDTH], [0.0, 0.0, -1.0]),
        ([-STALK_WIDTH, STALK_LENGTH, -STALK_WIDTH], [-1.0, 0.0, 0.0]),
        ([-STALK_WIDTH, STALK_LENGTH, STALK_WIDTH], [-1.0, 0.0, 0.0]),

        // bottom stalk
        ([-STALK_WIDTH, 0.0 , STALK_WIDTH], [0.0, 0.0, 1.0]),
        ([STALK_WIDTH, 0.0, STALK_WIDTH], [0.0, 0.0, 1.0]),
        ([STALK_WIDTH, 0.0, STALK_WIDTH], [1.0, 0.0, 0.0]),
        ([STALK_WIDTH, 0.0, -STALK_WIDTH], [1.0, 0.0, 0.0]),
        ([STALK_WIDTH, 0.0, -STALK_WIDTH], [0.0, 0.0, -1.0]),
        ([-STALK_WIDTH, 0.0, -STALK_WIDTH], [0.0, 0.0, -1.0]),
        ([-STALK_WIDTH, 0.0, -STALK_WIDTH], [-1.0, 0.0, 0.0]),
        ([-STALK_WIDTH, 0.0, STALK_WIDTH], [-1.0, 0.0, 0.0]),

    ];

    let mut positions = Vec::new();
    let mut normals = Vec::new();

    for (position, normal) in vertices.iter() {
        positions.push(*position);
        normals.push(*normal);
    }

    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    mesh
}
