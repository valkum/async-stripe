#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<stripe_shared::CaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on [Payment Intents](https://docs.stripe.com/api/payment_intents) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    /// For a non-card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,
    /// For a card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    /// Concatenated with the account's statement descriptor prefix to form the complete statement descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://docs.stripe.com/connect/separate-charges-and-transfers) for details.
    pub transfer_group: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourcePaymentIntentData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourcePaymentIntentDataBuilder {
    capture_method: Option<stripe_shared::CaptureMethod>,
    description: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    setup_future_usage: Option<stripe_shared::CreateSetupIntentUsage>,
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_suffix: Option<Option<String>>,
    transfer_group: Option<Option<String>>,
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

    impl Deserialize for PaymentLinksResourcePaymentIntentData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourcePaymentIntentData>,
        builder: PaymentLinksResourcePaymentIntentDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourcePaymentIntentData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourcePaymentIntentDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourcePaymentIntentDataBuilder {
        type Out = PaymentLinksResourcePaymentIntentData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.capture_method)
                }
                "description" => Deserialize::begin(&mut self.description),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "setup_future_usage" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.setup_future_usage)
                }
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "statement_descriptor_suffix" => {
                    Deserialize::begin(&mut self.statement_descriptor_suffix)
                }
                "transfer_group" => Deserialize::begin(&mut self.transfer_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: None,
                description: Some(None),
                metadata: None,
                setup_future_usage: None,
                statement_descriptor: Some(None),
                statement_descriptor_suffix: Some(None),
                transfer_group: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    capture_method,
                    description: Some(description),
                    metadata: Some(metadata),
                    setup_future_usage,
                    statement_descriptor: Some(statement_descriptor),
                    statement_descriptor_suffix: Some(statement_descriptor_suffix),
                    transfer_group: Some(transfer_group),
                } => Some(Self::Out {
                    capture_method,
                    description,
                    metadata,
                    setup_future_usage,
                    statement_descriptor,
                    statement_descriptor_suffix,
                    transfer_group,
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

    impl ObjectDeser for PaymentLinksResourcePaymentIntentData {
        type Builder = PaymentLinksResourcePaymentIntentDataBuilder;
    }

    impl FromValueOpt for PaymentLinksResourcePaymentIntentData {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Indicates when the funds will be captured from the customer's account.
pub type PaymentLinksResourcePaymentIntentDataCaptureMethod = stripe_shared::CaptureMethod;
/// Indicates that you intend to make future payments with the payment method collected during checkout.
pub type PaymentLinksResourcePaymentIntentDataSetupFutureUsage =
    stripe_shared::CreateSetupIntentUsage;
