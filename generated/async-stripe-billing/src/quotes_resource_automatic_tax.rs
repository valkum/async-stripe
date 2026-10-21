#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceAutomaticTax {
    /// Automatically calculate taxes
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
    /// The tax provider powering automatic tax.
    pub provider: Option<String>,
    /// The status of the most recent automated tax calculation for this quote.
    pub status: Option<stripe_shared::Status>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceAutomaticTax").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceAutomaticTaxBuilder {
    enabled: Option<bool>,
    liability: Option<stripe_shared::ConnectAccountReference>,
    provider: Option<Option<String>>,
    status: Option<stripe_shared::Status>,
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

    impl Deserialize for QuotesResourceAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceAutomaticTax>,
        builder: QuotesResourceAutomaticTaxBuilder,
    }

    impl Visitor for Place<QuotesResourceAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceAutomaticTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceAutomaticTaxBuilder {
        type Out = QuotesResourceAutomaticTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "liability" => stripe_types::miniserde_helpers::begin_nullable(&mut self.liability),
                "provider" => Deserialize::begin(&mut self.provider),
                "status" => stripe_types::miniserde_helpers::begin_nullable(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: None, liability: None, provider: Some(None), status: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { enabled: Some(enabled), liability, provider: Some(provider), status } => {
                    Some(Self::Out { enabled, liability, provider, status })
                }
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

    impl ObjectDeser for QuotesResourceAutomaticTax {
        type Builder = QuotesResourceAutomaticTaxBuilder;
    }

    impl FromValueOpt for QuotesResourceAutomaticTax {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The status of the most recent automated tax calculation for this quote.
pub type QuotesResourceAutomaticTaxStatus = stripe_shared::Status;
