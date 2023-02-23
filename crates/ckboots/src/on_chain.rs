pub trait OnChain: Sized {
    fn _capacity(&self) -> u64;

    fn _id() -> Option<&'static str> {
        None
    }

    fn _to_bytes(&self) -> Vec<u8>;

    fn _from_bytes(bytes: &[u8]) -> Option<Self>;

    fn _fixed_size() -> Option<u64>;

    fn _user_input() -> bool {
        false
    }
}

macro_rules! impl_on_chain_for_builtin {
    ($t:ty, $c:literal) => {
        impl OnChain for $t {
            fn _capacity(&self) -> u64 {
                $c
            }

            fn _to_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().into()
            }

            fn _from_bytes(bytes: &[u8]) -> Option<Self> {
                let mut slice = [0u8; $c];
                for i in 0..$c {
                    slice[i] = bytes.get(i)?.clone();
                }
                Some(<$t>::from_le_bytes(slice))
            }

            fn _fixed_size() -> Option<u64> {
                Some($c)
            }
        }
    };
}

impl_on_chain_for_builtin!(u8, 1);
impl_on_chain_for_builtin!(u16, 2);
impl_on_chain_for_builtin!(u32, 4);
impl_on_chain_for_builtin!(u64, 8);
impl_on_chain_for_builtin!(u128, 16);
impl_on_chain_for_builtin!(i8, 1);
impl_on_chain_for_builtin!(i16, 2);
impl_on_chain_for_builtin!(i32, 4);
impl_on_chain_for_builtin!(i64, 8);
impl_on_chain_for_builtin!(i128, 16);

impl<T: OnChain> OnChain for Vec<T> {
    fn _capacity(&self) -> u64 {
        let prefix = 8;
        self.iter().fold(0, |prev, item| prev + item._capacity()) + prefix
    }

    fn _to_bytes(&self) -> Vec<u8> {
        let mut total_capacity = 0;
        let mut bytes = vec![];
        self.iter().for_each(|element| {
            total_capacity += element._capacity();
            bytes.extend(element._to_bytes());
        });
        let mut res = total_capacity.to_le_bytes().to_vec();
        res.extend(bytes);
        res
    }

    fn _from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut result: Vec<T> = vec![];
        let mut bytes = bytes;
        while bytes.len() > 0 {
            let (item, left) = consume_and_decode::<T>(bytes)?;
            result.push(item);
            bytes = left;
        }
        Some(result)
    }

    fn _fixed_size() -> Option<u64> {
        None
    }
}

impl OnChain for String {
    fn _capacity(&self) -> u64 {
        self.as_bytes().len() as u64 + 8
    }

    fn _to_bytes(&self) -> Vec<u8> {
        let bytes = self.as_bytes().to_vec();
        let mut prefix = bytes.len().to_le_bytes().to_vec();
        prefix.extend(bytes);
        prefix
    }

    fn _from_bytes(bytes: &[u8]) -> Option<Self> {
        String::from_utf8(bytes.to_vec()).ok()
    }

    fn _fixed_size() -> Option<u64> {
        None
    }
}

pub fn consume_and_decode<T: OnChain>(bytes: &[u8]) -> Option<(T, &[u8])> {
    if let Some(capacity) = T::_fixed_size() {
        let end = capacity as usize;
        let item = T::_from_bytes(&bytes[0..end])?;
        Some((item, &bytes[end..]))
    } else {
        let size: [u8; 8] = bytes[0..8].try_into().unwrap();
        let end = usize::from_le_bytes(size);
        let item = T::_from_bytes(&bytes[8..end + 8])?;
        Some((item, &bytes[end + 8..]))
    }
}

#[cfg(test)]
mod tests {
    use super::consume_and_decode;
    use super::OnChain;
    use ckboots_derives::OnChain;
    use std::any::TypeId;

    #[test]
    fn builtin_onchain_test() {
        let bytes = 0u32._to_bytes();
        let (actual, left) = consume_and_decode::<u32>(&bytes).unwrap();
        assert_eq!(left.len(), 0);
        assert_eq!(actual, 0);
    }

    #[test]
    fn string_onchain_test() {
        let s = String::from("i am Tom");
        let bytes = s._to_bytes();
        let (actual, left) = consume_and_decode::<String>(&bytes).unwrap();
        assert_eq!(actual, s);
        assert_eq!(left.len(), 0);
    }

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

    #[test]
    fn type_id() {
        pub struct Person1 {
            pub age: u8,
        }
        pub struct Person2 {
            pub age: u8,
        }

        let person1_id = TypeId::of::<Person1>();
        let person2_id = TypeId::of::<Person2>();
        assert_ne!(person1_id, person2_id);
    }

    #[test]
    fn derive_onchain_struct() {
        #[derive(OnChain)]
        #[onchain(id = "person")]
        pub struct Person {
            pub age: u16,
            pub name: String,
        }

        let tom = Person {
            age: 12,
            name: String::from("Tom"),
        };
        let bytes = tom._to_bytes();
        let (actual, left) = consume_and_decode::<Person>(&bytes).unwrap();
        assert_eq!(actual.age, 12);
        assert_eq!(actual.name, "Tom");
        assert_eq!(left.len(), 0);
        assert_eq!(Person::_id(), Some("person"));
    }

    #[test]
    fn derive_onchain_enum() {
        #[derive(OnChain)]
        pub enum EnumTest {
            AA(String),
            BB(u32),
            CC,
        }
        let aa = EnumTest::AA(String::from("1"));
        let bytes = aa._to_bytes();
        let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
        assert_eq!(left.len(), 0);
        match actual {
            EnumTest::AA(s) => assert_eq!(s, "1"),
            _ => panic!(),
        }

        let bb = EnumTest::BB(123);
        let bytes = bb._to_bytes();
        let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
        assert_eq!(left.len(), 0);
        match actual {
            EnumTest::BB(n) => assert_eq!(n, 123),
            _ => panic!(),
        }

        let cc = EnumTest::CC;
        let bytes = cc._to_bytes();
        let (actual, left) = consume_and_decode::<EnumTest>(&bytes).unwrap();
        assert_eq!(left.len(), 0);
        match actual {
            EnumTest::CC => {}
            _ => panic!(),
        }
    }

    #[test]
    fn derive_onchain_user_input() {
        #[derive(OnChain)]
        #[onchain(user_input = true)]
        pub struct Test {}

        assert_eq!(Test::_user_input(), true);
    }
}
