use crate::math::_vec4_::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub columns: [f32; 16]
}

impl Mat4 {
    pub fn zero() -> Mat4 {
        Mat4 { columns: [0.0f32; 16] }
    }

    pub fn identity() -> Mat4 {
        let mut mat4: Mat4 = Mat4::zero();
        mat4.columns[0] = 1.0;
        mat4.columns[5] = 1.0;
        mat4.columns[10] = 1.0;
        mat4.columns[15] = 1.0;

        mat4
    }

    pub fn multiply(&self, other: Mat4) -> Mat4 {
        let mut result: Mat4 = Mat4::zero();

        for i in 0..4 {         // on parcourt les lignes
            for j in 0..4 {     // on parcourt les colonnes
                let mut sum: f32 = 0.0f32;
                for k in 0..4 { // on fait le produit scalaire
                    sum += self.columns[k * 4 + i] * other.columns[j * 4 + k];
                }
                result.columns[j * 4 + i] = sum;
            }
        } 
        result
    }

    pub fn transform(&self, vec: Vec4) -> Vec4 {
        let mut result: [f32; 4] = [0.0f32; 4];
        let v = [vec.x, vec.y, vec.z, vec.w];

        for i in 0..4 {
            let mut sum: f32 = 0.0;
            for k in 0..4 {
                sum += self.columns[k * 4 + i] * v[k];
            }
            result[i] = sum;
        }
        Vec4::new(result[0], result[1], result[2], result[3])
    }
}