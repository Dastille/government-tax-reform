//! The claims ledger for every input in [`crate::data`]: what each input claims, its
//! status, and its evidence. `CLAIMS.md` renders its inputs table from [`INPUTS`], and
//! `tests/sources.rs` enforces it:
//!
//! - every `pub const` in `data.rs` and every key of [`crate::data::FACTS`] is covered by
//!   exactly the claims that list it in `inputs`;
//! - every input of a [`Status::Source`] or [`Status::Derived`] claim has at least one
//!   check against a committed verbatim excerpt, and that excerpt is named in the evidence.
//!
//! So flipping an unverified input to `Source`, or deleting its row, fails the tests.

/// How a claim is supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Read from a primary source; the value is checked against a committed excerpt.
    Source,
    /// Arithmetic on sourced inputs; each input is checked against an excerpt.
    Derived,
    /// A policy choice in Ashlynn's design (ab193f4), not data.
    Design,
    /// A scenario parameter chosen for illustration, not data.
    Scenario,
    /// Not verified against any primary source.
    Unverified,
}

impl Status {
    /// Label shown in `CLAIMS.md`.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Source => "Verified (source excerpt)",
            Self::Derived => "Derived from verified inputs",
            Self::Design => "Design choice (ab193f4), not data",
            Self::Scenario => "Scenario parameter, not data",
            Self::Unverified => "**UNVERIFIED**",
        }
    }
}

/// One row of the inputs ledger. `claim` and `evidence` may hold `{{placeholders}}`.
#[derive(Clone, Copy, Debug)]
pub struct Claim {
    /// What the paper states.
    pub claim: &'static str,
    /// How it is supported.
    pub status: Status,
    /// Source and excerpt file, or the reason it is not verified.
    pub evidence: &'static str,
    /// Names of the `data.rs` constants or `FACTS` keys this row covers.
    pub inputs: &'static [&'static str],
}

const fn c(
    claim: &'static str,
    status: Status,
    evidence: &'static str,
    inputs: &'static [&'static str],
) -> Claim {
    Claim {
        claim,
        status,
        evidence,
        inputs,
    }
}

use Status::{Derived, Design, Scenario, Source, Unverified};

/// Every input claim, in the order shown in `CLAIMS.md`.
pub const INPUTS: &[Claim] = &[
    c(
        "PIT {{pit_m}}; CIT {{cit_m}}; non-resident income tax {{nr_m}} ($M, FY2024-25)",
        Source,
        "Public Accounts 2025, Vol I, Table 3.2 [R1]; `pa2025_vol1_s3`",
        &["PIT", "CIT", "NON_RESIDENT"],
    ),
    c(
        "GST {{gst_before_m}}, less the ministries' remission {{gst_remission_m}}, gives gross GST from outside parties {{gst_gross_m}}; less quarterly credits {{gst_credit_m}}, net GST {{gst_m}} ($M)",
        Source,
        "Public Accounts 2025, Vol I, Table 3.3 [R1]; `pa2025_vol1_s3`",
        &["GST_BEFORE_REMISSION", "GST_REMISSION", "GST_GROSS_EXTERNAL", "GST_CREDIT", "GST_NET"],
    ),
    c(
        "GST rate {{gst_rate}}%",
        Source,
        "Public Accounts 2025, Vol I, Section 3 text [R1]; `pa2025_vol1_s3`",
        &["GST_RATE"],
    ),
    c(
        "EI premiums {{ei_m}}; total revenues {{total_rev_m}} ($M); revenues {{rev_pct_gdp}}% of GDP",
        Source,
        "Annual Financial Report 2024-25, Table 4 and text [R2]; `afr2024_25`",
        &["EI_PREMIUMS", "TOTAL_REVENUE", "REVENUE_PCT_GDP"],
    ),
    c(
        "GDP ≈ ${{gdp_b}}B (total revenues ÷ revenue ratio)",
        Derived,
        "From the AFR values above; `afr2024_25`",
        &["TOTAL_REVENUE", "REVENUE_PCT_GDP"],
    ),
    c(
        "Tax-expenditure costs, calendar 2024 column: {{hh_count}} household zero-ratings and exemptions, public-body rebates and the small-supplier threshold",
        Source,
        "Federal Tax Expenditures 2026, Part 2 [R3]; `te2026_part2` (each row and column checked)",
        &["TE_HOUSEHOLD", "TE_PUBLIC_AND_THRESHOLD"],
    ),
    c(
        "GST holiday cost ${{holiday_b}}B ({{holiday_2024_m}} + {{holiday_2025_m}} $M)",
        Source,
        "Federal Tax Expenditures 2026, Part 2, \"Targeted Temporary GST/HST Relief\" [R3]; `te2026_part2`",
        &["GST_HOLIDAY_COST_2024", "GST_HOLIDAY_COST_2025", "GST_HOLIDAY_COST"],
    ),
    c(
        "Federal PIT bracket rates {{brackets_2023}}% (2023)",
        Source,
        "Federal Tax Expenditures 2026, Part 2, background table [R3]; `te2026_part2`",
        &["BRACKETS_2023"],
    ),
    c(
        "VRR: Canada {{vrr_ca}}, OECD average {{vrr_oecd}}, New Zealand {{vrr_nz}} (2022)",
        Source,
        "OECD Consumption Tax Trends 2024, Annex Table 2.A.7 [R4]; `oecd_ctt_2024`",
        &["VRR_CANADA", "VRR_OECD_AVERAGE", "VRR_NEW_ZEALAND"],
    ),
    c(
        "Household saving rate {{saving_2024}} (Q2 2024), {{saving_2025}} (Q2 2025)",
        Source,
        "StatCan, The Daily [R9, R10]; `statcan_daily_2024-09-12`, `statcan_daily_2025-09-11`",
        &["SAVING_RATE_Q2_2024", "SAVING_RATE_Q2_2025"],
    ),
    c(
        "Household consumption {{hfce_pct}}% and imports {{imports_pct}}% of GDP (Canada, 2024)",
        Source,
        "World Bank WDI, Table 4.8 [R11]; `wdi_table_4_8`",
        &["HFCE_PCT_GDP", "IMPORTS_PCT_GDP"],
    ),
    c(
        "Short-run multipliers: PIT measures {{mult_pit}}, CIT measures {{mult_cit}}",
        Source,
        "Budget 2009, Annex 1, Table A1.1, last column [R8]; `budget2009_annex1`",
        &["MULTIPLIER_PIT", "MULTIPLIER_CIT"],
    ),
    c(
        "National wealth ${{wealth_b}}B; residential real estate ${{res_re_b}}B (Q2 2025)",
        Source,
        "StatCan, The Daily [R9]; `statcan_daily_2025-09-11`",
        &["NATIONAL_WEALTH_Q2_2025", "RESIDENTIAL_REAL_ESTATE_Q2_2025"],
    ),
    c(
        "Provincial-local property tax ${{property_pc}} per person (2024); population {{population}} (1 July 2024)",
        Source,
        "Nova Scotia Finance, from StatCan CGFS; StatCan, The Daily; `ns_finance_gfs_2024`, `statcan_daily_2024-09-25`",
        &["PROPERTY_TAX_PER_CAPITA_2024", "POPULATION_2024"],
    ),
    c(
        "Recurrent property tax ≈ ${{property_b}}B (per person × population)",
        Derived,
        "From the values above; `ns_finance_gfs_2024`, `statcan_daily_2024-09-25`",
        &["PROPERTY_TAX_PER_CAPITA_2024", "POPULATION_2024"],
    ),
    c(
        "New Zealand GST {{nz_prev_rate}}% → {{nz_rate}}% from {{nz_rate_date}}",
        Source,
        "Inland Revenue NZ [R22]; `ird_nz_gst_2010`",
        &["nz_prev_rate", "nz_rate", "nz_rate_date"],
    ),
    c(
        "VAT as % of GDP / % of all taxes (2022): New Zealand {{nz_vat_gdp}} / {{nz_vat_taxes}}, Denmark {{dk_vat_gdp}} / {{dk_vat_taxes}}, Hungary {{hu_vat_gdp}} / {{hu_vat_taxes}}; standard rates Denmark {{dk_rate}}%, Hungary {{hu_rate}}%",
        Source,
        "OECD Consumption Tax Trends 2024, Annex Tables 1.A.4 and 2.A.7 [R4]; `oecd_ctt_2024`",
        &["nz_vat_gdp", "nz_vat_taxes", "dk_vat_gdp", "dk_vat_taxes", "hu_vat_gdp", "hu_vat_taxes", "dk_rate", "hu_rate"],
    ),
    c(
        "Saudi VAT {{sa_rate_2018}}% from {{sa_date_2018}}; {{sa_rate_2020}}% from {{sa_date_2020}}",
        Source,
        "ZATCA \"About VAT\" [R15]; Saudi Ministry of Finance [R15b]; `zatca_about_vat`, `saudi_mof_2020-05-11`",
        &["sa_rate_2018", "sa_date_2018", "sa_rate_2020", "sa_date_2020"],
    ),
    c(
        "Singapore: top resident PIT rate {{sg_pit_top}}%; GST {{sg_gst}}%",
        Source,
        "IRAS [R16]; `iras_rates`",
        &["sg_pit_top", "sg_gst"],
    ),
    c(
        "Canada 1991: {{gst_1991_rate}}% GST replaced the {{fst_rate}}% FST",
        Source,
        "Federal Tax Expenditures 2017, Part 10 [R14b]; Library of Parliament [R14]; `te2017_part10`, `lop_prb0003`",
        &["gst_1991_rate", "fst_rate"],
    ),
    c(
        "{{vat_countries}} countries and territories with a VAT",
        Source,
        "OECD Consumption Tax Trends 2024 [R4]; `oecd_ctt_2024`",
        &["vat_countries"],
    ),
    c(
        "CGEB increase {{cgeb_increase}}% (July 2026)",
        Source,
        "Canada Revenue Agency [R13]; `cra_cgeb`",
        &["cgeb_increase"],
    ),
    c(
        "FairTax rate {{fairtax_rate}}% of gross payments (2027)",
        Source,
        "H.R. 25 (119th Congress), introduced text [R20]; `hr25_119th`",
        &["fairtax_rate"],
    ),
    c(
        "Federal debt {{debt_pct_gdp}}% of GDP",
        Source,
        "Annual Financial Report 2024-25 [R2]; `afr2024_25`",
        &["debt_pct_gdp"],
    ),
    c(
        "Land-only value ~${{land_t}}T",
        Unverified,
        "Secondary source citing StatCan Table 36-10-0580-01; the table was not downloaded",
        &["LAND_VALUE_UNVERIFIED"],
    ),
    c(
        "Taxable share of land after exemptions {{lvt_share_pct}}%",
        Scenario,
        "Chosen to illustrate the design's exemptions",
        &["LVT_TAXABLE_SHARE_SCENARIO"],
    ),
    c(
        "Growth addition +{{growth_lo}}–{{growth_hi}} pp a year",
        Unverified,
        "From the original paper; no source found; what-if only",
        &["GROWTH_ADDITION_PP"],
    ),
    c(
        "VAT {{start_rate}}% → {{end_rate}}% over {{path_years}} years; LVT {{lvt_rate_lo}}–{{lvt_rate_hi}}% raising ${{lvt_claim_lo}}–{{lvt_claim_hi}}B; dividend for the bottom {{dividend_lo}}–{{dividend_hi}}%; {{invest_lo}}–{{invest_hi}}% of incremental revenue to infrastructure",
        Design,
        "Ashlynn's original design (ab193f4)",
        &[
            "START_RATE",
            "END_RATE",
            "PATH_YEARS",
            "LVT_RATES",
            "LVT_CLAIM_LOW",
            "LVT_CLAIM_HIGH",
            "DIVIDEND_BOTTOM_SHARE",
            "INVESTMENT_SHARE",
        ],
    ),
    // v3: debt-neutral transition and industry spin-up.
    c(
        "METR on new business investment {{v3_metr}}%, {{v3_metr_super}}% with the super-deduction",
        Source,
        "Federal budget, chapter on growth [R29]; excerpt `budget2025_ch1`",
        &["METR_2025", "METR_2025_SUPER"],
    ),
    c(
        "Input-output multipliers (latest year): GDP {{v3_io_gdp}} and {{v3_io_jobs}} jobs per $M of final demand; capital share {{v3_alpha}}",
        Unverified,
        "StatCan Table 36-10-0013-01 [R31], read from a search index copy; the live site was down on the access date, so no excerpt",
        &["IO_OUTPUT_2022", "IO_GDP_2022", "IO_GOS_2022", "IO_IMPORTS_2022", "IO_JOBS_2022"],
    ),
    c(
        "Capital semi-elasticity {{v3_semi}}% per point of METR",
        Unverified,
        "IMF working paper [R30], abstract as summarised by a search index; the IMF pages refused the fetch",
        &["USER_COST_SEMI_ELASTICITY_UNVERIFIED"],
    ),
    c(
        "GST/HST gap: net {{v3_gap_net}}%, gross {{v3_gap_gross}}% of revenue",
        Source,
        "CRA overall federal tax gap report [R34]; excerpt `cra_tax_gap_2026`",
        &["GST_GAP_NET_2022", "GST_GAP_GROSS_2022"],
    ),
    c(
        "Direct program expenses ${{v3_direct}}B; deficit ${{v3_deficit}}B; average rate on unmatured debt {{v3_interest}}%",
        Source,
        "Annual Financial Report, expense and debt tables and statement of financial position [R2]; excerpt `afr2024_25_expenses`",
        &["DIRECT_PROGRAM_EXPENSES", "OPERATING_DEFICIT", "PUBLIC_DEBT_CHARGES", "UNMATURED_DEBT"],
    ),
    c(
        "Program Review: about ${{pr_scope_b}}B covered; spending down {{v3_pr_pct}}% relative to {{pr_base_year}}; ${{pr_cuts_b}}B of cuts; interim target {{pr_deficit_target}}% of GDP (${{pr_deficit_target_b}}B)",
        Source,
        "Program Review budget plan [R23]; excerpt `budget1995`",
        &[
            "PROGRAM_REVIEW_SCOPE",
            "PROGRAM_REVIEW_DECLINE_PCT",
            "pr_scope_b",
            "pr_base_year",
            "pr_cuts_b",
            "pr_deficit_target",
            "pr_deficit_target_b",
            "pr_ratio",
        ],
    ),
    c(
        "Swiss debt brake: approved by {{ch_yes_vote}}% on {{ch_vote_date}}; debt ratio from {{ch_debt_2003}} to {{ch_debt_2019}}",
        Source,
        "Swiss Federal Finance Administration [R24]; excerpt `efv_debt_brake`",
        &["ch_yes_vote", "ch_vote_date", "ch_debt_2003", "ch_debt_2019"],
    ),
    c(
        "Germany: structural borrowing up to {{de_structural}}% of GDP; control account limit {{de_control}}%",
        Source,
        "Act implementing the German debt brake [R25]; excerpt `bmf_art115`",
        &["de_structural", "de_control"],
    ),
    c(
        "Sweden: debt anchor {{se_debt_anchor}}% of GDP; framework from {{se_origin}}",
        Source,
        "Government Offices of Sweden [R26]; excerpt `gov_se_framework`",
        &["se_debt_anchor", "se_origin"],
    ),
    c(
        "New Zealand: GST at {{nz_gst_1986}}% from {{nz_date}}; top income tax rate {{nz_top_1986}}c",
        Source,
        "Inland Revenue [R27, R28]; excerpts `ird_gst_bill_1985`, `ird_ita2_1986`",
        &["nz_gst_1986", "nz_date", "nz_top_1986"],
    ),
    c(
        "Ireland: GNI* {{ie_gni_share}}% of GDP (latest year)",
        Source,
        "CSO Annual National Accounts [R32]; excerpt `cso_ana2024_gni`",
        &["IE_GDP_2024", "IE_GNI_STAR_2024", "ie_gni_share"],
    ),
    c(
        "Estonia: profits taxed when distributed, at {{ee_rate}}",
        Source,
        "Estonian Tax and Customs Board [R33]; excerpt `emta_companies`",
        &["ee_rate"],
    ),
    c(
        "Spin-up cases: semi-elasticity scale, METR cut and speed; recession of {{v3_drop}}% in one year",
        Scenario,
        "Chosen to span low, central and high cases; not estimates",
        &[
            "SPINUP_ELASTICITY_SCALE",
            "SPINUP_METR_CUT_PP",
            "SPINUP_SPEED",
            "RECESSION_BASE_DROP_PCT",
            "RECESSION_YEAR",
        ],
    ),
    c(
        "Rule: repay the compensation account over {{v3_repay_design}} years; simulate {{v3_horizon}} years",
        Design,
        "Design choice of v3.0 (not in ab193f4); the Swiss rule [R24] sets no fixed repayment period",
        &["REPAY_YEARS", "HORIZON_YEARS"],
    ),
];
