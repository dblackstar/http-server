
use std::collections::HashMap;


#[derive(Debug)]
pub struct QueryString<'buff>{
    data:HashMap<&'buff str, Value<'buff>>
}

#[derive(Debug)]
pub enum Value <'buff> {
    Single(& 'buff str),
    Multiple(Vec<&'buff str>),
}

impl<'buff> QueryString<'buff> {
    pub fn get(&self, key:&str)->Option<&Value>{
        self.data.get(key)
    }
}


//  a=1&b=2&d=&e===&d=7&d=abc
impl<'buff> From<& 'buff str> for QueryString<'buff> {
    
    fn from(s: & 'buff str) -> Self{

        let mut data = HashMap::new();

        for sub_str in s.split('&'){

            // iterator return value between '&'
            let mut key = sub_str;
            let mut val = "";

            // return an option that wrap the index of the patterns we give in parenthesis
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify(|existing:&mut Value| match existing{
                Value::Single(prev_val) => {
                    *existing = Value::Multiple(vec![prev_val, val]);
                },
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));

        }
        QueryString{
            data:data,
        }
    }
}