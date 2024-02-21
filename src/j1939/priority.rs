use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePriorityError(u8);

impl std::fmt::Display for ParsePriorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parse '{:?}' failed because the permitted priority value is between 0 and 7!",
            self
        )
    }
}

/// The priority of a J1939 message
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Priority {
    /// You may also use [`Priority::HIGHEST`] as an alias
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    /// You may also use [`Priority::DEFAULT`] as an alias
    Six = 6,
    /// You may also use [`Priority::LOWEST`] as an alias
    Seven = 7,
}

impl Priority {
    /// The number of bits used to represent the priority
    pub const BIT_LENGTH: u8 = 3;
    /// The highest priority
    pub const HIGHEST: Priority = Priority::Zero;
    /// The default priority
    pub const DEFAULT: Priority = Priority::Six;
    /// The lowest priority
    pub const LOWEST: Priority = Priority::Seven;

    /// Returns if the priority is the [Priority::HIGHEST] / [Priority::Zero] priority
    #[inline]
    pub fn is_highest(self) -> bool {
        self == Self::HIGHEST
    }

    /// Returns if the priority is the [Priority::DEFAULT] / [Priority::Six] priority
    #[inline]
    pub fn is_default(self) -> bool {
        self == Self::DEFAULT
    }

    /// Returns if the priority is the [Priority::LOWEST] / [Priority::Seven] priority
    #[inline]
    pub fn is_lowest(self) -> bool {
        self == Self::LOWEST
    }

    /// Get the raw value of the priority
    #[inline]
    pub fn raw(self) -> u8 {
        self as u8
    }

    /// Get the raw bits of the priority
    pub fn raw_bits(&self) -> [bool; 3] {
        let priority_raw = self.raw();
        let mut priority_bits = priority_raw.view_bits::<Lsb0>().to_bitvec();
        priority_bits.truncate(3);
        priority_bits.reverse();
        [priority_bits[0], priority_bits[1], priority_bits[2]]
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::DEFAULT
    }
}

impl From<Priority> for u8 {
    fn from(priority: Priority) -> Self {
        priority.raw()
    }
}

impl From<Priority> for [bool; 3] {
    fn from(priority: Priority) -> Self {
        priority.raw_bits()
    }
}

impl From<[bool; 3]> for Priority {
    fn from(raw_priority: [bool; 3]) -> Self {
        let mut priority_bits: BitVec<u8> = BitVec::new();
        priority_bits.extend(raw_priority.iter());
        priority_bits.reverse();
        let priority_raw = priority_bits.load_be::<u8>();
        match priority_raw {
            0x0 => Priority::Zero,
            0x1 => Priority::One,
            0x2 => Priority::Two,
            0x3 => Priority::Three,
            0x4 => Priority::Four,
            0x5 => Priority::Five,
            0x6 => Priority::Six,
            0x7 => Priority::Seven,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Priority {
    type Error = ParsePriorityError;

    fn try_from(raw_priority: u8) -> Result<Self, Self::Error> {
        match raw_priority {
            0x0 => Ok(Priority::Zero),
            0x1 => Ok(Priority::One),
            0x2 => Ok(Priority::Two),
            0x3 => Ok(Priority::Three),
            0x4 => Ok(Priority::Four),
            0x5 => Ok(Priority::Five),
            0x6 => Ok(Priority::Six),
            0x7 => Ok(Priority::Seven),
            _ => Err(ParsePriorityError(raw_priority)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::j1939::priority::ParsePriorityError;
    use crate::j1939::Priority;

    #[test]
    fn test_try_from_u8_for_priority() {
        assert_eq!(Priority::try_from(0x0).unwrap(), Priority::Zero);
        assert_eq!(Priority::try_from(0x1).unwrap(), Priority::One);
        assert_eq!(Priority::try_from(0x2).unwrap(), Priority::Two);
        assert_eq!(Priority::try_from(0x3).unwrap(), Priority::Three);
        assert_eq!(Priority::try_from(0x4).unwrap(), Priority::Four);
        assert_eq!(Priority::try_from(0x5).unwrap(), Priority::Five);
        assert_eq!(Priority::try_from(0x6).unwrap(), Priority::Six);
        assert_eq!(Priority::try_from(0x7).unwrap(), Priority::Seven);
        assert_eq!(Priority::try_from(0x8).unwrap_err(), ParsePriorityError(8));
    }

    #[test]
    fn test_from_bool_array_for_priority() {
        assert_eq!(Priority::from([false, false, false]), Priority::Zero);
        assert_eq!(Priority::from([false, false, true]), Priority::One);
        assert_eq!(Priority::from([false, true, false]), Priority::Two);
        assert_eq!(Priority::from([false, true, true]), Priority::Three);
        assert_eq!(Priority::from([true, false, false]), Priority::Four);
        assert_eq!(Priority::from([true, false, true]), Priority::Five);
        assert_eq!(Priority::from([true, true, false]), Priority::Six);
        assert_eq!(Priority::from([true, true, true]), Priority::Seven);
    }

    #[test]
    fn test_priority() {
        assert_eq!(Priority::HIGHEST, Priority::Zero);
        assert_eq!(Priority::DEFAULT, Priority::Six);
        assert_eq!(Priority::LOWEST, Priority::Seven);

        assert_eq!(Priority::HIGHEST.is_highest(), true);
        assert_eq!(Priority::HIGHEST.is_default(), false);
        assert_eq!(Priority::HIGHEST.is_lowest(), false);

        let priority = Priority::try_from(0x0).unwrap();
        assert_eq!(priority.raw(), 0x0);
        assert_eq!(priority.raw_bits(), [false, false, false]);
    }

    #[test]
    fn test_raw_priority() {
        let priority = Priority::Five;
        assert_eq!(u8::from(priority), 0x5);
        assert_eq!(priority.raw(), 0x5);

        let priority = Priority::One;
        assert_eq!(u8::from(priority), 0x1);
        assert_eq!(priority.raw(), 0x1);
    }

    #[test]
    fn test_raw_bits_priority() {
        assert_eq!(Priority::Four.raw_bits(), [true, false, false]);
        assert_eq!(Priority::Six.raw_bits(), [true, true, false]);
        assert_eq!(Priority::One.raw_bits(), [false, false, true]);
    }
}
