#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentSettings {
    /// ID of the mandate to be used for this invoice.
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    pub default_mandate: Option<String>,
    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    pub payment_method_options: Option<stripe_shared::InvoicesPaymentMethodOptions>,
    /// The list of payment method types (e.g.
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<stripe_shared::PaymentMethodTypes>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesPaymentSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesPaymentSettings").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesPaymentSettingsBuilder {
    default_mandate: Option<Option<String>>,
    payment_method_options: Option<stripe_shared::InvoicesPaymentMethodOptions>,
    payment_method_types: Option<Option<Vec<stripe_shared::PaymentMethodTypes>>>,
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

    impl Deserialize for InvoicesPaymentSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesPaymentSettings>,
        builder: InvoicesPaymentSettingsBuilder,
    }

    impl Visitor for Place<InvoicesPaymentSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesPaymentSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesPaymentSettingsBuilder {
        type Out = InvoicesPaymentSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "default_mandate" => Deserialize::begin(&mut self.default_mandate),
                "payment_method_options" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.payment_method_options,
                ),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                default_mandate: Some(None),
                payment_method_options: None,
                payment_method_types: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    default_mandate: Some(default_mandate),
                    payment_method_options,
                    payment_method_types: Some(payment_method_types),
                } => Some(Self::Out {
                    default_mandate,
                    payment_method_options,
                    payment_method_types,
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

    impl ObjectDeser for InvoicesPaymentSettings {
        type Builder = InvoicesPaymentSettingsBuilder;
    }

    impl FromValueOpt for InvoicesPaymentSettings {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The list of payment method types (e.g.
/// card) to provide to the invoice’s PaymentIntent.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
pub type InvoicesPaymentSettingsPaymentMethodTypes = stripe_shared::PaymentMethodTypes;
