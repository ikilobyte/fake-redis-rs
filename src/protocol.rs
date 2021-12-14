use anyhow::Error;

#[derive(Debug)]
pub enum Protocol {
    // 暂时不做什么处理，只用来处理连接
    Command,

    // set key value [EX seconds] [PX milliseconds] [NX|XX]
    Set {
        key: String,               // key
        value: String,             // value
        ttl: Option<(TTL, usize)>, // 过期时间
        lock: Option<Lock>,        // 排斥相关
    },
    Get {
        key: String, // 获取哪个key
    },
    UnSupport,
    Error(String),
}

#[derive(Debug)]
pub enum TTL {
    EX,
    PX,
}

#[derive(Debug)]
pub enum Lock {
    NX, // 只在键不存在时， 才对键进行设置操作
    XX, // 只在键已经存在时， 才对键进行设置操作。
}

// vec转为命令
impl From<Vec<String>> for Protocol {
    fn from(params: Vec<String>) -> Self {
        if params.len() <= 0 {
            return Protocol::UnSupport;
        }

        //
        let cmd = &params[0][..];
        println!("cmd -> {}", cmd);
        return match cmd {
            "SET" => {
                // 获取过期时间
                let ttl = Parser::get_ttl(&params);
                if let Err(_) = ttl {
                    return Protocol::Error("ERR syntax error".to_string());
                }

                // 获取锁相关
                let lock = Parser::get_lock(&params);
                if let Err(_) = lock {
                    return Protocol::Error("ERR syntax error".to_string());
                }

                let set_cmd = Protocol::Set {
                    key: params[1].to_string(),
                    value: params[2].to_string(),
                    ttl: ttl.unwrap(),
                    lock: lock.unwrap(),
                };

                return set_cmd;
            }
            "GET" => Protocol::Get {
                key: params[1].to_string(),
            },
            "COMMAND" => Protocol::Command,
            _ => Protocol::UnSupport,
        };
    }
}

pub struct Parser;

impl Parser {
    // 解析redis协议
    // 参考：https://www.jianshu.com/p/f670dfc9409b
    pub fn start(bytes: String) -> Protocol {
        // 命令可能会有大小写不一致的，
        // key是大小写铭感的，A、a是两个key
        // ["*4", "$4", "HsEt", "$3", "Map", "$2", "u2", "$6", "111111", ""]
        let params = bytes.split("\r\n").collect::<Vec<&str>>();
        if params.len() < 2 {
            return Protocol::UnSupport;
        }

        // 下标2是固定的命令位置，且统一为大写字母
        let cmd = params[2].to_uppercase();

        let mut values: Vec<String> = vec![cmd];
        let params = &params[3..];

        for (index, param) in params.iter().enumerate() {
            if param.is_empty() {
                continue;
            }
            // 只需要取出数据，长度那些就不需要了
            // value也是大小写铭感的，不能随便改变大小写，下面这个是两个field
            // HSET map k1 value
            // HSET map K1 value
            if index % 2 != 0 {
                values.push(param.to_string());
            }
        }

        return values.into();
    }

    // 解析出get命令参数中的ttl参数
    // EX 秒
    // PX 毫秒
    pub fn get_ttl(data: &Vec<String>) -> Result<Option<(TTL, usize)>, ()> {
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
    pub fn get_lock(data: &Vec<String>) -> Result<Option<Lock>, ()> {
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
}
