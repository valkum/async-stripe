#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountUnificationAccountControllerStripeDashboard {
    /// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::Type,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountUnificationAccountControllerStripeDashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountUnificationAccountControllerStripeDashboard").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountUnificationAccountControllerStripeDashboardBuilder {
    type_: Option<stripe_shared::Type>,
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

    impl Deserialize for AccountUnificationAccountControllerStripeDashboard {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountUnificationAccountControllerStripeDashboard>,
        builder: AccountUnificationAccountControllerStripeDashboardBuilder,
    }

    impl Visitor for Place<AccountUnificationAccountControllerStripeDashboard> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountUnificationAccountControllerStripeDashboardBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountUnificationAccountControllerStripeDashboardBuilder {
        type Out = AccountUnificationAccountControllerStripeDashboard;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { type_: Some(type_) } => Some(Self::Out { type_ }),
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

    impl ObjectDeser for AccountUnificationAccountControllerStripeDashboard {
        type Builder = AccountUnificationAccountControllerStripeDashboardBuilder;
    }

    impl FromValueOpt for AccountUnificationAccountControllerStripeDashboard {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// A value indicating the Stripe dashboard this account has access to independent of the Connect application.
pub type AccountUnificationAccountControllerStripeDashboardType = stripe_shared::Type;
