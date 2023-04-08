struct Node<T> {
    prefix: String,
    path: String,
    value: Option<T>,
    children: Vec<Node<T>>,
    index: String,
    min: u8,
    max: u8,
}

struct Tree<T> {
    root: Node<T>,
}

impl<T> Tree<T> {
    fn find(&self, mut key: String) -> Option<&T> {
        let mut parent = &self.root;
        if key == "" {
            return if !parent.value.is_none() {
                Some(&parent.value.as_ref().unwrap())
            } else {
                None
            }
        }

        loop {
            let c = key.bytes().nth(0).unwrap();
            if c >= parent.min && c <= parent.max {
                let idx = parent.index.find(char::from_u32(c as u32).unwrap());
                match idx {
                    Some(idx) => {
                        let child = &parent.children[idx];
                        if child.prefix == *key {
                            return if !child.value.is_none() {
                                Some(&child.value.as_ref().unwrap())
                            } else {
                                None
                            }
                        } else if key.starts_with(&child.prefix) {
                            parent = &child;
                            key = key[child.prefix.len()..].to_string();
                        } else {
                            return None
                        }
                    }
                    None => return None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        struct TestScenario<'a, T> {
            key: &'a str,
            value: T
        }

        let tests = [
            TestScenario {
                key: "pine",
                value: 512,
            },
            TestScenario {
                key: "pineapple",
                value: 64
            },
            TestScenario {
                key: "strawberry",
                value: 128
            }
        ];

        let tree = build_test_tree();

        for test in tests.iter() {
            let actual_value = tree.find(test.key.to_string()).unwrap();
            assert_eq!(test.value, *actual_value, "expected: {}, got: {}", test.value, actual_value);
        }
    }

    fn build_test_tree() -> Tree<i32> {
        Tree{
            root: Node{
                prefix: "".to_string(),
                path: "".to_string(),
                value: None,
                children: vec![
                    Node{
                        prefix: "pine".to_string(),
                        path: "pine".to_string(),
                        value: Some(512),
                        children: vec![
                            Node{
                                prefix: "apple".to_string(),
                                path: "pineapple".to_string(),
                                value: Some(64),
                                children: vec![],
                                index: "".to_string(),
                                min: 0,
                                max: 0,
                            }
                        ],
                        index: "a".to_string(),
                        min: 97,
                        max: 97,
                    },
                    Node{
                        prefix: "strawberry".to_string(),
                        path: "strawberry".to_string(),
                        value: Some(128),
                        children: vec![],
                        index: "".to_string(),
                        min: 0,
                        max: 0,
                    }
                ],
                index: "ps".to_string(),
                min: 112,
                max: 115,
            }
        }
    }
}