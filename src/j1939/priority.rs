#[derive(Debug)]
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
    pub const HIGHEST: Priority = Priority::Zero;
    pub const DEFAULT: Priority = Priority::Six;
    pub const LOWEST: Priority = Priority::Seven;

    #[inline]
    pub fn is_highest(self) -> bool {
        self == Self::HIGHEST
    }

    #[inline]
    pub fn is_default(self) -> bool {
        self == Self::DEFAULT
    }

    #[inline]
    pub fn is_lowest(self) -> bool {
        self == Self::LOWEST
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::DEFAULT
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
