use std::{
    collections::HashMap,
    str::FromStr
};

pub fn get_param_default<T>(hashmap: &HashMap<String, String>, key: &str, default: T) -> T
where T: FromStr
{
    let limit = hashmap.get(key);
    if let None = limit { return default; }

    let limit: T = limit.unwrap().parse().unwrap_or(default);

    return limit;
}
