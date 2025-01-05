// 基本的生命周期示例
pub fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn share(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime() {
        let s1 = String::from("short");
        let s2 = String::from("longer");
        let result = lifetime_example(&s1, &s2);
        assert_eq!(result, "longer");
    }

    #[test]
    fn test_lifetime_2() {
        let mut foo = Foo;
        let loan = foo.mutate_and_share();
        foo.share();
        println!("{:?}", loan);
    }
}
