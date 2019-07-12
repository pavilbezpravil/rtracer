use std::path::Path;
use crate::triangle::Triangle;
use crate::Vec3;
use crate::material::Material;
use crate::scene_data::SceneObject;

pub fn load_geometry_obj(path: &Path) -> Result<Vec<Triangle>, String>{
    let (mut models, _) = tobj::load_obj(path).unwrap();

    let mut ts = vec![];

    for model in models {
        let mesh = model.mesh;

        let mut ps = vec![];

        let mps = &mesh.positions;
        for i in 0..mps.len() / 3 {
            ps.push(Vec3::new(mps[3 * i], mps[3 * i + 1], mps[3 * i + 2]));
        }

        let indxs = &mesh.indices;
        for i in 0..mesh.indices.len() / 3 {
            let v0 = ps[indxs[3 * i + 0] as usize];
            let v1 = ps[indxs[3 * i + 1] as usize];
            let v2 = ps[indxs[3 * i + 2] as usize];

            let t = Triangle::new(v0, v1, v2);

            ts.push(t);
        }
    }

    Ok(ts)
}
