#[derive(Copy, Clone)]
pub enum ChallengeEnum {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Copy, Clone, Default)]
pub struct Challenge(Option<ChallengeEnum>);

impl Challenge {
    #[inline]
    pub fn rotate(self) -> Self {
        let Self(challenge) = self;
        match challenge {
            None => Some(ChallengeEnum::First),
            Some(ChallengeEnum::First) => Some(ChallengeEnum::Second),
            Some(ChallengeEnum::Second) => Some(ChallengeEnum::Third),
            Some(ChallengeEnum::Third) => Some(ChallengeEnum::Fourth),
            Some(ChallengeEnum::Fourth) => None,
        }
        .into()
    }
}

impl From<Option<ChallengeEnum>> for Challenge {
    #[inline]
    fn from(val: Option<ChallengeEnum>) -> Self {
        Self(val)
    }
}

impl From<Challenge> for Option<ChallengeEnum> {
    #[inline]
    fn from(val: Challenge) -> Self {
        let Challenge(val) = val;
        val
    }
}
