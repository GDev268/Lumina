use std::collections::HashMap;

pub enum MouseButton{
    Left = 1,
    Middle = 2,
    Right = 3,
    Side1 = 4,
    Side2 = 5
}


pub struct Mouse{
    x:f64,
    y:f64,
    last_x:f64,
    last_y:f64,
    dx:f64,
    dy:f64,
    scroll_dx:f64,
    scroll_dy:f64,
    first_mouse:bool,
    buttons:HashMap<u32,bool>,
    buttons_changed:HashMap<u32,bool>
}

impl Mouse{
    pub fn new() -> Self{
        let mut buttons:HashMap<u32,bool> = HashMap::new();
        let mut buttons_changed:HashMap<u32,bool> = HashMap::new();

        let mouse_buttons:[MouseButton;5] = [
            MouseButton::Left,
            MouseButton::Middle,
            MouseButton::Right,
            MouseButton::Side1,
            MouseButton::Side2
        ];

        for button in mouse_buttons {
            let number = button as u32;
            buttons.insert(number, false);
            buttons_changed.insert(number,false);
        }

        return Self { x: 0.0, 
        y: 0.0, 
        last_x: 0.0, 
        last_y: 0.0, 
        dx: 0.0, 
        dy: 0.0,
        scroll_dx: 0.0,
        scroll_dy: 0.0, 
        first_mouse: false,
        buttons,
        buttons_changed
        };
    }

    pub fn get_mouse_x(&self) -> f64{
        return self.x;
    }

    pub fn get_mouse_y(&self) -> f64{
        return self.y;
    }

    pub fn get_dx(&mut self) -> f64{
        let _dx = self.dx;
        self.dx = 0f64;
        return _dx;
    }

    pub fn get_dy(&mut self) -> f64{
        let _dy = self.dy;
        self.dy = 0f64;
        return _dy;
    }
    
    pub fn get_scroll_dx(&mut self) -> f64{
        let _scroll_dx = self.scroll_dx;
        self.scroll_dx = 0f64;
        return _scroll_dx;
    }

    pub fn get_scroll_dy(&mut self) -> f64{
        let _scroll_dy = self.scroll_dy;
        self.scroll_dy = 0f64;
        return _scroll_dy;
    }

    pub fn change_button(&mut self,button:u32){
        *self.buttons.get_mut(&button).unwrap() = true;
        *self.buttons_changed.get_mut(&button).unwrap() = true;
    }

    pub fn change_motion(&mut self,x:i32,y:i32,dx:i32,dy:i32){
        self.x = x as f64;
        self.y = y as f64;
        self.dx = dx as f64;
        self.dy = dy as f64;
    }

    pub fn get_button(&self,button:MouseButton) -> bool{
        return *self.buttons.get(&(button as u32)).unwrap();
    }

    pub fn button_changed(&mut self,button:MouseButton) -> bool{
        return *self.buttons_changed.get(&(button as u32)).unwrap();
        let number = &(button as u32);

        let result:bool = if *self.buttons_changed.get(number).unwrap(){
            true
        }else{
            false
        };
        *self.buttons_changed.get_mut(number).unwrap() = false;
    }

    pub fn button_went_up(&mut self,button:MouseButton) -> bool {
        let number = &(button as u32);

        return !*self.buttons.get(number).unwrap() && self.button_changed(self.from_u32(number).unwrap());
    }

    pub fn from_u32(&self,value:&u32) -> Option<MouseButton> {
        match value {
            1 => Some(MouseButton::Left),
            2 => Some(MouseButton::Middle),
            3 => Some(MouseButton::Right),
            4 => Some(MouseButton::Side1),
            5 => Some(MouseButton::Side2),
            _ => None
        }
    }
}