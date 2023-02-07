use super::SaveDatum;

#[derive(Debug)]
pub struct Mdisks {
    pub disks: [bool; 16],
}

impl SaveDatum for Mdisks {
    const OFFSET: usize = 600;
    type Repr = u16;

    fn from_repr(repr: Self::Repr) -> Self {
        dbg!(repr);
        let mut arr = [false; 16];
        for (i, elem) in arr.iter_mut().enumerate() {
            if repr >> i & 1 != 0 {
                *elem = true;
            }
        }
        Self { disks: arr }
    }

    fn to_repr(&self) -> Self::Repr {
        let mut repr = 0;
        for (i, has_disk) in self.disks.iter().copied().enumerate() {
            if has_disk {
                repr |= 1 << i;
            }
        }
        repr
    }
}
