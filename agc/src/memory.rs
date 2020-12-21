use crate::word::{W10, W15, W16, W3, W6, W8};
use std::fmt;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::ops::{Index, IndexMut};
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MemoryWord {
    inner: W16,
}

impl MemoryWord {
    pub fn new(word: W15, parity: bool) -> Self {
        let mut inner = W16::from(word);

        if parity {
            inner.set(15, true);
        }

        Self { inner }
    }

    pub fn with_proper_parity(word: W15) -> Self {
        Self::new(word, word.count_ones() % 2 == 0)
    }

    pub fn with_wrong_parity(word: W15) -> Self {
        Self::new(word, word.count_ones() % 2 != 0)
    }

    pub fn value(&self) -> W15 {
        W15::from(self.inner)
    }

    pub fn parity(&self) -> bool {
        self.inner.get(15)
    }

    pub fn is_valid(&self) -> bool {
        self.inner.count_ones() % 2 != 0
    }

    pub fn as_register_value(&self) -> W16 {
        let mut result = W16::from(self.value());
        result.set(15, result.get(14));
        result
    }
}

impl fmt::Display for MemoryWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parity_char = match self.is_valid() {
            true => '|',
            false => '!',
        };
        write!(f, "{}{}{}", self.parity() as u8, parity_char, self.value())
    }
}

impl fmt::Debug for MemoryWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// Size of each erasable memory bank (in words)
pub const ERASABLE_BANK_SIZE: usize = 256;
/// Number of erasable memory banks
pub const ERASABLE_NUM_BANKS: usize = 8;

pub struct ErasableStorageBank {
    pub inner: Vec<MemoryWord>,
}

impl ErasableStorageBank {
    pub fn new() -> Self {
        Self {
            inner: vec![MemoryWord::with_proper_parity(W15::zero()); ERASABLE_BANK_SIZE],
        }
    }

    pub fn read(&self, index: W8) -> MemoryWord {
        self[index]
    }

    pub fn write(&mut self, index: W8, value: MemoryWord) {
        self[index] = value;
    }
}

impl Index<W8> for ErasableStorageBank {
    type Output = MemoryWord;

    fn index(&self, index: W8) -> &Self::Output {
        &self.inner[index.as_u16() as usize]
    }
}

impl IndexMut<W8> for ErasableStorageBank {
    fn index_mut(&mut self, index: W8) -> &mut Self::Output {
        &mut self.inner[index.as_u16() as usize]
    }
}

/// Fixed storage is made of 8 banks of 256 words of read-write memory.
pub struct ErasableStorage {
    pub banks: Vec<ErasableStorageBank>,
}

impl ErasableStorage {
    pub fn new() -> Self {
        Self {
            banks: vec![
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
                ErasableStorageBank::new(),
            ],
        }
    }

    pub fn read(&self, bank: W3, address: W8) -> MemoryWord {
        self[bank][address]
    }

    pub fn write(&mut self, bank: W3, address: W8, value: MemoryWord) {
        self[bank][address] = value
    }
}

impl Index<W3> for ErasableStorage {
    type Output = ErasableStorageBank;

    fn index(&self, index: W3) -> &ErasableStorageBank {
        &self.banks[index.as_u16() as usize]
    }
}

impl IndexMut<W3> for ErasableStorage {
    fn index_mut(&mut self, index: W3) -> &mut ErasableStorageBank {
        &mut self.banks[index.as_u16() as usize]
    }
}

/// Size of each fixed memory bank (in words)
pub const FIXED_BANK_SIZE: usize = 1024;
/// Number of fixed memory banks
pub const FIXED_NUM_BANKS: usize = 36;

pub struct FixedStorageBank {
    pub inner: Vec<MemoryWord>,
}

impl FixedStorageBank {
    pub fn new() -> Self {
        Self {
            inner: vec![MemoryWord::with_proper_parity(W15::zero()); FIXED_BANK_SIZE],
        }
    }

    pub fn read(&self, index: W10) -> MemoryWord {
        self[index]
    }

    pub fn write(&mut self, index: W10, value: MemoryWord) {
        self[index] = value;
    }
}

impl Index<W10> for FixedStorageBank {
    type Output = MemoryWord;

    fn index(&self, index: W10) -> &Self::Output {
        &self.inner[index.as_u16() as usize]
    }
}

impl IndexMut<W10> for FixedStorageBank {
    fn index_mut(&mut self, index: W10) -> &mut Self::Output {
        &mut self.inner[index.as_u16() as usize]
    }
}

/// Fixed storage is made of 36 banks of 1024 words of read-only memory.
pub struct FixedStorage {
    pub banks: Vec<FixedStorageBank>,
}

impl FixedStorage {
    pub fn new() -> Self {
        Self {
            banks: vec![
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
                FixedStorageBank::new(),
            ],
        }
    }

    pub fn read(&self, bank: W6, address: W10) -> MemoryWord {
        self[bank][address]
    }

    pub fn write(&mut self, bank: W6, address: W10, value: MemoryWord) {
        self[bank][address] = value
    }
}

impl Index<W6> for FixedStorage {
    type Output = FixedStorageBank;

    fn index(&self, index: W6) -> &FixedStorageBank {
        assert!((index.as_u16() as usize) < FIXED_NUM_BANKS);

        &self.banks[index.as_u16() as usize]
    }
}

impl IndexMut<W6> for FixedStorage {
    fn index_mut(&mut self, index: W6) -> &mut FixedStorageBank {
        assert!((index.as_u16() as usize) < FIXED_NUM_BANKS);

        &mut self.banks[index.as_u16() as usize]
    }
}

pub fn load_yayul_img_file<P: AsRef<Path>>(
    path: P,
) -> Result<FixedStorage, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;

    // Check file size
    if file.metadata()?.len() != (FIXED_NUM_BANKS * FIXED_BANK_SIZE * 2) as u64 {
        return Err(Error::new(ErrorKind::InvalidData, "invalid yayul file size").into());
    }

    let mut storage = FixedStorage::new();

    // Fill in the fixed storage
    for bank in 0..FIXED_NUM_BANKS {
        let bank_corrected = match bank {
            0 => 2,
            1 => 3,
            2 => 0,
            3 => 1,
            b => b,
        };

        let mut buf = [0; FIXED_BANK_SIZE * 2];
        file.read_exact(&mut buf)?;

        for address in 0..FIXED_BANK_SIZE {
            let msb = buf[address * 2] as u16;
            let lsb = buf[address * 2 + 1] as u16;
            let value = (msb << 7) | (lsb >> 1);
            storage.write(
                W6::from(bank_corrected as u16),
                W10::from(address as u16),
                MemoryWord::with_proper_parity(value.into()),
            )
        }
    }

    Ok(storage)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::catch_unwind;
    use std::path::PathBuf;

    #[test]
    fn memoryword_parity() {
        let word = MemoryWord::with_proper_parity(W15::from(0o12346));
        assert_eq!(word.value(), W15::from(0o12346));
        assert_eq!(word.parity(), false);
        assert!(word.is_valid());

        let word = MemoryWord::with_wrong_parity(W15::from(0o12346));
        assert_eq!(word.value(), W15::from(0o12346));
        assert_eq!(word.parity(), true);
        assert!(!word.is_valid());

        let word = MemoryWord::with_proper_parity(W15::from(0o12347));
        assert_eq!(word.value(), W15::from(0o12347));
        assert_eq!(word.parity(), true);
        assert!(word.is_valid());

        let word = MemoryWord::with_wrong_parity(W15::from(0o12347));
        assert_eq!(word.value(), W15::from(0o12347));
        assert_eq!(word.parity(), false);
        assert!(!word.is_valid());
    }

    #[test]
    fn memoryword_display() {
        let word = MemoryWord::with_proper_parity(W15::from(0o12346));
        assert_eq!(format!("{}", word), "0|12346");
        assert_eq!(format!("{:?}", word), "0|12346");

        let word = MemoryWord::with_wrong_parity(W15::from(0o12346));
        assert_eq!(format!("{}", word), "1!12346");
        assert_eq!(format!("{:?}", word), "1!12346");
    }

    #[test]
    fn erasablestoragebank_read_write() {
        let mut bank = ErasableStorageBank::new();

        assert_eq!(
            bank.read(W8::from(100)),
            MemoryWord::with_proper_parity(W15::zero())
        );
        bank.write(W8::from(100), MemoryWord::with_proper_parity(W15::from(76)));
        assert_eq!(
            bank.read(W8::from(100)),
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn erasablestoragebank_index() {
        let mut bank = ErasableStorageBank::new();

        assert_eq!(
            bank[W8::from(100)],
            MemoryWord::with_proper_parity(W15::zero())
        );
        bank[W8::from(100)] = MemoryWord::with_proper_parity(W15::from(76));
        assert_eq!(
            bank[W8::from(100)],
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn erasablestorage_read_write() {
        let mut storage = ErasableStorage::new();

        assert_eq!(
            storage.read(W3::from(3), W8::from(100)),
            MemoryWord::with_proper_parity(W15::zero())
        );
        storage.write(
            W3::from(3),
            W8::from(100),
            MemoryWord::with_proper_parity(W15::from(76)),
        );
        assert_eq!(
            storage.read(W3::from(3), W8::from(100)),
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn erasablestorage_index() {
        let mut storage = ErasableStorage::new();

        assert_eq!(
            storage[W3::from(3)][W8::from(100)],
            MemoryWord::with_proper_parity(W15::zero())
        );
        storage[W3::from(3)][W8::from(100)] = MemoryWord::with_proper_parity(W15::from(76));
        assert_eq!(
            storage[W3::from(3)][W8::from(100)],
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn fixedstoragebank_read_write() {
        let mut bank = ErasableStorageBank::new();

        assert_eq!(
            bank.read(W8::from(100)),
            MemoryWord::with_proper_parity(W15::zero())
        );
        bank.write(W8::from(100), MemoryWord::with_proper_parity(W15::from(76)));
        assert_eq!(
            bank.read(W8::from(100)),
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn fixedstoragebank_index() {
        let mut bank = FixedStorageBank::new();

        assert_eq!(
            bank[W10::from(100)],
            MemoryWord::with_proper_parity(W15::zero())
        );
        bank[W10::from(100)] = MemoryWord::with_proper_parity(W15::from(76));
        assert_eq!(
            bank[W10::from(100)],
            MemoryWord::with_proper_parity(W15::from(76))
        );
    }

    #[test]
    fn fixedstorage_read_write() {
        let mut storage = FixedStorage::new();

        assert_eq!(
            storage.read(W6::from(3), W10::from(100)),
            MemoryWord::with_proper_parity(W15::zero())
        );
        storage.write(
            W6::from(3),
            W10::from(100),
            MemoryWord::with_proper_parity(W15::from(76)),
        );
        assert_eq!(
            storage.read(W6::from(3), W10::from(100)),
            MemoryWord::with_proper_parity(W15::from(76))
        );

        assert!(catch_unwind(|| storage.read(W6::from(36), W10::from(100))).is_err());
        assert!(catch_unwind(move || storage.write(
            W6::from(36),
            W10::from(100),
            MemoryWord::with_proper_parity(W15::from(76))
        ))
        .is_err());
    }

    #[test]
    fn fixedstorage_index() {
        let mut storage = FixedStorage::new();

        assert_eq!(
            storage[W6::from(3)][W10::from(100)],
            MemoryWord::with_proper_parity(W15::zero())
        );
        storage[W6::from(3)][W10::from(100)] = MemoryWord::with_proper_parity(W15::from(76));
        assert_eq!(
            storage[W6::from(3)][W10::from(100)],
            MemoryWord::with_proper_parity(W15::from(76))
        );

        assert!(catch_unwind(|| storage[W6::from(36)][W10::from(100)]).is_err());
        assert!(catch_unwind(move || storage[W6::from(36)][W10::from(100)] =
            MemoryWord::with_proper_parity(W15::from(76)))
        .is_err());
    }

    #[test]
    fn load_yayul_aurora12() {
        let mut filepath = PathBuf::from("");
        filepath.push("..");
        filepath.push("listings");
        filepath.push("Aurora12.bin");

        let storage = load_yayul_img_file(filepath).unwrap();
        assert_eq!(
            storage[16.into()][2.into()],
            MemoryWord::with_proper_parity(0x02FC.into())
        ); // Address 0x8004 in file
        assert_eq!(
            storage[0.into()][0.into()],
            MemoryWord::with_proper_parity(0x3C72.into())
        ); // Address 0x1000 in file
        assert_eq!(
            storage[1.into()][0.into()],
            MemoryWord::with_proper_parity(0x0431.into())
        ); // Address 0x1800 in file
        assert_eq!(
            storage[2.into()][0.into()],
            MemoryWord::with_proper_parity(0x0004.into())
        ); // Address 0x0000 in file
        assert_eq!(
            storage[3.into()][0.into()],
            MemoryWord::with_proper_parity(0x0006.into())
        ); // Address 0x0800 in file
    }
}
