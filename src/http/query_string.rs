use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, QueryValue<'a>>,
}

#[derive(Debug)]
pub enum QueryValue<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&QueryValue> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();
        for substr in s.split('&') {
            let mut key = substr;
            let mut val = "";
            if let Some(i) = substr.find('=') {
                key = &substr[..i];
                val = &substr[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryValue| match existing {
                    QueryValue::Single(previous_value) => {
                        *existing = QueryValue::Multiple(vec![previous_value, val]);
                    }
                    QueryValue::Multiple(vec) => vec.push(val),
                })
                .or_insert(QueryValue::Single(val));
        }
        QueryString { data }
    }
}
