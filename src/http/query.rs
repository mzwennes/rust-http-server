use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryStringValue<'buf>>,
}

#[derive(Debug)]
pub enum QueryStringValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(value: &'buf str) -> Self {
        let mut data = HashMap::new();

        for pair in value.split("&") {
            let mut key = pair;
            let mut val = "";
            if let Some(i) = pair.find("=") {
                key = &pair[..i];
                val = &pair[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    QueryStringValue::Single(prev) => {
                        *existing = QueryStringValue::Multiple(vec![prev, val]);
                    }
                    QueryStringValue::Multiple(vec) => vec.push(val),
                })
                .or_insert(QueryStringValue::Single(val));
        }

        QueryString { data }
    }
}
