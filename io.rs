use meshx::TriMesh;

pub fn export_obj() {

        size = (8, 8);
        pos = (0, 0);

        let vertices =  gen_perlin(size, pos);
        let indices = generate_indicies(size);


        let trimesh = TriMesh::new(vertices, indices);
}