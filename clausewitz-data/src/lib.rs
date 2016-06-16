extern crate combine;

mod combinators;
pub mod file;

#[derive(Debug, Clone)]
pub enum CwValue {
    String(String),
    Table(CwTable),
    Array(Vec<CwValue>)
}

impl CwValue {
    pub fn from_str<S: Into<String>>(value: S) -> Self {
        CwValue::String(value.into())
    }

    pub fn from_color(r: u8, g: u8, b: u8) -> Self {
        CwValue::Array(vec![
            CwValue::String(r.to_string()),
            CwValue::String(g.to_string()),
            CwValue::String(b.to_string())
        ])
    }

    pub fn as_string(&self) -> Option<&String> {
        if let &CwValue::String(ref val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_table(&self) -> Option<&CwTable> {
        if let &CwValue::Table(ref val) = self {
            Some(val)
        } else {
            None
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut CwTable> {
        if let &mut CwValue::Table(ref mut val) = self {
            Some(val)
        } else {
            None
        }
    }
}

impl From<String> for CwValue {
    fn from(value: String) -> Self {
        CwValue::from_str(value)
    }
}

impl<'a> From<&'a String> for CwValue {
    fn from(value: &'a String) -> Self {
        CwValue::from_str(value.clone())
    }
}

#[derive(Debug, Clone)]
pub struct CwKeyValue {
    pub key: String,
    pub value: CwValue,
}

#[derive(Debug, Clone)]
pub struct CwTable {
    pub values: Vec<CwKeyValue>,
}

impl CwTable {
    pub fn new() -> Self {
        CwTable {
            values: Vec::new()
        }
    }

    pub fn parse(text: &str) -> CwTable {
        combinators::parse(text)
    }

    pub fn serialize(&self) -> String {
        let mut target = String::new();

        for key_value in &self.values {
            // Serialize the key if we have one
            if key_value.key != "" {
                target.push_str(&escape_str_if_needed(&key_value.key));
                target.push_str(" = ");
            }

            // Serialize the value
            key_value.value.serialize_to(&mut target);
        }

        target
    }

    pub fn get(&self, key: &str) -> Option<&CwValue> {
        self.values.iter().find(|v| v.key == key).map(|v| &v.value)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut CwValue> {
        self.values.iter_mut().find(|v| v.key == key).map(|v| &mut v.value)
    }

    pub fn set(&mut self, key: &str, value: CwValue) {
        // Check if a value already exists with this key
        if let Some(ref mut entry) = self.values.iter_mut().find(|v| v.key == key) {
            // It does, overwrite it
            entry.value = value;
            return; // < Can't use else, borrow checking complains
        }

        // It doesn't, add it
        self.values.push(CwKeyValue { key: key.into(), value: value });
    }

    pub fn add(&mut self, key: &str, value: CwValue) {
        self.values.push(CwKeyValue { key: key.into(), value: value });
    }
}

impl CwValue {
    fn serialize_to(&self, target: &mut String) {
        match self {
            &CwValue::String(ref v) => {
                target.push_str(&escape_str_if_needed(v));
                target.push('\n');
            },
            &CwValue::Table(ref t) => {
                target.push_str("{\n");
                target.push_str(&t.serialize());
                target.push_str("}\n");
            },
            &CwValue::Array(ref a) => {
                target.push_str("{\n");
                for val in a {
                    val.serialize_to(target);
                }
                target.push_str("}\n");
            }
        }
    }
}

fn escape_str(text: &str) -> String {
    let mut target = String::new();

    target.push('\"');
    for c in text.chars() {
        match c {
            '\\' => target.push_str("\\\\"),
            _ => target.push(c)
        };
    }
    target.push('\"');

    target
}

fn escape_str_if_needed(text: &str) -> String {
    if text.len() == 0 || text.chars().any(|c| c == '\\' || c == ' ') {
        escape_str(text)
    } else {
        text.into()
    }
}

#[cfg(test)]
mod tests {
    use super::{CwTable, CwValue, CwKeyValue};

    #[test]
    fn parse_value() {
        let data = CwTable::parse("foo=bar");
        assert_eq!(data.values.len(), 1);
        assert_keystr(&data.values[0], "foo", "bar");
    }

    #[test]
    fn parse_values() {
        let data = CwTable::parse("foo=bar\nbar=foo");
        assert_eq!(data.values.len(), 2);
        assert_keystr(&data.values[0], "foo", "bar");
        assert_keystr(&data.values[1], "bar", "foo");
    }

    #[test]
    fn parse_values_inline() {
        let data = CwTable::parse("foo=bar bar=foo");
        assert_eq!(data.values.len(), 2);
        assert_keystr(&data.values[0], "foo", "bar");
        assert_keystr(&data.values[1], "bar", "foo");
    }

    #[test]
    fn parse_whitespace() {
        let data = CwTable::parse(" foo  = bar  ");
        assert_eq!(data.values.len(), 1);
        assert_keystr(&data.values[0], "foo", "bar");
    }

    #[test]
    fn parse_comments() {
        let data = CwTable::parse("foo=bar #things\nbar=foo");
        assert_eq!(data.values.len(), 2);
        assert_keystr(&data.values[0], "foo", "bar");
        assert_keystr(&data.values[1], "bar", "foo");
    }

    #[test]
    fn parse_quoted() {
        let data = CwTable::parse("foo=\"I'm a little teapot\"");
        assert_eq!(data.values.len(), 1);
        assert_keystr(&data.values[0], "foo", "I'm a little teapot");

        let data = CwTable::parse(r#"foo="I'm a little teapot \"short and stout\"""#);
        assert_eq!(data.values.len(), 1);
        assert_keystr(&data.values[0], "foo", "I'm a little teapot \"short and stout\"");
    }

    #[test]
    fn parse_nested() {
        let data = CwTable::parse("foo={bar=chickens foobar=frogs}\ncheeze=unfrogged");
        assert_eq!(data.values.len(), 2);
        assert_keystr(&data.values[1], "cheeze", "unfrogged");

        if let &CwValue::Table(ref table) = &data.values[0].value {
            assert_eq!(table.values.len(), 2);
            assert_keystr(&table.values[0], "bar", "chickens");
            assert_keystr(&table.values[1], "foobar", "frogs");
        } else {
            assert!(false, "Wrong value type!");
        }
    }

    #[test]
    fn parse_annoying_nested() {
        let data = CwTable::parse("foo={bar=chickens foobar=frogs}cheeze=unfrogged");
        assert_eq!(data.values.len(), 2);
        assert_keystr(&data.values[1], "cheeze", "unfrogged");

        if let &CwValue::Table(ref table) = &data.values[0].value {
            assert_eq!(table.values.len(), 2);
            assert_keystr(&table.values[0], "bar", "chickens");
            assert_keystr(&table.values[1], "foobar", "frogs");
        } else {
            assert!(false, "Wrong value type!");
        }
    }

    #[test]
    fn parse_array() {
        let data = CwTable::parse("foo={why \"does this\" exist}");
        assert_eq!(data.values.len(), 1);
        assert_eq!(data.values[0].key, "foo");

        if let &CwValue::Array(ref array) = &data.values[0].value {
            assert_eq!(array.len(), 3);
            assert_valuestr(&array[0], "why");
            assert_valuestr(&array[1], "does this");
            assert_valuestr(&array[2], "exist");
        }
    }

    fn assert_keystr(cw_value: &CwKeyValue, key: &str, value: &str) {
        assert_eq!(cw_value.key, key);
        assert_valuestr(&cw_value.value, value);
    }

    fn assert_valuestr(cw_value: &CwValue, value: &str) {
        assert_eq!(cw_value.as_string(), Some(&String::from(value)));
    }
}
