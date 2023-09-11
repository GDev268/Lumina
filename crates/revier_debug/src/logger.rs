

static mut CURRENT_ID: u64 = 0;

pub enum SeverityLevel {
    LOW,
    NORMAL,
    HIGH,
    EXTREME
}

pub struct Message{
    parent_id:u64,
    id:u64,
    severity:SeverityLevel,
    message:String
}

impl Message {
    pub fn create_game_object(severity:SeverityLevel,message:String,parent_id:u64) -> Self {
        let message: Message = unsafe { Message::new(CURRENT_ID,severity,message,parent_id) };

        unsafe {
            CURRENT_ID = CURRENT_ID + 1;
        }

        return message;
    }

    fn new(id:u64,severity:SeverityLevel,message:String,parent_id:u64) -> Self {
        Self{
            parent_id,
            id,
            severity,
            message
        }
    }
}

pub struct Logger{
    pub message_pool:Vec<Message>,
    pub crash_pool:Vec<Message>
}

impl Logger{
    pub fn new() -> Self{
        Self{
            message_pool: Vec::new(),
            crash_pool: Vec::new()
        }
    }
    
}