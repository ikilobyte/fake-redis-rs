use crate::config::PARSE_LIST;
use crate::protocol::Protocol;
use crate::storage::types::Message;

pub mod command;
pub mod del;
pub mod get;
pub mod hdel;
pub mod hget;
pub mod hset;
pub mod set;

// 解析入口
pub fn entry(param: Vec<String>) -> Result<Message, ()> {
    // 参数少了，就是语法错误
    if param.len() < 2 {
        return Err(());
    }

    // 处理一下，只保留实用的数据
    let mut param_filter = vec![];
    for (index, param) in param[2..].to_vec().iter().enumerate() {
        if index % 2 == 0 {
            param_filter.push(param.clone());
        }
    }

    let cmd = param_filter[0].to_uppercase()[..].to_string();
    let mut key = "".to_string();

    // 兼容单指令的命令，如：info、command
    if let Some(k) = param_filter.get(1) {
        key = k.clone();
    }

    // 执行解析
    if let Some(parser) = PARSE_LIST.get(&cmd[..]) {
        let protocol: Protocol = parser(param_filter);
        return Ok(Message { protocol, cmd, key });
    }

    return Err(());
}
