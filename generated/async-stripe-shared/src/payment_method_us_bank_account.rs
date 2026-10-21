#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodUsBankAccount {
    /// Account holder type: individual or company.
    pub account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    /// Account type: checkings or savings. Defaults to checking if omitted.
    pub account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    /// The name of the bank.
    pub bank_name: Option<String>,
    /// The ID of the Financial Connections Account used to create the payment method.
    pub financial_connections_account: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// Contains information about US bank account networks that can be used.
    pub networks: Option<stripe_shared::UsBankAccountNetworks>,
    /// Routing number of the bank account.
    pub routing_number: Option<String>,
    /// Contains information about the future reusability of this PaymentMethod.
    pub status_details: Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodUsBankAccount").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodUsBankAccountBuilder {
    account_holder_type: Option<stripe_shared::IssuingCardholderType>,
    account_type: Option<stripe_shared::PaymentMethodUsBankAccountAccountType>,
    bank_name: Option<Option<String>>,
    financial_connections_account: Option<Option<String>>,
    fingerprint: Option<Option<String>>,
    last4: Option<Option<String>>,
    networks: Option<stripe_shared::UsBankAccountNetworks>,
    routing_number: Option<Option<String>>,
    status_details: Option<stripe_shared::PaymentMethodUsBankAccountStatusDetails>,
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

    impl Deserialize for PaymentMethodUsBankAccount {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodUsBankAccount>,
        builder: PaymentMethodUsBankAccountBuilder,
    }

    impl Visitor for Place<PaymentMethodUsBankAccount> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodUsBankAccountBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodUsBankAccountBuilder {
        type Out = PaymentMethodUsBankAccount;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_holder_type" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.account_holder_type)
                }
                "account_type" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.account_type)
                }
                "bank_name" => Deserialize::begin(&mut self.bank_name),
                "financial_connections_account" => {
                    Deserialize::begin(&mut self.financial_connections_account)
                }
                "fingerprint" => Deserialize::begin(&mut self.fingerprint),
                "last4" => Deserialize::begin(&mut self.last4),
                "networks" => stripe_types::miniserde_helpers::begin_nullable(&mut self.networks),
                "routing_number" => Deserialize::begin(&mut self.routing_number),
                "status_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.status_details)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_holder_type: None,
                account_type: None,
                bank_name: Some(None),
                financial_connections_account: Some(None),
                fingerprint: Some(None),
                last4: Some(None),
                networks: None,
                routing_number: Some(None),
                status_details: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    account_holder_type,
                    account_type,
                    bank_name: Some(bank_name),
                    financial_connections_account: Some(financial_connections_account),
                    fingerprint: Some(fingerprint),
                    last4: Some(last4),
                    networks,
                    routing_number: Some(routing_number),
                    status_details,
                } => Some(Self::Out {
                    account_holder_type,
                    account_type,
                    bank_name,
                    financial_connections_account,
                    fingerprint,
                    last4,
                    networks,
                    routing_number,
                    status_details,
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

    impl ObjectDeser for PaymentMethodUsBankAccount {
        type Builder = PaymentMethodUsBankAccountBuilder;
    }

    impl FromValueOpt for PaymentMethodUsBankAccount {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Account holder type: individual or company.
pub type PaymentMethodUsBankAccountAccountHolderType = stripe_shared::IssuingCardholderType;
/// Account type: checkings or savings. Defaults to checking if omitted.
pub type PaymentMethodUsBankAccountAccountType =
    stripe_shared::PaymentMethodUsBankAccountAccountType;
