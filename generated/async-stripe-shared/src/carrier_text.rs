/// The reason(s) the carrier text was rejected.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CarrierText {
    GeographicLocation,
    Inappropriate,
    NetworkName,
    NonFiatCurrency,
    Other,
    OtherEntity,
    PromotionalMaterial,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CarrierText {
    pub fn as_str(&self) -> &str {
        use CarrierText::*;
        match self {
            GeographicLocation => "geographic_location",
            Inappropriate => "inappropriate",
            NetworkName => "network_name",
            NonFiatCurrency => "non_fiat_currency",
            Other => "other",
            OtherEntity => "other_entity",
            PromotionalMaterial => "promotional_material",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CarrierText {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CarrierText::*;
        match s {
            "geographic_location" => Ok(GeographicLocation),
            "inappropriate" => Ok(Inappropriate),
            "network_name" => Ok(NetworkName),
            "non_fiat_currency" => Ok(NonFiatCurrency),
            "other" => Ok(Other),
            "other_entity" => Ok(OtherEntity),
            "promotional_material" => Ok(PromotionalMaterial),
            v => {
                stripe_types::warn_unknown_enum_value(v, "CarrierText");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CarrierText {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CarrierText)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CarrierText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CarrierText {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CarrierText> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = CarrierText::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt for CarrierText {
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CarrierText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
