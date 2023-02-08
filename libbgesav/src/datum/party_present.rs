use super::SaveDatum;

pub struct PartyPresent {
    pub peyj: bool,
    pub double_h: bool,
    pub alpha_soldier: bool,
}

impl SaveDatum for PartyPresent {
    type Repr = u8;

    fn from_repr(repr: Self::Repr) -> Self {
        Self {
            peyj: (repr & 0b0001_0000) >> 4 == 1,
            double_h: (repr & 0b0010_0000) >> 5 == 1,
            alpha_soldier: (repr & 0b0100_0000) >> 6 == 1,
        }
    }

    fn to_repr(&self) -> Self::Repr {
        let mut repr = 0;
        if self.peyj {
            repr |= 0b0001_0000;
        }
        if self.double_h {
            repr |= 0b0010_0000;
        }
        if self.alpha_soldier {
            repr |= 0b0100_0000;
        }
        repr
    }
}
