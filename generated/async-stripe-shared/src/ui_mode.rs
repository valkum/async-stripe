/// The UI mode of the Session. Defaults to `hosted_page`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiMode {
    Elements,
    EmbeddedPage,
    Form,
    HostedPage,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UiMode {
    pub fn as_str(&self) -> &str {
        use UiMode::*;
        match self {
            Elements => "elements",
            EmbeddedPage => "embedded_page",
            Form => "form",
            HostedPage => "hosted_page",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UiMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UiMode::*;
        match s {
            "elements" => Ok(Elements),
            "embedded_page" => Ok(EmbeddedPage),
            "form" => Ok(Form),
            "hosted_page" => Ok(HostedPage),
            v => {
                stripe_types::warn_unknown_enum_value(v, "UiMode");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UiMode)).finish_non_exhaustive()
    }
}
impl serde::Serialize for UiMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for UiMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<UiMode> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = UiMode::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt for UiMode {
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UiMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
