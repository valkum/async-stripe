#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks {
    /// If you provide a value for `address.line1`, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_line1_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    /// If you provide a address postal code, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub address_postal_code_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    /// If you provide a CVC, the check results is one of `pass`, `fail`, `unavailable`, or `unchecked`.
    pub cvc_check: Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder {
    address_line1_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    address_postal_code_check:
        Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
    cvc_check: Option<stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck>,
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

    impl Deserialize
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks,
        >,
        builder:
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder,
    }

    impl Visitor
        for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder
    {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address_line1_check" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.address_line1_check)
                }
                "address_postal_code_check" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.address_postal_code_check,
                ),
                "cvc_check" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cvc_check),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { address_line1_check: None, address_postal_code_check: None, cvc_check: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self { address_line1_check, address_postal_code_check, cvc_check } =
                core::mem::replace(self, Self::deser_default());
            Some(Self::Out { address_line1_check, address_postal_code_check, cvc_check })
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

    impl ObjectDeser
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        type Builder =
            PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksBuilder;
    }

    impl FromValueOpt
        for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecks
    {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// If you provide a value for `address.line1`, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`.
pub type PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressLine1Check =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
/// If you provide a address postal code, the check result is one of `pass`, `fail`, `unavailable`, or `unchecked`.
pub type PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksAddressPostalCodeCheck =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
/// If you provide a CVC, the check results is one of `pass`, `fail`, `unavailable`, or `unchecked`.
pub type PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetailsResourceChecksCvcCheck =
    stripe_shared::InsightsResourcesPaymentEvaluationRejectedCardCvcCheck;
