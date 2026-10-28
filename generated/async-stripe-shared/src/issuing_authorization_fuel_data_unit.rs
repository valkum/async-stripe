#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingAuthorizationFuelDataUnit {
    ChargingMinute,
    ImperialGallon,
    Kilogram,
    KilowattHour,
    Liter,
    Other,
    Pound,
    UsGallon,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingAuthorizationFuelDataUnit {
    pub fn as_str(&self) -> &str {
        use IssuingAuthorizationFuelDataUnit::*;
        match self {
            ChargingMinute => "charging_minute",
            ImperialGallon => "imperial_gallon",
            Kilogram => "kilogram",
            KilowattHour => "kilowatt_hour",
            Liter => "liter",
            Other => "other",
            Pound => "pound",
            UsGallon => "us_gallon",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingAuthorizationFuelDataUnit {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingAuthorizationFuelDataUnit::*;
        match s {
            "charging_minute" => Ok(ChargingMinute),
            "imperial_gallon" => Ok(ImperialGallon),
            "kilogram" => Ok(Kilogram),
            "kilowatt_hour" => Ok(KilowattHour),
            "liter" => Ok(Liter),
            "other" => Ok(Other),
            "pound" => Ok(Pound),
            "us_gallon" => Ok(UsGallon),
            v => {
                stripe_types::warn_unknown_enum_value(v, "IssuingAuthorizationFuelDataUnit");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingAuthorizationFuelDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingAuthorizationFuelDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingAuthorizationFuelDataUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingAuthorizationFuelDataUnit)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingAuthorizationFuelDataUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingAuthorizationFuelDataUnit {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingAuthorizationFuelDataUnit> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = IssuingAuthorizationFuelDataUnit::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt for IssuingAuthorizationFuelDataUnit {
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingAuthorizationFuelDataUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
