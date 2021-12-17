use crate::protocol::{KeyType, Lock, Protocol, TTL};

// 解析：set key value [EX seconds] [PX milliseconds] [NX|XX]
pub fn transform(param: Vec<String>) -> Protocol {
    // 数量最少要3个
    if param.len() < 3 {
        return Protocol::Error("ERR wrong number of arguments for 'set' command".to_string());
    }

    // 获取过期时间
    let ttl = get_ttl(&param);
    if let Err(_) = ttl {
        return Protocol::Error("ERR syntax error".to_string());
    }

    // 获取锁相关
    let lock = get_lock(&param);
    if let Err(_) = lock {
        return Protocol::Error("ERR syntax error".to_string());
    }

    Protocol::Set {
        typ: KeyType::String,
        key: param[1].to_string(),
        value: param[2].to_string(),
        ttl: ttl.unwrap(),
        lock: lock.unwrap(),
    }
}

// 解析出get命令参数中的ttl参数
// EX 秒
// PX 毫秒
fn get_ttl(data: &Vec<String>) -> Result<Option<(TTL, usize)>, ()> {
    // 长度不符合，直接不用处理了
    if data.len() < 3 {
        return Ok(None);
    }

    let ttl = match data.get(3) {
        None => None,
        Some(t) => match data.get(4) {
            None => None,
            Some(second) => {
                // Ex:
                let list = vec!["EX", "PX"];

                let t = &&t.to_uppercase()[..];

                // return 是用于函数的返回值，终于知道如何用法了
                if !list.contains(t) {
                    return Err(());
                }

                let ttl = if *t == "EX" { TTL::EX } else { TTL::PX };

                // 解析是对应的秒数或毫秒
                let num = second.parse::<usize>();
                if let Err(_) = num {
                    return Err(());
                }

                Some((ttl, num.unwrap()))
            }
        },
    };

    Ok(ttl)
}

// 解析出是NX还是XX
fn get_lock(data: &Vec<String>) -> Result<Option<Lock>, ()> {
    if data.len() < 4 {
        return Ok(None);
    }

    // 是否同时存在 NX XX
    let mut count = 0;
    let mut lock: Lock = Lock::NX;
    for datum in data.iter() {
        let upper = datum.to_uppercase();
        if upper == "NX" {
            count += 1;
            lock = Lock::NX;
        }

        if upper == "XX" {
            count += 1;
            lock = Lock::XX;
        }
    }

    // 都不存在，或者多个参数
    if count <= 0 || count > 1 {
        return Err(());
    }

    Ok(Some(lock))
}
