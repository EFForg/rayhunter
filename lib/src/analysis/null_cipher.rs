use std::borrow::Cow;

use telcom_parser::lte_rrc::{
    CipheringAlgorithm_r12, DL_DCCH_MessageType, DL_DCCH_MessageType_c1,
    RRCConnectionReconfiguration, RRCConnectionReconfigurationCriticalExtensions,
    RRCConnectionReconfigurationCriticalExtensions_c1, SCG_Configuration_r12,
    SecurityConfigHO_v1530HandoverType_v1530, SecurityModeCommand,
    SecurityModeCommandCriticalExtensions, SecurityModeCommandCriticalExtensions_c1,
};

use super::analyzer::{Analyzer, Event, EventType};
use super::information_element::{InformationElement, LteInformationElement};

pub struct NullCipherAnalyzer {}

impl NullCipherAnalyzer {
    fn check_rrc_connection_reconfiguration_cipher(
        &self,
        reconfiguration: &RRCConnectionReconfiguration,
    ) -> bool {
        let RRCConnectionReconfigurationCriticalExtensions::C1(c1) =
            &reconfiguration.critical_extensions
        else {
            return false;
        };
        let RRCConnectionReconfigurationCriticalExtensions_c1::RrcConnectionReconfiguration_r8(c1) =
            c1
        else {
            return false;
        };
        if let Some(handover) = &c1.security_config_ho {
            let maybe_security_config = match &handover.handover_type {
                telcom_parser::lte_rrc::SecurityConfigHOHandoverType::IntraLTE(lte) => {
                    lte.security_algorithm_config.as_ref()
                }
                telcom_parser::lte_rrc::SecurityConfigHOHandoverType::InterRAT(rat) => {
                    Some(&rat.security_algorithm_config)
                }
            };
            if let Some(security_config) = maybe_security_config
                && security_config.ciphering_algorithm.0 == CipheringAlgorithm_r12::EEA0
            {
                return true;
            }
        }
        // Use map/flatten to dig into a long chain of nested Option types
        let maybe_v1250 = c1
            .non_critical_extension
            .as_ref()
            .and_then(|v890| v890.non_critical_extension.as_ref())
            .and_then(|v920| v920.non_critical_extension.as_ref())
            .and_then(|v1020| v1020.non_critical_extension.as_ref())
            .and_then(|v1130| v1130.non_critical_extension.as_ref());
        let Some(v1250) = maybe_v1250 else {
            return false;
        };

        if let Some(SCG_Configuration_r12::Setup(scg_setup)) = v1250.scg_configuration_r12.as_ref()
        {
            let maybe_cipher = scg_setup
                .scg_config_part_scg_r12
                .as_ref()
                .and_then(|scg| scg.mobility_control_info_scg_r12.as_ref())
                .and_then(|mci| mci.ciphering_algorithm_scg_r12.as_ref());
            if let Some(cipher) = maybe_cipher
                && cipher.0 == CipheringAlgorithm_r12::EEA0
            {
                return true;
            }
        }

        let maybe_v1530_security_config = v1250
            .non_critical_extension
            .as_ref()
            .and_then(|v1310| v1310.non_critical_extension.as_ref())
            .and_then(|v1430| v1430.non_critical_extension.as_ref())
            .and_then(|v1510| v1510.non_critical_extension.as_ref())
            .and_then(|v1530| v1530.security_config_ho_v1530.as_ref());
        let Some(v1530_security_config) = maybe_v1530_security_config else {
            return false;
        };
        let maybe_security_algorithm = match &v1530_security_config.handover_type_v1530 {
            SecurityConfigHO_v1530HandoverType_v1530::Intra5GC(intra_5gc) => {
                intra_5gc.security_algorithm_config_r15.as_ref()
            }
            SecurityConfigHO_v1530HandoverType_v1530::Fivegc_ToEPC(to_epc) => {
                Some(&to_epc.security_algorithm_config_r15)
            }
            SecurityConfigHO_v1530HandoverType_v1530::Epc_To5GC(to_5gc) => {
                Some(&to_5gc.security_algorithm_config_r15)
            }
        };
        if let Some(security_algorithm) = maybe_security_algorithm
            && security_algorithm.ciphering_algorithm.0 == CipheringAlgorithm_r12::EEA0
        {
            return true;
        }
        false
    }

    fn check_security_mode_command_cipher(&self, command: &SecurityModeCommand) -> bool {
        let SecurityModeCommandCriticalExtensions::C1(c1) = &command.critical_extensions else {
            return false;
        };
        let SecurityModeCommandCriticalExtensions_c1::SecurityModeCommand_r8(r8) = &c1 else {
            return false;
        };
        if r8
            .security_config_smc
            .security_algorithm_config
            .ciphering_algorithm
            .0
            == CipheringAlgorithm_r12::EEA0
        {
            return true;
        }
        false
    }
}

impl Analyzer for NullCipherAnalyzer {
    fn get_name(&self) -> Cow<'_, str> {
        Cow::from("Null Cipher")
    }

    fn get_description(&self) -> Cow<'_, str> {
        Cow::from("Tests whether the cell suggests using a null cipher (EEA0)")
    }

    fn get_version(&self) -> u32 {
        1
    }

    fn analyze_information_element(
        &mut self,
        ie: &InformationElement,
        _packet_num: usize,
    ) -> Option<Event> {
        let dcch_msg = match ie {
            InformationElement::LTE(lte_ie) => match &**lte_ie {
                LteInformationElement::DlDcch(dcch_msg) => dcch_msg,
                _ => return None,
            },
            _ => return None,
        };
        let DL_DCCH_MessageType::C1(c1) = &dcch_msg.message else {
            return None;
        };
        let null_cipher_detected = match c1 {
            DL_DCCH_MessageType_c1::RrcConnectionReconfiguration(reconfiguration) => {
                self.check_rrc_connection_reconfiguration_cipher(reconfiguration)
            }
            DL_DCCH_MessageType_c1::SecurityModeCommand(command) => {
                self.check_security_mode_command_cipher(command)
            }
            _ => return None,
        };
        if null_cipher_detected {
            return Some(Event {
                event_type: EventType::High,
                message: "Cell suggested use of null cipher".to_string(),
            });
        }
        None
    }
}
