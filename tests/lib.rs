#[cfg(test)]
mod tests {
    use ckboots::consume_and_decode;
    use ckboots::OnChain;
    use ckboots_derives::OnChain;

    #[test]
    fn builtin_onchain_test() {
        let bytes = 0u32._to_bytes();
        let (actual, left) = consume_and_decode::<u32>(&bytes).unwrap();
        assert_eq!(left.len(), 0);
        assert_eq!(actual, 0);
    }

    // #[test]
    // fn string_onchain_test() {
    //     let s = String::from("i am Tom");
    //     let bytes = s._to_bytes();
    //     let (actual, left) = consume_and_decode::<String>(&bytes).unwrap();
    //     assert_eq!(actual, s);
    //     assert_eq!(left.len(), 0);
    // }

    #[test]
    fn vec_onchain_bytes() {
        let v: Vec<u32> = vec![0, 2, 4, 5, 6];
        let bytes = v._to_bytes();
        let actual = consume_and_decode::<Vec<u32>>(&bytes).unwrap().0;
        assert_eq!(v, actual);
        let v: Vec<Vec<u32>> = vec![vec![0, 2, 4], vec![]];
        let bytes = v._to_bytes();
        let actual = consume_and_decode::<Vec<Vec<u32>>>(&bytes).unwrap().0;
        assert_eq!(actual, v);
    }

    // #[test]
    // fn derive_onchain_struct() {
    //     #[derive(OnChain)]
    //     #[onchain(id = "person")]
    //     pub struct Person {
    //         pub age: u16,
    //         pub name: String,
    //     }

    //     let tom = Person {
    //         age: 12,
    //         name: String::from("Tom"),
    //     };
    //     let bytes = tom._to_bytes();
    //     let (actual, left) = consume_and_decode::<Person>(&bytes).unwrap();
    //     assert_eq!(actual.age, 12);
    //     assert_eq!(actual.name, "Tom");
    //     assert_eq!(left.len(), 0);
    //     assert_eq!(Person::_id(), Some("person"));
    // }

    // #[test]
    // fn derive_onchain_enum() {
    //     #[derive(OnChain)]
    //     pub enum EnumTest {
    //         AA(String),
    //         BB(u32),
    //         CC,
    //     }
    //     let aa = EnumTest::AA(String::from("1"));
    //     let bytes = aa._to_bytes();
    //     let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
    //     assert_eq!(left.len(), 0);
    //     match actual {
    //         EnumTest::AA(s) => assert_eq!(s, "1"),
    //         _ => panic!(),
    //     }

    //     let bb = EnumTest::BB(123);
    //     let bytes = bb._to_bytes();
    //     let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
    //     assert_eq!(left.len(), 0);
    //     match actual {
    //         EnumTest::BB(n) => assert_eq!(n, 123),
    //         _ => panic!(),
    //     }

    //     let cc = EnumTest::CC;
    //     let bytes = cc._to_bytes();
    //     let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
    //     assert_eq!(left.len(), 0);
    //     match actual {
    //         EnumTest::CC => {}
    //         _ => panic!(),
    //     }
    // }

    #[test]
    fn enum_default() {
        #[derive(OnChain)]
        pub enum EnumTest1 {
            AA,
            #[onchain(default)]
            BB,
            CC,
        }
        assert!(matches!(EnumTest1::_default(), EnumTest1::BB));

        #[derive(OnChain)]
        pub enum EnumTest2 {
            AA(u8),
            #[onchain(default = 4)]
            BB(u32),
            CC,
        }

        match EnumTest2::_default() {
            EnumTest2::BB(t) => assert_eq!(t, 4),
            _ => panic!(""),
        }

        #[derive(OnChain)]
        pub enum EnumTest3 {
            AA(u8),
            #[onchain(default)]
            BB(u32),
            CC,
        }

        match EnumTest3::_default() {
            EnumTest3::BB(t) => assert_eq!(t, 0),
            _ => panic!(""),
        }
    }
}
