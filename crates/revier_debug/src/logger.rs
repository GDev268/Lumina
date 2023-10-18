use std::sync::{Mutex, Arc};
use std::thread;

static mut CURRENT_ID: u64 = 0;

pub enum SeverityLevel {
    LOW,
    NORMAL,
    HIGH,
    EXTREME
}

pub struct Message{
    parent_id:Option<u64>,
    id:Option<u64>,
    severity:SeverityLevel,
    message:String
}

impl Message {
    pub fn create_message_node(severity:SeverityLevel,message:String,parent_id:Option<u64>) -> Self {
        let message: Message = unsafe { Message::new(Some(CURRENT_ID),severity,message,parent_id) };

        unsafe {
            CURRENT_ID += 1;
        }

        return message;
    }

    fn new(id:Option<u64>,severity:SeverityLevel,message:String,parent_id:Option<u64>) -> Self {
        Self{
            parent_id,
            id,
            severity,
            message
        }
    }
}

pub struct Logger{
    pub message_pool:Arc<Mutex<Vec<Message>>>,
    pub severe_pool:Vec<Message>
}

impl Logger{
    pub fn new() -> Self{
        let log_buffer = Arc::new(Mutex::new(Vec::new()));
            
        let buffer = log_buffer.clone();

        thread::spawn(move || {

            loop{
                let cur_message:Option<Message> = {
                    let mut buffer = buffer.lock().unwrap();
                    if buffer.is_empty() {
                        None
                    } else {
                        Some(buffer.remove(0))
                    }
                };

            }
        });

        Self{
            message_pool: log_buffer,
            severe_pool: Vec::new()
        }
    }
    
}
