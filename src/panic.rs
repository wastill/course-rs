fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // 实现下面的代码
        panic!("Exercise Failed if printing out this line!");
    }

    println!("Exercise Failed if printing out this line!");
}

fn divide(x: u8, y: u8) {
    if y == 0 {
        panic!("division by zero");
    }
    println!("{}", x / y)
}

fn production_rate_per_hour(speed: u8) -> f64 {
    let cph: u32 = 221;
    match speed {
        1..=4 => (speed as u32 * cph) as f64,
        5..=8 => (speed as u32 * cph) as f64 * 0.9,
        9..=10 => (speed as u32 * cph) as f64 * 0.77,
        _ => 0 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}

mod tests {
    use super::*;

    #[test]
    fn test_panic() {
        drink("lemonade");
    }

    // 修复所有的 panic，让代码工作
    #[test]
    fn test_panic_2() {
        assert_eq!("abc".as_bytes(), [97, 98, 99]);

        let v = vec![1, 2, 3];
        let ele = v[2];
        let ele = v.get(2).unwrap();

        // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
        let v = production_rate_per_hour(2);

        divide(15, 1);

        println!("Success!")
    }
}
