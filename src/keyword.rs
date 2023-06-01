use std::collections::HashMap;

#[derive(Clone)]
pub enum Keyword {
    Let
}

impl Keyword {
    fn keywords() -> Vec<Self> {
        vec![
            Keyword::Let
        ]
    }

    pub fn hash_map() -> HashMap<String, Self> {
        let mut map = HashMap::new();
        let keywords = Self::keywords();

        for keyword in keywords {
            map.insert(keyword.to_string(), keyword);
        }

        map
    }
}

impl ToString for Keyword {
    fn to_string(&self) -> String {
        match self {
            Keyword::Let => "let".to_string(),
        }
    }
}