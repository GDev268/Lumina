use std::sync::{Mutex, Arc};
use std::thread;

static mut CURRENT_ID: u64 = 0;

#[derive(Debug)]
pub enum SeverityLevel {
    LOG,
    INFO,
    WARNING,
    ERROR,
    CRASH
}

#[derive(Debug)]
pub struct Message{
    parent_id:Option<u64>,
    id:u64,
    severity:SeverityLevel,
    message:String
}

impl Message {
    pub fn create_message_node(severity:SeverityLevel,message:String,parent_id:Option<u64>) -> Self {
        let message: Message = unsafe { Message::new(CURRENT_ID,severity,message,parent_id) };

        unsafe {
            CURRENT_ID += 1;
        }

        return message;
    }

    fn new(id:u64,severity:SeverityLevel,message:String,parent_id:Option<u64>) -> Self {
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

                if cur_message.is_some(){
                    match cur_message.unwrap().severity {
                        SeverityLevel::LOG =>
                        SeverityLevel::INFO =>
                        SeverityLevel::WARNING =>
                        SeverityLevel::ERROR =>
                        SeverityLevel::CRASH => 
                    }   
                }

            }
        });

        Self{
            message_pool: log_buffer,
            severe_pool: Vec::new()
        }
    }


    pub fn push_message(&self,message:&str,severity:SeverityLevel,parent_id:Option<u64>){
        let mut buffer = self.message_pool.lock().unwrap();
        buffer.push(Message::create_message_node(severity, message.to_string(), parent_id));
    }
}
