use std::fmt::Display;

pub fn create_vector() -> i32 {
    let v = vec![1, 2, 3];
    v.len() as i32
}

fn create_vector_2() -> i32 {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.len() as i32
}

fn get_vector_item(v: Vec<i32>, index: i32) -> i32 {
    v[index as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vector() {
        let result = create_vector();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_create_vector_2() {
        let result = create_vector_2();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_get_vector_item() {
        let v = vec![1, 2, 3];
        let s = v.get(10);
        println!("{:?}", s);
        // let result = get_vector_item(v, 4);
        // assert_eq!(result, 2);
    }

    #[test]
    fn test_borrow_vector() {
        let v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        // v.push(6);

        println!("The first element is: {:?}", first);
    }

    #[test]
    fn test_iter_vector() {
        let v = vec![1, 2, 3, 4, 5];
        for i in v.iter() {
            println!("{:?}", i);
        }
        let mut v1 = vec![1, 2, 3, 4, 5];
        for i in &mut v1 {
            *i += 10
        }

        for i in 0..v1.len() {
            println!("{:?}", v[i]);
        }
        for v in &v {
            println!("{:?}", v);
        }
    }
    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // #[test]
    // fn test_enum_vector() {
    //     let v = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
    //     for i in v {
    //         println!("{:?}", i);
    //     }
    // }

    trait IpAddr {
        fn get_ip(&self) -> String;
    }

    struct V4(String);
    struct V6(String);

    impl IpAddr for V4 {
        fn get_ip(&self) -> String {
            self.0.clone()
        }
    }

    impl IpAddr for V6 {
        fn get_ip(&self) -> String {
            self.0.clone()
        }
    }

    fn sort_floats_with_nan(mut v: Vec<f64>) -> Vec<f64> {
        v.sort_by(|a, b| {
            if a.is_nan() {
                std::cmp::Ordering::Greater
            } else if b.is_nan() {
                std::cmp::Ordering::Less
            } else {
                a.partial_cmp(b).unwrap()
            }
        });
        v
    }

    #[test]
    fn test_enum_vector() {
        let vec: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
        for i in vec {
            println!("{:?}", i.get_ip());
        }
    }

    #[test]
    fn test_init_vector() {
        let v = vec![1; 10];
        println!("{:?}", v);
    }

    #[test]
    fn test_init_vector_1() {
        let mut v = Vec::with_capacity(10);
        println!("{:?}", v.capacity());
        println!("{:?}", v);
        v.push(1);
        println!("{:?}", v.capacity());
        println!("{:?}", v);
        println!("{:?}", v.len());
        v.extend([1, 2, 3, 4, 5]);
        println!("{:?}", v.capacity());
        println!("{:?}", v);
        // v.reserve(7);
        // println!("{:?}", v.capacity());
        // println!("{:?}", v);
        // println!("{:?}", v.len());
        v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
        println!("{:?}", v.capacity());
        println!("{:?}", v);
        println!("{:?}", v.len());
    }

    #[test]
    fn test_vector_suit() {
        let mut v = vec![1, 2];
        assert!(!v.is_empty()); // 检查 v 是否为空

        v.insert(2, 3); // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3]
        assert_eq!(v.remove(1), 2); // 移除指定位置的元素并返回, v: [1, 3]
        assert_eq!(v.pop(), Some(3)); // 删除并返回 v 尾部的元素，v: [1]
        assert_eq!(v.pop(), Some(1)); // v: []
        assert_eq!(v.pop(), None); // 记得 pop 方法返回的是 Option 枚举值
        v.clear(); // 清空 v, v: []

        let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
        v.append(&mut v1); // 将 v1 中的所有元素附加到 v 中, v1: []
        v.truncate(1); // 截断到指定长度，多余的元素被删除, v: [11]
        v.retain(|x| *x > 10); // 保留满足条件的元素，即删除不满足条件的元素

        let mut v = vec![11, 22, 33, 44, 55];
        // 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
        let mut m: Vec<_> = v.drain(1..=3).collect();

        let v2 = m.split_off(1); // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
        println!("{:?}", m);
        println!("{:?}", v2);
    }

    #[test]
    fn test_vector_sort() {
        let mut v = vec![1, 2, 9, 4, 5];
        v.sort();
        println!("{:?}", v);
        v.sort_by(|a, b| b.cmp(a));
        println!("{:?}", v);
        v.sort_unstable();
        println!("{:?}", v);
    }

    #[test]
    fn test_vector_sort_with_nan() {
        let v = vec![1.0, 2.0, 9.0, 4.0, 5.0, f64::NAN];
        let sorted = sort_floats_with_nan(v);
        println!("{:?}", sorted);
    }
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    impl PartialEq for Person {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age
        }
    }

    impl Eq for Person {
        fn assert_receiver_is_total_eq(&self) {
            self.age.assert_receiver_is_total_eq();
            self.name.assert_receiver_is_total_eq();
        }
    }

    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Person {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.age.cmp(&other.age)
        }
    }

    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}", self.name, self.age)
        }
    }

    impl Person {
        fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
    }
    #[test]
    fn test_vector_sort_by() {
        let mut v = vec![
            Person::new("Alice".to_string(), 25),
            Person::new("Bob".to_string(), 30),
            Person::new("Charlie".to_string(), 20),
        ];
        v.sort_by(|a, b| a.age.cmp(&b.age));
        println!("{:?}", v);
    }
}
