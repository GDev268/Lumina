
pub enum SeverityLevel {
    LOW,
    NORMAL,
    HIGH,
    EXTREME
}

pub struct Message{
    pub severity:SeverityLevel,

}

pub struct Logger{
    pub message_pool:Vec<Message>
}

impl Logger{
    pub fn new() -> Self{
        Self{
            message_pool: Vec::new()
        }
    }
}