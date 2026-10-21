#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingModeFlexible {
    /// Controls how invoices and invoice items display proration amounts and discount amounts.
    pub proration_discounts: Option<stripe_shared::ProrationDiscounts>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingModeFlexible {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingModeFlexible").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeFlexibleBuilder {
    proration_discounts: Option<stripe_shared::ProrationDiscounts>,
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

    impl Deserialize for SubscriptionsResourceBillingModeFlexible {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingModeFlexible>,
        builder: SubscriptionsResourceBillingModeFlexibleBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingModeFlexible> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingModeFlexibleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingModeFlexibleBuilder {
        type Out = SubscriptionsResourceBillingModeFlexible;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "proration_discounts" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.proration_discounts)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { proration_discounts: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { proration_discounts } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { proration_discounts })
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

    impl ObjectDeser for SubscriptionsResourceBillingModeFlexible {
        type Builder = SubscriptionsResourceBillingModeFlexibleBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingModeFlexible {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Controls how invoices and invoice items display proration amounts and discount amounts.
pub type SubscriptionsResourceBillingModeFlexibleProrationDiscounts =
    stripe_shared::ProrationDiscounts;
