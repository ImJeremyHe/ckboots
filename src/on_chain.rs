pub trait OnChain: Sized {
    fn _capacity(&self) -> u64;

    fn _id() -> Option<&'static str> {
        None
    }

    fn _to_bytes(&self) -> Vec<u8>;

    fn _from_bytes(bytes: &[u8]) -> Option<Self>;

    fn _fixed_size() -> Option<u64>;

    fn _eq(&self, other: &Self) -> bool;

    fn _default() -> Self;
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

            fn _eq(&self, other: &Self) -> bool {
                self == other
            }

            fn _default() -> Self {
                0
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
        let mut total_capacity: u64 = 0;
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

    fn _eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 0..self.len() {
            let t = self.get(i).unwrap();
            let o = other.get(i).unwrap();
            if !t._eq(o) {
                return false;
            }
        }

        return true;
    }

    fn _default() -> Self {
        vec![]
    }
}

// impl OnChain for String {
//     fn _capacity(&self) -> u64 {
//         self.as_bytes().len() as u64 + 8
//     }

//     fn _to_bytes(&self) -> Vec<u8> {
//         let bytes = self.as_bytes().to_vec();
//         let mut prefix = bytes.len().to_le_bytes().to_vec();
//         prefix.extend(bytes);
//         prefix
//     }

//     fn _from_bytes(bytes: &[u8]) -> Option<Self> {
//         String::from_utf8(bytes.to_vec()).ok()
//     }

//     fn _fixed_size() -> Option<u64> {
//         None
//     }

//     fn _eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

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
