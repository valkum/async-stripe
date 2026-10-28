/// The field kinds to be replaced in the forwarded request.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Replacements {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
    RequestSignature,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl Replacements {
    pub fn as_str(&self) -> &str {
        use Replacements::*;
        match self {
            CardCvc => "card_cvc",
            CardExpiry => "card_expiry",
            CardNumber => "card_number",
            CardholderName => "cardholder_name",
            RequestSignature => "request_signature",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for Replacements {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Replacements::*;
        match s {
            "card_cvc" => Ok(CardCvc),
            "card_expiry" => Ok(CardExpiry),
            "card_number" => Ok(CardNumber),
            "cardholder_name" => Ok(CardholderName),
            "request_signature" => Ok(RequestSignature),
            v => {
                stripe_types::warn_unknown_enum_value(v, "Replacements");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for Replacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for Replacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for Replacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(Replacements)).finish_non_exhaustive()
    }
}
impl serde::Serialize for Replacements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for Replacements {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<Replacements> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = Replacements::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt for Replacements {
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for Replacements {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
