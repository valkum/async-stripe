#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionConsentCollection {
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    pub payment_method_reuse_agreement:
        Option<stripe_shared::PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,
    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    /// The Checkout.
    /// Session will determine whether to display an option to opt into promotional communication
    /// from the merchant depending on the customer's locale.
    /// Only available to US merchants and US customers.
    pub promotions: Option<stripe_shared::Promotions>,
    /// If set to `required`, it requires customers to accept the terms of service before being able to pay.
    pub terms_of_service: Option<stripe_shared::TermsOfService>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionConsentCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionConsentCollection").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionConsentCollectionBuilder {
    payment_method_reuse_agreement:
        Option<stripe_shared::PaymentPagesCheckoutSessionPaymentMethodReuseAgreement>,
    promotions: Option<stripe_shared::Promotions>,
    terms_of_service: Option<stripe_shared::TermsOfService>,
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

    impl Deserialize for PaymentPagesCheckoutSessionConsentCollection {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionConsentCollection>,
        builder: PaymentPagesCheckoutSessionConsentCollectionBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionConsentCollection> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionConsentCollectionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionConsentCollectionBuilder {
        type Out = PaymentPagesCheckoutSessionConsentCollection;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_reuse_agreement" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.payment_method_reuse_agreement,
                    )
                }
                "promotions" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.promotions)
                }
                "terms_of_service" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.terms_of_service)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_method_reuse_agreement: None, promotions: None, terms_of_service: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { payment_method_reuse_agreement, promotions, terms_of_service } =
                core::mem::replace(self, Self::deser_default());
            Some(Self::Out { payment_method_reuse_agreement, promotions, terms_of_service })
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

    impl ObjectDeser for PaymentPagesCheckoutSessionConsentCollection {
        type Builder = PaymentPagesCheckoutSessionConsentCollectionBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionConsentCollection {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// If set to `auto`, enables the collection of customer consent for promotional communications.
/// The Checkout.
/// Session will determine whether to display an option to opt into promotional communication
/// from the merchant depending on the customer's locale.
/// Only available to US merchants and US customers.
pub type PaymentPagesCheckoutSessionConsentCollectionPromotions = stripe_shared::Promotions;
/// If set to `required`, it requires customers to accept the terms of service before being able to pay.
pub type PaymentPagesCheckoutSessionConsentCollectionTermsOfService = stripe_shared::TermsOfService;
