pub fn vec_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [b[0] - a[0], b[1] - a[1], b[2] - a[2]];        
}

fn vec_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [b[0] + a[0], b[1] + a[1], b[2] + a[2]];        
}

pub fn cross_prod (a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
        return [a[1]*b[2] - a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
}

pub fn tr_wt_avg(o: [f32; 3], a: [f32; 3], b: [f32; 3], c: [f32; 3], d: [f32; 3]) -> [f32; 3] {
        let mut final_vec: [f32; 3] = [0.0, 0.0, 0.0];

        let a = vec_norm(vec_sub(o, a));
        let b = vec_norm(vec_sub(o, b));
        let c = vec_norm(vec_sub(o, c));
        let d = vec_norm(vec_sub(o, d));
        
        let area1 = tr_area(a, b);
        let area2 = tr_area(b, c);
        let area3 = tr_area(c, d);
        let area4 = tr_area(d, a);

        let tr1 = cross_prod(a, b);
        let tr2 = cross_prod(b, c);
        let tr3 = cross_prod(c, d);
        let tr4 = cross_prod(d, a);

        final_vec = vec_add(final_vec, vec_scale(tr1, area1));
        final_vec = vec_add(final_vec, vec_scale(tr2, area2));
        final_vec = vec_add(final_vec, vec_scale(tr3, area3));
        final_vec = vec_add(final_vec, vec_scale(tr4, area4));

        final_vec = vec_scale(final_vec, 1.0/(area1 + area2 + area3 + area4));

        return final_vec
}

fn tr_area(a: [f32; 3], b: [f32; 3]) -> f32 {
        return 0.5*( (a[1]*b[2] - a[2]*b[1]).powi(2) + (a[2]*b[0] - a[0]*b[2]).powi(2) + (a[0]*b[1] - a[1]*b[0]).powi(2) ).powi(1/2);
}

fn vec_mag(v: [f32; 3]) -> f32 {
        return (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).powi(1/2).abs()
}

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

