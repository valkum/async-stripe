/// A payment link is a shareable URL that will take your customers to a hosted payment page.
/// A payment link can be shared and used multiple times.
///
/// When a customer opens a payment link it will open a new [checkout session](https://docs.stripe.com/api/checkout/sessions) to render the payment page.
/// You can use [checkout session events](https://docs.stripe.com/api/events/types#event_types-checkout.session.completed) to track payments through payment links.
///
/// Related guide: [Payment Links API](https://docs.stripe.com/payment-links)
///
/// For more details see <<https://stripe.com/docs/api/payment_links/payment_links/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLink {
    /// Whether the payment link's `url` is active.
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    pub after_completion: stripe_shared::PaymentLinksResourceAfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The ID of the Connect application that created the Payment Link.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_shared::PaymentLinksResourceAutomaticTax,
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
    pub billing_address_collection: stripe_shared::BillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<stripe_shared::PaymentLinksResourceConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    /// You can't set this parameter if `ui_mode` is `custom`.
    pub custom_fields: Vec<stripe_shared::PaymentLinksResourceCustomFields>,
    pub custom_text: stripe_shared::PaymentLinksResourceCustomText,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: stripe_shared::PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentLinkId,
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub inactive_message: Option<String>,
    /// Configuration for creating invoice for payment mode payment links.
    pub invoice_creation: Option<stripe_shared::PaymentLinksResourceInvoiceCreation>,
    /// The line items representing what is being sold.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Settings for Managed Payments for this Payment Link and resulting [CheckoutSessions](/api/checkout/sessions/object), [PaymentIntents](/api/payment_intents/object), [Invoices](/api/invoices/object), and [Subscriptions](/api/subscriptions/object).
    pub managed_payments: Option<stripe_shared::PaymentPagesCheckoutSessionManagedPayments>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    pub name_collection: Option<stripe_shared::PaymentLinksResourceNameCollection>,
    /// The account on behalf of which to charge.
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The optional items presented to the customer at checkout.
    pub optional_items: Option<Vec<stripe_shared::PaymentLinksResourceOptionalItem>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<stripe_shared::PaymentLinksResourcePaymentIntentData>,
    /// Configuration for collecting a payment method during checkout. Defaults to `always`.
    pub payment_method_collection: stripe_shared::PaymentLinkCustomerCreation,
    /// Payment-method-specific configuration.
    pub payment_method_options: Option<stripe_shared::PaymentLinksResourcePaymentMethodOptions>,
    /// The list of payment method types that customers can use.
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection: stripe_shared::PaymentLinksResourcePhoneNumberCollection,
    /// Settings that restrict the usage of a payment link.
    pub restrictions: Option<stripe_shared::PaymentLinksResourceRestrictions>,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection:
        Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<stripe_shared::PaymentLinksResourceShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: stripe_shared::SubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<stripe_shared::PaymentLinksResourceSubscriptionData>,
    pub tax_id_collection: stripe_shared::PaymentLinksResourceTaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<stripe_shared::PaymentLinksResourceTransferData>,
    /// The public URL that can be shared with customers.
    pub url: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLink").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinkBuilder {
    active: Option<bool>,
    after_completion: Option<stripe_shared::PaymentLinksResourceAfterCompletion>,
    allow_promotion_codes: Option<bool>,
    application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    application_fee_amount: Option<Option<i64>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<stripe_shared::PaymentLinksResourceAutomaticTax>,
    billing_address_collection: Option<stripe_shared::BillingAddressCollection>,
    consent_collection: Option<stripe_shared::PaymentLinksResourceConsentCollection>,
    currency: Option<stripe_types::Currency>,
    custom_fields: Option<Vec<stripe_shared::PaymentLinksResourceCustomFields>>,
    custom_text: Option<stripe_shared::PaymentLinksResourceCustomText>,
    customer_creation: Option<stripe_shared::PaymentLinkCustomerCreation>,
    id: Option<stripe_shared::PaymentLinkId>,
    inactive_message: Option<Option<String>>,
    invoice_creation: Option<stripe_shared::PaymentLinksResourceInvoiceCreation>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    managed_payments: Option<stripe_shared::PaymentPagesCheckoutSessionManagedPayments>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name_collection: Option<stripe_shared::PaymentLinksResourceNameCollection>,
    on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    optional_items: Option<Option<Vec<stripe_shared::PaymentLinksResourceOptionalItem>>>,
    payment_intent_data: Option<stripe_shared::PaymentLinksResourcePaymentIntentData>,
    payment_method_collection: Option<stripe_shared::PaymentLinkCustomerCreation>,
    payment_method_options: Option<stripe_shared::PaymentLinksResourcePaymentMethodOptions>,
    payment_method_types: Option<Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>>,
    phone_number_collection: Option<stripe_shared::PaymentLinksResourcePhoneNumberCollection>,
    restrictions: Option<stripe_shared::PaymentLinksResourceRestrictions>,
    shipping_address_collection:
        Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>,
    shipping_options: Option<Vec<stripe_shared::PaymentLinksResourceShippingOption>>,
    submit_type: Option<stripe_shared::SubmitType>,
    subscription_data: Option<stripe_shared::PaymentLinksResourceSubscriptionData>,
    tax_id_collection: Option<stripe_shared::PaymentLinksResourceTaxIdCollection>,
    transfer_data: Option<stripe_shared::PaymentLinksResourceTransferData>,
    url: Option<String>,
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

    impl Deserialize for PaymentLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLink>,
        builder: PaymentLinkBuilder,
    }

    impl Visitor for Place<PaymentLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinkBuilder {
        type Out = PaymentLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "after_completion" => Deserialize::begin(&mut self.after_completion),
                "allow_promotion_codes" => Deserialize::begin(&mut self.allow_promotion_codes),
                "application" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.application)
                }
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "application_fee_percent" => Deserialize::begin(&mut self.application_fee_percent),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "billing_address_collection" => {
                    Deserialize::begin(&mut self.billing_address_collection)
                }
                "consent_collection" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.consent_collection)
                }
                "currency" => Deserialize::begin(&mut self.currency),
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "custom_text" => Deserialize::begin(&mut self.custom_text),
                "customer_creation" => Deserialize::begin(&mut self.customer_creation),
                "id" => Deserialize::begin(&mut self.id),
                "inactive_message" => Deserialize::begin(&mut self.inactive_message),
                "invoice_creation" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.invoice_creation)
                }
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "managed_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.managed_payments)
                }
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name_collection" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.name_collection)
                }
                "on_behalf_of" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.on_behalf_of)
                }
                "optional_items" => Deserialize::begin(&mut self.optional_items),
                "payment_intent_data" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payment_intent_data)
                }
                "payment_method_collection" => {
                    Deserialize::begin(&mut self.payment_method_collection)
                }
                "payment_method_options" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.payment_method_options,
                ),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "phone_number_collection" => Deserialize::begin(&mut self.phone_number_collection),
                "restrictions" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.restrictions)
                }
                "shipping_address_collection" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.shipping_address_collection,
                ),
                "shipping_options" => Deserialize::begin(&mut self.shipping_options),
                "submit_type" => Deserialize::begin(&mut self.submit_type),
                "subscription_data" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.subscription_data)
                }
                "tax_id_collection" => Deserialize::begin(&mut self.tax_id_collection),
                "transfer_data" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.transfer_data)
                }
                "url" => Deserialize::begin(&mut self.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: None,
                after_completion: None,
                allow_promotion_codes: None,
                application: None,
                application_fee_amount: Some(None),
                application_fee_percent: Some(None),
                automatic_tax: None,
                billing_address_collection: None,
                consent_collection: None,
                currency: None,
                custom_fields: None,
                custom_text: None,
                customer_creation: None,
                id: None,
                inactive_message: Some(None),
                invoice_creation: None,
                line_items: Some(None),
                livemode: None,
                managed_payments: None,
                metadata: None,
                name_collection: None,
                on_behalf_of: None,
                optional_items: Some(None),
                payment_intent_data: None,
                payment_method_collection: None,
                payment_method_options: None,
                payment_method_types: Some(None),
                phone_number_collection: None,
                restrictions: None,
                shipping_address_collection: None,
                shipping_options: None,
                submit_type: None,
                subscription_data: None,
                tax_id_collection: None,
                transfer_data: None,
                url: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(after_completion),
                Some(allow_promotion_codes),
                application,
                Some(application_fee_amount),
                Some(application_fee_percent),
                Some(automatic_tax),
                Some(billing_address_collection),
                consent_collection,
                Some(currency),
                Some(custom_fields),
                Some(custom_text),
                Some(customer_creation),
                Some(id),
                Some(inactive_message),
                invoice_creation,
                Some(line_items),
                Some(livemode),
                managed_payments,
                Some(metadata),
                name_collection,
                on_behalf_of,
                Some(optional_items),
                payment_intent_data,
                Some(payment_method_collection),
                payment_method_options,
                Some(payment_method_types),
                Some(phone_number_collection),
                restrictions,
                shipping_address_collection,
                Some(shipping_options),
                Some(submit_type),
                subscription_data,
                Some(tax_id_collection),
                transfer_data,
                Some(url),
            ) = (
                self.active,
                self.after_completion.take(),
                self.allow_promotion_codes,
                self.application.take(),
                self.application_fee_amount,
                self.application_fee_percent,
                self.automatic_tax.take(),
                self.billing_address_collection.take(),
                self.consent_collection.take(),
                self.currency.take(),
                self.custom_fields.take(),
                self.custom_text.take(),
                self.customer_creation.take(),
                self.id.take(),
                self.inactive_message.take(),
                self.invoice_creation.take(),
                self.line_items.take(),
                self.livemode,
                self.managed_payments,
                self.metadata.take(),
                self.name_collection,
                self.on_behalf_of.take(),
                self.optional_items.take(),
                self.payment_intent_data.take(),
                self.payment_method_collection.take(),
                self.payment_method_options.take(),
                self.payment_method_types.take(),
                self.phone_number_collection,
                self.restrictions,
                self.shipping_address_collection.take(),
                self.shipping_options.take(),
                self.submit_type.take(),
                self.subscription_data.take(),
                self.tax_id_collection.take(),
                self.transfer_data.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                active,
                after_completion,
                allow_promotion_codes,
                application,
                application_fee_amount,
                application_fee_percent,
                automatic_tax,
                billing_address_collection,
                consent_collection,
                currency,
                custom_fields,
                custom_text,
                customer_creation,
                id,
                inactive_message,
                invoice_creation,
                line_items,
                livemode,
                managed_payments,
                metadata,
                name_collection,
                on_behalf_of,
                optional_items,
                payment_intent_data,
                payment_method_collection,
                payment_method_options,
                payment_method_types,
                phone_number_collection,
                restrictions,
                shipping_address_collection,
                shipping_options,
                submit_type,
                subscription_data,
                tax_id_collection,
                transfer_data,
                url,
            })
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

    impl ObjectDeser for PaymentLink {
        type Builder = PaymentLinkBuilder;
    }

    impl FromValueOpt for PaymentLink {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentLink", 37)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("after_completion", &self.after_completion)?;
        s.serialize_field("allow_promotion_codes", &self.allow_promotion_codes)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("application_fee_percent", &self.application_fee_percent)?;
        s.serialize_field("automatic_tax", &self.automatic_tax)?;
        s.serialize_field("billing_address_collection", &self.billing_address_collection)?;
        s.serialize_field("consent_collection", &self.consent_collection)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("custom_fields", &self.custom_fields)?;
        s.serialize_field("custom_text", &self.custom_text)?;
        s.serialize_field("customer_creation", &self.customer_creation)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("inactive_message", &self.inactive_message)?;
        s.serialize_field("invoice_creation", &self.invoice_creation)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("managed_payments", &self.managed_payments)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name_collection", &self.name_collection)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("optional_items", &self.optional_items)?;
        s.serialize_field("payment_intent_data", &self.payment_intent_data)?;
        s.serialize_field("payment_method_collection", &self.payment_method_collection)?;
        s.serialize_field("payment_method_options", &self.payment_method_options)?;
        s.serialize_field("payment_method_types", &self.payment_method_types)?;
        s.serialize_field("phone_number_collection", &self.phone_number_collection)?;
        s.serialize_field("restrictions", &self.restrictions)?;
        s.serialize_field("shipping_address_collection", &self.shipping_address_collection)?;
        s.serialize_field("shipping_options", &self.shipping_options)?;
        s.serialize_field("submit_type", &self.submit_type)?;
        s.serialize_field("subscription_data", &self.subscription_data)?;
        s.serialize_field("tax_id_collection", &self.tax_id_collection)?;
        s.serialize_field("transfer_data", &self.transfer_data)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "payment_link")?;
        s.end()
    }
}
impl stripe_types::Object for PaymentLink {
    type Id = stripe_shared::PaymentLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentLinkId);
/// Configuration for collecting the customer's billing address. Defaults to `auto`.
pub type PaymentLinkBillingAddressCollection = stripe_shared::BillingAddressCollection;
/// Configuration for Customer creation during checkout.
pub type PaymentLinkCustomerCreation = stripe_shared::PaymentLinkCustomerCreation;
/// Configuration for collecting a payment method during checkout. Defaults to `always`.
pub type PaymentLinkPaymentMethodCollection = stripe_shared::PaymentLinkCustomerCreation;
/// The list of payment method types that customers can use.
/// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
pub type PaymentLinkPaymentMethodTypes = stripe_shared::PaymentLinkPaymentMethodTypes;
/// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
pub type PaymentLinkSubmitType = stripe_shared::SubmitType;
