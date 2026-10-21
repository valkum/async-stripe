#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsUsBankAccountMandateOptions {
    /// Mandate collection method
    pub collection_method: Option<stripe_shared::MandateUsBankAccountCollectionMethod>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodOptionsUsBankAccountMandateOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodOptionsUsBankAccountMandateOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodOptionsUsBankAccountMandateOptionsBuilder {
    collection_method: Option<stripe_shared::MandateUsBankAccountCollectionMethod>,
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

    impl Deserialize for PaymentMethodOptionsUsBankAccountMandateOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsUsBankAccountMandateOptions>,
        builder: PaymentMethodOptionsUsBankAccountMandateOptionsBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsUsBankAccountMandateOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsUsBankAccountMandateOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsUsBankAccountMandateOptionsBuilder {
        type Out = PaymentMethodOptionsUsBankAccountMandateOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collection_method" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.collection_method)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { collection_method: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { collection_method } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out { collection_method })
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

    impl ObjectDeser for PaymentMethodOptionsUsBankAccountMandateOptions {
        type Builder = PaymentMethodOptionsUsBankAccountMandateOptionsBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsUsBankAccountMandateOptions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Mandate collection method
pub type PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod =
    stripe_shared::MandateUsBankAccountCollectionMethod;
