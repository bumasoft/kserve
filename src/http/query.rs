use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Query<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Query<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for Query<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_value) => {
                    let vec = vec![prev_value, val];
                    *existing = Value::Multiple(vec);
                },
                Value::Multiple(vec) => vec.push(val)
            })
            .or_insert(Value::Single(val));
        }

        Query { data }
    }
}