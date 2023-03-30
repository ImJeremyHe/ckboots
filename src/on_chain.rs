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

// It is almost the same as
// #[derive(OnChain)]
pub struct OnChainWrapper {
    pub idx: u8,
    pub data: Vec<u8>,
}

impl OnChain for OnChainWrapper {
    fn _capacity(&self) -> u64 {
        self.idx._capacity() + self.data._capacity()
    }
    fn _to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self._capacity() as usize);
        result.extend(<u8 as OnChain>::_to_bytes(&self.idx));
        result.extend(<Vec<u8> as OnChain>::_to_bytes(&self.data));
        if let Some(_) = OnChainWrapper::_fixed_size() {
            result
        } else {
            let mut prefix: Vec<u8> = result.len().to_le_bytes().to_vec();
            prefix.extend(result);
            prefix
        }
    }
    fn _from_bytes(bytes: &[u8]) -> Option<Self> {
        let left = bytes;
        let (idx, left) = consume_and_decode::<u8>(left)?;
        let (data, _) = consume_and_decode::<Vec<u8>>(left)?;
        Some(Self { idx, data })
    }
    fn _fixed_size() -> Option<u64> {
        let size = <u8 as OnChain>::_fixed_size()? + <Vec<u8> as OnChain>::_fixed_size()?;
        Some(size)
    }
    fn _id() -> Option<&'static str> {
        None
    }
    fn _eq(&self, other: &Self) -> bool {
        if !self.idx._eq(&other.idx) {
            return false;
        }
        if !self.data._eq(&other.data) {
            return false;
        }
        true
    }
    fn _default() -> Self {
        Self {
            idx: <u8 as OnChain>::_default(),
            data: <Vec<u8> as OnChain>::_default(),
        }
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
