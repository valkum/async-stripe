#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceAfterCompletion {
    pub hosted_confirmation:
        Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    pub redirect: Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>,
    /// The specified behavior after the purchase is complete.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreatePaymentLinkAfterCompletionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceAfterCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceAfterCompletion").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceAfterCompletionBuilder {
    hosted_confirmation:
        Option<stripe_shared::PaymentLinksResourceCompletionBehaviorConfirmationPage>,
    redirect: Option<stripe_shared::PaymentLinksResourceCompletionBehaviorRedirect>,
    type_: Option<stripe_shared::CreatePaymentLinkAfterCompletionType>,
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

    impl Deserialize for PaymentLinksResourceAfterCompletion {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceAfterCompletion>,
        builder: PaymentLinksResourceAfterCompletionBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceAfterCompletion> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceAfterCompletionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourceAfterCompletionBuilder {
        type Out = PaymentLinksResourceAfterCompletion;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "hosted_confirmation" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.hosted_confirmation)
                }
                "redirect" => stripe_types::miniserde_helpers::begin_nullable(&mut self.redirect),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { hosted_confirmation: None, redirect: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { hosted_confirmation, redirect, type_: Some(type_) } => {
                    Some(Self::Out { hosted_confirmation, redirect, type_ })
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

    impl ObjectDeser for PaymentLinksResourceAfterCompletion {
        type Builder = PaymentLinksResourceAfterCompletionBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceAfterCompletion {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The specified behavior after the purchase is complete.
pub type PaymentLinksResourceAfterCompletionType =
    stripe_shared::CreatePaymentLinkAfterCompletionType;
