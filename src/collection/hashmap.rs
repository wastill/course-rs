use std::collections::HashMap;

// 这里可以放置 hashmap 模块的实现代码
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hashmap() {
        let mut h = HashMap::new();
        h.insert("key", "value");
        println!("{:?}", h);
    }

    #[test]
    fn test_get_hashmap_item() {
        let mut h = HashMap::new();
        h.insert("key", "value");
        let value = h.get("key");
        println!("{:?}", value);
    }

    #[test]
    fn test_borrow_hashmap() {
        use std::collections::HashMap;

        let name = String::from("Sunface");
        let age = 18;

        let mut handsome_boys = HashMap::new();
        handsome_boys.insert(&name, age);

        println!("因为过于无耻，{:?}已经被除名", handsome_boys);
        println!("还有，他的真实年龄远远不止{}岁", age);
        std::mem::drop(name);
    }

    #[test]
    fn test_iter_hashmap() {
        let mut h = HashMap::new();
        h.insert("key", "value");
        h.insert("key2", "value2");
        for (key, value) in &h {
            println!("{}: {}", key, value);
        }
    }
    #[test]
    fn test_hashmap_entry() {
        let mut h = HashMap::new();
        h.insert("key", "value");
        let v = h.entry("key");
        println!("{:?}", v);
    }
    #[test]
    fn test_vec_to_hashmap() {
        let v = vec![(1, "one"), (2, "two"), (3, "three")];
        let h: HashMap<i32, &str> = v.into_iter().collect(); // h 需要指定类型, collect 会把 v 的元素类型推断为 (i32, &str)
        println!("{:?}", h);
    }
}
