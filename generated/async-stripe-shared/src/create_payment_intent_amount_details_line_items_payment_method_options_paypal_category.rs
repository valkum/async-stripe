/// Type of the line item.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory {
    DigitalGoods,
    Donation,
    PhysicalGoods,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory {
    pub fn as_str(&self) -> &str {
        use CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory::*;
        match self {
            DigitalGoods => "digital_goods",
            Donation => "donation",
            PhysicalGoods => "physical_goods",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory::*;
        match s {
            "digital_goods" => Ok(DigitalGoods),
            "donation" => Ok(Donation),
            "physical_goods" => Ok(PhysicalGoods),
            v => {
                stripe_types::warn_unknown_enum_value(
                    v,
                    "CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory",
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
        ))
        .finish_non_exhaustive()
    }
}
impl serde::Serialize
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = {
            let Ok(v) = CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory::from_str(s);
            Some(v)
        };
        Ok(())
    }
}

impl stripe_types::miniserde_helpers::FromValueOpt
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn from_value(v: miniserde::json::Value) -> Option<Self> {
        stripe_types::miniserde_helpers::from_value_via_deserialize(v)
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for CreatePaymentIntentAmountDetailsLineItemsPaymentMethodOptionsPaypalCategory
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        let Ok(v) = Self::from_str(&s);
        Ok(v)
    }
}
