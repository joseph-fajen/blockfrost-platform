use std::{
    arch::x86_64, collections::HashMap, fmt::{self}, ops::Deref
};

 use pallas_addresses::{
    byron::{AddrAttrProperty, AddrType, AddressPayload},
    Address, ByronAddress, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart,
};
use pallas_codec::utils::OrderPreservingProperties;
use pallas_primitives::{
    conway::{
        Anchor, Certificate, Constitution, CostModels, DRepVotingThresholds, ExUnitPrices, GovAction, GovActionId, PoolVotingThresholds, ProposalProcedure, ProtocolParamUpdate, VKeyWitness, Voter
    },
    Bytes, DatumHash, ExUnits, Hash, KeyValuePairs, Nullable, ProtocolVersion, RationalNumber,
    RewardAccount, ScriptHash, StakeCredential, TransactionInput,
};

use crate::cbor::haskell_types::get_network_and_credentials;

use super::haskell_types::{
    AddressBytes, Array, AsItem, AsIx, BabbageTxOut, CollectError, ConwayCertPredFailure, ConwayDelegCert, ConwayDelegPredFailure, ConwayGovCertPredFailure, ConwayGovPredFailure, ConwayPlutusPurpose, ConwayTxCert, ConwayUtxoWPredFailure, ConwayUtxosPredFailure, Credential, CustomSet258, DatumEnum, Delegatee, DeltaCoin, DisplayAddress, DisplayAssetName, DisplayCoin, DisplayDatum, DisplayDatumHash, DisplayHash, DisplayPolicyId, DisplayScriptHash, DisplayValue, EpochNo, EraScript, FailureDescription, KeyHash, MaryValue, Mismatch, MultiAsset, PlutusPurpose, PurposeAs, RewardAccountFielded, SafeHash, SerializableTxOut, ShelleyPoolPredFailure, SlotNo, StrictMaybe, TagMismatchDescription, Timelock, TimelockRaw, Utxo, VKey, ValidityInterval
};

use super::haskells_show_string::haskell_show_string;

pub trait HaskellDisplay {
    fn to_haskell_str(&self) -> String;
    fn to_haskell_str_p(&self) -> String {
        format!("({})", self.to_haskell_str())
    }
}

impl fmt::Display for ConwayGovCertPredFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConwayGovCertPredFailure::*;

        match self {
            ConwayDRepAlreadyRegistered(cred) => {
                write!(f, "ConwayDRepAlreadyRegistered ({})", cred)
            }
            ConwayDRepNotRegistered(cred) => write!(f, "ConwayDRepNotRegistered ({})", cred),
            ConwayDRepIncorrectDeposit(expected, actual) => {
                write!(f, "ConwayDRepIncorrectDeposit ({}) ({})", expected, actual)
            }
            ConwayCommitteeHasPreviouslyResigned(cred) => {
                write!(f, "ConwayCommitteeHasPreviouslyResigned ({})", cred)
            }
            ConwayDRepIncorrectRefund(expected, actual) => {
                write!(f, "ConwayDRepIncorrectRefund ({}) ({})", expected, actual)
            }
            ConwayCommitteeIsUnknown(cred) => write!(f, "ConwayCommitteeIsUnknown ({})", cred),
        }
    }
}

impl fmt::Display for ConwayCertPredFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConwayCertPredFailure::*;

        match self {
            DelegFailure(e) => write!(f, "DelegFailure ({})", e.to_haskell_str()),
            PoolFailure(e) => write!(f, "PoolFailure ({})", e.to_haskell_str()),
            GovCertFailure(e) => write!(f, "GovCertFailure ({})", e),
        }
    }
}

impl HaskellDisplay for ShelleyPoolPredFailure {
    fn to_haskell_str(&self) -> String {
        use ShelleyPoolPredFailure::*;
        match self {
            StakePoolNotRegisteredOnKeyPOOL(kh) => {
                format!("StakePoolNotRegisteredOnKeyPOOL {}", kh.to_haskell_str_p())
            }
            StakePoolRetirementWrongEpochPOOL(mis1, mis2) => {
                format!(
                    "StakePoolRetirementWrongEpochPOOL {} {}",
                    mis1.to_haskell_str_p(),
                    mis2.to_haskell_str_p()
                )
            }
            StakePoolCostTooLowPOOL(mis1) => {
                format!("StakePoolCostTooLowPOOL {}", mis1.to_haskell_str_p())
            }
            WrongNetworkPOOL(mis1, kh) => {
                format!(
                    "WrongNetworkPOOL {} {}",
                    mis1.to_haskell_str_p(),
                    kh.to_haskell_str_p()
                )
            }
            PoolMedataHashTooBig(kh, size) => {
                format!(
                    "PoolMedataHashTooBig {} {}",
                    kh.to_haskell_str_p(),
                    size.to_haskell_str()
                )
            }
        }
    }
}

impl fmt::Display for ConwayUtxoWPredFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConwayUtxoWPredFailure::*;

        match self {
            UtxoFailure(e) => write!(f, "(UtxoFailure {})", e),
            InvalidWitnessesUTXOW(e) => {
                write!(f, "(InvalidWitnessesUTXOW {})", e.to_haskell_str())
            }
            MissingVKeyWitnessesUTXOW(e) => {
                write!(f, "(MissingVKeyWitnessesUTXOW ({}))", e.to_haskell_str())
            }
            MissingScriptWitnessesUTXOW(e) => {
                write!(f, "(MissingScriptWitnessesUTXOW ({}))", e.to_haskell_str())
            }
            ScriptWitnessNotValidatingUTXOW(e) => {
                write!(
                    f,
                    "(ScriptWitnessNotValidatingUTXOW ({}))",
                    e.to_haskell_str()
                )
            }
            MissingTxBodyMetadataHash(b) => {
                write!(f, "(MissingTxBodyMetadataHash ({}))", b.as_aux_data_hash())
            }
            MissingTxMetadata(e) => write!(f, "(MissingTxMetadata ({}))", e.as_aux_data_hash()),
            ConflictingMetadataHash(e1, e2) => {
                write!(
                    f,
                    "(ConflictingMetadataHash ({}) ({}))",
                    e1.as_aux_data_hash(),
                    e2.as_aux_data_hash()
                )
            }
            InvalidMetadata() => write!(f, "InvalidMetadata"),
            ExtraneousScriptWitnessesUTXOW(vec) => {
                write!(
                    f,
                    "(ExtraneousScriptWitnessesUTXOW ({}))",
                    vec.to_haskell_str()
                )
            }
            MissingRedeemers(e) => write!(f, "(MissingRedeemers {})", e.to_haskell_str()),
            MissingRequiredDatums(e1, e2) => write!(
                f,
                "(MissingRequiredDatums ({}) ({}))",
                e1.to_haskell_str(),
                e2.to_haskell_str()
            ),
            NotAllowedSupplementalDatums(e1, e2) => write!(
                f,
                "(NotAllowedSupplementalDatums ({}) ({}))",
                e1.to_haskell_str(),
                e2.to_haskell_str()
            ),
            PPViewHashesDontMatch(h1, h2) => write!(
                f,
                "(PPViewHashesDontMatch {} {})",
                h1.to_haskell_str_p(),
                h2.to_haskell_str_p()
            ),
            UnspendableUTxONoDatumHash(e) => {
                write!(f, "(UnspendableUTxONoDatumHash ({}))", e.to_haskell_str())
            }
            ExtraRedeemers(e) => write!(f, "(ExtraRedeemers {})", e.to_haskell_str()),
            MalformedScriptWitnesses(set) => {
                write!(f, "(MalformedScriptWitnesses ({}))", set.to_haskell_str())
            }
            MalformedReferenceScripts(set) => {
                write!(f, "(MalformedReferenceScripts ({}))", set.to_haskell_str())
            }
        }
    }
}

impl fmt::Display for ConwayGovPredFailure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ConwayGovPredFailure::*;
        match self {
            GovActionsDoNotExist(vec) => {
                write!(f, "GovActionsDoNotExist ({})", vec.to_haskell_str())
            }
            MalformedProposal(act) => write!(f, "MalformedProposal {}", act.to_haskell_str_p()),
            ProposalProcedureNetworkIdMismatch(ra, n) => {
                write!(
                    f,
                    "ProposalProcedureNetworkIdMismatch ({}) {}",
                    ra,
                    n.to_haskell_str()
                )
            }
            TreasuryWithdrawalsNetworkIdMismatch(set, n) => {
                write!(
                    f,
                    "TreasuryWithdrawalsNetworkIdMismatch ({}) {}",
                    set.to_haskell_str(),
                    n.to_haskell_str()
                )
            }
            ProposalDepositIncorrect(c1, c2) => {
                write!(f, "ProposalDepositIncorrect ({}) ({})", c1, c2)
            }
            DisallowedVoters(v) => write!(f, "DisallowedVoters ({})", v.to_haskell_str()),
            ConflictingCommitteeUpdate(set) => {
                write!(f, "ConflictingCommitteeUpdate ({})", set.to_haskell_str())
            }
            ExpirationEpochTooSmall(map) => {
                write!(f, "ExpirationEpochTooSmall ({})", map.to_haskell_str())
            }
            InvalidPrevGovActionId(s) => {
                write!(f, "InvalidPrevGovActionId ({})", s.to_haskell_str())
            }
            VotingOnExpiredGovAction(vec) => {
                write!(f, "VotingOnExpiredGovAction ({})", vec.to_haskell_str())
            }
            ProposalCantFollow(a, p1, p2) => write!(
                f,
                "ProposalCantFollow {} ({}) ({})",
                a.to_haskell_str_p(),
                p1.as_protocol_version(),
                p2.as_protocol_version()
            ),
            InvalidPolicyHash(maybe1, maybe2) => write!(
                f,
                "InvalidPolicyHash {} {}",
                maybe1.to_haskell_str_p(),
                maybe2.to_haskell_str_p()
            ),
            DisallowedProposalDuringBootstrap(s) => {
                write!(
                    f,
                    "DisallowedProposalDuringBootstrap ({})",
                    s.to_haskell_str()
                )
            }
            DisallowedVotesDuringBootstrap(v) => {
                write!(f, "DisallowedVotesDuringBootstrap ({})", v.to_haskell_str())
            }
            VotersDoNotExist(s) => write!(f, "VotersDoNotExist ({})", s.to_haskell_str()),
            ZeroTreasuryWithdrawals(s) => {
                write!(f, "ZeroTreasuryWithdrawals {}", s.to_haskell_str_p())
            }
            ProposalReturnAccountDoesNotExist(s) => {
                write!(f, "ProposalReturnAccountDoesNotExist ({})", s)
            }
            TreasuryWithdrawalReturnAccountsDoNotExist(s) => {
                write!(
                    f,
                    "TreasuryWithdrawalReturnAccountsDoNotExist ({})",
                    s.to_haskell_str()
                )
            }
        }
    }
}

impl HaskellDisplay for ConwayUtxosPredFailure {
    fn to_haskell_str(&self) -> String {
        use ConwayUtxosPredFailure::*;

        match self {
            ValidationTagMismatch(is_valid, desc) => 
            format!("ValidationTagMismatch ({}) {}", is_valid.as_is_valid(), desc.to_haskell_str_p()),
            CollectErrors(errors) => format!("CollectErrors {}", errors.to_haskell_str()),
        }
    }
}

impl HaskellDisplay for CollectError {
    fn to_haskell_str(&self) -> String {
        format!("CollectError {{ ... }}")
    }
}

impl HaskellDisplay for TagMismatchDescription {
    fn to_haskell_str(&self) -> String {
        use TagMismatchDescription::*;
        match self {
            PassedUnexpectedly => "PassedUnexpectedly".to_string(),
            FailedUnexpectedly(desc) => format!("FailedUnexpectedly {}", desc.to_haskell_str_p()),
        }
    }
}

impl HaskellDisplay for FailureDescription {
    fn to_haskell_str(&self) -> String {
        
        use FailureDescription::*;
        match self {
            PlutusFailure(s, b) => format!("PlutusFailure {} {}", s.to_haskell_str(), b.to_haskell_str()),
        }
    }
}


impl HaskellDisplay for ConwayDelegPredFailure {
    fn to_haskell_str(&self) -> String {
        use ConwayDelegPredFailure::*;

        match self {
            IncorrectDepositDELEG(coin) => {
                format!("IncorrectDepositDELEG ({})", coin.to_haskell_str())
            }
            StakeKeyRegisteredDELEG(cred) => {
                format!("StakeKeyRegisteredDELEG ({})", cred.to_haskell_str())
            }
            StakeKeyNotRegisteredDELEG(cred) => {
                format!("StakeKeyNotRegisteredDELEG ({})", cred.to_haskell_str())
            }
            StakeKeyHasNonZeroRewardAccountBalanceDELEG(coin) => format!(
                "StakeKeyHasNonZeroRewardAccountBalanceDELEG ({})",
                coin.to_haskell_str()
            ),
            DelegateeDRepNotRegisteredDELEG(cred) => format!(
                "DelegateeDRepNotRegisteredDELEG ({})",
                cred.to_haskell_str()
            ),
            DelegateeStakePoolNotRegisteredDELEG(hash) => format!(
                "DelegateeStakePoolNotRegisteredDELEG ({})",
                hash.to_haskell_str()
            ),
        }
    }
}

impl HaskellDisplay for TransactionInput {
    fn to_haskell_str(&self) -> String {
        format!(
            "TxIn ({}) ({})",
            self.transaction_id.as_tx_id(),
            self.index.as_tx_ix()
        )
    }
}

impl<T> HaskellDisplay for Mismatch<T>
where
    T: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        format!(
            "Mismatch {{mismatchSupplied = {}, mismatchExpected = {}}}",
            self.0.to_haskell_str(),
            self.1.to_haskell_str()
        )
    }
}
impl HaskellDisplay for RewardAccountFielded {
    fn to_haskell_str(&self) -> String {
        format!(
            "RewardAccount {{raNetwork = {}, raCredential = {}}}",
            self.ra_network.to_haskell_str(),
            self.ra_credential.to_haskell_str()
        )
    }
}

impl fmt::Display for RewardAccountFielded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_haskell_str())
    }
}

impl HaskellDisplay for KeyHash {
    fn to_haskell_str(&self) -> String {
        format!("KeyHash {{unKeyHash = \"{}\"}}", self.0)
    }
}

impl HaskellDisplay for SafeHash {
    fn to_haskell_str(&self) -> String {
        self.0.as_safe_hash()
    }
}

impl HaskellDisplay for GovActionId {
    fn to_haskell_str(&self) -> String {
        format!(
            "GovActionId {{gaidTxId = {}, gaidGovActionIx = {}}}",
            self.transaction_id.as_tx_id(),
            display_governance_action_id_index(&self.action_index)
        )
    }
}

impl HaskellDisplay for ValidityInterval {
    fn to_haskell_str(&self) -> String {
        format!(
            "(ValidityInterval {{invalidBefore = {}, invalidHereafter = {}}})",
            &self.invalid_before.to_haskell_str(),
            &self.invalid_hereafter.to_haskell_str()
        )
    }
}

impl<T> HaskellDisplay for Nullable<T>
where
    T: HaskellDisplay + std::clone::Clone + 'static,
{
    fn to_haskell_str(&self) -> String {
        match self {
            Nullable::Some(v) => {
                if is_primitive::<T>() {
                    format!("SJust {}", v.to_haskell_str())
                } else {
                    format!("SJust ({})", v.to_haskell_str())
                }
            }
            _ => "SNothing".to_string(),
        }
    }

    fn to_haskell_str_p(&self) -> String {
        let str = self.to_haskell_str();
        if &str == "SNothing" {
            return str;
        }
        format!("({})", self.to_haskell_str())
    }
}

impl<T> HaskellDisplay for Option<T>
where
    T: HaskellDisplay + 'static,
{
    fn to_haskell_str(&self) -> String {
        match self {
            Option::Some(v) => {
                if is_primitive::<T>() {
                    format!("SJust {}", v.to_haskell_str())
                } else {
                    format!("SJust ({})", v.to_haskell_str())
                }
            }
            _ => "SNothing".to_string(),
        }
    }

    fn to_haskell_str_p(&self) -> String {
        let str = self.to_haskell_str();
        if &str == "SNothing" {
            return str;
        }
        format!("({})", self.to_haskell_str())
    }
}

fn is_primitive<T: 'static>() -> bool {
    std::any::TypeId::of::<T>() == std::any::TypeId::of::<bool>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<char>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<u8>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<u16>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<u64>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<i8>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<i16>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<i64>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<f32>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<f64>()
        || std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>()
}
impl HaskellDisplay for GovAction {
    fn to_haskell_str(&self) -> String {
        use GovAction::*;

        match self {
            ParameterChange(a, b, c) => {
                format!(
                    "ParameterChange {} {} {}",
                    a.to_haskell_str_p(),
                    b.to_haskell_str(),
                    c.to_haskell_str(),
                )
            }
            HardForkInitiation(a, b) => {
                format!(
                    "HardForkInitiation {} ({})",
                    a.to_haskell_str_p(),
                    b.as_protocol_version()
                )
            }
            TreasuryWithdrawals(a, b) => {
                let data: KeyValuePairs<RewardAccountFielded, DisplayCoin> =
                    a.iter().map(|(k, v)| (k.into(), v.into())).collect();

                format!(
                    "TreasuryWithdrawals {} {}",
                    data.to_haskell_str_p(),
                    b.to_haskell_str_p(),
                )
            }
            NoConfidence(a) => {
                format!("NoConfidence {}", a.to_haskell_str_p())
            }
            UpdateCommittee(a, b, c, d) => {
                format!(
                    "UpdateCommittee {} {} {} {}",
                    a.to_haskell_str_p(),
                    b.to_haskell_str_p(),
                    c.to_haskell_str_p(),
                    d.to_haskell_str_p()
                )
            }
            NewConstitution(a, c) => {
                format!(
                    "NewConstitution ({}) ({})",
                    a.to_haskell_str(),
                    c.to_haskell_str()
                )
            }
            Information => "InfoAction".to_string(),
        }
    }

    fn to_haskell_str_p(&self) -> String {
        let str = self.to_haskell_str();

        if str == "InfoAction" {
            return str;
        }
        format!("({})", self.to_haskell_str())
    }
}

// https://github.com/IntersectMBO/cardano-ledger/blob/7683b73971a800b36ca7317601552685fa0701ed/eras/conway/impl/src/Cardano/Ledger/Conway/PParams.hs#L511
impl HaskellDisplay for ProtocolParamUpdate {
    fn to_haskell_str(&self) -> String {
        format!(
            "(PParamsUpdate (ConwayPParams {{cppMinFeeA = {}, cppMinFeeB = {}, cppMaxBBSize = {}, cppMaxTxSize = {}, cppMaxBHSize = {}, cppKeyDeposit = {}, cppPoolDeposit = {}, \
             cppEMax = {}, cppNOpt = {}, cppA0 = {}, cppRho = {}, cppTau = {}, cppProtocolVersion = {}, cppMinPoolCost = {}, cppCoinsPerUTxOByte = {}, cppCostModels = {}, \
             cppPrices = {}, cppMaxTxExUnits = {}, cppMaxBlockExUnits = {}, cppMaxValSize = {}, cppCollateralPercentage = {}, cppMaxCollateralInputs = {}, \
             cppPoolVotingThresholds = {}, cppDRepVotingThresholds = {}, cppCommitteeMinSize = {}, cppCommitteeMaxTermLength = {}, cppGovActionLifetime = {}, \
             cppGovActionDeposit = {}, cppDRepDeposit = {}, cppDRepActivity = {}, cppMinFeeRefScriptCostPerByte = {}}}))",
            self.minfee_a.as_display_coin(),
            self.minfee_b.as_display_coin(),
            self.max_block_body_size.to_haskell_str(),
            self.max_transaction_size.to_haskell_str(),
            self.max_block_header_size.to_haskell_str(),
            self.key_deposit.as_display_coin(),
            self.pool_deposit.as_display_coin(),
            self.maximum_epoch.as_epoch_interval(),
            self.desired_number_of_stake_pools.to_haskell_str(),
            self.pool_pledge_influence.to_haskell_str(),
            self.expansion_rate.to_haskell_str(),
            self.treasury_growth_rate.to_haskell_str(),
            "NoUpdate",
            self.min_pool_cost.to_haskell_str(),
            self.ada_per_utxo_byte.as_display_coin(),
            self.cost_models_for_script_languages.to_haskell_str(),
            self.execution_costs.to_haskell_str(),
            self.max_tx_ex_units.to_haskell_str(),
            self.max_block_ex_units.to_haskell_str(),
            self.max_value_size.to_haskell_str(),
            self.collateral_percentage.to_haskell_str(),
            self.max_collateral_inputs.to_haskell_str(),
            self.pool_voting_thresholds.to_haskell_str(),
            self.drep_voting_thresholds.to_haskell_str(),
            self.min_committee_size.to_haskell_str(),
            self.committee_term_limit.as_epoch_interval(),
            self.governance_action_validity_period.to_haskell_str(),
            self.governance_action_deposit.as_display_coin(),
            self.drep_deposit.as_display_coin(),
            self.drep_inactivity_period.as_epoch_interval(),
            self.minfee_refscript_cost_per_byte.to_haskell_str()




        )
    }
}

impl HaskellDisplay for PoolVotingThresholds {
    fn to_haskell_str(&self) -> String {
        // Implement your display logic here
        format!("PoolVotingThresholds {{pvtMotionNoConfidence = {}, pvtCommitteeNormal = {}, pvtCommitteeNoConfidence = {}, pvtHardForkInitiation = {}, pvtPPSecurityGroup = {}}}",
        self.motion_no_confidence.to_haskell_str(),
        self.committee_normal.to_haskell_str(),
        self.committee_no_confidence.to_haskell_str(),
        self.hard_fork_initiation.to_haskell_str(),
        self.security_voting_threshold.to_haskell_str()
)
    }
}

impl HaskellDisplay for DRepVotingThresholds {
    fn to_haskell_str(&self) -> String {
        // Implement your display logic here
        format!("DRepVotingThresholds {{dvtMotionNoConfidence = {}, dvtCommitteeNormal = {}, dvtCommitteeNoConfidence = {}, \
     dvtUpdateToConstitution = {}, dvtHardForkInitiation = {}, dvtPPNetworkGroup = {}, dvtPPEconomicGroup = {}, dvtPPTechnicalGroup = {}, dvtPPGovGroup = {}, dvtTreasuryWithdrawal = {}}}",
        self.motion_no_confidence.to_haskell_str(),
        self.committee_normal.to_haskell_str(),
        self.committee_no_confidence.to_haskell_str(),
        self.update_constitution.to_haskell_str(),
        self.hard_fork_initiation.to_haskell_str(),
        self.pp_network_group.to_haskell_str(),
        self.pp_economic_group.to_haskell_str(),
        self.pp_technical_group.to_haskell_str(),
        self.pp_governance_group.to_haskell_str(),
        self.treasury_withdrawal.to_haskell_str()
)
    }
}

impl HaskellDisplay for CostModels {
    fn to_haskell_str(&self) -> String {
        format!("CostModels [{:?}]", self)
    }
}
impl HaskellDisplay for ExUnits {
    fn to_haskell_str(&self) -> String {
        format!(
            "WrapExUnits {{unWrapExUnits = ExUnits' {{exUnitsMem' = {:?}, exUnitsSteps' = {}}}}}",
            self.mem, self.steps
        )
    }
}
impl HaskellDisplay for ExUnitPrices {
    fn to_haskell_str(&self) -> String {
        format!(
            "Prices {{prMem = {}, prSteps = {}}}",
            self.mem_price.to_haskell_str(),
            self.step_price.to_haskell_str()
        )
    }
}
impl HaskellDisplay for RationalNumber {
    fn to_haskell_str(&self) -> String {
        format!("{} % {}", self.numerator, self.denominator)
    }
}

impl HaskellDisplay for Constitution {
    fn to_haskell_str(&self) -> String {
        format!(
            "Constitution {{constitutionAnchor = {}, constitutionScript = {}}}",
            self.anchor.to_haskell_str(),
            self.guardrail_script.to_haskell_str()
        )
    }
}
impl HaskellDisplay for Anchor {
    fn to_haskell_str(&self) -> String {
        format!(
            "Anchor {{anchorUrl = {}, anchorDataHash = {}}}",
            self.url.as_url(),
            self.content_hash.as_safe_hash()
        )
    }
}

impl HaskellDisplay for ProposalProcedure {
    fn to_haskell_str(&self) -> String {
        format!(
            "ProposalProcedure {{pProcDeposit = {}, pProcReturnAddr = {}, pProcGovAction = {}, pProcAnchor = {}}}",
            self.deposit.as_display_coin(), self.reward_account.as_reward_account_fielded(), self.gov_action.to_haskell_str(), self.anchor.to_haskell_str()
        )
    }
}

impl HaskellDisplay for ScriptHash {
    fn to_haskell_str(&self) -> String {
        format!("ScriptHash \"{}\"", self)
    }
}

impl HaskellDisplay for StakeCredential {
    fn to_haskell_str(&self) -> String {
        use StakeCredential::*;

        match self {
            AddrKeyhash(key_hash) => key_hash.as_key_hash_obj(),
            ScriptHash(scrpt) => scrpt.as_script_hash_obj(),
        }
    }
}

impl fmt::Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Credential::*;

        match self {
            ScriptHashObj(key_hash) => write!(f, "ScriptHashObj ({})", key_hash.as_script_hash()),
            KeyHashObj(script_hash) => write!(f, "KeyHashObj ({})", script_hash.as_key_hash()),
        }
    }
}

impl<K, V> HaskellDisplay for HashMap<K, V>
where
    K: HaskellDisplay + Eq + std::hash::Hash,
    V: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        let result = self
            .iter()
            .map(|item| format!("({},{})", item.0.to_haskell_str(), item.1.to_haskell_str()))
            .collect::<Vec<_>>()
            .join(", ");

        format!("fromList [{}]", result)
    }
}

impl HaskellDisplay for EpochNo {
    fn to_haskell_str(&self) -> String {
        format!("EpochNo {}", &self.0)
    }
}

impl HaskellDisplay for i8 {
    fn to_haskell_str(&self) -> String {
        if *self >= 0 {
            format!("{}", self)
        } else {
            format!("({})", self)
        }
    }
}
impl HaskellDisplay for i64 {
    fn to_haskell_str(&self) -> String {
        if *self >= 0 {
            format!("{}", self)
        } else {
            format!("({})", self)
        }
    }
}
impl HaskellDisplay for u8 {
    fn to_haskell_str(&self) -> String {
        format!("{}", self)
    }
}

impl<T> HaskellDisplay for Vec<T>
where
    T: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            let mut result = first.to_haskell_str();
            result.push_str(" :| [");

            if iter.len() > 0 {
                for item in iter {
                    result.push_str(&format!("{} ,", item.to_haskell_str()));
                }
                result.truncate(result.len() - 2);
            }
            result.push(']');

            result
        } else {
            "fromList []".to_string()
        }
    }
}

impl HaskellDisplay for Credential {
    fn to_haskell_str(&self) -> String {
        use Credential::*;

        match self {
            ScriptHashObj(key_hash) => key_hash.as_script_hash_obj(),
            KeyHashObj(key_hash) => key_hash.as_key_hash_obj(),
        }
    }
}
impl<T, H> HaskellDisplay for (T, H)
where
    T: HaskellDisplay,
    H: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        format!("({},{})", self.0.to_haskell_str(), self.1.to_haskell_str())
    }
}

impl HaskellDisplay for Voter {
    fn to_haskell_str(&self) -> String {
        use Voter::*;

        match self {
            ConstitutionalCommitteeKey(addr) => {
                format!("CommitteeVoter ({})", addr.as_key_hash_obj())
            }
            ConstitutionalCommitteeScript(scrpt) => {
                format!("CommitteeVoter ({})", scrpt.as_script_hash_obj())
            }
            DRepKey(addr) => {
                format!("DRepVoter ({})", addr.as_key_hash_obj())
            }
            DRepScript(scrpt) => {
                format!("DRepVoter ({})", scrpt.as_script_hash_obj())
            }
            StakePoolKey(addr) => {
                format!("StakePoolVoter ({})", addr.as_key_hash())
            }
        }
    }
}

impl<T> HaskellDisplay for CustomSet258<T>
where
    T: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        self.0.as_from_list()
    }
}

impl HaskellDisplay for DisplayScriptHash {
    fn to_haskell_str(&self) -> String {
        self.0.as_script_hash()
    }
}

impl HaskellDisplay for VKey {
    fn to_haskell_str(&self) -> String {
        self.0.as_vkey()
    }
}
trait AsTransactionId {
    fn as_tx_id(&self) -> String;
}

trait AsTransactionIx {
    fn as_tx_ix(&self) -> String;
}

trait AsSafeHash {
    fn as_safe_hash(&self) -> String;
}

trait AsKeyHash {
    fn as_key_hash(&self) -> String;
}

trait AsVKey {
    fn as_vkey(&self) -> String;
}

trait AsScriptHashObject {
    fn as_script_hash_obj(&self) -> String;
}

trait AsFromList {
    fn as_from_list(&self) -> String;
}

trait AsKeyHashObject {
    fn as_key_hash_obj(&self) -> String;
}

trait AsScriptHash {
    fn as_script_hash(&self) -> String;
}

trait AsUrl {
    fn as_url(&self) -> String;
}

trait AsProtocolVersion {
    fn as_protocol_version(&self) -> String;
}

impl AsUrl for String {
    fn as_url(&self) -> String {
        format!("Url {{urlToText = \"{}\"}}", self)
    }
}
impl AsTransactionId for [u8] {
    fn as_tx_id(&self) -> String {
        format!("TxId {{unTxId = {}}}", self.as_safe_hash())
    }
}

impl AsTransactionIx for u64 {
    fn as_tx_ix(&self) -> String {
        format!("TxIx {{unTxIx = {}}}", self)
    }
}

impl AsSafeHash for [u8] {
    fn as_safe_hash(&self) -> String {
        let hex = hex::encode(self);
        format!("SafeHash \"{}\"", hex)
    }
}
impl AsSafeHash for Hash<28> {
    fn as_safe_hash(&self) -> String {
        let hex = hex::encode(self.as_ref());
        format!("SafeHash \"{}\"", hex)
    }
}

impl<T> AsSafeHash for Nullable<T>
where
    T: AsSafeHash + std::clone::Clone,
{
    fn as_safe_hash(&self) -> String {
        match self {
            Nullable::Some(v) => v.as_safe_hash(),
            _ => "SNothing".to_string(),
        }
    }
}

impl AsKeyHash for [u8] {
    fn as_key_hash(&self) -> String {
        let hex = hex::encode(self);
        format!("KeyHash {{unKeyHash = \"{}\"}}", hex)
    }
}

impl AsVKey for [u8] {
    fn as_vkey(&self) -> String {
        let hex = hex::encode(self);
        format!("VKey (VerKeyEd25519DSIGN \"{}\")", hex)
    }
}

impl AsScriptHashObject for [u8] {
    fn as_script_hash_obj(&self) -> String {
        format!("ScriptHashObj ({})", self.as_script_hash())
    }
}

impl AsKeyHashObject for [u8] {
    fn as_key_hash_obj(&self) -> String {
        format!("KeyHashObj ({})", self.as_key_hash())
    }
}

impl AsScriptHash for [u8] {
    fn as_script_hash(&self) -> String {
        let hex = hex::encode(self);
        format!("ScriptHash \"{}\"", hex)
    }
}

impl AsProtocolVersion for ProtocolVersion {
    fn as_protocol_version(&self) -> String {
        format!(
            "ProtVer {{pvMajor = Version {}, pvMinor = {}}}",
            self.0, self.1
        )
    }
}

trait AsConwayCertificate {
    fn as_conway_certificate(&self) -> String;
}
trait AsRewardAccountFielded {
    fn as_reward_account_fielded(&self) -> String;
}

impl AsRewardAccountFielded for RewardAccount {
    fn as_reward_account_fielded(&self) -> String {
        let hex = hex::encode(self.as_ref() as &[u8]);
        RewardAccountFielded::new(hex).to_haskell_str()
    }
}

impl<T: HaskellDisplay> AsFromList for Vec<T> {
    fn as_from_list(&self) -> String {
        // let mut result =String::new();

        let result = self
            .iter()
            .map(|item| item.to_haskell_str())
            .collect::<Vec<_>>()
            .join(",");

        format!("fromList [{}]", result)
    }
}

trait AsDisplayCoin {
    fn as_display_coin(&self) -> String;
}

impl AsDisplayCoin for u64 {
    fn as_display_coin(&self) -> String {
        format!("Coin {}", self)
    }
}

trait AsEpochInterval {
    fn as_epoch_interval(&self) -> String;
}

impl AsEpochInterval for Option<u64> {
    fn as_epoch_interval(&self) -> String {
        match self {
            Option::Some(v) => format!("SJust (EpochInterval {})", v.to_haskell_str()),
            _ => "SNothing".to_string(),
        }
    }
}

impl AsDisplayCoin for Option<u64> {
    fn as_display_coin(&self) -> String {
        match self {
            Option::Some(v) => format!("SJust (Coin {})", v.to_haskell_str()),
            _ => "SNothing".to_string(),
        }
    }
}

// This trait is used to present values coming from a Map, but we don't use Map
trait AsInMap {
    fn as_in_map(&self) -> String;
}

impl<T> AsInMap for StrictMaybe<T>
where
    T: HaskellDisplay + 'static,
{
    fn as_in_map(&self) -> String {
        match self {
            StrictMaybe::Just(v) => format!("SJust {}", v.to_haskell_str()),
            StrictMaybe::Nothing => "SNothing".to_string(),
        }
    }
}

impl HaskellDisplay for AsIx {
    fn to_haskell_str(&self) -> String {
        format!("AsIx {{unAsIx = {}}}", self.0)
    }
}

impl HaskellDisplay for u64 {
    fn to_haskell_str(&self) -> String {
        self.to_string()
    }
}

impl HaskellDisplay for String {
    fn to_haskell_str(&self) -> String {
        self.as_text()
    }
}

trait AsStrictSeq {
    fn as_strict_seq(&self) -> String;
}

impl<T> AsStrictSeq for Vec<T>
where
    T: HaskellDisplay,
{
    fn as_strict_seq(&self) -> String {
        format!("StrictSeq {{fromStrict = {}}}", self.as_from_list())
    }
}

trait AsText {
    fn as_text(&self) -> String;
}

impl AsText for String {
    fn as_text(&self) -> String {
        haskell_show_string(self)
    }
}

impl AsText for Bytes {
    fn as_text(&self) -> String {
        let s = std::str::from_utf8(self.as_ref()).unwrap_or("<invalid UTF-8>");
        haskell_show_string(s)
    }
}
impl HaskellDisplay for Bytes {
    fn to_haskell_str(&self) -> String {
        format!("\"{}\"", self)
    }
}
impl HaskellDisplay for MultiAsset {
    fn to_haskell_str(&self) -> String {
        format!("MultiAsset ({})", self.0.to_haskell_str())
    }
}

impl<K, V> HaskellDisplay for KeyValuePairs<K, V>
where
    K: Clone + HaskellDisplay,
    V: Clone + HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        let result = self
            .iter()
            .map(|(k, v)| format!("({},{})", k.to_haskell_str(), v.to_haskell_str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("fromList [{}]", result)
    }
}

impl HaskellDisplay for DisplayPolicyId {
    fn to_haskell_str(&self) -> String {
        format!("PolicyID {{policyID = {}}}", self.0.as_script_hash())
    }
}

impl HaskellDisplay for DisplayAssetName {
    fn to_haskell_str(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl HaskellDisplay for DisplayCoin {
    fn to_haskell_str(&self) -> String {
        self.0.as_display_coin()
    }
}

impl HaskellDisplay for SerializableTxOut {
    fn to_haskell_str(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl HaskellDisplay for BabbageTxOut {
    fn to_haskell_str(&self) -> String {
        match self {
            BabbageTxOut::TxOutCompactRefScript(
                address,
                (value, multiasset),
                datum_hash,
                era_script,
            ) => {
                format!(
                    "({},{} ({}),{},{})",
                    address.to_haskell_str(),
                    value.to_haskell_str(),
                    multiasset.to_haskell_str(),
                    datum_hash.to_haskell_str(),
                    era_script.as_in_map()
                )
            }
            _ => "HaskellDisplay not implemented yet".to_string(),
        }
    }
}


impl HaskellDisplay for ByronAddress {
    fn to_haskell_str(&self) -> String {
        let payload = self.decode().unwrap();
        format!("BootstrapAddress {}", payload.to_haskell_str_p())
    }
}

impl HaskellDisplay for AddressPayload {
    fn to_haskell_str(&self) -> String {
        let root = hex::encode(self.root);

        use AddrType::*;
        let addr_type = match self.addrtype  {
            PubKey =>  "ATVerKey",
            Script=> "Script todo",
            Redeem=> "Redeem todo ",
            Other(v) =>"Other todo"
        };
        format!(
            "Address {{addrRoot = {}, addrAttributes = {}, addrType = {}}}",
            root,
            self.attributes.to_haskell_str(),
            addr_type
        )
    }
}

impl HaskellDisplay for OrderPreservingProperties<AddrAttrProperty> {
    fn to_haskell_str(&self) -> String {
        let items = self.deref();

        let mut att_map: HashMap<&str, String> = HashMap::with_capacity(2);

        for item in items {
            use AddrAttrProperty::*;

            match item {
                DerivationPath(bv) => {
                    att_map.insert("aaVKDerivationPath", format!("{:?}", bv));
                }
                NetworkTag(bv) => {
                    att_map.insert("aaNetworkMagic", format!("{:?}", bv));
                }
                _ => {}
            }
        }

        format!("Attributes {{ data_ = AddrAttributes {{aaVKDerivationPath = {}, aaNetworkMagic = {}}} }}", 
        att_map.get("aaVKDerivationPath").unwrap_or(&"Nothing".to_string()),
        att_map.get("aaVKDerivationPath").unwrap_or(&"NetworkMainOrStage".to_string())

    )
    }
}

/*
impl HaskellDisplay for AddrAttrProperty
{
    fn to_haskell_str(&self) -> String {

        use AddrAttrProperty::*;

       let str =  match self {
            AddrDistr(dist) => "AddrDistr not implemented yet".to_string(),
            DerivationPath(bv ) => {
                let bytes = bv.deref();
                let dp = if(bytes.len() == 0) {
                     "Nothing"
                } else {
                    "aaVKDerivationPath not implemented yet"
                };

                format!("aaVKDerivationPath {}", dp)
            } ,
            NetworkTag(bv) => "NetworkTag not implemented yet".to_string(),
        };

       format!("AddrAttrProperty {{unAddrAttrProperty = {}}}", str)
    }

}

*/

impl HaskellDisplay for Utxo {
    fn to_haskell_str(&self) -> String {
        format!(
            "UTxO {}", self.0.to_haskell_str_p()
        )
    }
}
impl fmt::Display for DisplayValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.0)
    }
}

impl HaskellDisplay for MaryValue {
    fn to_haskell_str(&self) -> String {
        format!("MaryValue ({})", self.0)
    }
}

impl HaskellDisplay for ShelleyAddress {
    fn to_haskell_str(&self) -> String {
        format!(
            "Addr {} ({}) ({})",
            self.network().to_haskell_str(),
            self.payment().to_haskell_str(),
            self.delegation().to_haskell_str()
        )
    }
}

impl HaskellDisplay for pallas_addresses::Network {
    fn to_haskell_str(&self) -> String {
        match self {
            pallas_addresses::Network::Mainnet => "Mainnet".to_string(),
            pallas_addresses::Network::Testnet => "Testnet".to_string(),
            pallas_addresses::Network::Other(magic) => format!("Other {}", magic),
        }
    }
}

impl HaskellDisplay for ShelleyPaymentPart {
    fn to_haskell_str(&self) -> String {
        match self {
            ShelleyPaymentPart::Key(hash) => hash.as_key_hash_obj(),
            ShelleyPaymentPart::Script(hash) => hash.as_script_hash_obj(),
        }
    }
}

impl HaskellDisplay for ShelleyDelegationPart {
    fn to_haskell_str(&self) -> String {
        let str = match self {
            ShelleyDelegationPart::Key(hash) => hash.as_key_hash_obj(),
            ShelleyDelegationPart::Script(hash) => hash.as_script_hash_obj(),
            ShelleyDelegationPart::Pointer(pointer) => format!("DelegationPointer ({:?})", pointer),
            ShelleyDelegationPart::Null => "".to_string(),
        };
        format!("StakeRefBase ({})", str)
    }
}

impl HaskellDisplay for Address {
    fn to_haskell_str(&self) -> String {
        let str = match self {
            Address::Byron(addr) => format!("AddrBootstrap {}", addr.to_haskell_str_p()),
            Address::Shelley(addr) => addr.to_haskell_str(),
            Address::Stake(addr) => addr.to_hex(),
        };

        format!("{}", str)
    }
}

impl HaskellDisplay for AddressBytes {
    fn to_haskell_str(&self) -> String {
        let (network, credential) = get_network_and_credentials(&self.0);

        format!(
            "Addr {} ({})",
            network.to_haskell_str(),
            credential.to_haskell_str()
        )
    }
}
impl HaskellDisplay for DatumEnum {
    fn to_haskell_str(&self) -> String {
        use DatumEnum::*;

        match self {
            DatumHash(datum_hash) => datum_hash.to_haskell_str().to_string(),
            Datum(datum) => format!("Datum ({:?})", datum),
            NoDatum => "NoDatum".to_string(),
        }
    }
}

impl HaskellDisplay for DisplayDatumHash {
    fn to_haskell_str(&self) -> String {
        format!("DatumHash ({})", self.0.as_safe_hash())
    }
}

impl HaskellDisplay for DisplayDatum {
    fn to_haskell_str(&self) -> String {
        format!("Datum ({:?})", self.0)
    }
}

impl HaskellDisplay for EraScript {
    fn to_haskell_str(&self) -> String {
        use EraScript::*;

        match self {
            Native(timelock) => format!("TimelockScript {}", timelock.to_haskell_str()),
            PlutusV1(hash) => format!("PlutusScript PlutusV1 {}", hash.to_haskell_str()),
            PlutusV2(hash) => format!("PlutusScript PlutusV2 {}", hash.to_haskell_str()),
            PlutusV3(hash) => format!("PlutusScript PlutusV3 {}", hash.to_haskell_str()),
        }
    }
}

impl HaskellDisplay for TimelockRaw {
    fn to_haskell_str(&self) -> String {
        use TimelockRaw::*;

        match self {
            Signature(key_hash) => format!("Signature ({})", key_hash.to_haskell_str()),
            AllOf(vec) => format!("AllOf ({})", vec.to_haskell_str()),
            AnyOf(vec) => format!("AnyOf ({})", vec.to_haskell_str()),
            MOfN(m, vec) => format!("MOfN {} ({})", m, vec.as_strict_seq()),
            TimeStart(slot_no) => format!("TimeStart ({})", slot_no.to_haskell_str()),
            TimeExpire(slot_no) => format!("TimeExpire ({})", slot_no.to_haskell_str()),
        }
    }
}

impl HaskellDisplay for Timelock {
    fn to_haskell_str(&self) -> String {
        let raw = self.raw.to_haskell_str();
        let memo = self.memo.to_haskell_str();
        format!("TimelockConstr {} {}", raw, memo)
    }
}
impl HaskellDisplay for SlotNo {
    fn to_haskell_str(&self) -> String {
        format!("SlotNo {}", self.0)
    }
}
impl HaskellDisplay for DisplayHash {
    fn to_haskell_str(&self) -> String {
        format!("(blake2b_256: SafeHash \"{}\")", hex::encode(self.0))
    }
}

impl HaskellDisplay for PlutusPurpose {
    fn to_haskell_str(&self) -> String {
        use PlutusPurpose::*;

        match self {
            Minting(policy_id) => format!("ConwayMinting ({})", policy_id.to_haskell_str()),
            Spending(txin) => format!("ConwaySpending ({})", txin.to_haskell_str()),
            Rewarding(item) => {
                format!("ConwayRewarding ({})", item.to_haskell_str())
            }
            Certifying(cert_index) => format!("ConwayCertifying ({})", cert_index.to_haskell_str()),
            Voting(gov_id) => format!("ConwayVoting ({})", gov_id.to_haskell_str()),
            Proposing(proposal_id) => format!("ConwayProposing ({})", proposal_id.to_haskell_str()),
        }
    }
}
impl HaskellDisplay for ConwayPlutusPurpose {
    fn to_haskell_str(&self) -> String {
        use ConwayPlutusPurpose::*;

        match self {
            ConwayMinting(policy_id) => format!("ConwayMinting ({})", policy_id.to_haskell_str()),
            ConwaySpending(txin) => format!("ConwaySpending ({})", txin.to_haskell_str()),
            ConwayRewarding(item) => format!("ConwayRewarding ({})", item.to_haskell_str()),
            ConwayCertifying(cert) => {
                format!("ConwayCertifying ({})", cert.to_haskell_str())
            }
            ConwayVoting(gov_id) => format!("ConwayVoting ({})", gov_id.to_haskell_str()),
            ConwayProposing(proposal_id) => {
                format!("ConwayProposing ({})", proposal_id.to_haskell_str())
            }
        }
    }
}

impl HaskellDisplay for ConwayTxCert {
    fn to_haskell_str(&self) -> String {
        
        use ConwayTxCert::*;
        match self {
            ConwayTxCertDeleg(conway_deleg_cert) => format!("ConwayTxCertDeleg {}", conway_deleg_cert.to_haskell_str_p()),
            ConwayTxCertPool(pool_cert) => todo!(),
            ConwayTxCertGov(conway_gov_cert) => todo!(),
        }
    }
}

impl HaskellDisplay for ConwayDelegCert {
    fn to_haskell_str(&self) -> String {

        match self {
            ConwayDelegCert::ConwayRegCert(stake_credential, display_coin) => 
            format!("ConwayRegCert {} {}", stake_credential.to_haskell_str_p(), display_coin.to_haskell_str()),
            ConwayDelegCert::ConwayUnRegCert(stake_credential, display_coin) => 
            format!("ConwayUnRegCert {} {}", stake_credential.to_haskell_str_p(), display_coin.to_haskell_str()),
            
            ConwayDelegCert::ConwayDelegCert(stake_credential, delegatee) => 
            format!("ConwayDelegCert {} {}", stake_credential.to_haskell_str_p(), delegatee.to_haskell_str()),
                        ConwayDelegCert::ConwayRegDelegCert(stake_credential, delegatee, display_coin) =>
            format!("ConwayRegDelegCert {} {}", stake_credential.to_haskell_str_p(), display_coin.to_haskell_str()),
            
        }
    }
}
impl HaskellDisplay for Delegatee {
    fn to_haskell_str(&self) -> String {
        use Delegatee::*;

         "Delegatee not implemented".to_string()
    }
}

impl HaskellDisplay for Certificate {
    fn to_haskell_str(&self) -> String {
        

        use Certificate::*;
        match self {
            /* 
            StakeDeregistration(stake_credential) => todo!(),
            StakeDelegation(stake_credential, hash) => todo!(),
            PoolRegistration { operator, vrf_keyhash, pledge, cost, margin, reward_account, pool_owners, relays, pool_metadata } => todo!(),
            PoolRetirement(hash, _) => todo!(),
             UnReg(stake_credential, _) => todo!(),
            VoteDeleg(stake_credential, drep) => todo!(),
            StakeVoteDeleg(stake_credential, hash, drep) => todo!(),
            StakeRegDeleg(stake_credential, hash, _) => todo!(),
            VoteRegDeleg(stake_credential, drep, _) => todo!(),
            StakeVoteRegDeleg(stake_credential, hash, drep, _) => todo!(),
            AuthCommitteeHot(stake_credential, stake_credential1) => todo!(),
            ResignCommitteeCold(stake_credential, nullable) => todo!(),
            RegDRepCert(stake_credential, _, nullable) => todo!(),
            UnRegDRepCert(stake_credential, _) => todo!(),
            UpdateDRepCert(stake_credential, nullable) => todo!(),
            */
            StakeRegistration(stake_credential) =>  format!("ConwayRegCert {} SNothing", stake_credential.to_haskell_str()),
            Reg(stake_credential, coin) => format!("ConwayRegCert {} {}", stake_credential.to_haskell_str(), coin.to_haskell_str()),
           
            _ => format!("Certificate not implemented: {:?}", self),

        }
    }
}

impl HaskellDisplay for PurposeAs {
    fn to_haskell_str(&self) -> String {
        use PurposeAs::*;

        match self {
            Ix(i) => i.to_haskell_str(),
            Item(i) => i.to_haskell_str(),
        }
    }
}

impl<T> HaskellDisplay for AsItem<T>
where
    T: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        // Implement the display logic for AsItem here
        format!("AsItem {{unAsItem = {}}}", self.0.to_haskell_str())
    }
}

impl HaskellDisplay for DisplayAddress {
    fn to_haskell_str(&self) -> String {
        format!("{}",self.0.to_haskell_str())
    }
}

impl<T> HaskellDisplay for StrictMaybe<T>
where
    T: HaskellDisplay + 'static,
{
    fn to_haskell_str(&self) -> String {
        match self {
            StrictMaybe::Just(v) => {
                if is_primitive::<T>() {
                    format!("SJust {}", v.to_haskell_str())
                } else {
                    format!("SJust ({})", v.to_haskell_str())
                }
            }
            StrictMaybe::Nothing => "SNothing".to_string(),
        }
    }

    fn to_haskell_str_p(&self) -> String {
        let str = self.to_haskell_str();
        if &str == "SNothing" {
            return str;
        }
        format!("({})", self.to_haskell_str())
    }
}

impl HaskellDisplay for VKeyWitness {
    fn to_haskell_str(&self) -> String {
        format!(
            "VKeyWitness {{ vkey: {}, signature: {} }}",
            self.vkey, self.signature
        )
    }
}

impl<T> HaskellDisplay for Array<T>
where
    T: HaskellDisplay,
{
    fn to_haskell_str(&self) -> String {
        let value = self
            .0
            .iter()
            .map(|item| item.to_haskell_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", value)
    }
}

impl HaskellDisplay for DeltaCoin {
    fn to_haskell_str(&self) -> String {
        format!("DeltaCoin {}", self.0.to_haskell_str())
    }
}

impl HaskellDisplay for i32 {
    fn to_haskell_str(&self) -> String {
        if *self >= 0 {
            format!("{}", self)
        } else {
            format!("({})", self)
        }
    }
}
impl HaskellDisplay for DatumHash {
    fn to_haskell_str(&self) -> String {
        format!("DatumHash \"{}\"", hex::encode(self.as_ref()))
    }
}

fn display_governance_action_id_index(index: &u32) -> String {
    format!("GovActionIx {{unGovActionIx = {}}}", index)
}

trait AsAuxDataHash {
    fn as_aux_data_hash(&self) -> String;
}

impl AsAuxDataHash for Bytes {
    fn as_aux_data_hash(&self) -> String {
        format!(
            "AuxiliaryDataHash {{unsafeAuxiliaryDataHash = SafeHash \"{}\"}}",
            self
        )
    }
}

trait AsIsValid {
    fn as_is_valid(&self) -> &str;
}

impl AsIsValid for bool {
    fn as_is_valid(&self) -> &str {
        if *self {
            "IsValid True" 
        } else {
            "IsValid False" 
        }
    }
}