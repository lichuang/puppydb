// Copyright (c) 2025 Lichuang
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;

use bytes::Bytes;

pub const VERSION_DEFAULT: u64 = 0;

pub struct Key<T: AsRef<[u8]>> {
    key: T,
    version: u64,
}

pub type KeyBytes = Key<Bytes>;
pub type KeySlice<'a> = Key<&'a [u8]>;

impl<T: AsRef<[u8]>> Key<T> {
    pub fn version(&self) -> u64 {
        self.version
    }

    pub fn key_ref(&self) -> &[u8] {
        self.key.as_ref()
    }

    pub fn key_len(&self) -> usize {
        self.key.as_ref().len()
    }

    pub fn raw_len(&self) -> usize {
        self.key.as_ref().len() + std::mem::size_of::<u64>()
    }
}

impl KeyBytes {
    pub fn new(key: Bytes, version: u64) -> Self {
        Self { key, version }
    }

    pub fn to_key_slice(&self) -> KeySlice {
        KeySlice {
            key: self.key.as_ref(),
            version: self.version,
        }
    }
}

impl<'a> KeySlice<'a> {
    pub fn to_key_bytes(&self) -> KeyBytes {
        KeyBytes::new(self.key.to_vec().into(), self.version)
    }
}

impl<T: AsRef<[u8]> + Debug> Debug for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)
    }
}

impl<T: AsRef<[u8]> + Default> Default for Key<T> {
    fn default() -> Self {
        Self {
            key: T::default(),
            version: VERSION_DEFAULT,
        }
    }
}

impl<T: AsRef<[u8]> + PartialEq> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.key_ref(), self.version()).eq(&(other.key_ref(), other.version()))
    }
}

impl<T: AsRef<[u8]> + Eq> Eq for Key<T> {}

impl<T: AsRef<[u8]> + PartialOrd> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.key_ref(), self.version()).partial_cmp(&(other.key_ref(), other.version()))
    }
}

impl<T: AsRef<[u8]> + Ord> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.key_ref(), self.version()).cmp(&(other.key_ref(), other.version()))
    }
}

#[cfg(test)]
mod tests {
    use super::KeyBytes;
    use bytes::Bytes;

    #[test]
    fn test_compare_key() {
        let key_a = KeyBytes::new(Bytes::from("hello"), 1);
        let key_b = KeyBytes::new(Bytes::from("world"), 1);
        let key_c = KeyBytes::new(Bytes::from("hello"), 2);
        let key_d = KeyBytes::new(Bytes::from("hello"), 2);
        assert!(key_c > key_a);
        assert!(key_b > key_a);
        assert!(key_b > key_a);
        assert_eq!(key_c, key_d);
    }
}
