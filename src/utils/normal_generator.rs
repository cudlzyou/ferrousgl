use glam::Vec3;

pub fn generate_normals(vertices: &Vec<f32>, indices: &Vec<u32>) -> Vec<f32> {
    let mut normals = vec![0.0f32; vertices.len()]; // one normal per vertex (x, y, z)

    // Iterate over each triangle
    for tri in indices.chunks(3) {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;

        let v0 = Vec3::new(vertices[i0 * 3], vertices[i0 * 3 + 1], vertices[i0 * 3 + 2]);
        let v1 = Vec3::new(vertices[i1 * 3], vertices[i1 * 3 + 1], vertices[i1 * 3 + 2]);
        let v2 = Vec3::new(vertices[i2 * 3], vertices[i2 * 3 + 1], vertices[i2 * 3 + 2]);

        // Calculate face normal
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let face_normal = edge1.cross(edge2).normalize();

        // Add the face normal to each vertex normal
        for &i in &[i0, i1, i2] {
            normals[i * 3] += face_normal.x;
            normals[i * 3 + 1] += face_normal.y;
            normals[i * 3 + 2] += face_normal.z;
        }
    }

    // Normalize the summed vertex normals
    for i in 0..(vertices.len() / 3) {
        let n = Vec3::new(normals[i * 3], normals[i * 3 + 1], normals[i * 3 + 2]).normalize();
        normals[i * 3] = n.x;
        normals[i * 3 + 1] = n.y;
        normals[i * 3 + 2] = n.z;
    }

    normals
}