struct Camera{
    projection_matrix:glam::Mat4,
    view_matrix:glam::Mat4,
    inverse_view_matrix:glam::Mat4
}

impl Camera{
    pub fn set_orthographic_projection(left:f32,right:f32,top:f32,bottom:f32,near:f32,far:f32){

    }

    pub fn set_perspective_projection(fovy:f32,aspect:f32,near:f32,far:f32){

    }

    pub fn set_view_direction(position:glam::Vec3,direction:glam::Vec3,up:Option<glam::Vec3>){

    }

    pub fn set_view_target(position:glam::Vec3,target:glam::Vec3,up:Option<glam::Vec3>){

    }

    pub fn set_view_xyz(position:glam::Vec3,rotation:glam::Vec3){

    }

    pub fn get_projection(&self) -> glam::Mat4{
        return self.projection_matrix;
    }

    pub fn get_view(&self) -> glam::Mat4{
        return self.view_matrix;
    }

    pub fn get_inverse_view(&self) -> glam::Mat4{
        return self.inverse_view_matrix;
    }

    /*pub fn get_position(&self) -> glam::Vec3{
        return glam::Vec3::from(self.inverse_view_matrix[3]);
    }*/
}