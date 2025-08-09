use dioxus::prelude::*;
use std::collections::HashMap;

// 1. 定义事件数据类型
#[derive(Clone, Debug)]
pub enum EventData {
    String(String),
    Number(i32),
    // 可以扩展更多类型
}

// 2. 定义 Emitter 结构体
pub struct Emitter {
    sender: UnboundedSender<(String, EventData)>,
}

impl Emitter {
    pub fn new(sender: UnboundedSender<(String, EventData)>) -> Self {
        Self { sender }
    }

    // emit 函数：传入事件名和数据
    pub fn emit(&self, event_name: &str, data: EventData) {
        let _ = self.sender.unbounded_send((event_name.to_string(), data));
    }
}
