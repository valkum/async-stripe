#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceLineItemTaxBreakdown {
    /// The amount of tax, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub amount: i64,
    pub jurisdiction: stripe_misc::TaxProductResourceJurisdiction,
    /// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
    pub sourcing: TaxProductResourceLineItemTaxBreakdownSourcing,
    /// Details regarding the rate for this tax.
    /// This field will be `null` when the tax is not imposed, for example if the product is exempt from tax.
    pub tax_rate_details: Option<stripe_misc::TaxProductResourceLineItemTaxRateDetails>,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: stripe_shared::TaxabilityReason,
    /// The amount on which tax is calculated, in the [smallest currency unit](https://docs.stripe.com/currencies#minor-units).
    pub taxable_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceLineItemTaxBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxProductResourceLineItemTaxBreakdown").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxProductResourceLineItemTaxBreakdownBuilder {
    amount: Option<i64>,
    jurisdiction: Option<stripe_misc::TaxProductResourceJurisdiction>,
    sourcing: Option<TaxProductResourceLineItemTaxBreakdownSourcing>,
    tax_rate_details: Option<stripe_misc::TaxProductResourceLineItemTaxRateDetails>,
    taxability_reason: Option<stripe_shared::TaxabilityReason>,
    taxable_amount: Option<i64>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TaxProductResourceLineItemTaxBreakdown {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceLineItemTaxBreakdown>,
        builder: TaxProductResourceLineItemTaxBreakdownBuilder,
    }

    impl Visitor for Place<TaxProductResourceLineItemTaxBreakdown> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceLineItemTaxBreakdownBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceLineItemTaxBreakdownBuilder {
        type Out = TaxProductResourceLineItemTaxBreakdown;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "jurisdiction" => Deserialize::begin(&mut self.jurisdiction),
                "sourcing" => Deserialize::begin(&mut self.sourcing),
                "tax_rate_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.tax_rate_details)
                }
                "taxability_reason" => Deserialize::begin(&mut self.taxability_reason),
                "taxable_amount" => Deserialize::begin(&mut self.taxable_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: None,
                jurisdiction: None,
                sourcing: None,
                tax_rate_details: None,
                taxability_reason: None,
                taxable_amount: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    amount: Some(amount),
                    jurisdiction: Some(jurisdiction),
                    sourcing: Some(sourcing),
                    tax_rate_details,
                    taxability_reason: Some(taxability_reason),
                    taxable_amount: Some(taxable_amount),
                } => Some(Self::Out {
                    amount,
                    jurisdiction,
                    sourcing,
                    tax_rate_details,
                    taxability_reason,
                    taxable_amount,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for TaxProductResourceLineItemTaxBreakdown {
        type Builder = TaxProductResourceLineItemTaxBreakdownBuilder;
    }

    impl FromValueOpt for TaxProductResourceLineItemTaxBreakdown {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Indicates whether the jurisdiction was determined by the origin (merchant's address) or destination (customer's address).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceLineItemTaxBreakdownSourcing {
    Destination,
    Origin,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceLineItemTaxBreakdownSourcing {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceLineItemTaxBreakdownSourcing::*;
        match self {
            Destination => "destination",
            Origin => "origin",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceLineItemTaxBreakdownSourcing {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceLineItemTaxBreakdownSourcing::*;
        match s {
            "destination" => Ok(Destination),
            "origin" => Ok(Origin),
            v => {
                stripe_types::warn_unknown_enum_value(
                    v,
                    "TaxProductResourceLineItemTaxBreakdownSourcing",
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxProductResourceLineItemTaxBreakdownSourcing))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceLineItemTaxBreakdownSourcing> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = TaxProductResourceLineItemTaxBreakdownSourcing::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt
    for TaxProductResourceLineItemTaxBreakdownSourcing
{
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceLineItemTaxBreakdownSourcing {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
/// The possible values for this field may be extended as new tax rules are supported.
pub type TaxProductResourceLineItemTaxBreakdownTaxabilityReason = stripe_shared::TaxabilityReason;
