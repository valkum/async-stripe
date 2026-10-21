#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TreasuryTransactionsResourceFlowDetails {
    pub credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    pub debit_reversal: Option<stripe_treasury::TreasuryDebitReversal>,
    pub inbound_transfer: Option<stripe_treasury::TreasuryInboundTransfer>,
    pub issuing_authorization: Option<stripe_shared::IssuingAuthorization>,
    pub outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    pub outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfer>,
    pub received_credit: Option<stripe_treasury::TreasuryReceivedCredit>,
    pub received_debit: Option<stripe_treasury::TreasuryReceivedDebit>,
    /// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::TreasuryTransactionFlowType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasuryTransactionsResourceFlowDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasuryTransactionsResourceFlowDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TreasuryTransactionsResourceFlowDetailsBuilder {
    credit_reversal: Option<stripe_treasury::TreasuryCreditReversal>,
    debit_reversal: Option<stripe_treasury::TreasuryDebitReversal>,
    inbound_transfer: Option<stripe_treasury::TreasuryInboundTransfer>,
    issuing_authorization: Option<stripe_shared::IssuingAuthorization>,
    outbound_payment: Option<stripe_treasury::TreasuryOutboundPayment>,
    outbound_transfer: Option<stripe_treasury::TreasuryOutboundTransfer>,
    received_credit: Option<stripe_treasury::TreasuryReceivedCredit>,
    received_debit: Option<stripe_treasury::TreasuryReceivedDebit>,
    type_: Option<stripe_shared::TreasuryTransactionFlowType>,
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

    impl Deserialize for TreasuryTransactionsResourceFlowDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TreasuryTransactionsResourceFlowDetails>,
        builder: TreasuryTransactionsResourceFlowDetailsBuilder,
    }

    impl Visitor for Place<TreasuryTransactionsResourceFlowDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TreasuryTransactionsResourceFlowDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TreasuryTransactionsResourceFlowDetailsBuilder {
        type Out = TreasuryTransactionsResourceFlowDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credit_reversal" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.credit_reversal)
                }
                "debit_reversal" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.debit_reversal)
                }
                "inbound_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.inbound_transfer)
                }
                "issuing_authorization" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.issuing_authorization)
                }
                "outbound_payment" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.outbound_payment)
                }
                "outbound_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.outbound_transfer)
                }
                "received_credit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.received_credit)
                }
                "received_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.received_debit)
                }
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                credit_reversal: None,
                debit_reversal: None,
                inbound_transfer: None,
                issuing_authorization: None,
                outbound_payment: None,
                outbound_transfer: None,
                received_credit: None,
                received_debit: None,
                type_: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    credit_reversal,
                    debit_reversal,
                    inbound_transfer,
                    issuing_authorization,
                    outbound_payment,
                    outbound_transfer,
                    received_credit,
                    received_debit,
                    type_: Some(type_),
                } => Some(Self::Out {
                    credit_reversal,
                    debit_reversal,
                    inbound_transfer,
                    issuing_authorization,
                    outbound_payment,
                    outbound_transfer,
                    received_credit,
                    received_debit,
                    type_,
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

    impl ObjectDeser for TreasuryTransactionsResourceFlowDetails {
        type Builder = TreasuryTransactionsResourceFlowDetailsBuilder;
    }

    impl FromValueOpt for TreasuryTransactionsResourceFlowDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// Type of the flow that created the Transaction. Set to the same value as `flow_type`.
pub type TreasuryTransactionsResourceFlowDetailsType = stripe_shared::TreasuryTransactionFlowType;
