#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct AccountCapabilities {
    /// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
    pub acss_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
    pub affirm_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
    pub afterpay_clearpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Alma capability of the account, or whether the account can directly process Alma payments.
    pub alma_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments.
    pub amazon_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other accounts.
    pub app_distribution: Option<stripe_shared::MandateStatus>,
    /// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
    pub au_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
    pub bacs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
    pub bancontact_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
    pub bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Billie capability of the account, or whether the account can directly process Billie payments.
    pub billie_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Bizum capability of the account, or whether the account can directly process Bizum payments.
    pub bizum_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
    pub blik_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
    pub boleto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
    pub card_issuing: Option<stripe_shared::MandateStatus>,
    /// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
    pub card_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
    pub cartes_bancaires_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
    pub cashapp_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Crypto capability of the account, or whether the account can directly process Crypto payments.
    pub crypto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
    pub eps_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
    pub fpx_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can directly process GB customer_balance charges.
    pub gb_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
    pub giropay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
    pub grabpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
    pub ideal_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
    pub india_international_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
    pub jcb_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account can directly process Japanese customer_balance charges.
    pub jp_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments.
    pub kakao_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
    pub klarna_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
    pub konbini_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the KrCard capability of the account, or whether the account can directly process KrCard payments.
    pub kr_card_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the legacy payments capability of the account.
    pub legacy_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
    pub link_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges.
    pub mb_way_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges.
    pub mobilepay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Multibanco payments capability of the account, or whether the account can directly process Multibanco charges.
    pub multibanco_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account can directly process Mexican customer_balance charges.
    pub mx_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments.
    pub naver_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can directly process New Zealand BECS Direct Debit charges.
    pub nz_bank_account_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
    pub oxxo_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
    pub p24_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_bank charges.
    pub pay_by_bank_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Payco capability of the account, or whether the account can directly process Payco payments.
    pub payco_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
    pub paynow_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the PayTo capability of the account, or whether the account can directly process PayTo charges.
    pub payto_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the pix payments capability of the account, or whether the account can directly process pix charges.
    pub pix_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
    pub promptpay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
    pub revolut_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments.
    pub samsung_pay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Satispay capability of the account, or whether the account can directly process Satispay payments.
    pub satispay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments.
    pub scalapay_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
    pub sepa_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
    pub sepa_debit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
    pub sofort_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments.
    pub sunbit_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
    pub swish_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the tax reporting 1099-K (US) capability of the account.
    pub tax_reporting_us_1099_k: Option<stripe_shared::MandateStatus>,
    /// The status of the tax reporting 1099-MISC (US) capability of the account.
    pub tax_reporting_us_1099_misc: Option<stripe_shared::MandateStatus>,
    /// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
    pub transfers: Option<stripe_shared::MandateStatus>,
    /// The status of the banking capability, or whether the account can have bank accounts.
    pub treasury: Option<stripe_shared::MandateStatus>,
    /// The status of the TWINT capability of the account, or whether the account can directly process TWINT charges.
    pub twint_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the upi payments capability of the account, or whether the account can directly process upi charges.
    pub upi_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
    pub us_bank_account_ach_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
    pub us_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    /// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
    pub zip_payments: Option<stripe_shared::MandateStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountCapabilities").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct AccountCapabilitiesBuilder {
    acss_debit_payments: Option<stripe_shared::MandateStatus>,
    affirm_payments: Option<stripe_shared::MandateStatus>,
    afterpay_clearpay_payments: Option<stripe_shared::MandateStatus>,
    alma_payments: Option<stripe_shared::MandateStatus>,
    amazon_pay_payments: Option<stripe_shared::MandateStatus>,
    app_distribution: Option<stripe_shared::MandateStatus>,
    au_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    bacs_debit_payments: Option<stripe_shared::MandateStatus>,
    bancontact_payments: Option<stripe_shared::MandateStatus>,
    bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    billie_payments: Option<stripe_shared::MandateStatus>,
    bizum_payments: Option<stripe_shared::MandateStatus>,
    blik_payments: Option<stripe_shared::MandateStatus>,
    boleto_payments: Option<stripe_shared::MandateStatus>,
    card_issuing: Option<stripe_shared::MandateStatus>,
    card_payments: Option<stripe_shared::MandateStatus>,
    cartes_bancaires_payments: Option<stripe_shared::MandateStatus>,
    cashapp_payments: Option<stripe_shared::MandateStatus>,
    crypto_payments: Option<stripe_shared::MandateStatus>,
    eps_payments: Option<stripe_shared::MandateStatus>,
    fpx_payments: Option<stripe_shared::MandateStatus>,
    gb_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    giropay_payments: Option<stripe_shared::MandateStatus>,
    grabpay_payments: Option<stripe_shared::MandateStatus>,
    ideal_payments: Option<stripe_shared::MandateStatus>,
    india_international_payments: Option<stripe_shared::MandateStatus>,
    jcb_payments: Option<stripe_shared::MandateStatus>,
    jp_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    kakao_pay_payments: Option<stripe_shared::MandateStatus>,
    klarna_payments: Option<stripe_shared::MandateStatus>,
    konbini_payments: Option<stripe_shared::MandateStatus>,
    kr_card_payments: Option<stripe_shared::MandateStatus>,
    legacy_payments: Option<stripe_shared::MandateStatus>,
    link_payments: Option<stripe_shared::MandateStatus>,
    mb_way_payments: Option<stripe_shared::MandateStatus>,
    mobilepay_payments: Option<stripe_shared::MandateStatus>,
    multibanco_payments: Option<stripe_shared::MandateStatus>,
    mx_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    naver_pay_payments: Option<stripe_shared::MandateStatus>,
    nz_bank_account_becs_debit_payments: Option<stripe_shared::MandateStatus>,
    oxxo_payments: Option<stripe_shared::MandateStatus>,
    p24_payments: Option<stripe_shared::MandateStatus>,
    pay_by_bank_payments: Option<stripe_shared::MandateStatus>,
    payco_payments: Option<stripe_shared::MandateStatus>,
    paynow_payments: Option<stripe_shared::MandateStatus>,
    payto_payments: Option<stripe_shared::MandateStatus>,
    pix_payments: Option<stripe_shared::MandateStatus>,
    promptpay_payments: Option<stripe_shared::MandateStatus>,
    revolut_pay_payments: Option<stripe_shared::MandateStatus>,
    samsung_pay_payments: Option<stripe_shared::MandateStatus>,
    satispay_payments: Option<stripe_shared::MandateStatus>,
    scalapay_payments: Option<stripe_shared::MandateStatus>,
    sepa_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    sepa_debit_payments: Option<stripe_shared::MandateStatus>,
    sofort_payments: Option<stripe_shared::MandateStatus>,
    sunbit_payments: Option<stripe_shared::MandateStatus>,
    swish_payments: Option<stripe_shared::MandateStatus>,
    tax_reporting_us_1099_k: Option<stripe_shared::MandateStatus>,
    tax_reporting_us_1099_misc: Option<stripe_shared::MandateStatus>,
    transfers: Option<stripe_shared::MandateStatus>,
    treasury: Option<stripe_shared::MandateStatus>,
    twint_payments: Option<stripe_shared::MandateStatus>,
    upi_payments: Option<stripe_shared::MandateStatus>,
    us_bank_account_ach_payments: Option<stripe_shared::MandateStatus>,
    us_bank_transfer_payments: Option<stripe_shared::MandateStatus>,
    zip_payments: Option<stripe_shared::MandateStatus>,
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

    impl Deserialize for AccountCapabilities {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<AccountCapabilities>,
        builder: AccountCapabilitiesBuilder,
    }

    impl Visitor for Place<AccountCapabilities> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: AccountCapabilitiesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for AccountCapabilitiesBuilder {
        type Out = AccountCapabilities;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit_payments)
                }
                "affirm_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm_payments)
                }
                "afterpay_clearpay_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.afterpay_clearpay_payments,
                ),
                "alma_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.alma_payments)
                }
                "amazon_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay_payments)
                }
                "app_distribution" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.app_distribution)
                }
                "au_becs_debit_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.au_becs_debit_payments,
                ),
                "bacs_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit_payments)
                }
                "bancontact_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bancontact_payments)
                }
                "bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.bank_transfer_payments,
                ),
                "billie_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.billie_payments)
                }
                "bizum_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum_payments)
                }
                "blik_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.blik_payments)
                }
                "boleto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto_payments)
                }
                "card_issuing" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_issuing)
                }
                "card_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_payments)
                }
                "cartes_bancaires_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.cartes_bancaires_payments,
                ),
                "cashapp_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp_payments)
                }
                "crypto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto_payments)
                }
                "eps_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.eps_payments)
                }
                "fpx_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.fpx_payments)
                }
                "gb_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.gb_bank_transfer_payments,
                ),
                "giropay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay_payments)
                }
                "grabpay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay_payments)
                }
                "ideal_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal_payments)
                }
                "india_international_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.india_international_payments,
                ),
                "jcb_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.jcb_payments)
                }
                "jp_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.jp_bank_transfer_payments,
                ),
                "kakao_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay_payments)
                }
                "klarna_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna_payments)
                }
                "konbini_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini_payments)
                }
                "kr_card_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card_payments)
                }
                "legacy_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.legacy_payments)
                }
                "link_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.link_payments)
                }
                "mb_way_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mb_way_payments)
                }
                "mobilepay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.mobilepay_payments)
                }
                "multibanco_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco_payments)
                }
                "mx_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.mx_bank_transfer_payments,
                ),
                "naver_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay_payments)
                }
                "nz_bank_account_becs_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(
                        &mut self.nz_bank_account_becs_debit_payments,
                    )
                }
                "oxxo_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo_payments)
                }
                "p24_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.p24_payments)
                }
                "pay_by_bank_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pay_by_bank_payments)
                }
                "payco_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payco_payments)
                }
                "paynow_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.paynow_payments)
                }
                "payto_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.payto_payments)
                }
                "pix_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pix_payments)
                }
                "promptpay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.promptpay_payments)
                }
                "revolut_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.revolut_pay_payments)
                }
                "samsung_pay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.samsung_pay_payments)
                }
                "satispay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.satispay_payments)
                }
                "scalapay_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.scalapay_payments)
                }
                "sepa_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.sepa_bank_transfer_payments,
                ),
                "sepa_debit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit_payments)
                }
                "sofort_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort_payments)
                }
                "sunbit_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sunbit_payments)
                }
                "swish_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.swish_payments)
                }
                "tax_reporting_us_1099_k" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.tax_reporting_us_1099_k,
                ),
                "tax_reporting_us_1099_misc" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.tax_reporting_us_1099_misc,
                ),
                "transfers" => stripe_types::miniserde_helpers::begin_nullable(&mut self.transfers),
                "treasury" => stripe_types::miniserde_helpers::begin_nullable(&mut self.treasury),
                "twint_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.twint_payments)
                }
                "upi_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.upi_payments)
                }
                "us_bank_account_ach_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.us_bank_account_ach_payments,
                ),
                "us_bank_transfer_payments" => stripe_types::miniserde_helpers::begin_nullable(
                    &mut self.us_bank_transfer_payments,
                ),
                "zip_payments" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.zip_payments)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit_payments: None,
                affirm_payments: None,
                afterpay_clearpay_payments: None,
                alma_payments: None,
                amazon_pay_payments: None,
                app_distribution: None,
                au_becs_debit_payments: None,
                bacs_debit_payments: None,
                bancontact_payments: None,
                bank_transfer_payments: None,
                billie_payments: None,
                bizum_payments: None,
                blik_payments: None,
                boleto_payments: None,
                card_issuing: None,
                card_payments: None,
                cartes_bancaires_payments: None,
                cashapp_payments: None,
                crypto_payments: None,
                eps_payments: None,
                fpx_payments: None,
                gb_bank_transfer_payments: None,
                giropay_payments: None,
                grabpay_payments: None,
                ideal_payments: None,
                india_international_payments: None,
                jcb_payments: None,
                jp_bank_transfer_payments: None,
                kakao_pay_payments: None,
                klarna_payments: None,
                konbini_payments: None,
                kr_card_payments: None,
                legacy_payments: None,
                link_payments: None,
                mb_way_payments: None,
                mobilepay_payments: None,
                multibanco_payments: None,
                mx_bank_transfer_payments: None,
                naver_pay_payments: None,
                nz_bank_account_becs_debit_payments: None,
                oxxo_payments: None,
                p24_payments: None,
                pay_by_bank_payments: None,
                payco_payments: None,
                paynow_payments: None,
                payto_payments: None,
                pix_payments: None,
                promptpay_payments: None,
                revolut_pay_payments: None,
                samsung_pay_payments: None,
                satispay_payments: None,
                scalapay_payments: None,
                sepa_bank_transfer_payments: None,
                sepa_debit_payments: None,
                sofort_payments: None,
                sunbit_payments: None,
                swish_payments: None,
                tax_reporting_us_1099_k: None,
                tax_reporting_us_1099_misc: None,
                transfers: None,
                treasury: None,
                twint_payments: None,
                upi_payments: None,
                us_bank_account_ach_payments: None,
                us_bank_transfer_payments: None,
                zip_payments: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self {
                acss_debit_payments,
                affirm_payments,
                afterpay_clearpay_payments,
                alma_payments,
                amazon_pay_payments,
                app_distribution,
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                billie_payments,
                bizum_payments,
                blik_payments,
                boleto_payments,
                card_issuing,
                card_payments,
                cartes_bancaires_payments,
                cashapp_payments,
                crypto_payments,
                eps_payments,
                fpx_payments,
                gb_bank_transfer_payments,
                giropay_payments,
                grabpay_payments,
                ideal_payments,
                india_international_payments,
                jcb_payments,
                jp_bank_transfer_payments,
                kakao_pay_payments,
                klarna_payments,
                konbini_payments,
                kr_card_payments,
                legacy_payments,
                link_payments,
                mb_way_payments,
                mobilepay_payments,
                multibanco_payments,
                mx_bank_transfer_payments,
                naver_pay_payments,
                nz_bank_account_becs_debit_payments,
                oxxo_payments,
                p24_payments,
                pay_by_bank_payments,
                payco_payments,
                paynow_payments,
                payto_payments,
                pix_payments,
                promptpay_payments,
                revolut_pay_payments,
                samsung_pay_payments,
                satispay_payments,
                scalapay_payments,
                sepa_bank_transfer_payments,
                sepa_debit_payments,
                sofort_payments,
                sunbit_payments,
                swish_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                twint_payments,
                upi_payments,
                us_bank_account_ach_payments,
                us_bank_transfer_payments,
                zip_payments,
            } = core::mem::replace(self, Self::deser_default());
            Some(Self::Out {
                acss_debit_payments,
                affirm_payments,
                afterpay_clearpay_payments,
                alma_payments,
                amazon_pay_payments,
                app_distribution,
                au_becs_debit_payments,
                bacs_debit_payments,
                bancontact_payments,
                bank_transfer_payments,
                billie_payments,
                bizum_payments,
                blik_payments,
                boleto_payments,
                card_issuing,
                card_payments,
                cartes_bancaires_payments,
                cashapp_payments,
                crypto_payments,
                eps_payments,
                fpx_payments,
                gb_bank_transfer_payments,
                giropay_payments,
                grabpay_payments,
                ideal_payments,
                india_international_payments,
                jcb_payments,
                jp_bank_transfer_payments,
                kakao_pay_payments,
                klarna_payments,
                konbini_payments,
                kr_card_payments,
                legacy_payments,
                link_payments,
                mb_way_payments,
                mobilepay_payments,
                multibanco_payments,
                mx_bank_transfer_payments,
                naver_pay_payments,
                nz_bank_account_becs_debit_payments,
                oxxo_payments,
                p24_payments,
                pay_by_bank_payments,
                payco_payments,
                paynow_payments,
                payto_payments,
                pix_payments,
                promptpay_payments,
                revolut_pay_payments,
                samsung_pay_payments,
                satispay_payments,
                scalapay_payments,
                sepa_bank_transfer_payments,
                sepa_debit_payments,
                sofort_payments,
                sunbit_payments,
                swish_payments,
                tax_reporting_us_1099_k,
                tax_reporting_us_1099_misc,
                transfers,
                treasury,
                twint_payments,
                upi_payments,
                us_bank_account_ach_payments,
                us_bank_transfer_payments,
                zip_payments,
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

    impl ObjectDeser for AccountCapabilities {
        type Builder = AccountCapabilitiesBuilder;
    }

    impl FromValueOpt for AccountCapabilities {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// The status of the Canadian pre-authorized debits payments capability of the account, or whether the account can directly process Canadian pre-authorized debits charges.
pub type AccountCapabilitiesAcssDebitPayments = stripe_shared::MandateStatus;
/// The status of the Affirm capability of the account, or whether the account can directly process Affirm charges.
pub type AccountCapabilitiesAffirmPayments = stripe_shared::MandateStatus;
/// The status of the Afterpay Clearpay capability of the account, or whether the account can directly process Afterpay Clearpay charges.
pub type AccountCapabilitiesAfterpayClearpayPayments = stripe_shared::MandateStatus;
/// The status of the Alma capability of the account, or whether the account can directly process Alma payments.
pub type AccountCapabilitiesAlmaPayments = stripe_shared::MandateStatus;
/// The status of the AmazonPay capability of the account, or whether the account can directly process AmazonPay payments.
pub type AccountCapabilitiesAmazonPayPayments = stripe_shared::MandateStatus;
/// The status of the `app_distribution` capability of the account, or whether the platform can distribute apps to other accounts.
pub type AccountCapabilitiesAppDistribution = stripe_shared::MandateStatus;
/// The status of the BECS Direct Debit (AU) payments capability of the account, or whether the account can directly process BECS Direct Debit (AU) charges.
pub type AccountCapabilitiesAuBecsDebitPayments = stripe_shared::MandateStatus;
/// The status of the Bacs Direct Debits payments capability of the account, or whether the account can directly process Bacs Direct Debits charges.
pub type AccountCapabilitiesBacsDebitPayments = stripe_shared::MandateStatus;
/// The status of the Bancontact payments capability of the account, or whether the account can directly process Bancontact charges.
pub type AccountCapabilitiesBancontactPayments = stripe_shared::MandateStatus;
/// The status of the customer_balance payments capability of the account, or whether the account can directly process customer_balance charges.
pub type AccountCapabilitiesBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the Billie capability of the account, or whether the account can directly process Billie payments.
pub type AccountCapabilitiesBilliePayments = stripe_shared::MandateStatus;
/// The status of the Bizum capability of the account, or whether the account can directly process Bizum payments.
pub type AccountCapabilitiesBizumPayments = stripe_shared::MandateStatus;
/// The status of the blik payments capability of the account, or whether the account can directly process blik charges.
pub type AccountCapabilitiesBlikPayments = stripe_shared::MandateStatus;
/// The status of the boleto payments capability of the account, or whether the account can directly process boleto charges.
pub type AccountCapabilitiesBoletoPayments = stripe_shared::MandateStatus;
/// The status of the card issuing capability of the account, or whether you can use Issuing to distribute funds on cards.
pub type AccountCapabilitiesCardIssuing = stripe_shared::MandateStatus;
/// The status of the card payments capability of the account, or whether the account can directly process credit and debit card charges.
pub type AccountCapabilitiesCardPayments = stripe_shared::MandateStatus;
/// The status of the Cartes Bancaires payments capability of the account, or whether the account can directly process Cartes Bancaires card charges in EUR currency.
pub type AccountCapabilitiesCartesBancairesPayments = stripe_shared::MandateStatus;
/// The status of the Cash App Pay capability of the account, or whether the account can directly process Cash App Pay payments.
pub type AccountCapabilitiesCashappPayments = stripe_shared::MandateStatus;
/// The status of the Crypto capability of the account, or whether the account can directly process Crypto payments.
pub type AccountCapabilitiesCryptoPayments = stripe_shared::MandateStatus;
/// The status of the EPS payments capability of the account, or whether the account can directly process EPS charges.
pub type AccountCapabilitiesEpsPayments = stripe_shared::MandateStatus;
/// The status of the FPX payments capability of the account, or whether the account can directly process FPX charges.
pub type AccountCapabilitiesFpxPayments = stripe_shared::MandateStatus;
/// The status of the GB customer_balance payments (GBP currency) capability of the account, or whether the account can directly process GB customer_balance charges.
pub type AccountCapabilitiesGbBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the giropay payments capability of the account, or whether the account can directly process giropay charges.
pub type AccountCapabilitiesGiropayPayments = stripe_shared::MandateStatus;
/// The status of the GrabPay payments capability of the account, or whether the account can directly process GrabPay charges.
pub type AccountCapabilitiesGrabpayPayments = stripe_shared::MandateStatus;
/// The status of the iDEAL payments capability of the account, or whether the account can directly process iDEAL charges.
pub type AccountCapabilitiesIdealPayments = stripe_shared::MandateStatus;
/// The status of the india_international_payments capability of the account, or whether the account can process international charges (non INR) in India.
pub type AccountCapabilitiesIndiaInternationalPayments = stripe_shared::MandateStatus;
/// The status of the JCB payments capability of the account, or whether the account (Japan only) can directly process JCB credit card charges in JPY currency.
pub type AccountCapabilitiesJcbPayments = stripe_shared::MandateStatus;
/// The status of the Japanese customer_balance payments (JPY currency) capability of the account, or whether the account can directly process Japanese customer_balance charges.
pub type AccountCapabilitiesJpBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the KakaoPay capability of the account, or whether the account can directly process KakaoPay payments.
pub type AccountCapabilitiesKakaoPayPayments = stripe_shared::MandateStatus;
/// The status of the Klarna payments capability of the account, or whether the account can directly process Klarna charges.
pub type AccountCapabilitiesKlarnaPayments = stripe_shared::MandateStatus;
/// The status of the konbini payments capability of the account, or whether the account can directly process konbini charges.
pub type AccountCapabilitiesKonbiniPayments = stripe_shared::MandateStatus;
/// The status of the KrCard capability of the account, or whether the account can directly process KrCard payments.
pub type AccountCapabilitiesKrCardPayments = stripe_shared::MandateStatus;
/// The status of the legacy payments capability of the account.
pub type AccountCapabilitiesLegacyPayments = stripe_shared::MandateStatus;
/// The status of the link_payments capability of the account, or whether the account can directly process Link charges.
pub type AccountCapabilitiesLinkPayments = stripe_shared::MandateStatus;
/// The status of the MB WAY payments capability of the account, or whether the account can directly process MB WAY charges.
pub type AccountCapabilitiesMbWayPayments = stripe_shared::MandateStatus;
/// The status of the MobilePay capability of the account, or whether the account can directly process MobilePay charges.
pub type AccountCapabilitiesMobilepayPayments = stripe_shared::MandateStatus;
/// The status of the Multibanco payments capability of the account, or whether the account can directly process Multibanco charges.
pub type AccountCapabilitiesMultibancoPayments = stripe_shared::MandateStatus;
/// The status of the Mexican customer_balance payments (MXN currency) capability of the account, or whether the account can directly process Mexican customer_balance charges.
pub type AccountCapabilitiesMxBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the NaverPay capability of the account, or whether the account can directly process NaverPay payments.
pub type AccountCapabilitiesNaverPayPayments = stripe_shared::MandateStatus;
/// The status of the New Zealand BECS Direct Debit payments capability of the account, or whether the account can directly process New Zealand BECS Direct Debit charges.
pub type AccountCapabilitiesNzBankAccountBecsDebitPayments = stripe_shared::MandateStatus;
/// The status of the OXXO payments capability of the account, or whether the account can directly process OXXO charges.
pub type AccountCapabilitiesOxxoPayments = stripe_shared::MandateStatus;
/// The status of the P24 payments capability of the account, or whether the account can directly process P24 charges.
pub type AccountCapabilitiesP24Payments = stripe_shared::MandateStatus;
/// The status of the pay_by_bank payments capability of the account, or whether the account can directly process pay_by_bank charges.
pub type AccountCapabilitiesPayByBankPayments = stripe_shared::MandateStatus;
/// The status of the Payco capability of the account, or whether the account can directly process Payco payments.
pub type AccountCapabilitiesPaycoPayments = stripe_shared::MandateStatus;
/// The status of the paynow payments capability of the account, or whether the account can directly process paynow charges.
pub type AccountCapabilitiesPaynowPayments = stripe_shared::MandateStatus;
/// The status of the PayTo capability of the account, or whether the account can directly process PayTo charges.
pub type AccountCapabilitiesPaytoPayments = stripe_shared::MandateStatus;
/// The status of the pix payments capability of the account, or whether the account can directly process pix charges.
pub type AccountCapabilitiesPixPayments = stripe_shared::MandateStatus;
/// The status of the promptpay payments capability of the account, or whether the account can directly process promptpay charges.
pub type AccountCapabilitiesPromptpayPayments = stripe_shared::MandateStatus;
/// The status of the RevolutPay capability of the account, or whether the account can directly process RevolutPay payments.
pub type AccountCapabilitiesRevolutPayPayments = stripe_shared::MandateStatus;
/// The status of the SamsungPay capability of the account, or whether the account can directly process SamsungPay payments.
pub type AccountCapabilitiesSamsungPayPayments = stripe_shared::MandateStatus;
/// The status of the Satispay capability of the account, or whether the account can directly process Satispay payments.
pub type AccountCapabilitiesSatispayPayments = stripe_shared::MandateStatus;
/// The status of the Scalapay capability of the account, or whether the account can directly process Scalapay payments.
pub type AccountCapabilitiesScalapayPayments = stripe_shared::MandateStatus;
/// The status of the SEPA customer_balance payments (EUR currency) capability of the account, or whether the account can directly process SEPA customer_balance charges.
pub type AccountCapabilitiesSepaBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the SEPA Direct Debits payments capability of the account, or whether the account can directly process SEPA Direct Debits charges.
pub type AccountCapabilitiesSepaDebitPayments = stripe_shared::MandateStatus;
/// The status of the Sofort payments capability of the account, or whether the account can directly process Sofort charges.
pub type AccountCapabilitiesSofortPayments = stripe_shared::MandateStatus;
/// The status of the Sunbit capability of the account, or whether the account can directly process Sunbit payments.
pub type AccountCapabilitiesSunbitPayments = stripe_shared::MandateStatus;
/// The status of the Swish capability of the account, or whether the account can directly process Swish payments.
pub type AccountCapabilitiesSwishPayments = stripe_shared::MandateStatus;
/// The status of the tax reporting 1099-K (US) capability of the account.
pub type AccountCapabilitiesTaxReportingUs1099K = stripe_shared::MandateStatus;
/// The status of the tax reporting 1099-MISC (US) capability of the account.
pub type AccountCapabilitiesTaxReportingUs1099Misc = stripe_shared::MandateStatus;
/// The status of the transfers capability of the account, or whether your platform can transfer funds to the account.
pub type AccountCapabilitiesTransfers = stripe_shared::MandateStatus;
/// The status of the banking capability, or whether the account can have bank accounts.
pub type AccountCapabilitiesTreasury = stripe_shared::MandateStatus;
/// The status of the TWINT capability of the account, or whether the account can directly process TWINT charges.
pub type AccountCapabilitiesTwintPayments = stripe_shared::MandateStatus;
/// The status of the upi payments capability of the account, or whether the account can directly process upi charges.
pub type AccountCapabilitiesUpiPayments = stripe_shared::MandateStatus;
/// The status of the US bank account ACH payments capability of the account, or whether the account can directly process US bank account charges.
pub type AccountCapabilitiesUsBankAccountAchPayments = stripe_shared::MandateStatus;
/// The status of the US customer_balance payments (USD currency) capability of the account, or whether the account can directly process US customer_balance charges.
pub type AccountCapabilitiesUsBankTransferPayments = stripe_shared::MandateStatus;
/// The status of the Zip capability of the account, or whether the account can directly process Zip charges.
pub type AccountCapabilitiesZipPayments = stripe_shared::MandateStatus;
pub type AccountCapabilitiesStatus = stripe_shared::MandateStatus;
