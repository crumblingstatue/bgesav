use crate::SaveDatum;

pub struct HovercraftCondition {
    pub bitflags: u8,
}

pub struct HovercraftState {
    pub value: u8,
}

impl SaveDatum for HovercraftCondition {
    type Repr = u8;

    fn from_repr(bitflags: Self::Repr) -> Self {
        Self { bitflags }
    }

    fn to_repr(&self) -> Self::Repr {
        self.bitflags
    }
}

impl SaveDatum for HovercraftState {
    type Repr = u8;

    fn from_repr(value: Self::Repr) -> Self {
        Self { value }
    }

    fn to_repr(&self) -> Self::Repr {
        self.value
    }
}
