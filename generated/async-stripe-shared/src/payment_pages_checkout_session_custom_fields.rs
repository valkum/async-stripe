#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentPagesCheckoutSessionCustomFields {
    pub dropdown: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    /// String of your choice that your integration can use to reconcile this field.
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    pub label: stripe_shared::PaymentPagesCheckoutSessionCustomFieldsLabel,
    pub numeric: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    /// Whether the customer is required to complete the field before completing the Checkout Session.
    /// Defaults to `false`.
    pub optional: bool,
    pub text: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsText>,
    /// The type of the field.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreatePaymentLinkCustomFieldsType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentPagesCheckoutSessionCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentPagesCheckoutSessionCustomFields").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentPagesCheckoutSessionCustomFieldsBuilder {
    dropdown: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    key: Option<String>,
    label: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsLabel>,
    numeric: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    optional: Option<bool>,
    text: Option<stripe_shared::PaymentPagesCheckoutSessionCustomFieldsText>,
    type_: Option<stripe_shared::CreatePaymentLinkCustomFieldsType>,
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

    impl Deserialize for PaymentPagesCheckoutSessionCustomFields {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentPagesCheckoutSessionCustomFields>,
        builder: PaymentPagesCheckoutSessionCustomFieldsBuilder,
    }

    impl Visitor for Place<PaymentPagesCheckoutSessionCustomFields> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentPagesCheckoutSessionCustomFieldsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentPagesCheckoutSessionCustomFieldsBuilder {
        type Out = PaymentPagesCheckoutSessionCustomFields;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "dropdown" => stripe_types::miniserde_helpers::begin_nullable(&mut self.dropdown),
                "key" => Deserialize::begin(&mut self.key),
                "label" => Deserialize::begin(&mut self.label),
                "numeric" => stripe_types::miniserde_helpers::begin_nullable(&mut self.numeric),
                "optional" => Deserialize::begin(&mut self.optional),
                "text" => stripe_types::miniserde_helpers::begin_nullable(&mut self.text),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                dropdown: None,
                key: None,
                label: None,
                numeric: None,
                optional: None,
                text: None,
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    dropdown,
                    key: Some(key),
                    label: Some(label),
                    numeric,
                    optional: Some(optional),
                    text,
                    type_: Some(type_),
                } => Some(Self::Out { dropdown, key, label, numeric, optional, text, type_ }),
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

    impl ObjectDeser for PaymentPagesCheckoutSessionCustomFields {
        type Builder = PaymentPagesCheckoutSessionCustomFieldsBuilder;
    }

    impl FromValueOpt for PaymentPagesCheckoutSessionCustomFields {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The type of the field.
pub type PaymentPagesCheckoutSessionCustomFieldsType =
    stripe_shared::CreatePaymentLinkCustomFieldsType;
