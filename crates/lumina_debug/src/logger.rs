use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Instant;
use color_print::cprintln;


static mut CURRENT_ID: u64 = 0;

#[derive(Debug)]
pub enum SeverityLevel {
    TRACE,
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
            let time = Instant::now();
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
                    let secs = (time.elapsed().as_secs() % 60) as u128;
                    let mins = (time.elapsed().as_secs() / 60) as u128;
                    let millis = time.elapsed().as_millis() % 10000 / 10; 
                    
                    match cur_message.as_ref().unwrap().severity {
                        SeverityLevel::TRACE =>  cprintln!("[{}:{}:{}]<bright-cyan>[LOG] {}",mins,secs,millis,cur_message.unwrap().message),
                        SeverityLevel::INFO => cprintln!("[{}:{}:{}]<green>[INFO] {}",mins,secs,millis,cur_message.unwrap().message),
                        SeverityLevel::WARNING => cprintln!("[{}:{}:{}]<yellow>[WARNING] {}",mins,secs,millis,cur_message.unwrap().message),
                        SeverityLevel::ERROR => cprintln!("[{}:{}:{}]<bright-red>[ERROR] {}",mins,secs,millis,cur_message.unwrap().message),
                        SeverityLevel::CRASH => cprintln!("[{}:{}:{}]<red>[CRASH] {}",mins,secs,millis,cur_message.unwrap().message)
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
