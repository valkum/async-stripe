#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodCardWallet {
    pub amex_express_checkout: Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<stripe_shared::PaymentMethodCardWalletApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<stripe_shared::PaymentMethodCardWalletGooglePay>,
    pub link: Option<stripe_shared::PaymentMethodCardWalletLink>,
    pub masterpass: Option<stripe_shared::PaymentMethodCardWalletMasterpass>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodCardWalletSamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::PaymentMethodCardWalletType,
    pub visa_checkout: Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodCardWallet").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodCardWalletBuilder {
    amex_express_checkout: Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>,
    apple_pay: Option<stripe_shared::PaymentMethodCardWalletApplePay>,
    dynamic_last4: Option<Option<String>>,
    google_pay: Option<stripe_shared::PaymentMethodCardWalletGooglePay>,
    link: Option<stripe_shared::PaymentMethodCardWalletLink>,
    masterpass: Option<stripe_shared::PaymentMethodCardWalletMasterpass>,
    samsung_pay: Option<stripe_shared::PaymentMethodCardWalletSamsungPay>,
    type_: Option<stripe_shared::PaymentMethodCardWalletType>,
    visa_checkout: Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>,
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

    impl Deserialize for PaymentMethodCardWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodCardWallet>,
        builder: PaymentMethodCardWalletBuilder,
    }

    impl Visitor for Place<PaymentMethodCardWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodCardWalletBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodCardWalletBuilder {
        type Out = PaymentMethodCardWallet;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amex_express_checkout" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amex_express_checkout)
                }
                "apple_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.apple_pay),
                "dynamic_last4" => Deserialize::begin(&mut self.dynamic_last4),
                "google_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.google_pay)
                }
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "masterpass" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.masterpass)
                }
                "samsung_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.samsung_pay)
                }
                "type" => Deserialize::begin(&mut self.type_),
                "visa_checkout" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.visa_checkout)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amex_express_checkout: None,
                apple_pay: None,
                dynamic_last4: Some(None),
                google_pay: None,
                link: None,
                masterpass: None,
                samsung_pay: None,
                type_: None,
                visa_checkout: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    amex_express_checkout,
                    apple_pay,
                    dynamic_last4: Some(dynamic_last4),
                    google_pay,
                    link,
                    masterpass,
                    samsung_pay,
                    type_: Some(type_),
                    visa_checkout,
                } => Some(Self::Out {
                    amex_express_checkout,
                    apple_pay,
                    dynamic_last4,
                    google_pay,
                    link,
                    masterpass,
                    samsung_pay,
                    type_,
                    visa_checkout,
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

    impl ObjectDeser for PaymentMethodCardWallet {
        type Builder = PaymentMethodCardWalletBuilder;
    }

    impl FromValueOpt for PaymentMethodCardWallet {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
pub type PaymentMethodCardWalletType = stripe_shared::PaymentMethodCardWalletType;
