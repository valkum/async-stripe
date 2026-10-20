#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionPaymentMethodOptionsCard {
    pub mandate_options: Option<stripe_shared::InvoiceMandateOptionsCard>,
    /// Selected network to process this Subscription on.
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    pub network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionPaymentMethodOptionsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionPaymentMethodOptionsCard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionPaymentMethodOptionsCardBuilder {
    mandate_options: Option<stripe_shared::InvoiceMandateOptionsCard>,
    network: Option<stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork>,
    request_three_d_secure: Option<stripe_shared::RequestThreeDSecure>,
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

    impl Deserialize for SubscriptionPaymentMethodOptionsCard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionPaymentMethodOptionsCard>,
        builder: SubscriptionPaymentMethodOptionsCardBuilder,
    }

    impl Visitor for Place<SubscriptionPaymentMethodOptionsCard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionPaymentMethodOptionsCardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionPaymentMethodOptionsCardBuilder {
        type Out = SubscriptionPaymentMethodOptionsCard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "mandate_options" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mandate_options)
                }
                "network" => stripe_types::miniserde_helpers::begin_nullable(&mut self.network),
                "request_three_d_secure" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.request_three_d_secure,
                ),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { mandate_options: None, network: None, request_three_d_secure: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (mandate_options, network, request_three_d_secure) = (
                self.mandate_options.take(),
                self.network.take(),
                self.request_three_d_secure.take(),
            ) else {
                return None;
            };
            Some(Self::Out { mandate_options, network, request_three_d_secure })
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

    impl ObjectDeser for SubscriptionPaymentMethodOptionsCard {
        type Builder = SubscriptionPaymentMethodOptionsCardBuilder;
    }

    impl FromValueOpt for SubscriptionPaymentMethodOptionsCard {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Selected network to process this Subscription on.
/// Depends on the available networks of the card attached to the Subscription.
/// Can be only set confirm-time.
pub type SubscriptionPaymentMethodOptionsCardNetwork =
    stripe_shared::SetupIntentPaymentMethodOptionsCardNetwork;
/// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://docs.stripe.com/strong-customer-authentication).
/// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
/// Read our guide on [manually requesting 3D Secure](https://docs.stripe.com/payments/3d-secure/authentication-flow#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
pub type SubscriptionPaymentMethodOptionsCardRequestThreeDSecure =
    stripe_shared::RequestThreeDSecure;
