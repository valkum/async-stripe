#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingDisputeEvidence {
    pub canceled: Option<stripe_shared::IssuingDisputeCanceledEvidence>,
    pub duplicate: Option<stripe_shared::IssuingDisputeDuplicateEvidence>,
    pub fraudulent: Option<stripe_shared::IssuingDisputeFraudulentEvidence>,
    pub merchandise_not_as_described:
        Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    pub no_valid_authorization: Option<stripe_shared::IssuingDisputeNoValidAuthorizationEvidence>,
    pub not_received: Option<stripe_shared::IssuingDisputeNotReceivedEvidence>,
    pub other: Option<stripe_shared::IssuingDisputeOtherEvidence>,
    /// The reason for filing the dispute. Its value will match the field containing the evidence.
    pub reason: stripe_shared::IssuingDisputeEvidenceReason,
    pub service_not_as_described:
        Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingDisputeEvidence").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingDisputeEvidenceBuilder {
    canceled: Option<stripe_shared::IssuingDisputeCanceledEvidence>,
    duplicate: Option<stripe_shared::IssuingDisputeDuplicateEvidence>,
    fraudulent: Option<stripe_shared::IssuingDisputeFraudulentEvidence>,
    merchandise_not_as_described:
        Option<stripe_shared::IssuingDisputeMerchandiseNotAsDescribedEvidence>,
    no_valid_authorization: Option<stripe_shared::IssuingDisputeNoValidAuthorizationEvidence>,
    not_received: Option<stripe_shared::IssuingDisputeNotReceivedEvidence>,
    other: Option<stripe_shared::IssuingDisputeOtherEvidence>,
    reason: Option<stripe_shared::IssuingDisputeEvidenceReason>,
    service_not_as_described: Option<stripe_shared::IssuingDisputeServiceNotAsDescribedEvidence>,
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

    impl Deserialize for IssuingDisputeEvidence {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingDisputeEvidence>,
        builder: IssuingDisputeEvidenceBuilder,
    }

    impl Visitor for Place<IssuingDisputeEvidence> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingDisputeEvidenceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingDisputeEvidenceBuilder {
        type Out = IssuingDisputeEvidence;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "canceled" => stripe_types::miniserde_helpers::begin_nullable(&mut self.canceled),
                "duplicate" => stripe_types::miniserde_helpers::begin_nullable(&mut self.duplicate),
                "fraudulent" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.fraudulent)
                }
                "merchandise_not_as_described" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.merchandise_not_as_described,
                ),
                "no_valid_authorization" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.no_valid_authorization,
                ),
                "not_received" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.not_received)
                }
                "other" => stripe_types::miniserde_helpers::begin_nullable(&mut self.other),
                "reason" => Deserialize::begin(&mut self.reason),
                "service_not_as_described" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.service_not_as_described,
                ),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                canceled: None,
                duplicate: None,
                fraudulent: None,
                merchandise_not_as_described: None,
                no_valid_authorization: None,
                not_received: None,
                other: None,
                reason: None,
                service_not_as_described: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    canceled,
                    duplicate,
                    fraudulent,
                    merchandise_not_as_described,
                    no_valid_authorization,
                    not_received,
                    other,
                    reason: Some(reason),
                    service_not_as_described,
                } => Some(Self::Out {
                    canceled,
                    duplicate,
                    fraudulent,
                    merchandise_not_as_described,
                    no_valid_authorization,
                    not_received,
                    other,
                    reason,
                    service_not_as_described,
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

    impl ObjectDeser for IssuingDisputeEvidence {
        type Builder = IssuingDisputeEvidenceBuilder;
    }

    impl FromValueOpt for IssuingDisputeEvidence {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The reason for filing the dispute. Its value will match the field containing the evidence.
pub type IssuingDisputeEvidenceReason = stripe_shared::IssuingDisputeEvidenceReason;
