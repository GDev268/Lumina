struct Mouse{
    x:f64,
    y:f64,
    last_x:f64,
    last_y:f64,
    dx:f64,
    dy:f64,
    scroll_dx:f64,
    scroll_dy:f64,
    first_mouse:bool,
}

impl Mouse{
    pub fn new() -> Self{
        return Self { x: 0.0, 
        y: 0.0, 
        last_x: 0.0, 
        last_y: 0.0, 
        dx: 0.0, 
        dy: 0.0,
        scroll_dx: 0.0,
        scroll_dy: 0.0, 
        first_mouse: false };
    }
}
