// A, B vector

// A-B
pub fn vec_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [b[0] - a[0], b[1] - a[1], b[2] - a[2]];        
}

// A+B
fn vec_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [b[0] + a[0], b[1] + a[1], b[2] + a[2]];        
}

// AxB
pub fn cross_prod (a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [a[1]*b[2] - a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
}

// Weighted average normal to O of 5 points with O being the center one.
pub fn tr_wt_avg(o: [f32; 3], a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3]) -> [f32; 3] {
        let mut final_vec: [f32; 3] = [0.0, 0.0, 0.0];
        
        // Directional vectors
        let oa = vec_sub(o, a);
        let ob = vec_sub(o, b);
        let oc = vec_sub(o, c);
        let od = vec_sub(o, d);

        // First find area
        let area1 = tr_area(oa, ob);
        let area2 = tr_area(ob, oc);
        let area3 = tr_area(oc, od);
        let area4 = tr_area(od, oa);

        // Only then normalise
        let a = vec_norm(oa);
        let b = vec_norm(ob);
        let c = vec_norm(oc);
        let d = vec_norm(od);

        // Each normal to triangles
        let tr1 = cross_prod(a, b);
        let tr2 = cross_prod(b, c);
        let tr3 = cross_prod(c, d);
        let tr4 = cross_prod(d, a);

        // Sum their multiple
        final_vec = vec_add(final_vec, vec_scale(tr1, area1));
        final_vec = vec_add(final_vec, vec_scale(tr2, area2));
        final_vec = vec_add(final_vec, vec_scale(tr3, area3));
        final_vec = vec_add(final_vec, vec_scale(tr4, area4));

        // And divide by the sum of areas
        final_vec = vec_scale(final_vec, 1.0/(area1 + area2 + area3 + area4));

        return final_vec
}

// Area of a triangle with vector sides A and B
fn tr_area(a: [f32; 3], b: [f32; 3]) -> f32 {
        return 0.5*( (a[1]*b[2] - a[2]*b[1]).powi(2) + (a[2]*b[0] - a[0]*b[2]).powi(2) + (a[0]*b[1] - a[1]*b[0]).powi(2) ).powi(1/2);
}

// |V|
fn vec_mag(v: [f32; 3]) -> f32 {
        return (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).powi(1/2).abs()
}

// A --> nA
pub fn vec_scale(v: [f32; 3], s: f32) -> [f32; 3] {
        let mut vs: [f32; 3] = [0.0, 0.0, 0.0];
        vs[0] = v[0] * s;
        vs[1] = v[1] * s;
        vs[2] = v[2] * s;
        return vs
}

pub fn inv_vec(v: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let mut final_vec: Vec<[f32; 3]> = Vec::new();

        for v in v {
                final_vec.push(vec_scale(v, -1.0))
        }
        
        return final_vec
}

pub fn vec_norm(v: [f32; 3]) -> [f32; 3] {
        return vec_scale(v, vec_mag(v));
}

