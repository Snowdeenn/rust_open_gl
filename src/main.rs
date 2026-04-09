mod math;
use math::_vec4_::Vec4;
use math::_mat4_::Mat4;




fn main() {
    
    let mat = Mat4::identity();
    let vec_test = mat
                        .transform(Vec4::new(1.0, 2.0, 3.0, 1.0));

    println!("x : {}, y : {}, z : {}, w : {}", vec_test.x, vec_test.y, vec_test.z, vec_test.w);
    mat.multiply(Mat4::identity());
    for i in 0..15 {
        println!("{}", mat.columns[i]);
    }
    
}
