#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AmazonPayUnderlyingPaymentMethodFundingDetails {
    pub card: Option<stripe_shared::PaymentMethodDetailsPassthroughCard>,
    /// funding type of the underlying payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<stripe_shared::PaymentIntentProcessingType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AmazonPayUnderlyingPaymentMethodFundingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AmazonPayUnderlyingPaymentMethodFundingDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder {
    card: Option<stripe_shared::PaymentMethodDetailsPassthroughCard>,
    type_: Option<stripe_shared::PaymentIntentProcessingType>,
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

    impl Deserialize for AmazonPayUnderlyingPaymentMethodFundingDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AmazonPayUnderlyingPaymentMethodFundingDetails>,
        builder: AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder,
    }

    impl Visitor for Place<AmazonPayUnderlyingPaymentMethodFundingDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder {
        type Out = AmazonPayUnderlyingPaymentMethodFundingDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "type" => stripe_types::miniserde_helpers::begin_nullable(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { card: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { card, type_ } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { card, type_ })
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

    impl ObjectDeser for AmazonPayUnderlyingPaymentMethodFundingDetails {
        type Builder = AmazonPayUnderlyingPaymentMethodFundingDetailsBuilder;
    }

    impl FromValueOpt for AmazonPayUnderlyingPaymentMethodFundingDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// funding type of the underlying payment method.
pub type AmazonPayUnderlyingPaymentMethodFundingDetailsType =
    stripe_shared::PaymentIntentProcessingType;
