#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LegalEntityCompany {
    pub address: Option<stripe_shared::Address>,
    /// The Kana variation of the company's primary address (Japan only).
    pub address_kana: Option<stripe_shared::LegalEntityJapanAddress>,
    /// The Kanji variation of the company's primary address (Japan only).
    pub address_kanji: Option<stripe_shared::LegalEntityJapanAddress>,
    pub administrative_address: Option<stripe_shared::Address>,
    /// Whether the company's directors have been provided.
    /// This Boolean will be `true` if you've manually indicated that all directors are provided via [the `directors_provided` parameter](https://docs.stripe.com/api/accounts/update#update_account-company-directors_provided).
    pub directors_provided: Option<bool>,
    /// This hash is used to attest that the director information provided to Stripe is both current and correct.
    pub directorship_declaration: Option<stripe_shared::LegalEntityDirectorshipDeclaration>,
    /// Whether the company's executives have been provided.
    /// This Boolean will be `true` if you've manually indicated that all executives are provided via [the `executives_provided` parameter](https://docs.stripe.com/api/accounts/update#update_account-company-executives_provided), or if Stripe determined that sufficient executives were provided.
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    pub export_license_id: Option<String>,
    /// The purpose code to use for export transactions (India only).
    pub export_purpose_code: Option<String>,
    /// The company's legal name.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub name: Option<String>,
    /// The Kana variation of the company's legal name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub name_kana: Option<String>,
    /// The Kanji variation of the company's legal name (Japan only).
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    pub name_kanji: Option<String>,
    /// Whether the company's owners have been provided.
    /// This Boolean will be `true` if you've manually indicated that all owners are provided via [the `owners_provided` parameter](https://docs.stripe.com/api/accounts/update#update_account-company-owners_provided), or if Stripe determined that sufficient owners were provided.
    /// Stripe determines ownership requirements using both the number of owners provided and their total percent ownership (calculated by adding the `percent_ownership` of each owner together).
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    pub ownership_declaration: Option<stripe_shared::LegalEntityUboDeclaration>,
    /// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
    /// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
    pub ownership_exemption_reason: Option<stripe_shared::OwnershipExemptionReason>,
    /// The company's phone number (used for verification).
    pub phone: Option<String>,
    pub principal_place_of_business: Option<stripe_shared::Address>,
    pub registration_date: Option<stripe_shared::LegalEntityRegistrationDate>,
    /// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
    pub representative_declaration: Option<stripe_shared::LegalEntityRepresentativeDeclaration>,
    /// The category identifying the legal structure of the company or legal entity.
    /// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
    /// See [Business structure](https://docs.stripe.com/connect/identity-verification#business-structure) for more details.
    pub structure: Option<stripe_shared::Structure>,
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: Option<bool>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    pub tax_id_registrar: Option<String>,
    /// Whether the company's business VAT number was provided.
    pub vat_id_provided: Option<bool>,
    /// Information on the verification state of the company.
    pub verification: Option<stripe_shared::LegalEntityCompanyVerification>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for LegalEntityCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LegalEntityCompany").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct LegalEntityCompanyBuilder {
    address: Option<stripe_shared::Address>,
    address_kana: Option<stripe_shared::LegalEntityJapanAddress>,
    address_kanji: Option<stripe_shared::LegalEntityJapanAddress>,
    administrative_address: Option<stripe_shared::Address>,
    directors_provided: Option<Option<bool>>,
    directorship_declaration: Option<stripe_shared::LegalEntityDirectorshipDeclaration>,
    executives_provided: Option<Option<bool>>,
    export_license_id: Option<Option<String>>,
    export_purpose_code: Option<Option<String>>,
    name: Option<Option<String>>,
    name_kana: Option<Option<String>>,
    name_kanji: Option<Option<String>>,
    owners_provided: Option<Option<bool>>,
    ownership_declaration: Option<stripe_shared::LegalEntityUboDeclaration>,
    ownership_exemption_reason: Option<stripe_shared::OwnershipExemptionReason>,
    phone: Option<Option<String>>,
    principal_place_of_business: Option<stripe_shared::Address>,
    registration_date: Option<stripe_shared::LegalEntityRegistrationDate>,
    representative_declaration: Option<stripe_shared::LegalEntityRepresentativeDeclaration>,
    structure: Option<stripe_shared::Structure>,
    tax_id_provided: Option<Option<bool>>,
    tax_id_registrar: Option<Option<String>>,
    vat_id_provided: Option<Option<bool>>,
    verification: Option<stripe_shared::LegalEntityCompanyVerification>,
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

    impl Deserialize for LegalEntityCompany {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<LegalEntityCompany>,
        builder: LegalEntityCompanyBuilder,
    }

    impl Visitor for Place<LegalEntityCompany> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: LegalEntityCompanyBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for LegalEntityCompanyBuilder {
        type Out = LegalEntityCompany;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => stripe_types::miniserde_helpers::begin_nullable(&mut self.address),
                "address_kana" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.address_kana)
                }
                "address_kanji" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.address_kanji)
                }
                "administrative_address" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.administrative_address,
                ),
                "directors_provided" => Deserialize::begin(&mut self.directors_provided),
                "directorship_declaration" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.directorship_declaration,
                ),
                "executives_provided" => Deserialize::begin(&mut self.executives_provided),
                "export_license_id" => Deserialize::begin(&mut self.export_license_id),
                "export_purpose_code" => Deserialize::begin(&mut self.export_purpose_code),
                "name" => Deserialize::begin(&mut self.name),
                "name_kana" => Deserialize::begin(&mut self.name_kana),
                "name_kanji" => Deserialize::begin(&mut self.name_kanji),
                "owners_provided" => Deserialize::begin(&mut self.owners_provided),
                "ownership_declaration" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.ownership_declaration)
                }
                "ownership_exemption_reason" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.ownership_exemption_reason,
                ),
                "phone" => Deserialize::begin(&mut self.phone),
                "principal_place_of_business" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.principal_place_of_business,
                ),
                "registration_date" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.registration_date)
                }
                "representative_declaration" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.representative_declaration,
                ),
                "structure" => stripe_types::miniserde_helpers::begin_nullable(&mut self.structure),
                "tax_id_provided" => Deserialize::begin(&mut self.tax_id_provided),
                "tax_id_registrar" => Deserialize::begin(&mut self.tax_id_registrar),
                "vat_id_provided" => Deserialize::begin(&mut self.vat_id_provided),
                "verification" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.verification)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                address: None,
                address_kana: None,
                address_kanji: None,
                administrative_address: None,
                directors_provided: Some(None),
                directorship_declaration: None,
                executives_provided: Some(None),
                export_license_id: Some(None),
                export_purpose_code: Some(None),
                name: Some(None),
                name_kana: Some(None),
                name_kanji: Some(None),
                owners_provided: Some(None),
                ownership_declaration: None,
                ownership_exemption_reason: None,
                phone: Some(None),
                principal_place_of_business: None,
                registration_date: None,
                representative_declaration: None,
                structure: None,
                tax_id_provided: Some(None),
                tax_id_registrar: Some(None),
                vat_id_provided: Some(None),
                verification: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    address,
                    address_kana,
                    address_kanji,
                    administrative_address,
                    directors_provided: Some(directors_provided),
                    directorship_declaration,
                    executives_provided: Some(executives_provided),
                    export_license_id: Some(export_license_id),
                    export_purpose_code: Some(export_purpose_code),
                    name: Some(name),
                    name_kana: Some(name_kana),
                    name_kanji: Some(name_kanji),
                    owners_provided: Some(owners_provided),
                    ownership_declaration,
                    ownership_exemption_reason,
                    phone: Some(phone),
                    principal_place_of_business,
                    registration_date,
                    representative_declaration,
                    structure,
                    tax_id_provided: Some(tax_id_provided),
                    tax_id_registrar: Some(tax_id_registrar),
                    vat_id_provided: Some(vat_id_provided),
                    verification,
                } => Some(Self::Out {
                    address,
                    address_kana,
                    address_kanji,
                    administrative_address,
                    directors_provided,
                    directorship_declaration,
                    executives_provided,
                    export_license_id,
                    export_purpose_code,
                    name,
                    name_kana,
                    name_kanji,
                    owners_provided,
                    ownership_declaration,
                    ownership_exemption_reason,
                    phone,
                    principal_place_of_business,
                    registration_date,
                    representative_declaration,
                    structure,
                    tax_id_provided,
                    tax_id_registrar,
                    vat_id_provided,
                    verification,
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

    impl ObjectDeser for LegalEntityCompany {
        type Builder = LegalEntityCompanyBuilder;
    }

    impl FromValueOpt for LegalEntityCompany {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
pub type LegalEntityCompanyOwnershipExemptionReason = stripe_shared::OwnershipExemptionReason;
/// The category identifying the legal structure of the company or legal entity.
/// Also available for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `stripe`.
/// See [Business structure](https://docs.stripe.com/connect/identity-verification#business-structure) for more details.
pub type LegalEntityCompanyStructure = stripe_shared::Structure;
