pub const MAX_ADD_MEAS_AO_D_R17: i64 = 23;

pub const MAX_ADD_MEAS_RTT_R17: i64 = 31;

pub const MAX_ADD_MEAS_TDOA_R17: i64 = 31;

pub const MAX_ADD_PR_SCONFIG_R14: i64 = 2;

pub const MAX_AVAIL_NARROW_BANDS_MINUS1_R14: i64 = 15;

pub const MAX_BT_BEACON_R13: i64 = 32;

pub const MAX_BT_BEACON_AD_R18: i64 = 64;

pub const MAX_BT_BEACON_ANT_ELT_R18: i64 = 74;

pub const MAX_BAND_COMB_R16: i64 = 1024;

pub const MAX_BANDS: i64 = 64;

pub const MAX_CARRIER_R14: i64 = 5;

pub const MAX_CELL_I_DS_R19: i64 = 256;

pub const MAX_CELL_I_DS_PER_AREA_R17: i64 = 256;

pub const MAX_CELLS_R14: i64 = 72;

pub const MAX_EARFCN: i64 = 65535;

pub const MAX_EARFCN_PLUS1: i64 = 65536;

pub const MAX_EARFCN2: i64 = 262143;

pub const MAX_EPDU: i64 = 16;

pub const MAX_FBI: i64 = 64;

pub const MAX_FBI_PLUS1: i64 = 65;

pub const MAX_FBI2: i64 = 256;

pub const MAX_FREQ_LAYERS: i64 = 3;

pub const MAX_KNOWN_A_PS_R14: i64 = 2048;

pub const MAX_MBS_R14: i64 = 64;

pub const MAX_MEAS_INSTANCES_R17: i64 = 32;

pub const MAX_NR_OF_AREAS_R17: i64 = 16;

pub const MAX_NUM_OF_RX_TE_GS_1_R17: i64 = 31;

pub const MAX_NUM_OF_RX_TE_GS_R17: i64 = 32;

pub const MAX_NUM_OF_RX_TX_TE_GS_1_R17: i64 = 255;

pub const MAX_NUM_OF_SRS_POS_RESOURCES_1_R17: i64 = 63;

pub const MAX_NUM_OF_SRS_POS_RESOURCES_R17: i64 = 64;

pub const MAX_NUM_OF_TRP_TX_TE_GS_1_R17: i64 = 7;

pub const MAX_NUM_OF_TX_TE_GS_1_R17: i64 = 7;

pub const MAX_NUM_PRIO_RESOURCES_R17: i64 = 24;

pub const MAX_NUM_RESOURCES_PER_ANGLE_R17: i64 = 24;

pub const MAX_OD_DL_PRS_CONFIGS_R17: i64 = 8;

pub const MAX_PATHS_R14: i64 = 2;

pub const MAX_SIMULTANEOUS_BANDS_R16: i64 = 4;

pub const MAX_TX_TEG_SETS_R17: i64 = 256;

pub const MAX_VISIBLE_A_PS_R14: i64 = 32;

pub const MAX_WLAN_AP_R13: i64 = 64;

pub const MAX_WLAN_AP_R14: i64 = 128;

pub const MAX_WLAN_DATA_SETS_R14: i64 = 8;

pub const NR_MAX_BANDS_R16: i64 = 1024;

pub const NR_MAX_CONFIGURED_BANDS_R16: i64 = 16;

pub const NR_MAX_FREQ_LAYERS_1_R16: i64 = 3;

pub const NR_MAX_FREQ_LAYERS_R16: i64 = 4;

pub const NR_MAX_NUM_DL_PRS_RESOURCE_SETS_PER_TRP_1_R16: i64 = 7;

pub const NR_MAX_NUM_DL_PRS_RESOURCES_PER_SET_1_R16: i64 = 63;

pub const NR_MAX_NUM_PRS_BAND_WIDTH_AGGREGATION_R18: i64 = 256;

pub const NR_MAX_RESOURCE_I_DS_R16: i64 = 64;

pub const NR_MAX_RESOURCE_OFFSET_VALUE_1_R16: i64 = 511;

pub const NR_MAX_RESOURCES_PER_SET_R16: i64 = 64;

pub const NR_MAX_SETS_PER_TRP_PER_FREQ_LAYER_1_R16: i64 = 1;

pub const NR_MAX_SETS_PER_TRP_PER_FREQ_LAYER_R16: i64 = 2;

pub const NR_MAX_TR_PS_R16: i64 = 256;

pub const NR_MAX_TR_PS_PER_FREQ_1_R16: i64 = 63;

pub const NR_MAX_TR_PS_PER_FREQ_R16: i64 = 64;

pub const NR_NUM_OF_SAMPLES_1_R18: i64 = 3;

pub const NR_NUM_OF_SAMPLES_R18: i64 = 4;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum A_GNSS_Error {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses(GNSS_LocationServerErrorCauses),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses(GNSS_TargetDeviceErrorCauses),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct A_GNSS_ProvideAssistanceData {
    #[asn(optional_idx = 0)]
    pub gnss_common_assist_data: Option<GNSS_CommonAssistData>,
    #[asn(optional_idx = 1)]
    pub gnss_generic_assist_data: Option<GNSS_GenericAssistData>,
    #[asn(optional_idx = 2)]
    pub gnss_error: Option<A_GNSS_Error>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct A_GNSS_ProvideCapabilities {
    #[asn(optional_idx = 0)]
    pub gnss_support_list: Option<GNSS_SupportList>,
    #[asn(optional_idx = 1)]
    pub assistance_data_support_list: Option<AssistanceDataSupportList>,
    #[asn(optional_idx = 2)]
    pub location_coordinate_types: Option<LocationCoordinateTypes>,
    #[asn(optional_idx = 3)]
    pub velocity_types: Option<VelocityTypes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct A_GNSS_ProvideLocationInformation {
    #[asn(optional_idx = 0)]
    pub gnss_signal_measurement_information: Option<GNSS_SignalMeasurementInformation>,
    #[asn(optional_idx = 1)]
    pub gnss_location_information: Option<GNSS_LocationInformation>,
    #[asn(optional_idx = 2)]
    pub gnss_error: Option<A_GNSS_Error>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct A_GNSS_RequestAssistanceData {
    #[asn(optional_idx = 0)]
    pub gnss_common_assist_data_req: Option<GNSS_CommonAssistDataReq>,
    #[asn(optional_idx = 1)]
    pub gnss_generic_assist_data_req: Option<GNSS_GenericAssistDataReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct A_GNSS_RequestCapabilities {
    pub gnss_support_list_req: A_GNSS_RequestCapabilitiesGnss_SupportListReq,
    pub assistance_data_support_list_req: A_GNSS_RequestCapabilitiesAssistanceDataSupportListReq,
    pub location_velocity_types_req: A_GNSS_RequestCapabilitiesLocationVelocityTypesReq,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct A_GNSS_RequestLocationInformation {
    pub gnss_positioning_instructions: GNSS_PositioningInstructions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct ARFCN_ValueEUTRA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "262143")]
pub struct ARFCN_ValueEUTRA_r14(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "65536", ub = "262143")]
pub struct ARFCN_ValueEUTRA_v9a0(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3279165")]
pub struct ARFCN_ValueNR_r15(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct ARFCN_ValueUTRA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AUX_ReferenceStationID_Element_r15 {
    pub aux_station_id_r15: GNSS_ReferenceStationID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AUX_ReferenceStationList_r15(pub Vec<AUX_ReferenceStationID_Element_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Abort {
    pub critical_extensions: AbortCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Abort_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_abort: Option<CommonIEsAbort>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AccessTypes {
    pub access_types: AccessTypesAccessTypes,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct Acknowledgement {
    pub ack_requested: AcknowledgementAckRequested,
    #[asn(optional_idx = 0)]
    pub ack_indicator: Option<SequenceNumber>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Add_PRSconfigNeighbourElement_r14 {
    #[asn(optional_idx = 0)]
    pub add_prs_info_r14: Option<PRS_Info>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct AdditionalInformation(pub u8);
impl AdditionalInformation {
    pub const ONLY_RETURN_INFORMATION_REQUESTED: u8 = 0u8;
    pub const MAY_RETURN_ADDITIONAL_INFORMATION: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AdditionalPath_r14 {
    pub relative_time_difference_r14: AdditionalPath_r14RelativeTimeDifference_r14,
    #[asn(optional_idx = 0)]
    pub path_quality_r14: Option<OTDOA_MeasQuality>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct AdditionalPathList_r14(pub Vec<AdditionalPath_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct AlmanacBDS_AlmanacSet_r12 {
    pub sv_id: SV_ID,
    #[asn(optional_idx = 0)]
    pub bds_alm_toa_r12: Option<AlmanacBDS_AlmanacSet_r12BdsAlmToa_r12>,
    pub bds_alm_sqrt_a_r12: AlmanacBDS_AlmanacSet_r12BdsAlmSqrtA_r12,
    pub bds_alm_e_r12: AlmanacBDS_AlmanacSet_r12BdsAlmE_r12,
    pub bds_alm_w_r12: AlmanacBDS_AlmanacSet_r12BdsAlmW_r12,
    pub bds_alm_m0_r12: AlmanacBDS_AlmanacSet_r12BdsAlmM0_r12,
    pub bds_alm_omega0_r12: AlmanacBDS_AlmanacSet_r12BdsAlmOmega0_r12,
    pub bds_alm_omega_dot_r12: AlmanacBDS_AlmanacSet_r12BdsAlmOmegaDot_r12,
    pub bds_alm_delta_i_r12: AlmanacBDS_AlmanacSet_r12BdsAlmDeltaI_r12,
    pub bds_alm_a0_r12: AlmanacBDS_AlmanacSet_r12BdsAlmA0_r12,
    pub bds_alm_a1_r12: AlmanacBDS_AlmanacSet_r12BdsAlmA1_r12,
    #[asn(optional_idx = 1)]
    pub bds_sv_health_r12: Option<AlmanacBDS_AlmanacSet_r12BdsSvHealth_r12>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AlmanacECEF_SBAS_AlmanacSet {
    pub sbas_alm_data_id: AlmanacECEF_SBAS_AlmanacSetSbasAlmDataID,
    pub sv_id: SV_ID,
    pub sbas_alm_health: AlmanacECEF_SBAS_AlmanacSetSbasAlmHealth,
    pub sbas_alm_xg: AlmanacECEF_SBAS_AlmanacSetSbasAlmXg,
    pub sbas_alm_yg: AlmanacECEF_SBAS_AlmanacSetSbasAlmYg,
    pub sbas_alm_zg: AlmanacECEF_SBAS_AlmanacSetSbasAlmZg,
    pub sbas_alm_xgdot: AlmanacECEF_SBAS_AlmanacSetSbasAlmXgdot,
    pub sbas_alm_yg_dot: AlmanacECEF_SBAS_AlmanacSetSbasAlmYgDot,
    pub sbas_alm_zg_dot: AlmanacECEF_SBAS_AlmanacSetSbasAlmZgDot,
    pub sbas_alm_to: AlmanacECEF_SBAS_AlmanacSetSbasAlmTo,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AlmanacGLONASS_AlmanacSet {
    pub glo_alm_na: AlmanacGLONASS_AlmanacSetGloAlm_NA,
    pub glo_almn_a: AlmanacGLONASS_AlmanacSetGloAlmnA,
    pub glo_alm_ha: AlmanacGLONASS_AlmanacSetGloAlmHA,
    pub glo_alm_lambda_a: AlmanacGLONASS_AlmanacSetGloAlmLambdaA,
    pub glo_almtlambda_a: AlmanacGLONASS_AlmanacSetGloAlmtlambdaA,
    pub glo_alm_delta_ia: AlmanacGLONASS_AlmanacSetGloAlmDeltaIa,
    pub glo_alm_delta_ta: AlmanacGLONASS_AlmanacSetGloAlmDeltaTA,
    pub glo_alm_delta_tdot_a: AlmanacGLONASS_AlmanacSetGloAlmDeltaTdotA,
    pub glo_alm_epsilon_a: AlmanacGLONASS_AlmanacSetGloAlmEpsilonA,
    pub glo_alm_omega_a: AlmanacGLONASS_AlmanacSetGloAlmOmegaA,
    pub glo_alm_tau_a: AlmanacGLONASS_AlmanacSetGloAlmTauA,
    pub glo_alm_ca: AlmanacGLONASS_AlmanacSetGloAlmCA,
    #[asn(optional_idx = 0)]
    pub glo_alm_ma: Option<AlmanacGLONASS_AlmanacSetGloAlmMA>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AlmanacKeplerianSet {
    pub sv_id: SV_ID,
    pub kep_almanac_e: AlmanacKeplerianSetKepAlmanacE,
    pub kep_almanac_delta_i: AlmanacKeplerianSetKepAlmanacDeltaI,
    pub kep_almanac_omega_dot: AlmanacKeplerianSetKepAlmanacOmegaDot,
    pub kep_sv_status_inav: AlmanacKeplerianSetKepSV_StatusINAV,
    #[asn(optional_idx = 0)]
    pub kep_sv_status_fnav: Option<AlmanacKeplerianSetKepSV_StatusFNAV>,
    pub kep_almanac_a_power_half: AlmanacKeplerianSetKepAlmanacAPowerHalf,
    pub kep_almanac_omega0: AlmanacKeplerianSetKepAlmanacOmega0,
    pub kep_almanac_w: AlmanacKeplerianSetKepAlmanacW,
    pub kep_almanac_m0: AlmanacKeplerianSetKepAlmanacM0,
    pub kep_almanac_af0: AlmanacKeplerianSetKepAlmanacAF0,
    pub kep_almanac_af1: AlmanacKeplerianSetKepAlmanacAF1,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AlmanacMidiAlmanacSet {
    pub sv_id: SV_ID,
    pub midi_alm_e: AlmanacMidiAlmanacSetMidiAlmE,
    pub midi_alm_delta_i: AlmanacMidiAlmanacSetMidiAlmDeltaI,
    pub midi_alm_omega_dot: AlmanacMidiAlmanacSetMidiAlmOmegaDot,
    pub midi_alm_sqrt_a: AlmanacMidiAlmanacSetMidiAlmSqrtA,
    pub midi_alm_omega0: AlmanacMidiAlmanacSetMidiAlmOmega0,
    pub midi_alm_omega: AlmanacMidiAlmanacSetMidiAlmOmega,
    pub midi_alm_mo: AlmanacMidiAlmanacSetMidiAlmMo,
    pub midi_almaf0: AlmanacMidiAlmanacSetMidiAlmaf0,
    pub midi_almaf1: AlmanacMidiAlmanacSetMidiAlmaf1,
    pub midi_alm_l1_health: AlmanacMidiAlmanacSetMidiAlmL1Health,
    pub midi_alm_l2_health: AlmanacMidiAlmanacSetMidiAlmL2Health,
    pub midi_alm_l5_health: AlmanacMidiAlmanacSetMidiAlmL5Health,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AlmanacNAV_KeplerianSet {
    pub sv_id: SV_ID,
    pub nav_alm_e: AlmanacNAV_KeplerianSetNavAlmE,
    pub nav_alm_delta_i: AlmanacNAV_KeplerianSetNavAlmDeltaI,
    pub nav_alm_omegadot: AlmanacNAV_KeplerianSetNavAlmOMEGADOT,
    pub nav_alm_sv_health: AlmanacNAV_KeplerianSetNavAlmSVHealth,
    pub nav_alm_sqrt_a: AlmanacNAV_KeplerianSetNavAlmSqrtA,
    pub nav_alm_omeg_ao: AlmanacNAV_KeplerianSetNavAlmOMEGAo,
    pub nav_alm_omega: AlmanacNAV_KeplerianSetNavAlmOmega,
    pub nav_alm_mo: AlmanacNAV_KeplerianSetNavAlmMo,
    pub nav_almaf0: AlmanacNAV_KeplerianSetNavAlmaf0,
    pub nav_almaf1: AlmanacNAV_KeplerianSetNavAlmaf1,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AlmanacNavIC_AlmanacSet_r16 {
    pub sv_id_r16: SV_ID,
    #[asn(optional_idx = 0)]
    pub navic_alm_toa_r16: Option<AlmanacNavIC_AlmanacSet_r16Navic_AlmToa_r16>,
    pub navic_alm_e_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmE_r16,
    pub navic_alm_omegadot_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmOMEGADOT_r16,
    pub navic_alm_sqrt_a_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmSqrtA_r16,
    pub navic_alm_omeg_ao_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmOMEGAo_r16,
    pub navic_alm_omega_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmOmega_r16,
    pub navic_alm_mo_r16: AlmanacNavIC_AlmanacSet_r16Navic_AlmMo_r16,
    pub navic_almaf0_r16: AlmanacNavIC_AlmanacSet_r16Navic_Almaf0_r16,
    pub navic_almaf1_r16: AlmanacNavIC_AlmanacSet_r16Navic_Almaf1_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AlmanacNavIC_AlmanacSet2_r19 {
    pub sv_id_r19: SV_ID,
    #[asn(optional_idx = 0)]
    pub navic_l1_alm_toa_r19: Option<AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmToa_r19>,
    pub navic_l1_alm_e_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmE_r19,
    pub navic_l1_i0_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_i0_r19,
    pub navic_l1_alm_omegadot_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOMEGADOT_r19,
    pub navic_l1_alm_sqrt_a_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmSqrtA_r19,
    pub navic_l1_alm_omeg_ao_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOMEGAo_r19,
    pub navic_l1_alm_omega_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOmega_r19,
    pub navic_l1_alm_mo_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmMo_r19,
    pub navic_l1_almaf0_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_Almaf0_r19,
    pub navic_l1_almaf1_r19: AlmanacNavIC_AlmanacSet2_r19NavicL1_Almaf1_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AlmanacReducedKeplerianSet {
    pub sv_id: SV_ID,
    pub red_alm_delta_a: AlmanacReducedKeplerianSetRedAlmDeltaA,
    pub red_alm_omega0: AlmanacReducedKeplerianSetRedAlmOmega0,
    pub red_alm_phi0: AlmanacReducedKeplerianSetRedAlmPhi0,
    pub red_alm_l1_health: AlmanacReducedKeplerianSetRedAlmL1Health,
    pub red_alm_l2_health: AlmanacReducedKeplerianSetRedAlmL2Health,
    pub red_alm_l5_health: AlmanacReducedKeplerianSetRedAlmL5Health,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AntennaDescription_r15 {
    pub antenna_descriptor_r15: AntennaDescription_r15AntennaDescriptor_r15,
    #[asn(optional_idx = 0)]
    pub antenna_set_up_id_r15: Option<AntennaDescription_r15AntennaSetUpID_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AntennaReferencePointUnc_r15 {
    pub uncertainty_x_r15: AntennaReferencePointUnc_r15Uncertainty_X_r15,
    pub confidence_x_r15: AntennaReferencePointUnc_r15Confidence_X_r15,
    pub uncertainty_y_r15: AntennaReferencePointUnc_r15Uncertainty_Y_r15,
    pub confidence_y_r15: AntennaReferencePointUnc_r15Confidence_Y_r15,
    pub uncertainty_z_r15: AntennaReferencePointUnc_r15Uncertainty_Z_r15,
    pub confidence_z_r15: AntennaReferencePointUnc_r15Confidence_Z_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct AreaID_CellList_r17(pub Vec<NR_Cell_IDs_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ArrayOfGridPoints_r18 {
    pub reference_point_latitude_r18: ArrayOfGridPoints_r18ReferencePointLatitude_r18,
    pub reference_point_longitude_r18: ArrayOfGridPoints_r18ReferencePointLongitude_r18,
    pub number_of_steps_south_r18: ArrayOfGridPoints_r18NumberOfStepsSouth_r18,
    pub number_of_steps_east_r18: ArrayOfGridPoints_r18NumberOfStepsEast_r18,
    pub step_south_r18: SpatialDelta_r18,
    pub step_east_r18: SpatialDelta_r18,
    #[asn(optional_idx = 0)]
    pub bitmask_of_grids_r18: Option<ArrayOfGridPoints_r18BitmaskOfGrids_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct AssistanceDataSupportList {
    pub gnss_common_assistance_data_support: GNSS_CommonAssistanceDataSupport,
    pub gnss_generic_assistance_data_support: GNSS_GenericAssistanceDataSupport,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Aux_ARP_Unc_r15 {
    pub horizontal_uncertainty_r15: Aux_ARP_Unc_r15HorizontalUncertainty_r15,
    pub horizontal_confidence_r15: Aux_ARP_Unc_r15HorizontalConfidence_r15,
    #[asn(optional_idx = 0)]
    pub vertical_uncertainty_r15: Option<Aux_ARP_Unc_r15VerticalUncertainty_r15>,
    #[asn(optional_idx = 1)]
    pub vertical_confidence_r15: Option<Aux_ARP_Unc_r15VerticalConfidence_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct AuxiliaryStationElement_r15 {
    pub aux_reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub aux_master_delta_latitude_r15: AuxiliaryStationElement_r15Aux_master_delta_latitude_r15,
    pub aux_master_delta_longitude_r15: AuxiliaryStationElement_r15Aux_master_delta_longitude_r15,
    pub aux_master_delta_height_r15: AuxiliaryStationElement_r15Aux_master_delta_height_r15,
    #[asn(optional_idx = 0)]
    pub aux_arp_unc_r15: Option<Aux_ARP_Unc_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct AuxiliaryStationList_r15(pub Vec<AuxiliaryStationElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_ClockModel_r12 {
    pub bds_aodc_r12: BDS_ClockModel_r12BdsAODC_r12,
    pub bds_toc_r12: BDS_ClockModel_r12BdsToc_r12,
    pub bds_a0_r12: BDS_ClockModel_r12BdsA0_r12,
    pub bds_a1_r12: BDS_ClockModel_r12BdsA1_r12,
    pub bds_a2_r12: BDS_ClockModel_r12BdsA2_r12,
    pub bds_tgd1_r12: BDS_ClockModel_r12BdsTgd1_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_ClockModel2_r16 {
    pub bds_toc_r16: BDS_ClockModel2_r16BdsToc_r16,
    pub bds_a0_r16: BDS_ClockModel2_r16BdsA0_r16,
    pub bds_a1_r16: BDS_ClockModel2_r16BdsA1_r16,
    pub bds_a2_r16: BDS_ClockModel2_r16BdsA2_r16,
    pub bds_tgd_b1_cp_r16: BDS_ClockModel2_r16BdsTgdB1Cp_r16,
    pub bds_isc_b1_cd_r16: BDS_ClockModel2_r16BdsIscB1Cd_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_DifferentialCorrections_r12 {
    pub dbds_ref_time_r12: BDS_DifferentialCorrections_r12Dbds_RefTime_r12,
    pub bds_sgn_type_list_r12: BDS_SgnTypeList_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_DifferentialCorrectionsReq_r12 {
    pub dgnss_signals_req: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_DifferentialCorrectionsSupport_r12 {
    pub gnss_signal_i_ds: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_GridModelParameter_r12 {
    pub bds_ref_time_r12: BDS_GridModelParameter_r12Bds_RefTime_r12,
    pub grid_ion_list_r12: GridIonList_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_GridModelReq_r12 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BDS_GridModelSupport_r12 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BDS_SgnTypeElement_r12 {
    #[asn(optional_idx = 0)]
    pub gnss_signal_id: Option<GNSS_SignalID>,
    pub dbds_correction_list_r12: DBDS_CorrectionList_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct BDS_SgnTypeList_r12(pub Vec<BDS_SgnTypeElement_r12>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum BT_AntArrayConfig_r18 {
    #[asn(key = 0, extended = false)]
    Bt_UniformLinearArray_r18(BT_UniformLinearArray_r18),
    #[asn(key = 1, extended = false)]
    Bt_UniformRectangularArray_r18(BT_UniformRectangularArray_r18),
    #[asn(key = 2, extended = false)]
    Bt_UniformCircularArray_r18(BT_UniformCircularArray_r18),
    #[asn(key = 3, extended = false)]
    Bt_GenericArray_r18(BT_GenericArray_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_AntElement_r18 {
    pub polarization_r18: BT_AntElement_r18Polarization_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BT_AntSwitchElement_r18 {
    pub ant_element_index_short_r18: BT_AntSwitchElement_r18AntElementIndexShort_r18,
    #[asn(optional_idx = 0)]
    pub ant_element_index_offset_r18: Option<BT_AntSwitchElement_r18AntElementIndexOffset_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct BT_AoA_Config_r18 {
    pub bt_addr_r18: BT_AoA_Config_r18Bt_Addr_r18,
    #[asn(optional_idx = 0)]
    pub cte_status_r18: Option<BT_AoA_Config_r18Cte_Status_r18>,
    #[asn(optional_idx = 1)]
    pub primary_adv_interval_r18: Option<BT_AoA_Config_r18PrimaryAdvInterval_r18>,
    #[asn(optional_idx = 2)]
    pub second_adv_interval_r18: Option<BT_AoA_Config_r18SecondAdvInterval_r18>,
    #[asn(optional_idx = 3)]
    pub tx_power_r18: Option<BT_AoA_Config_r18Tx_Power_r18>,
    #[asn(optional_idx = 4)]
    pub cte_length_r18: Option<BT_AoA_Config_r18Cte_Length_r18>,
    #[asn(optional_idx = 5)]
    pub cte_count_r18: Option<BT_AoA_Config_r18Cte_Count_r18>,
    #[asn(optional_idx = 6)]
    pub tx_phy_m2_r18: Option<BT_AoA_Config_r18Tx_PHY_M2_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct BT_AoD_TransmConfig_r18 {
    pub primary_adv_interval_r18: BT_AoD_TransmConfig_r18PrimaryAdvInterval_r18,
    pub second_adv_interval_r18: BT_AoD_TransmConfig_r18SecondAdvInterval_r18,
    pub cte_length_r18: BT_AoD_TransmConfig_r18Cte_Length_r18,
    pub cte_count_r18: BT_AoD_TransmConfig_r18Cte_Count_r18,
    #[asn(optional_idx = 0)]
    pub cte_type2us_r18: Option<BT_AoD_TransmConfig_r18Cte_Type2us_r18>,
    #[asn(optional_idx = 1)]
    pub tx_phy_m2_r18: Option<BT_AoD_TransmConfig_r18Tx_PHY_M2_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_BeaconInfo_r18 {
    pub reference_point_r18: ReferencePoint_r16,
    pub bt_beacon_info_list_r18: BT_BeaconInfo_r18Bt_BeaconInfoList_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct BT_BeaconInfoElement_r18 {
    pub bt_addr_r18: BT_BeaconInfoElement_r18Bt_Addr_r18,
    #[asn(optional_idx = 0)]
    pub bt_beacon_location_r18: Option<RelativeLocation_r16>,
    #[asn(optional_idx = 1)]
    pub bt_lcs_gcs_translation_parameter_r18: Option<LCS_GCS_TranslationParameter_r16>,
    #[asn(optional_idx = 2)]
    pub bt_ant_array_config_r18: Option<BT_AntArrayConfig_r18>,
    #[asn(optional_idx = 3)]
    pub bt_ant_element_list_r18: Option<BT_BeaconInfoElement_r18Bt_antElementList_r18>,
    #[asn(optional_idx = 4)]
    pub bt_ant_switching_pattern_r18: Option<BT_BeaconInfoElement_r18Bt_antSwitchingPattern_r18>,
    #[asn(optional_idx = 5)]
    pub bt_ao_d_transm_config_r18: Option<BT_AoD_TransmConfig_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum BT_Error_r13 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r13(BT_LocationServerErrorCauses_r13),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r13(BT_TargetDeviceErrorCauses_r13),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "74")]
pub struct BT_GenericArray_r18(pub Vec<BT_ULA_GenericAntElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_LocationServerErrorCauses_r13 {
    pub cause_r13: BT_LocationServerErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BT_MeasurementElement_r13 {
    pub bt_addr_r13: BT_MeasurementElement_r13BtAddr_r13,
    #[asn(optional_idx = 0)]
    pub rssi_r13: Option<BT_MeasurementElement_r13Rssi_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct BT_MeasurementElement_r18 {
    pub bt_addr_r18: BT_MeasurementElement_r18BtAddr_r18,
    pub bt_azimuth_r18: BT_MeasurementElement_r18Bt_azimuth_r18,
    #[asn(optional_idx = 0)]
    pub bt_elevation_r18: Option<BT_MeasurementElement_r18Bt_elevation_r18>,
    #[asn(optional_idx = 1)]
    pub rssi_r18: Option<BT_MeasurementElement_r18Rssi_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct BT_MeasurementInformation_r13 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r13:
        Option<BT_MeasurementInformation_r13MeasurementReferenceTime_r13>,
    #[asn(optional_idx = 1)]
    pub bt_measurement_list_r13: Option<BT_MeasurementList_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct BT_MeasurementList_r13(pub Vec<BT_MeasurementElement_r13>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct BT_MeasurementList_r18(pub Vec<BT_MeasurementElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BT_ProvideAssistanceData_r18 {
    pub bt_beacon_info_r18: BT_BeaconInfo_r18,
    #[asn(optional_idx = 0)]
    pub bt_error_r18: Option<BT_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_ProvideCapabilities_r13 {
    pub bt_modes_r13: BT_ProvideCapabilities_r13Bt_Modes_r13,
    pub bt_meas_supported_r13: BT_ProvideCapabilities_r13Bt_MeasSupported_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct BT_ProvideLocationInformation_r13 {
    #[asn(optional_idx = 0)]
    pub bt_measurement_information_r13: Option<BT_MeasurementInformation_r13>,
    #[asn(optional_idx = 1)]
    pub bt_error_r13: Option<BT_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_RequestAssistanceData_r18 {
    pub requested_ad_r18: BT_RequestAssistanceData_r18RequestedAD_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_RequestCapabilities_r13 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct BT_RequestLocationInformation_r13 {
    pub requested_measurements_r13: BT_RequestLocationInformation_r13RequestedMeasurements_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct BT_SuggestedAoA_Config_r18 {
    #[asn(optional_idx = 0)]
    pub cte_status_r18: Option<BT_SuggestedAoA_Config_r18Cte_Status_r18>,
    #[asn(optional_idx = 1)]
    pub primary_adv_interval_r18: Option<BT_SuggestedAoA_Config_r18PrimaryAdvInterval_r18>,
    #[asn(optional_idx = 2)]
    pub second_adv_interval_r18: Option<BT_SuggestedAoA_Config_r18SecondAdvInterval_r18>,
    #[asn(optional_idx = 3)]
    pub tx_power_r18: Option<BT_SuggestedAoA_Config_r18Tx_Power_r18>,
    #[asn(optional_idx = 4)]
    pub cte_length_r18: Option<BT_SuggestedAoA_Config_r18Cte_Length_r18>,
    #[asn(optional_idx = 5)]
    pub cte_count_r18: Option<BT_SuggestedAoA_Config_r18Cte_Count_r18>,
    #[asn(optional_idx = 6)]
    pub tx_phy_m2_r18: Option<BT_SuggestedAoA_Config_r18Tx_PHY_M2_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BT_TargetDeviceErrorCauses_r13 {
    pub cause_r13: BT_TargetDeviceErrorCauses_r13Cause_r13,
    #[asn(optional_idx = 0)]
    pub bt_beacon_rssi_measurement_not_possible_r13:
        Option<BT_TargetDeviceErrorCauses_r13Bt_Beacon_rssiMeasurementNotPossible_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct BT_ULA_GenericAntElement_r18 {
    #[asn(optional_idx = 0)]
    pub delta_y_r18: Option<BT_ULA_GenericAntElement_r18DeltaY_r18>,
    #[asn(optional_idx = 1)]
    pub delta_x_r18: Option<BT_ULA_GenericAntElement_r18DeltaX_r18>,
    #[asn(optional_idx = 2)]
    pub delta_z_r18: Option<BT_ULA_GenericAntElement_r18DeltaZ_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BT_UniformCircularArray_r18 {
    pub bt_no_elements_r18: BT_UniformCircularArray_r18Bt_NoElements_r18,
    pub bt_inter_element_dist_r18: BT_UniformCircularArray_r18Bt_InterElementDist_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BT_UniformLinearArray_r18 {
    pub bt_no_elements_r18: BT_UniformLinearArray_r18Bt_NoElements_r18,
    pub bt_inter_element_dist_r18: BT_UniformLinearArray_r18Bt_InterElementDist_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct BT_UniformRectangularArray_r18 {
    pub bt_no_elements_y_r18: BT_UniformRectangularArray_r18Bt_NoElementsY_r18,
    pub bt_no_elements_z_r18: BT_UniformRectangularArray_r18Bt_NoElementsZ_r18,
    pub bt_inter_element_dist_y_r18: BT_UniformRectangularArray_r18Bt_InterElementDistY_r18,
    pub bt_inter_element_dist_z_r18: BT_UniformRectangularArray_r18Bt_InterElementDistZ_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct BadSignalElement {
    pub bad_svid: SV_ID,
    #[asn(optional_idx = 0)]
    pub bad_signal_id: Option<GNSS_SignalIDs>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct BeamPowerElement_r17 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_set_id_r17: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_dl_prs_resource_id_r17: NR_DL_PRS_ResourceID_r16,
    pub nr_dl_prs_relative_power_r17: BeamPowerElement_r17Nr_dl_prs_RelativePower_r17,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_relative_power_fine_r17:
        Option<BeamPowerElement_r17Nr_dl_prs_RelativePowerFine_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CLOCK_IntegrityParameters_r17 {
    pub clock_range_error_correlation_time_r17:
        CLOCK_IntegrityParameters_r17ClockRangeErrorCorrelationTime_r17,
    pub clock_range_rate_error_correlation_time_r17:
        CLOCK_IntegrityParameters_r17ClockRangeRateErrorCorrelationTime_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct CNAV_ClockModel {
    pub cnav_toc: CNAV_ClockModelCnavToc,
    pub cnav_top: CNAV_ClockModelCnavTop,
    pub cnav_ura0: CNAV_ClockModelCnavURA0,
    pub cnav_ura1: CNAV_ClockModelCnavURA1,
    pub cnav_ura2: CNAV_ClockModelCnavURA2,
    pub cnav_af2: CNAV_ClockModelCnavAf2,
    pub cnav_af1: CNAV_ClockModelCnavAf1,
    pub cnav_af0: CNAV_ClockModelCnavAf0,
    pub cnav_tgd: CNAV_ClockModelCnavTgd,
    #[asn(optional_idx = 0)]
    pub cnav_is_cl1cp: Option<CNAV_ClockModelCnavISCl1cp>,
    #[asn(optional_idx = 1)]
    pub cnav_is_cl1cd: Option<CNAV_ClockModelCnavISCl1cd>,
    #[asn(optional_idx = 2)]
    pub cnav_is_cl1ca: Option<CNAV_ClockModelCnavISCl1ca>,
    #[asn(optional_idx = 3)]
    pub cnav_is_cl2c: Option<CNAV_ClockModelCnavISCl2c>,
    #[asn(optional_idx = 4)]
    pub cnav_is_cl5i5: Option<CNAV_ClockModelCnavISCl5i5>,
    #[asn(optional_idx = 5)]
    pub cnav_is_cl5q5: Option<CNAV_ClockModelCnavISCl5q5>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CarrierFreq_NB_r14 {
    pub carrier_freq_r14: ARFCN_ValueEUTRA_r14,
    #[asn(optional_idx = 0)]
    pub carrier_freq_offset_r14: Option<CarrierFreqOffsetNB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "20")]
pub struct CarrierFreqOffsetNB_r14(pub u8);
impl CarrierFreqOffsetNB_r14 {
    pub const V_10: u8 = 0u8;
    pub const V_9: u8 = 1u8;
    pub const V_8: u8 = 2u8;
    pub const V_7: u8 = 3u8;
    pub const V_6: u8 = 4u8;
    pub const V_5: u8 = 5u8;
    pub const V_4: u8 = 6u8;
    pub const V_3: u8 = 7u8;
    pub const V_2: u8 = 8u8;
    pub const V_1: u8 = 9u8;
    pub const V_0DOT5: u8 = 10u8;
    pub const V0: u8 = 11u8;
    pub const V1: u8 = 12u8;
    pub const V2: u8 = 13u8;
    pub const V3: u8 = 14u8;
    pub const V4: u8 = 15u8;
    pub const V5: u8 = 16u8;
    pub const V6: u8 = 17u8;
    pub const V7: u8 = 18u8;
    pub const V8: u8 = 19u8;
    pub const V9: u8 = 20u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellGlobalIdEUTRA_AndUTRA {
    pub plmn_identity: CellGlobalIdEUTRA_AndUTRAPlmn_Identity,
    pub cell_identity: CellGlobalIdEUTRA_AndUTRACellIdentity,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CellGlobalIdGERAN {
    pub plmn_identity: CellGlobalIdGERANPlmn_Identity,
    pub location_area_code: CellGlobalIdGERANLocationAreaCode,
    pub cell_identity: CellGlobalIdGERANCellIdentity,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIEsAbort {
    pub abort_cause: CommonIEsAbortAbortCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CommonIEsError {
    pub error_cause: CommonIEsErrorErrorCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CommonIEsProvideAssistanceData {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CommonIEsProvideCapabilities {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct CommonIEsProvideLocationInformation {
    #[asn(optional_idx = 0)]
    pub location_estimate: Option<LocationCoordinates>,
    #[asn(optional_idx = 1)]
    pub velocity_estimate: Option<Velocity>,
    #[asn(optional_idx = 2)]
    pub location_error: Option<LocationError>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct CommonIEsRequestAssistanceData {
    #[asn(optional_idx = 0)]
    pub primary_cell_id: Option<ECGI>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct CommonIEsRequestCapabilities {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct CommonIEsRequestLocationInformation {
    pub location_information_type: LocationInformationType,
    #[asn(optional_idx = 0)]
    pub triggered_reporting: Option<TriggeredReportingCriteria>,
    #[asn(optional_idx = 1)]
    pub periodical_reporting: Option<PeriodicalReportingCriteria>,
    #[asn(optional_idx = 2)]
    pub additional_information: Option<AdditionalInformation>,
    #[asn(optional_idx = 3)]
    pub qos: Option<QoS>,
    #[asn(optional_idx = 4)]
    pub environment: Option<Environment>,
    #[asn(optional_idx = 5)]
    pub location_coordinate_types: Option<LocationCoordinateTypes>,
    #[asn(optional_idx = 6)]
    pub velocity_types: Option<VelocityTypes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DBDS_CorrectionElement_r12 {
    pub sv_id: SV_ID,
    pub bds_udrei_r12: DBDS_CorrectionElement_r12Bds_UDREI_r12,
    pub bds_rurai_r12: DBDS_CorrectionElement_r12Bds_RURAI_r12,
    pub bds_ecc_delta_t_r12: DBDS_CorrectionElement_r12Bds_ECC_DeltaT_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DBDS_CorrectionList_r12(pub Vec<DBDS_CorrectionElement_r12>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DGNSS_CorrectionsElement {
    pub sv_id: SV_ID,
    pub iod: DGNSS_CorrectionsElementIod,
    pub udre: DGNSS_CorrectionsElementUdre,
    pub pseudo_range_cor: DGNSS_CorrectionsElementPseudoRangeCor,
    pub range_rate_cor: DGNSS_CorrectionsElementRangeRateCor,
    #[asn(optional_idx = 0)]
    pub udre_growth_rate: Option<DGNSS_CorrectionsElementUdreGrowthRate>,
    #[asn(optional_idx = 1)]
    pub udre_validity_time: Option<DGNSS_CorrectionsElementUdreValidityTime>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DGNSS_SatList(pub Vec<DGNSS_CorrectionsElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DGNSS_SgnTypeElement {
    pub gnss_signal_id: GNSS_SignalID,
    pub gnss_status_health: DGNSS_SgnTypeElementGnss_StatusHealth,
    pub dgnss_sat_list: DGNSS_SatList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct DGNSS_SgnTypeList(pub Vec<DGNSS_SgnTypeElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DL_AoD_MeasCapabilityPerBand_r16 {
    pub freq_band_indicator_nr_r16: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub simul_nr_dl_ao_d_dl_tdoa_r16:
        Option<DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_DL_TDOA_r16>,
    #[asn(optional_idx = 1)]
    pub simul_nr_dl_ao_d_multi_rtt_r16:
        Option<DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_Multi_RTT_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DL_PRS_BeamInfoElement_r16 {
    pub dl_prs_azimuth_r16: DL_PRS_BeamInfoElement_r16Dl_PRS_Azimuth_r16,
    #[asn(optional_idx = 0)]
    pub dl_prs_azimuth_fine_r16: Option<DL_PRS_BeamInfoElement_r16Dl_PRS_Azimuth_fine_r16>,
    #[asn(optional_idx = 1)]
    pub dl_prs_elevation_r16: Option<DL_PRS_BeamInfoElement_r16Dl_PRS_Elevation_r16>,
    #[asn(optional_idx = 2)]
    pub dl_prs_elevation_fine_r16: Option<DL_PRS_BeamInfoElement_r16Dl_PRS_Elevation_fine_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_PRS_BeamInfoResourceSet_r16(pub Vec<DL_PRS_BeamInfoElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DL_PRS_BeamInfoSet_r16(pub Vec<DL_PRS_BeamInfoResourceSet_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_Configuration_ID_r17 {
    pub nr_dl_prs_configuration_id_r17: DL_PRS_Configuration_ID_r17Nr_dl_prs_configuration_id_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct DL_PRS_ID_Info_r16 {
    pub dl_prs_id_r16: DL_PRS_ID_Info_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_id_list_r16: Option<DL_PRS_ID_Info_r16Nr_DL_PRS_ResourceID_List_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18 {
    #[asn(optional_idx = 0)]
    pub maximum_prs_bandwidth_across_all_hops_fr1_r18: Option<
        DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR1_r18,
    >,
    #[asn(optional_idx = 1)]
    pub maximum_prs_bandwidth_across_all_hops_fr2_r18: Option<
        DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR2_r18,
    >,
    #[asn(optional_idx = 2)]
    pub maximum_fh_hops_r18: Option<DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumFH_Hops_r18>,
    #[asn(optional_idx = 3)]
    pub processing_duration_r18:
        Option<DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18>,
    #[asn(optional_idx = 4)]
    pub rf_rx_retune_time_fr1_r18:
        Option<DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR1_r18>,
    #[asn(optional_idx = 5)]
    pub rf_rx_retune_time_fr2_r18:
        Option<DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR2_r18>,
    #[asn(optional_idx = 6)]
    pub num_of_overlapping_prb_r18:
        Option<DL_PRS_MeasurementWithRxFH_RRC_Connected_r18NumOfOverlappingPRB_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_PRS_MutingOption1_r16 {
    #[asn(optional_idx = 0)]
    pub dl_prs_muting_bit_repetition_factor_r16:
        Option<DL_PRS_MutingOption1_r16Dl_prs_MutingBitRepetitionFactor_r16>,
    pub nr_option1_muting_r16: NR_MutingPattern_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_MutingOption2_r16 {
    pub nr_option2_muting_r16: NR_MutingPattern_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum DL_PRS_QCL_Info_r16 {
    #[asn(key = 0, extended = false)]
    Ssb_r16(DL_PRS_QCL_Info_r16_ssb_r16),
    #[asn(key = 1, extended = false)]
    Dl_PRS_r16(DL_PRS_QCL_Info_r16_dl_PRS_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_QCL_InfoReq_r17 {
    pub nr_dl_prs_resource_set_id_r17: NR_DL_PRS_ResourceSetID_r16,
    pub dl_prs_qcl_information_req_r17: DL_PRS_QCL_InfoReq_r17Dl_prs_QCL_InformationReq_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DL_PRS_QCL_InformationReqPerTRP_r17 {
    pub dl_prs_id_r17: DL_PRS_QCL_InformationReqPerTRP_r17Dl_PRS_ID_r17,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r17: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r17: Option<ARFCN_ValueNR_r15>,
    pub dl_prs_qcl_information_req_set_r17:
        DL_PRS_QCL_InformationReqPerTRP_r17Dl_prs_QCL_InformationReqSet_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_PRS_QCL_InformationReqTRPlist_r17(pub Vec<DL_PRS_QCL_InformationReqPerTRP_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DL_PRS_QCL_ProcessingCapabilityPerBand_r16 {
    pub freq_band_indicator_nr_r16: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub ssb_from_neigh_cell_as_qcl_r16:
        Option<DL_PRS_QCL_ProcessingCapabilityPerBand_r16Ssb_FromNeighCellAsQCL_r16>,
    #[asn(optional_idx = 1)]
    pub prs_from_serv_neigh_cell_as_qcl_r16:
        Option<DL_PRS_QCL_ProcessingCapabilityPerBand_r16Prs_FromServNeighCellAsQCL_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DL_PRS_Resource_ARP_Element_r16 {
    #[asn(optional_idx = 0)]
    pub dl_prs_resource_arp_location_r16: Option<RelativeLocation_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "24")]
pub struct DL_PRS_ResourcePrioritySubset_r17(pub Vec<NR_DL_PRSResourcePriorityItem_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DL_PRS_ResourceSets_TRP_Element_r16 {
    #[asn(optional_idx = 0)]
    pub dl_prs_resource_set_arp_r16: Option<RelativeLocation_r16>,
    #[asn(optional_idx = 1)]
    pub dl_prs_resource_arp_list_r16:
        Option<DL_PRS_ResourceSets_TRP_Element_r16Dl_PRS_Resource_ARP_List_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_ResourcesBandCombination_r16 {
    pub band_list_r16: DL_PRS_ResourcesBandCombination_r16BandList_r16,
    pub max_nr_of_dl_prs_resources_across_all_fl_trp_resource_set_r16:
        DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct DL_PRS_ResourcesBandCombinationList_r16(pub Vec<DL_PRS_ResourcesBandCombination_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_ResourcesCapabilityPerBand_r16 { pub freq_band_indicator_nr_r16 : FreqBandIndicatorNR_r16 , pub max_nr_of_dl_prs_resources_per_resource_set_r16 : DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerResourceSet_r16 , pub max_nr_of_dl_prs_resources_per_positioning_frequencylayer_r16 : DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerPositioningFrequencylayer_r16 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DL_PRS_StartTime_and_Duration_r17 {
    #[asn(optional_idx = 0)]
    pub dl_prs_start_time_r17: Option<DL_PRS_StartTime_and_Duration_r17Dl_prs_start_time_r17>,
    #[asn(optional_idx = 1)]
    pub dl_prs_duration_r17: Option<DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_TEG_InfoElement_r17 {
    pub dl_prs_trp_tx_teg_id_r17: DL_PRS_TEG_InfoElement_r17Dl_prs_trp_Tx_TEG_ID_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_PRS_TEG_InfoPerResourceSet_r17(pub Vec<DL_PRS_TEG_InfoElement_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_SelectedPRS_ResourceIndex_r16 {
    pub nr_dl_selected_prs_resource_id_index_r16:
        DL_SelectedPRS_ResourceIndex_r16Nr_DL_SelectedPRS_ResourceIdIndex_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct DL_SelectedPRS_ResourceSetIndex_r16 {
    pub nr_dl_selected_prs_resource_set_index_r16:
        DL_SelectedPRS_ResourceSetIndex_r16Nr_DL_SelectedPRS_ResourceSetIndex_r16,
    #[asn(optional_idx = 0)]
    pub dl_selected_prs_resource_index_list_r16:
        Option<DL_SelectedPRS_ResourceSetIndex_r16Dl_SelectedPRS_ResourceIndexList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct DL_TDOA_MeasCapabilityPerBand_r17 {
    pub freq_band_indicator_nr_r17: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub support_of_dl_prs_first_path_rsrp_r17:
        Option<DL_TDOA_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17>,
    #[asn(optional_idx = 1)]
    pub dl_prs_meas_rrc_inactive_r17:
        Option<DL_TDOA_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Delta_Height_r16 {
    pub delta_height_r16: Delta_Height_r16Delta_Height_r16,
    #[asn(optional_idx = 0)]
    pub coarse_delta_height_r16: Option<Delta_Height_r16Coarse_delta_Height_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Delta_Latitude_r16 {
    pub delta_latitude_r16: Delta_Latitude_r16Delta_Latitude_r16,
    #[asn(optional_idx = 0)]
    pub coarse_delta_latitude_r16: Option<Delta_Latitude_r16Coarse_delta_Latitude_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Delta_Longitude_r16 {
    pub delta_longitude_r16: Delta_Longitude_r16Delta_Longitude_r16,
    #[asn(optional_idx = 0)]
    pub coarse_delta_longitude_r16: Option<Delta_Longitude_r16Coarse_delta_Longitude_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum DeltaTime_r15 {
    #[asn(key = 0, extended = false)]
    DeltaTimeSec_r15(DeltaTime_r15_deltaTimeSec_r15),
    #[asn(key = 1, extended = false)]
    DeltaTimeSFN_r15(DeltaTime_r15_deltaTimeSFN_r15),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct Displacement_r15 {
    pub bearing_r15: Displacement_r15Bearing_r15,
    #[asn(optional_idx = 0)]
    pub bearing_unc_confidence_r15: Option<Displacement_r15BearingUncConfidence_r15>,
    pub bearing_ref_r15: Displacement_r15BearingRef_r15,
    pub horizontal_distance_r15: Displacement_r15HorizontalDistance_r15,
    #[asn(optional_idx = 1)]
    pub horizontal_distance_unc_r15: Option<Displacement_r15HorizontalDistanceUnc_r15>,
    #[asn(optional_idx = 2)]
    pub horizontal_unc_confidence_r15: Option<Displacement_r15HorizontalUncConfidence_r15>,
    #[asn(optional_idx = 3)]
    pub vertical_direction_r15: Option<Displacement_r15VerticalDirection_r15>,
    #[asn(optional_idx = 4)]
    pub vertical_distance_r15: Option<Displacement_r15VerticalDistance_r15>,
    #[asn(optional_idx = 5)]
    pub vertical_distance_unc_r15: Option<Displacement_r15VerticalDistanceUnc_r15>,
    #[asn(optional_idx = 6)]
    pub vertical_unc_confidence_r15: Option<Displacement_r15VerticalUncConfidence_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct DisplacementInfoList_r15(pub Vec<DisplacementInfoListElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct DisplacementInfoListElement_r15 {
    pub delta_time_stamp_r15: DeltaTime_r15,
    #[asn(optional_idx = 0)]
    pub displacement_r15: Option<Displacement_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum DisplacementTimeStamp_r15 {
    #[asn(key = 0, extended = false)]
    UtcTime_r15(UTC_Time_r15),
    #[asn(key = 1, extended = false)]
    GnssTime_r15(MeasurementReferenceTime),
    #[asn(key = 2, extended = false)]
    SystemFrameNumber_r15(SFN_r15),
    #[asn(key = 3, extended = false)]
    MeasurementSFN_r15(DisplacementTimeStamp_r15_measurementSFN_r15),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ECGI {
    pub mcc: ECGIMcc,
    pub mnc: ECGIMnc,
    pub cellidentity: ECGICellidentity,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ECID_Error {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses(ECID_LocationServerErrorCauses),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses(ECID_TargetDeviceErrorCauses),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECID_LocationServerErrorCauses {
    pub cause: ECID_LocationServerErrorCausesCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECID_ProvideCapabilities {
    pub ecid_meas_supported: ECID_ProvideCapabilitiesEcid_MeasSupported,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ECID_ProvideLocationInformation {
    #[asn(optional_idx = 0)]
    pub ecid_signal_measurement_information: Option<ECID_SignalMeasurementInformation>,
    #[asn(optional_idx = 1)]
    pub ecid_error: Option<ECID_Error>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECID_RequestCapabilities {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ECID_RequestLocationInformation {
    pub requested_measurements: ECID_RequestLocationInformationRequestedMeasurements,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ECID_SignalMeasurementInformation {
    #[asn(optional_idx = 0)]
    pub primary_cell_measured_results: Option<MeasuredResultsElement>,
    pub measured_results_list: MeasuredResultsList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ECID_TargetDeviceErrorCauses {
    pub cause: ECID_TargetDeviceErrorCausesCause,
    #[asn(optional_idx = 0)]
    pub rsrp_measurement_not_possible:
        Option<ECID_TargetDeviceErrorCausesRsrpMeasurementNotPossible>,
    #[asn(optional_idx = 1)]
    pub rsrq_measurement_not_possible:
        Option<ECID_TargetDeviceErrorCausesRsrqMeasurementNotPossible>,
    #[asn(optional_idx = 2)]
    pub ue_rx_tx_measurement_not_possible:
        Option<ECID_TargetDeviceErrorCausesUeRxTxMeasurementNotPossible>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EPDU {
    pub epdu_identifier: EPDU_Identifier,
    pub epdu_body: EPDU_Body,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "OCTET-STRING")]
pub struct EPDU_Body(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct EPDU_ID(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct EPDU_Identifier {
    pub epdu_id: EPDU_ID,
    #[asn(optional_idx = 0)]
    pub epdu_name: Option<EPDU_Name>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct EPDU_Name(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct EPDU_Sequence(pub Vec<EPDU>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EarlyFixReport_r12(pub u8);
impl EarlyFixReport_r12 {
    pub const NO_MORE_MESSAGES: u8 = 0u8;
    pub const MORE_MESSAGES_ON_THE_WAY: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ElevationElement_R17 {
    #[asn(optional_idx = 0)]
    pub elevation_r17: Option<ElevationElement_R17Elevation_r17>,
    #[asn(optional_idx = 1)]
    pub elevation_fine_r17: Option<ElevationElement_R17Elevation_fine_r17>,
    pub beam_power_list_r17: ElevationElement_R17BeamPowerList_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ellipsoid_Point {
    pub latitude_sign: Ellipsoid_PointLatitudeSign,
    pub degrees_latitude: Ellipsoid_PointDegreesLatitude,
    pub degrees_longitude: Ellipsoid_PointDegreesLongitude,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Ellipsoid_PointWithUncertaintyCircle {
    pub latitude_sign: Ellipsoid_PointWithUncertaintyCircleLatitudeSign,
    pub degrees_latitude: Ellipsoid_PointWithUncertaintyCircleDegreesLatitude,
    pub degrees_longitude: Ellipsoid_PointWithUncertaintyCircleDegreesLongitude,
    pub uncertainty: Ellipsoid_PointWithUncertaintyCircleUncertainty,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EllipsoidArc {
    pub latitude_sign: EllipsoidArcLatitudeSign,
    pub degrees_latitude: EllipsoidArcDegreesLatitude,
    pub degrees_longitude: EllipsoidArcDegreesLongitude,
    pub inner_radius: EllipsoidArcInnerRadius,
    pub uncertainty_radius: EllipsoidArcUncertaintyRadius,
    pub offset_angle: EllipsoidArcOffsetAngle,
    pub included_angle: EllipsoidArcIncludedAngle,
    pub confidence: EllipsoidArcConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EllipsoidPointWithAltitude {
    pub latitude_sign: EllipsoidPointWithAltitudeLatitudeSign,
    pub degrees_latitude: EllipsoidPointWithAltitudeDegreesLatitude,
    pub degrees_longitude: EllipsoidPointWithAltitudeDegreesLongitude,
    pub altitude_direction: EllipsoidPointWithAltitudeAltitudeDirection,
    pub altitude: EllipsoidPointWithAltitudeAltitude,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoid {
    pub latitude_sign: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidLatitudeSign,
    pub degrees_latitude: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidDegreesLatitude,
    pub degrees_longitude: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidDegreesLongitude,
    pub altitude_direction: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidAltitudeDirection,
    pub altitude: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidAltitude,
    pub uncertainty_semi_major:
        EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintySemiMajor,
    pub uncertainty_semi_minor:
        EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintySemiMinor,
    pub orientation_major_axis:
        EllipsoidPointWithAltitudeAndUncertaintyEllipsoidOrientationMajorAxis,
    pub uncertainty_altitude: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintyAltitude,
    pub confidence: EllipsoidPointWithAltitudeAndUncertaintyEllipsoidConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct EllipsoidPointWithUncertaintyEllipse {
    pub latitude_sign: EllipsoidPointWithUncertaintyEllipseLatitudeSign,
    pub degrees_latitude: EllipsoidPointWithUncertaintyEllipseDegreesLatitude,
    pub degrees_longitude: EllipsoidPointWithUncertaintyEllipseDegreesLongitude,
    pub uncertainty_semi_major: EllipsoidPointWithUncertaintyEllipseUncertaintySemiMajor,
    pub uncertainty_semi_minor: EllipsoidPointWithUncertaintyEllipseUncertaintySemiMinor,
    pub orientation_major_axis: EllipsoidPointWithUncertaintyEllipseOrientationMajorAxis,
    pub confidence: EllipsoidPointWithUncertaintyEllipseConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct Environment(pub u8);
impl Environment {
    pub const BAD_AREA: u8 = 0u8;
    pub const NOT_BAD_AREA: u8 = 1u8;
    pub const MIXED_AREA: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum EqualIntegerAmbiguityLevel_r16 {
    #[asn(key = 0, extended = false)]
    AllReferenceStations_r16(EqualIntegerAmbiguityLevel_r16_allReferenceStations_r16),
    #[asn(key = 1, extended = false)]
    ReferenceStationList_r16(ReferenceStationList_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum Error {
    #[asn(key = 0, extended = false)]
    Error_r9(Error_r9_IEs),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(Error_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Error_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_error: Option<CommonIEsError>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct FKP_Gradients_Element_r15 {
    pub sv_id_r15: SV_ID,
    pub iod_r15: FKP_Gradients_Element_r15Iod_r15,
    pub north_geometric_gradient_r15: FKP_Gradients_Element_r15North_geometric_gradient_r15,
    pub east_geometric_gradient_r15: FKP_Gradients_Element_r15East_geometric_gradient_r15,
    pub north_ionospheric_gradient_r15: FKP_Gradients_Element_r15North_ionospheric_gradient_r15,
    pub east_ionospheric_gradient_r15: FKP_Gradients_Element_r15East_ionospheric_gradient_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct FKP_Gradients_List_r15(pub Vec<FKP_Gradients_Element_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1024")]
pub struct FreqBandIndicatorNR_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GLO_RTK_BiasInformation_r15 {
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub cpb_indicator_r15: GLO_RTK_BiasInformation_r15CpbIndicator_r15,
    #[asn(optional_idx = 0)]
    pub l1_ca_cp_bias_r15: Option<GLO_RTK_BiasInformation_r15L1_ca_cpBias_r15>,
    #[asn(optional_idx = 1)]
    pub l1_p_cp_bias_r15: Option<GLO_RTK_BiasInformation_r15L1_p_cpBias_r15>,
    #[asn(optional_idx = 2)]
    pub l2_ca_cp_bias_r15: Option<GLO_RTK_BiasInformation_r15L2_ca_cpBias_r15>,
    #[asn(optional_idx = 3)]
    pub l2_p_cp_bias_r15: Option<GLO_RTK_BiasInformation_r15L2_p_cpBias_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GLO_RTK_BiasInformationReq_r15 {
    #[asn(optional_idx = 0)]
    pub station_id_r15: Option<GNSS_ReferenceStationID_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GLO_RTK_BiasInformationSupport_r15 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GLONASS_ClockModel {
    pub glo_tau: GLONASS_ClockModelGloTau,
    pub glo_gamma: GLONASS_ClockModelGloGamma,
    #[asn(optional_idx = 0)]
    pub glo_delta_tau: Option<GLONASS_ClockModelGloDeltaTau>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AcquisitionAssistElement {
    pub sv_id: SV_ID,
    pub doppler0: GNSS_AcquisitionAssistElementDoppler0,
    pub doppler1: GNSS_AcquisitionAssistElementDoppler1,
    pub doppler_uncertainty: GNSS_AcquisitionAssistElementDopplerUncertainty,
    pub code_phase: GNSS_AcquisitionAssistElementCodePhase,
    pub int_code_phase: GNSS_AcquisitionAssistElementIntCodePhase,
    pub code_phase_search_window: GNSS_AcquisitionAssistElementCodePhaseSearchWindow,
    pub azimuth: GNSS_AcquisitionAssistElementAzimuth,
    pub elevation: GNSS_AcquisitionAssistElementElevation,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_AcquisitionAssistList(pub Vec<GNSS_AcquisitionAssistElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AcquisitionAssistance {
    pub gnss_signal_id: GNSS_SignalID,
    pub gnss_acquisition_assist_list: GNSS_AcquisitionAssistList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AcquisitionAssistanceReq {
    pub gnss_signal_id_req: GNSS_SignalID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AcquisitionAssistanceSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GNSS_Almanac {
    #[asn(optional_idx = 0)]
    pub week_number: Option<GNSS_AlmanacWeekNumber>,
    #[asn(optional_idx = 1)]
    pub toa: Option<GNSS_AlmanacToa>,
    #[asn(optional_idx = 2)]
    pub ioda: Option<GNSS_AlmanacIoda>,
    pub complete_almanac_provided: GNSS_AlmanacCompleteAlmanacProvided,
    pub gnss_almanac_list: GNSS_AlmanacList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum GNSS_AlmanacElement {
    #[asn(key = 0, extended = false)]
    KeplerianAlmanacSet(AlmanacKeplerianSet),
    #[asn(key = 1, extended = false)]
    KeplerianNAV_Almanac(AlmanacNAV_KeplerianSet),
    #[asn(key = 2, extended = false)]
    KeplerianReducedAlmanac(AlmanacReducedKeplerianSet),
    #[asn(key = 3, extended = false)]
    KeplerianMidiAlmanac(AlmanacMidiAlmanacSet),
    #[asn(key = 4, extended = false)]
    KeplerianGLONASS(AlmanacGLONASS_AlmanacSet),
    #[asn(key = 5, extended = false)]
    Ecef_SBAS_Almanac(AlmanacECEF_SBAS_AlmanacSet),
    #[asn(key = 0, extended = true)]
    KeplerianBDS_Almanac_r12(AlmanacBDS_AlmanacSet_r12),
    #[asn(key = 1, extended = true)]
    KeplerianNavIC_Almanac_r16(AlmanacNavIC_AlmanacSet_r16),
    #[asn(key = 2, extended = true)]
    KeplerianNavIC_Almanac2_r19(AlmanacNavIC_AlmanacSet2_r19),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_AlmanacList(pub Vec<GNSS_AlmanacElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_AlmanacReq {
    #[asn(optional_idx = 0)]
    pub model_id: Option<GNSS_AlmanacReqModelID>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_AlmanacSupport {
    #[asn(optional_idx = 0)]
    pub almanac_model: Option<GNSS_AlmanacSupportAlmanacModel>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum GNSS_AuxiliaryInformation {
    #[asn(key = 0, extended = false)]
    Gnss_ID_GPS(GNSS_ID_GPS),
    #[asn(key = 1, extended = false)]
    Gnss_ID_GLONASS(GNSS_ID_GLONASS),
    #[asn(key = 0, extended = true)]
    Gnss_ID_BDS_r16(GNSS_ID_BDS_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AuxiliaryInformationReq {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_AuxiliaryInformationSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_BadSignalList(pub Vec<BadSignalElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum GNSS_ClockModel {
    #[asn(key = 0, extended = false)]
    StandardClockModelList(StandardClockModelList),
    #[asn(key = 1, extended = false)]
    Nav_ClockModel(NAV_ClockModel),
    #[asn(key = 2, extended = false)]
    Cnav_ClockModel(CNAV_ClockModel),
    #[asn(key = 3, extended = false)]
    Glonass_ClockModel(GLONASS_ClockModel),
    #[asn(key = 4, extended = false)]
    Sbas_ClockModel(SBAS_ClockModel),
    #[asn(key = 0, extended = true)]
    Bds_ClockModel_r12(BDS_ClockModel_r12),
    #[asn(key = 1, extended = true)]
    Bds_ClockModel2_r16(BDS_ClockModel2_r16),
    #[asn(key = 2, extended = true)]
    Navic_ClockModel_r16(NavIC_ClockModel_r16),
    #[asn(key = 3, extended = true)]
    Navic_ClockModel2_r19(NavIC_ClockModel2_r19),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_CommonAssistData {
    #[asn(optional_idx = 0)]
    pub gnss_reference_time: Option<GNSS_ReferenceTime>,
    #[asn(optional_idx = 1)]
    pub gnss_reference_location: Option<GNSS_ReferenceLocation>,
    #[asn(optional_idx = 2)]
    pub gnss_ionospheric_model: Option<GNSS_IonosphericModel>,
    #[asn(optional_idx = 3)]
    pub gnss_earth_orientation_parameters: Option<GNSS_EarthOrientationParameters>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_CommonAssistDataReq {
    #[asn(optional_idx = 0)]
    pub gnss_reference_time_req: Option<GNSS_ReferenceTimeReq>,
    #[asn(optional_idx = 1)]
    pub gnss_reference_location_req: Option<GNSS_ReferenceLocationReq>,
    #[asn(optional_idx = 2)]
    pub gnss_ionospheric_model_req: Option<GNSS_IonosphericModelReq>,
    #[asn(optional_idx = 3)]
    pub gnss_earth_orientation_parameters_req: Option<GNSS_EarthOrientationParametersReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_CommonAssistanceDataSupport {
    #[asn(optional_idx = 0)]
    pub gnss_reference_time_support: Option<GNSS_ReferenceTimeSupport>,
    #[asn(optional_idx = 1)]
    pub gnss_reference_location_support: Option<GNSS_ReferenceLocationSupport>,
    #[asn(optional_idx = 2)]
    pub gnss_ionospheric_model_support: Option<GNSS_IonosphericModelSupport>,
    #[asn(optional_idx = 3)]
    pub gnss_earth_orientation_parameters_support: Option<GNSS_EarthOrientationParametersSupport>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_DataBitAssistance {
    pub gnss_tod: GNSS_DataBitAssistanceGnss_TOD,
    #[asn(optional_idx = 0)]
    pub gnss_to_dfrac: Option<GNSS_DataBitAssistanceGnss_TODfrac>,
    pub gnss_data_bits_sat_list: GNSS_DataBitsSatList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_DataBitAssistanceReq {
    pub gnss_tod_req: GNSS_DataBitAssistanceReqGnss_TOD_Req,
    #[asn(optional_idx = 0)]
    pub gnss_tod_frac_req: Option<GNSS_DataBitAssistanceReqGnss_TOD_FracReq>,
    pub data_bit_interval: GNSS_DataBitAssistanceReqDataBitInterval,
    pub gnss_signal_type: GNSS_SignalIDs,
    #[asn(optional_idx = 1)]
    pub gnss_data_bits_req: Option<GNSS_DataBitsReqSatList>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DataBitAssistanceSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DataBitsReqSatElement {
    pub sv_id: SV_ID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_DataBitsReqSatList(pub Vec<GNSS_DataBitsReqSatElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DataBitsSatElement {
    pub sv_id: SV_ID,
    pub gnss_data_bits_sgn_list: GNSS_DataBitsSgnList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_DataBitsSatList(pub Vec<GNSS_DataBitsSatElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DataBitsSgnElement {
    pub gnss_signal_type: GNSS_SignalID,
    pub gnss_data_bits: GNSS_DataBitsSgnElementGnss_DataBits,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_DataBitsSgnList(pub Vec<GNSS_DataBitsSgnElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DifferentialCorrections {
    pub dgnss_ref_time: GNSS_DifferentialCorrectionsDgnss_RefTime,
    pub dgnss_sgn_type_list: DGNSS_SgnTypeList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DifferentialCorrectionsReq {
    pub dgnss_signals_req: GNSS_SignalIDs,
    pub dgnss_validity_time_req: GNSS_DifferentialCorrectionsReqDgnss_ValidityTimeReq,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_DifferentialCorrectionsSupport {
    pub gnss_signal_i_ds: GNSS_SignalIDs,
    pub dgnss_validity_time_sup: GNSS_DifferentialCorrectionsSupportDgnss_ValidityTimeSup,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_EarthOrientationParameters {
    pub teop: GNSS_EarthOrientationParametersTeop,
    pub pm_x: GNSS_EarthOrientationParametersPmX,
    pub pm_xdot: GNSS_EarthOrientationParametersPmXdot,
    pub pm_y: GNSS_EarthOrientationParametersPmY,
    pub pm_ydot: GNSS_EarthOrientationParametersPmYdot,
    pub delta_ut1: GNSS_EarthOrientationParametersDeltaUT1,
    pub delta_ut1dot: GNSS_EarthOrientationParametersDeltaUT1dot,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_EarthOrientationParametersReq {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_EarthOrientationParametersSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_FrequencyID_r15 {
    pub gnss_frequency_id_r15: GNSS_FrequencyID_r15Gnss_FrequencyID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_GenericAssistData(pub Vec<GNSS_GenericAssistDataElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct GNSS_GenericAssistDataElement {
    pub gnss_id: GNSS_ID,
    #[asn(optional_idx = 0)]
    pub sbas_id: Option<SBAS_ID>,
    #[asn(optional_idx = 1)]
    pub gnss_time_models: Option<GNSS_TimeModelList>,
    #[asn(optional_idx = 2)]
    pub gnss_differential_corrections: Option<GNSS_DifferentialCorrections>,
    #[asn(optional_idx = 3)]
    pub gnss_navigation_model: Option<GNSS_NavigationModel>,
    #[asn(optional_idx = 4)]
    pub gnss_real_time_integrity: Option<GNSS_RealTimeIntegrity>,
    #[asn(optional_idx = 5)]
    pub gnss_data_bit_assistance: Option<GNSS_DataBitAssistance>,
    #[asn(optional_idx = 6)]
    pub gnss_acquisition_assistance: Option<GNSS_AcquisitionAssistance>,
    #[asn(optional_idx = 7)]
    pub gnss_almanac: Option<GNSS_Almanac>,
    #[asn(optional_idx = 8)]
    pub gnss_utc_model: Option<GNSS_UTC_Model>,
    #[asn(optional_idx = 9)]
    pub gnss_auxiliary_information: Option<GNSS_AuxiliaryInformation>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_GenericAssistDataReq(pub Vec<GNSS_GenericAssistDataReqElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct GNSS_GenericAssistDataReqElement {
    pub gnss_id: GNSS_ID,
    #[asn(optional_idx = 0)]
    pub sbas_id: Option<SBAS_ID>,
    #[asn(optional_idx = 1)]
    pub gnss_time_models_req: Option<GNSS_TimeModelListReq>,
    #[asn(optional_idx = 2)]
    pub gnss_differential_corrections_req: Option<GNSS_DifferentialCorrectionsReq>,
    #[asn(optional_idx = 3)]
    pub gnss_navigation_model_req: Option<GNSS_NavigationModelReq>,
    #[asn(optional_idx = 4)]
    pub gnss_real_time_integrity_req: Option<GNSS_RealTimeIntegrityReq>,
    #[asn(optional_idx = 5)]
    pub gnss_data_bit_assistance_req: Option<GNSS_DataBitAssistanceReq>,
    #[asn(optional_idx = 6)]
    pub gnss_acquisition_assistance_req: Option<GNSS_AcquisitionAssistanceReq>,
    #[asn(optional_idx = 7)]
    pub gnss_almanac_req: Option<GNSS_AlmanacReq>,
    #[asn(optional_idx = 8)]
    pub gnss_utc_model_req: Option<GNSS_UTC_ModelReq>,
    #[asn(optional_idx = 9)]
    pub gnss_auxiliary_information_req: Option<GNSS_AuxiliaryInformationReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct GNSS_GenericAssistDataSupportElement {
    pub gnss_id: GNSS_ID,
    #[asn(optional_idx = 0)]
    pub sbas_id: Option<SBAS_ID>,
    #[asn(optional_idx = 1)]
    pub gnss_time_models_support: Option<GNSS_TimeModelListSupport>,
    #[asn(optional_idx = 2)]
    pub gnss_differential_corrections_support: Option<GNSS_DifferentialCorrectionsSupport>,
    #[asn(optional_idx = 3)]
    pub gnss_navigation_model_support: Option<GNSS_NavigationModelSupport>,
    #[asn(optional_idx = 4)]
    pub gnss_real_time_integrity_support: Option<GNSS_RealTimeIntegritySupport>,
    #[asn(optional_idx = 5)]
    pub gnss_data_bit_assistance_support: Option<GNSS_DataBitAssistanceSupport>,
    #[asn(optional_idx = 6)]
    pub gnss_acquisition_assistance_support: Option<GNSS_AcquisitionAssistanceSupport>,
    #[asn(optional_idx = 7)]
    pub gnss_almanac_support: Option<GNSS_AlmanacSupport>,
    #[asn(optional_idx = 8)]
    pub gnss_utc_model_support: Option<GNSS_UTC_ModelSupport>,
    #[asn(optional_idx = 9)]
    pub gnss_auxiliary_information_support: Option<GNSS_AuxiliaryInformationSupport>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_GenericAssistanceDataSupport(pub Vec<GNSS_GenericAssistDataSupportElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ID {
    pub gnss_id: GNSS_IDGnss_id,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ID_BDS_SatElement_r16 {
    pub sv_id_r16: SV_ID,
    pub sat_type_r16: GNSS_ID_BDS_SatElement_r16SatType_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_ID_BDS_r16(pub Vec<GNSS_ID_BDS_SatElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ID_Bitmap {
    pub gnss_ids: GNSS_ID_BitmapGnss_ids,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_ID_GLONASS(pub Vec<GNSS_ID_GLONASS_SatElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_ID_GLONASS_SatElement {
    pub sv_id: SV_ID,
    pub signals_available: GNSS_SignalIDs,
    #[asn(optional_idx = 0)]
    pub channel_number: Option<GNSS_ID_GLONASS_SatElementChannelNumber>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_ID_GPS(pub Vec<GNSS_ID_GPS_SatElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ID_GPS_SatElement {
    pub sv_id: SV_ID,
    pub signals_available: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceAlert_r17 {
    pub ionosphere_do_not_use_r17: GNSS_Integrity_ServiceAlert_r17IonosphereDoNotUse_r17,
    pub troposphere_do_not_use_r17: GNSS_Integrity_ServiceAlert_r17TroposphereDoNotUse_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceAlertReq_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceAlertSupport_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceParameters_r17 {
    pub ir_minimum_r17: GNSS_Integrity_ServiceParameters_r17IrMinimum_r17,
    pub ir_maximum_r17: GNSS_Integrity_ServiceParameters_r17IrMaximum_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceParametersReq_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Integrity_ServiceParametersSupport_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_IonosphericModel {
    #[asn(optional_idx = 0)]
    pub klobuchar_model: Option<KlobucharModelParameter>,
    #[asn(optional_idx = 1)]
    pub ne_quick_model: Option<NeQuickModelParameter>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_IonosphericModelReq {
    #[asn(optional_idx = 0)]
    pub klobuchar_model_req: Option<GNSS_IonosphericModelReqKlobucharModelReq>,
    #[asn(optional_idx = 1)]
    pub ne_quick_model_req: Option<GNSS_IonosphericModelReqNeQuickModelReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_IonosphericModelSupport {
    pub iono_model: GNSS_IonosphericModelSupportIonoModel,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_LOS_InfoElement_r18 {
    pub sv_id_r18: SV_ID,
    pub los_r18: GNSS_LOS_InfoElement_r18Los_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_LOS_InfoList_r18(pub Vec<GNSS_LOS_InfoElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_LOS_NLOS_GridPoints_r18 {
    pub grid_points_set_id_r18: GNSS_LOS_NLOS_GridPoints_r18GridPointsSetID_r18,
    pub horizontal_grid_points_r18: ArrayOfGridPoints_r18,
    #[asn(optional_idx = 0)]
    pub reference_altitude_fine_r18: Option<GNSS_LOS_NLOS_GridPoints_r18ReferenceAltitudeFine_r18>,
    #[asn(optional_idx = 1)]
    pub vertical_grid_points_r18: Option<VerticalGridPoints_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_LOS_NLOS_GriddedIndications_r18 {
    pub grid_points_set_id_r18: GNSS_LOS_NLOS_GriddedIndications_r18GridPointsSetID_r18,
    #[asn(optional_idx = 0)]
    pub expiration_time_r18: Option<GNSS_LOS_NLOS_GriddedIndications_r18ExpirationTime_r18>,
    pub grid_list_r18: GridList_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18 {
    #[asn(optional_idx = 0)]
    pub grid_points_set_id_req_r18:
        Option<GNSS_LOS_NLOS_GriddedIndicationsReq_r18GridPointsSetID_Req_r18>,
    #[asn(optional_idx = 1)]
    pub relative_location_info_r18:
        Option<GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18>,
    pub vertical_grid_type_r18: GNSS_LOS_NLOS_GriddedIndicationsReq_r18VerticalGridType_r18,
    #[asn(optional_idx = 2)]
    pub reference_altitude_fine_r18:
        Option<GNSS_LOS_NLOS_GriddedIndicationsReq_r18ReferenceAltitudeFine_r18>,
    #[asn(optional_idx = 3)]
    pub reference_altitude_coarse_r18:
        Option<GNSS_LOS_NLOS_GriddedIndicationsReq_r18ReferenceAltitudeCoarse_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_LOS_NLOS_GriddedIndicationsSupport_r18 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_Link_Combinations_r15 {
    pub l1_r15: GNSS_FrequencyID_r15,
    pub l2_r15: GNSS_FrequencyID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_Link_CombinationsList_r15(pub Vec<GNSS_Link_Combinations_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_LocationInformation {
    pub measurement_reference_time: MeasurementReferenceTime,
    pub agnss_list: GNSS_ID_Bitmap,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_LocationServerErrorCauses {
    pub cause: GNSS_LocationServerErrorCausesCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_MeasurementForOneGNSS {
    pub gnss_id: GNSS_ID,
    pub gnss_sgn_meas_list: GNSS_SgnMeasList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_MeasurementList(pub Vec<GNSS_MeasurementForOneGNSS>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_NavListInfo_r15(pub Vec<SatListElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_NavModelSatelliteElement {
    pub sv_id: SV_ID,
    pub sv_health: GNSS_NavModelSatelliteElementSvHealth,
    pub iod: GNSS_NavModelSatelliteElementIod,
    pub gnss_clock_model: GNSS_ClockModel,
    pub gnss_orbit_model: GNSS_OrbitModel,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_NavModelSatelliteList(pub Vec<GNSS_NavModelSatelliteElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_NavigationModel {
    pub non_broadcast_ind_flag: GNSS_NavigationModelNonBroadcastIndFlag,
    pub gnss_satellite_list: GNSS_NavModelSatelliteList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum GNSS_NavigationModelReq {
    #[asn(key = 0, extended = false)]
    StoredNavList(StoredNavListInfo),
    #[asn(key = 1, extended = false)]
    ReqNavList(ReqNavListInfo),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_NavigationModelSupport {
    #[asn(optional_idx = 0)]
    pub clock_model: Option<GNSS_NavigationModelSupportClockModel>,
    #[asn(optional_idx = 1)]
    pub orbit_model: Option<GNSS_NavigationModelSupportOrbitModel>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_NetworkID_r15 {
    pub network_id_r15: GNSS_NetworkID_r15NetworkID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_ObservationList_r15(pub Vec<GNSS_RTK_SatelliteDataElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "4", extensible = true)]
pub enum GNSS_OrbitModel {
    #[asn(key = 0, extended = false)]
    KeplerianSet(NavModelKeplerianSet),
    #[asn(key = 1, extended = false)]
    Nav_KeplerianSet(NavModelNAV_KeplerianSet),
    #[asn(key = 2, extended = false)]
    Cnav_KeplerianSet(NavModelCNAV_KeplerianSet),
    #[asn(key = 3, extended = false)]
    Glonass_ECEF(NavModel_GLONASS_ECEF),
    #[asn(key = 4, extended = false)]
    Sbas_ECEF(NavModel_SBAS_ECEF),
    #[asn(key = 0, extended = true)]
    Bds_KeplerianSet_r12(NavModel_BDS_KeplerianSet_r12),
    #[asn(key = 1, extended = true)]
    Bds_KeplerianSet2_r16(NavModel_BDS_KeplerianSet2_r16),
    #[asn(key = 2, extended = true)]
    Navic_KeplerianSet_r16(NavModel_NavIC_KeplerianSet_r16),
    #[asn(key = 3, extended = true)]
    Navic_KeplerianSet2_r19(NavModel_NavIC_KeplerianSet2_r19),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct GNSS_PeriodicAssistData_r15 {
    #[asn(optional_idx = 0)]
    pub gnss_rtk_periodic_observations_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 1)]
    pub glo_rtk_periodic_bias_information_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 2)]
    pub gnss_rtk_mac_periodic_correction_differences_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 3)]
    pub gnss_rtk_periodic_residuals_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 4)]
    pub gnss_rtk_fkp_periodic_gradients_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 5)]
    pub gnss_ssr_periodic_orbit_corrections_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 6)]
    pub gnss_ssr_periodic_clock_corrections_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 7)]
    pub gnss_ssr_periodic_code_bias_r15: Option<GNSS_PeriodicControlParam_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct GNSS_PeriodicAssistDataReq_r15 {
    #[asn(optional_idx = 0)]
    pub gnss_rtk_periodic_observations_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 1)]
    pub glo_rtk_periodic_bias_information_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 2)]
    pub gnss_rtk_mac_periodic_correction_differences_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 3)]
    pub gnss_rtk_periodic_residuals_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 4)]
    pub gnss_rtk_fkp_periodic_gradients_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 5)]
    pub gnss_ssr_periodic_orbit_corrections_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 6)]
    pub gnss_ssr_periodic_clock_corrections_req_r15: Option<GNSS_PeriodicControlParam_r15>,
    #[asn(optional_idx = 7)]
    pub gnss_ssr_periodic_code_bias_req_r15: Option<GNSS_PeriodicControlParam_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_PeriodicControlParam_r15 {
    pub delivery_amount_r15: GNSS_PeriodicControlParam_r15DeliveryAmount_r15,
    pub delivery_interval_r15: GNSS_PeriodicControlParam_r15DeliveryInterval_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_PositioningInstructions {
    pub gnss_methods: GNSS_ID_Bitmap,
    pub fine_time_assistance_meas_req: GNSS_PositioningInstructionsFineTimeAssistanceMeasReq,
    pub adr_meas_req: GNSS_PositioningInstructionsAdrMeasReq,
    pub multi_freq_meas_req: GNSS_PositioningInstructionsMultiFreqMeasReq,
    pub assistance_availability: GNSS_PositioningInstructionsAssistanceAvailability,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_RTK_AuxiliaryStationData_r15 {
    pub network_id_r15: GNSS_NetworkID_r15,
    #[asn(optional_idx = 0)]
    pub sub_network_id_r15: Option<GNSS_SubNetworkID_r15>,
    pub master_reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub auxiliary_station_list_r15: AuxiliaryStationList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_RTK_AuxiliaryStationDataReq_r15 {
    #[asn(optional_idx = 0)]
    pub master_reference_station_id_r15: Option<GNSS_ReferenceStationID_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_AuxiliaryStationDataSupport_r15 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_CommonObservationInfo_r15 {
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub clock_steering_indicator_r15: GNSS_RTK_CommonObservationInfo_r15ClockSteeringIndicator_r15,
    pub external_clock_indicator_r15: GNSS_RTK_CommonObservationInfo_r15ExternalClockIndicator_r15,
    pub smoothing_indicator_r15: GNSS_RTK_CommonObservationInfo_r15SmoothingIndicator_r15,
    pub smoothing_interval_r15: GNSS_RTK_CommonObservationInfo_r15SmoothingInterval_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_FKP_Gradients_r15 {
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub epoch_time_r15: GNSS_SystemTime,
    #[asn(optional_idx = 0)]
    pub l1_r15: Option<GNSS_FrequencyID_r15>,
    #[asn(optional_idx = 1)]
    pub l2_r15: Option<GNSS_FrequencyID_r15>,
    pub fkp_gradients_list_r15: FKP_Gradients_List_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_FKP_GradientsReq_r15 {
    #[asn(optional_idx = 0)]
    pub station_id_r15: Option<GNSS_ReferenceStationID_r15>,
    #[asn(optional_idx = 1)]
    pub link_combinations_pref_list_r15: Option<GNSS_Link_CombinationsList_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_FKP_GradientsSupport_r15 {
    pub link_combinations_support_r15: GNSS_Link_CombinationsList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GNSS_RTK_MAC_CorrectionDifferences_r15 {
    pub network_id_r15: GNSS_NetworkID_r15,
    #[asn(optional_idx = 0)]
    pub sub_network_id_r15: Option<GNSS_SubNetworkID_r15>,
    pub master_reference_station_id_r15: GNSS_ReferenceStationID_r15,
    #[asn(optional_idx = 1)]
    pub l1_r15: Option<GNSS_FrequencyID_r15>,
    #[asn(optional_idx = 2)]
    pub l2_r15: Option<GNSS_FrequencyID_r15>,
    pub rtk_correction_differences_list_r15: RTK_CorrectionDifferencesList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GNSS_RTK_MAC_CorrectionDifferencesReq_r15 {
    #[asn(optional_idx = 0)]
    pub master_reference_station_id_r15: Option<GNSS_ReferenceStationID_r15>,
    #[asn(optional_idx = 1)]
    pub aux_reference_station_list_r15: Option<AUX_ReferenceStationList_r15>,
    #[asn(optional_idx = 2)]
    pub link_combinations_pref_list_r15: Option<GNSS_Link_CombinationsList_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_MAC_CorrectionDifferencesSupport_r15 {
    pub link_combinations_support_r15: GNSS_Link_CombinationsList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_Observations_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub gnss_observation_list_r15: GNSS_ObservationList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_RTK_ObservationsReq_r15 {
    pub gnss_rtk_signals_req_r15: GNSS_SignalIDs,
    pub gnss_rtk_integer_ms_req_r15: GNSS_RTK_ObservationsReq_r15Gnss_RTK_Integer_ms_Req_r15,
    pub gnss_rtk_phase_range_rate_req_r15:
        GNSS_RTK_ObservationsReq_r15Gnss_RTK_PhaseRangeRateReq_r15,
    pub gnss_rtk_cnr_req_r15: GNSS_RTK_ObservationsReq_r15Gnss_RTK_CNR_Req_r15,
    #[asn(optional_idx = 0)]
    pub station_id_r15: Option<GNSS_ReferenceStationID_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_ObservationsSupport_r15 {
    pub gnss_signal_i_ds_r15: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_RTK_ReferenceStationInfo_r15 {
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub reference_station_indicator_r15:
        GNSS_RTK_ReferenceStationInfo_r15ReferenceStationIndicator_r15,
    pub antenna_reference_point_ecef_x_r15:
        GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_X_r15,
    pub antenna_reference_point_ecef_y_r15:
        GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_Y_r15,
    pub antenna_reference_point_ecef_z_r15:
        GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_Z_r15,
    #[asn(optional_idx = 0)]
    pub antenna_height_r15: Option<GNSS_RTK_ReferenceStationInfo_r15AntennaHeight_r15>,
    #[asn(optional_idx = 1)]
    pub antenna_description_r15: Option<AntennaDescription_r15>,
    #[asn(optional_idx = 2)]
    pub antenna_reference_point_unc_r15: Option<AntennaReferencePointUnc_r15>,
    #[asn(optional_idx = 3)]
    pub physical_reference_station_info_r15: Option<PhysicalReferenceStationInfo_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_RTK_ReferenceStationInfoReq_r15 {
    pub antenna_description_req_r15: GNSS_RTK_ReferenceStationInfoReq_r15AntennaDescriptionReq_r15,
    pub antenna_height_req_r15: GNSS_RTK_ReferenceStationInfoReq_r15AntennaHeightReq_r15,
    pub physical_reference_station_req_r15:
        GNSS_RTK_ReferenceStationInfoReq_r15PhysicalReferenceStationReq_r15,
    #[asn(optional_idx = 0)]
    pub station_id_r15: Option<GNSS_ReferenceStationID_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_ReferenceStationInfoSupport_r15 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_Residuals_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub n_refs_r15: GNSS_RTK_Residuals_r15N_Refs_r15,
    #[asn(optional_idx = 0)]
    pub l1_r15: Option<GNSS_FrequencyID_r15>,
    #[asn(optional_idx = 1)]
    pub l2_r15: Option<GNSS_FrequencyID_r15>,
    pub rtk_residuals_list_r15: RTK_Residuals_List_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_ResidualsReq_r15 {
    #[asn(optional_idx = 0)]
    pub station_id_r15: Option<GNSS_ReferenceStationID_r15>,
    #[asn(optional_idx = 1)]
    pub link_combinations_pref_list_r15: Option<GNSS_Link_CombinationsList_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RTK_ResidualsSupport_r15 {
    pub link_combinations_support_r15: GNSS_Link_CombinationsList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_SatelliteDataElement_r15 {
    pub sv_id_r15: SV_ID,
    #[asn(optional_idx = 0)]
    pub integer_ms_r15: Option<GNSS_RTK_SatelliteDataElement_r15Integer_ms_r15>,
    pub rough_range_r15: GNSS_RTK_SatelliteDataElement_r15Rough_range_r15,
    #[asn(optional_idx = 1)]
    pub rough_phase_range_rate_r15:
        Option<GNSS_RTK_SatelliteDataElement_r15Rough_phase_range_rate_r15>,
    pub gnss_rtk_satellite_signal_data_list_r15: GNSS_RTK_SatelliteSignalDataList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15 {
    pub gnss_signal_id_r15: GNSS_SignalID,
    pub fine_pseudo_range_r15: GNSS_RTK_SatelliteSignalDataElement_r15Fine_PseudoRange_r15,
    pub fine_phase_range_r15: GNSS_RTK_SatelliteSignalDataElement_r15Fine_PhaseRange_r15,
    pub lock_time_indicator_r15: GNSS_RTK_SatelliteSignalDataElement_r15LockTimeIndicator_r15,
    pub half_cycle_ambiguity_indicator_r15:
        GNSS_RTK_SatelliteSignalDataElement_r15HalfCycleAmbiguityIndicator_r15,
    #[asn(optional_idx = 0)]
    pub carrier_to_noise_ratio_r15:
        Option<GNSS_RTK_SatelliteSignalDataElement_r15Carrier_to_noise_ratio_r15>,
    #[asn(optional_idx = 1)]
    pub fine_phase_range_rate_r15:
        Option<GNSS_RTK_SatelliteSignalDataElement_r15Fine_PhaseRangeRate_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "24")]
pub struct GNSS_RTK_SatelliteSignalDataList_r15(pub Vec<GNSS_RTK_SatelliteSignalDataElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RealTimeIntegrity {
    pub gnss_bad_signal_list: GNSS_BadSignalList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RealTimeIntegrityReq {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_RealTimeIntegritySupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ReferenceLocation {
    pub three_dlocation: EllipsoidPointWithAltitudeAndUncertaintyEllipsoid,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ReferenceLocationReq {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_ReferenceLocationSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_ReferenceStationID_r15 {
    pub reference_station_id_r15: GNSS_ReferenceStationID_r15ReferenceStationID_r15,
    #[asn(optional_idx = 0)]
    pub provider_name_r15: Option<GNSS_ReferenceStationID_r15ProviderName_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_ReferenceTime {
    pub gnss_system_time: GNSS_SystemTime,
    #[asn(optional_idx = 0)]
    pub reference_time_unc: Option<GNSS_ReferenceTimeReferenceTimeUnc>,
    #[asn(optional_idx = 1)]
    pub gnss_reference_time_for_cells: Option<GNSS_ReferenceTimeGnss_ReferenceTimeForCells>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_ReferenceTimeForOneCell {
    pub network_time: NetworkTime,
    pub reference_time_unc: GNSS_ReferenceTimeForOneCellReferenceTimeUnc,
    #[asn(optional_idx = 0)]
    pub bs_align: Option<GNSS_ReferenceTimeForOneCellBsAlign>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_ReferenceTimeReq {
    pub gnss_time_req_pref_list: GNSS_ReferenceTimeReqGnss_TimeReqPrefList,
    #[asn(optional_idx = 0)]
    pub gps_tow_assist_req: Option<GNSS_ReferenceTimeReqGps_TOW_assistReq>,
    #[asn(optional_idx = 1)]
    pub not_of_leap_sec_req: Option<GNSS_ReferenceTimeReqNotOfLeapSecReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_ReferenceTimeSupport {
    pub gnss_system_time: GNSS_ID_Bitmap,
    #[asn(optional_idx = 0)]
    pub fta_support: Option<AccessTypes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16 {
    pub reference_point_latitude_r16:
        GNSS_SSR_ArrayOfCorrectionPoints_r16ReferencePointLatitude_r16,
    pub reference_point_longitude_r16:
        GNSS_SSR_ArrayOfCorrectionPoints_r16ReferencePointLongitude_r16,
    pub number_of_steps_latitude_r16: GNSS_SSR_ArrayOfCorrectionPoints_r16NumberOfStepsLatitude_r16,
    pub number_of_steps_longitude_r16:
        GNSS_SSR_ArrayOfCorrectionPoints_r16NumberOfStepsLongitude_r16,
    pub step_of_latitude_r16: GNSS_SSR_ArrayOfCorrectionPoints_r16StepOfLatitude_r16,
    pub step_of_longitude_r16: GNSS_SSR_ArrayOfCorrectionPoints_r16StepOfLongitude_r16,
    #[asn(optional_idx = 0)]
    pub bitmask_of_grids_r16: Option<GNSS_SSR_ArrayOfCorrectionPoints_r16BitmaskOfGrids_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_ClockCorrections_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub ssr_update_interval_r15: GNSS_SSR_ClockCorrections_r15SsrUpdateInterval_r15,
    pub iod_ssr_r15: GNSS_SSR_ClockCorrections_r15Iod_ssr_r15,
    pub ssr_clock_correction_list_r15: SSR_ClockCorrectionList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_ClockCorrectionsReq_r15 {
    #[asn(optional_idx = 0)]
    pub stored_nav_list_r15: Option<GNSS_NavListInfo_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_ClockCorrectionsSet2_r17 {
    pub ref_eph_r17: GNSS_SSR_ClockCorrectionsSet2_r17RefEph_r17,
    pub gnss_ssr_clock_corrections_r17: GNSS_SSR_ClockCorrections_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_ClockCorrectionsSet2Req_r17 {
    pub ref_eph_req_r17: GNSS_SSR_ClockCorrectionsSet2Req_r17RefEphReq_r17,
    #[asn(optional_idx = 0)]
    pub gnss_ssr_clock_corrections_set2_req_r17: Option<GNSS_SSR_ClockCorrectionsReq_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_ClockCorrectionsSet2Support_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_ClockCorrectionsSupport_r15 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_CodeBias_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub ssr_update_interval_r15: GNSS_SSR_CodeBias_r15SsrUpdateInterval_r15,
    pub iod_ssr_r15: GNSS_SSR_CodeBias_r15Iod_ssr_r15,
    pub ssr_code_bias_sat_list_r15: SSR_CodeBiasSatList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_CodeBiasReq_r15 {
    pub signal_and_tracking_mode_id_map_r15: GNSS_SignalIDs,
    #[asn(optional_idx = 0)]
    pub stored_nav_list_r15: Option<GNSS_NavListInfo_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_CodeBiasSupport_r15 {
    pub signal_and_tracking_mode_id_sup_r15: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_CorrectionPoints_r16 {
    pub correction_point_set_id_r16: GNSS_SSR_CorrectionPoints_r16CorrectionPointSetID_r16,
    pub correction_points_r16: GNSS_SSR_CorrectionPoints_r16CorrectionPoints_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_CorrectionPointsReq_r16 {
    #[asn(optional_idx = 0)]
    pub correction_point_set_id_req_r16:
        Option<GNSS_SSR_CorrectionPointsReq_r16CorrectionPointSetID_Req_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_GriddedCorrection_r16 {
    pub epoch_time_r16: GNSS_SystemTime,
    pub ssr_update_interval_r16: GNSS_SSR_GriddedCorrection_r16SsrUpdateInterval_r16,
    pub iod_ssr_r16: GNSS_SSR_GriddedCorrection_r16Iod_ssr_r16,
    #[asn(optional_idx = 0)]
    pub tropospheric_delay_quality_indicator_r16:
        Option<GNSS_SSR_GriddedCorrection_r16TroposphericDelayQualityIndicator_r16>,
    pub correction_point_set_id_r16: GNSS_SSR_GriddedCorrection_r16CorrectionPointSetID_r16,
    pub grid_list_r16: GridList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_GriddedCorrectionReq_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_GriddedCorrectionSupport_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_IOD_Update_r18 {
    pub epoch_time_r18: GNSS_SystemTime,
    pub ssr_update_interval_r18: GNSS_SSR_IOD_Update_r18SsrUpdateInterval_r18,
    pub iod_ssr_r18: GNSS_SSR_IOD_Update_r18Iod_ssr_r18,
    pub iod_ssr_pcv_residuals_r18: GNSS_SSR_IOD_Update_r18Iod_ssr_PCVResiduals_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_IOD_UpdateReq_r18 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_IOD_UpdateSupport_r18 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_ListOfCorrectionPoints_r16 {
    pub reference_point_latitude_r16: GNSS_SSR_ListOfCorrectionPoints_r16ReferencePointLatitude_r16,
    pub reference_point_longitude_r16:
        GNSS_SSR_ListOfCorrectionPoints_r16ReferencePointLongitude_r16,
    pub relative_locations_list_r16: GNSS_SSR_ListOfCorrectionPoints_r16RelativeLocationsList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_OrbitCorrections_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub ssr_update_interval_r15: GNSS_SSR_OrbitCorrections_r15SsrUpdateInterval_r15,
    pub satellite_reference_datum_r15: GNSS_SSR_OrbitCorrections_r15SatelliteReferenceDatum_r15,
    pub iod_ssr_r15: GNSS_SSR_OrbitCorrections_r15Iod_ssr_r15,
    pub ssr_orbit_correction_list_r15: SSR_OrbitCorrectionList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_OrbitCorrectionsReq_r15 {
    #[asn(optional_idx = 0)]
    pub stored_nav_list_r15: Option<GNSS_NavListInfo_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_OrbitCorrectionsSet2_r17 {
    pub ref_eph_r17: GNSS_SSR_OrbitCorrectionsSet2_r17RefEph_r17,
    pub gnss_ssr_orbit_corrections_r17: GNSS_SSR_OrbitCorrections_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_OrbitCorrectionsSet2Req_r17 {
    pub ref_eph_req_r17: GNSS_SSR_OrbitCorrectionsSet2Req_r17RefEphReq_r17,
    #[asn(optional_idx = 0)]
    pub gnss_ssr_orbit_corrections_set2_req_r17: Option<GNSS_SSR_OrbitCorrectionsReq_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_OrbitCorrectionsSet2Support_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_OrbitCorrectionsSupport_r15 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_PhaseBias_r16 {
    pub epoch_time_r16: GNSS_SystemTime,
    pub ssr_update_interval_r16: GNSS_SSR_PhaseBias_r16SsrUpdateInterval_r16,
    pub iod_ssr_r16: GNSS_SSR_PhaseBias_r16Iod_ssr_r16,
    pub ssr_phase_bias_sat_list_r16: SSR_PhaseBiasSatList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_PhaseBiasReq_r16 {
    pub signal_and_tracking_mode_id_map_r16: GNSS_SignalIDs,
    #[asn(optional_idx = 0)]
    pub stored_nav_list_r16: Option<GNSS_NavListInfo_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_PhaseBiasSupport_r16 {
    pub signal_and_tracking_mode_id_sup_r16: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SSR_ProviderInfo_r19 {
    pub ssr_provider_id_r19: GNSS_SSR_ProviderInfo_r19Ssr_ProviderID_r19,
    #[asn(optional_idx = 0)]
    pub ssr_solution_id_r19: Option<GNSS_SSR_ProviderInfo_r19Ssr_SolutionID_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_STEC_Correction_r16 {
    pub epoch_time_r16: GNSS_SystemTime,
    pub ssr_update_interval_r16: GNSS_SSR_STEC_Correction_r16SsrUpdateInterval_r16,
    pub iod_ssr_r16: GNSS_SSR_STEC_Correction_r16Iod_ssr_r16,
    pub correction_point_set_id_r16: GNSS_SSR_STEC_Correction_r16CorrectionPointSetID_r16,
    pub stec_sat_list_r16: STEC_SatList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_STEC_CorrectionReq_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_STEC_CorrectionSupport_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_SatellitePCVResiduals_r18 {
    pub iod_ssr_pcv_residuals_r18: GNSS_SSR_SatellitePCVResiduals_r18Iod_ssr_PCVResiduals_r18,
    pub ssr_satellite_pcv_list_r18: SSR_SatellitePCV_List_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_SatellitePCVResidualsReq_r18 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_SatellitePCVResidualsSupport_r18 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_Req_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_Set2_r17 {
    pub ref_eph_r17: GNSS_SSR_URA_Set2_r17RefEph_r17,
    pub gnss_ssr_ura_r17: GNSS_SSR_URA_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_Set2Req_r17 {
    pub ref_eph_req_r17: GNSS_SSR_URA_Set2Req_r17RefEphReq_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_Set2Support_r17 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_Support_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SSR_URA_r16 {
    pub epoch_time_r16: GNSS_SystemTime,
    pub ssr_update_interval_r16: GNSS_SSR_URA_r16SsrUpdateInterval_r16,
    pub iod_ssr_r16: GNSS_SSR_URA_r16Iod_ssr_r16,
    pub ssr_ura_sat_list_r16: SSR_URA_SatList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_SatMeasElement {
    pub sv_id: SV_ID,
    pub c_no: GNSS_SatMeasElementCNo,
    pub mpath_det: GNSS_SatMeasElementMpathDet,
    #[asn(optional_idx = 0)]
    pub carrier_quality_ind: Option<GNSS_SatMeasElementCarrierQualityInd>,
    pub code_phase: GNSS_SatMeasElementCodePhase,
    #[asn(optional_idx = 1)]
    pub integer_code_phase: Option<GNSS_SatMeasElementIntegerCodePhase>,
    pub code_phase_rms_error: GNSS_SatMeasElementCodePhaseRMSError,
    #[asn(optional_idx = 2)]
    pub doppler: Option<GNSS_SatMeasElementDoppler>,
    #[asn(optional_idx = 3)]
    pub adr: Option<GNSS_SatMeasElementAdr>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GNSS_SatMeasList(pub Vec<GNSS_SatMeasElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_SgnMeasElement {
    pub gnss_signal_id: GNSS_SignalID,
    #[asn(optional_idx = 0)]
    pub gnss_code_phase_ambiguity: Option<GNSS_SgnMeasElementGnss_CodePhaseAmbiguity>,
    pub gnss_sat_meas_list: GNSS_SatMeasList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_SgnMeasList(pub Vec<GNSS_SgnMeasElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SignalID {
    pub gnss_signal_id: GNSS_SignalIDGnss_SignalID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SignalIDs {
    pub gnss_signal_i_ds: GNSS_SignalIDsGnss_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SignalMeasurementInformation {
    pub measurement_reference_time: MeasurementReferenceTime,
    pub gnss_measurement_list: GNSS_MeasurementList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SubNetworkID_r15 {
    pub sub_network_id_r15: GNSS_SubNetworkID_r15SubNetworkID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GNSS_SupportElement {
    pub gnss_id: GNSS_ID,
    #[asn(optional_idx = 0)]
    pub sbas_i_ds: Option<SBAS_IDs>,
    pub agnss_modes: PositioningModes,
    pub gnss_signals: GNSS_SignalIDs,
    #[asn(optional_idx = 1)]
    pub fta_meas_support: Option<GNSS_SupportElementFta_MeasSupport>,
    pub adr_support: GNSS_SupportElementAdr_Support,
    pub velocity_measurement_support: GNSS_SupportElementVelocityMeasurementSupport,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_SupportList(pub Vec<GNSS_SupportElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GNSS_SystemTime {
    pub gnss_time_id: GNSS_ID,
    pub gnss_day_number: GNSS_SystemTimeGnss_DayNumber,
    pub gnss_time_of_day: GNSS_SystemTimeGnss_TimeOfDay,
    #[asn(optional_idx = 0)]
    pub gnss_time_of_day_frac_msec: Option<GNSS_SystemTimeGnss_TimeOfDayFrac_msec>,
    #[asn(optional_idx = 1)]
    pub notification_of_leap_second: Option<GNSS_SystemTimeNotificationOfLeapSecond>,
    #[asn(optional_idx = 2)]
    pub gps_tow_assist: Option<GPS_TOW_Assist>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct GNSS_TargetDeviceErrorCauses {
    pub cause: GNSS_TargetDeviceErrorCausesCause,
    #[asn(optional_idx = 0)]
    pub fine_time_assistance_measurements_not_possible:
        Option<GNSS_TargetDeviceErrorCausesFineTimeAssistanceMeasurementsNotPossible>,
    #[asn(optional_idx = 1)]
    pub adr_measurements_not_possible:
        Option<GNSS_TargetDeviceErrorCausesAdrMeasurementsNotPossible>,
    #[asn(optional_idx = 2)]
    pub multi_frequency_measurements_not_possible:
        Option<GNSS_TargetDeviceErrorCausesMultiFrequencyMeasurementsNotPossible>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct GNSS_TimeModelElement {
    pub gnss_time_model_ref_time: GNSS_TimeModelElementGnss_TimeModelRefTime,
    pub ta0: GNSS_TimeModelElementTA0,
    #[asn(optional_idx = 0)]
    pub ta1: Option<GNSS_TimeModelElementTA1>,
    #[asn(optional_idx = 1)]
    pub ta2: Option<GNSS_TimeModelElementTA2>,
    pub gnss_to_id: GNSS_TimeModelElementGnss_TO_ID,
    #[asn(optional_idx = 2)]
    pub week_number: Option<GNSS_TimeModelElementWeekNumber>,
    #[asn(optional_idx = 3)]
    pub delta_t: Option<GNSS_TimeModelElementDeltaT>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_TimeModelElementReq {
    pub gnss_to_i_ds_req: GNSS_TimeModelElementReqGnss_TO_IDsReq,
    pub delta_treq: GNSS_TimeModelElementReqDeltaTreq,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct GNSS_TimeModelList(pub Vec<GNSS_TimeModelElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "15")]
pub struct GNSS_TimeModelListReq(pub Vec<GNSS_TimeModelElementReq>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_TimeModelListSupport {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum GNSS_UTC_Model {
    #[asn(key = 0, extended = false)]
    UtcModel1(UTC_ModelSet1),
    #[asn(key = 1, extended = false)]
    UtcModel2(UTC_ModelSet2),
    #[asn(key = 2, extended = false)]
    UtcModel3(UTC_ModelSet3),
    #[asn(key = 3, extended = false)]
    UtcModel4(UTC_ModelSet4),
    #[asn(key = 0, extended = true)]
    UtcModel5_r12(UTC_ModelSet5_r12),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_UTC_ModelReq {
    #[asn(optional_idx = 0)]
    pub model_id: Option<GNSS_UTC_ModelReqModelID>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GNSS_UTC_ModelSupport {
    #[asn(optional_idx = 0)]
    pub utc_model: Option<GNSS_UTC_ModelSupportUtc_Model>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GPS_TOW_Assist(pub Vec<GPS_TOW_AssistElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GPS_TOW_AssistElement {
    pub satellite_id: GPS_TOW_AssistElementSatelliteID,
    pub tlm_word: GPS_TOW_AssistElementTlmWord,
    pub anti_spoof: GPS_TOW_AssistElementAntiSpoof,
    pub alert: GPS_TOW_AssistElementAlert,
    pub tlm_rsvd_bits: GPS_TOW_AssistElementTlmRsvdBits,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15 { pub sv_id_r15 : SV_ID , pub ambiguity_status_flag_r15 : Geometric_Ionospheric_Corrections_Differences_Element_r15AmbiguityStatusFlag_r15 , pub non_synch_count_r15 : Geometric_Ionospheric_Corrections_Differences_Element_r15Non_synch_count_r15 , pub geometric_carrier_phase_correction_difference_r15 : Geometric_Ionospheric_Corrections_Differences_Element_r15GeometricCarrierPhaseCorrectionDifference_r15 , pub iod_r15 : Geometric_Ionospheric_Corrections_Differences_Element_r15Iod_r15 , pub ionospheric_carrier_phase_correction_difference_r15 : Geometric_Ionospheric_Corrections_Differences_Element_r15IonosphericCarrierPhaseCorrectionDifference_r15 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct Geometric_Ionospheric_Corrections_Differences_r15(
    pub Vec<Geometric_Ionospheric_Corrections_Differences_Element_r15>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct GridElement_r16 {
    #[asn(optional_idx = 0)]
    pub troposperic_delay_correction_r16: Option<TropospericDelayCorrection_r16>,
    #[asn(optional_idx = 1)]
    pub stec_residual_sat_list_r16: Option<STEC_ResidualSatList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct GridElement_r18 {
    #[asn(optional_idx = 0)]
    pub gnss_los_info_list_r18: Option<GNSS_LOS_InfoList_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GridIonElement_r12 {
    pub igp_id_r12: GridIonElement_r12Igp_ID_r12,
    pub dt_r12: GridIonElement_r12Dt_r12,
    pub givei_r12: GridIonElement_r12Givei_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "320"
)]
pub struct GridIonList_r12(pub Vec<GridIonElement_r12>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct GridList_r16(pub Vec<GridElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct GridList_r18(pub Vec<GridElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16 { pub degrees_latitude_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16DegreesLatitude_r16 , pub degrees_longitude_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16DegreesLongitude_r16 , pub altitude_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Altitude_r16 , pub uncertainty_semi_major_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintySemiMajor_r16 , pub uncertainty_semi_minor_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintySemiMinor_r16 , pub orientation_major_axis_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16OrientationMajorAxis_r16 , pub horizontal_confidence_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16HorizontalConfidence_r16 , pub uncertainty_altitude_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintyAltitude_r16 , pub vertical_confidence_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16VerticalConfidence_r16 , pub ha_horizontal_extended_range_used_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Ha_HorizontalExtendedRangeUsed_r16 , pub ha_vertical_extended_range_used_r16 : HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Ha_VerticalExtendedRangeUsed_r16 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16 {
    pub degrees_latitude_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16DegreesLatitude_r16,
    pub degrees_longitude_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16DegreesLongitude_r16,
    pub uncertainty_semi_major_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16UncertaintySemiMajor_r16,
    pub uncertainty_semi_minor_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16UncertaintySemiMinor_r16,
    pub orientation_major_axis_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16OrientationMajorAxis_r16,
    pub confidence_r16: HA_EllipsoidPointWithScalableUncertaintyEllipse_r16Confidence_r16,
    pub ha_extended_uncertainty_range_used_r16:
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16Ha_ExtendedUncertaintyRangeUsed_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct HA_GNSS_Metrics_r17 {
    pub nr_of_used_satellites_r17: HA_GNSS_Metrics_r17NrOfUsedSatellites_r17,
    #[asn(optional_idx = 0)]
    pub hdopi_r17: Option<HA_GNSS_Metrics_r17Hdopi_r17>,
    #[asn(optional_idx = 1)]
    pub pdopi_r17: Option<HA_GNSS_Metrics_r17Pdopi_r17>,
    #[asn(optional_idx = 2)]
    pub age_r17: Option<HA_GNSS_Metrics_r17Age_r17>,
    #[asn(optional_idx = 3)]
    pub fix_type_r17: Option<HA_GNSS_Metrics_r17FixType_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15 {
    pub degrees_latitude_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15DegreesLatitude_r15,
    pub degrees_longitude_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15DegreesLongitude_r15,
    pub altitude_r15: HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15Altitude_r15,
    pub uncertainty_semi_major_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintySemiMajor_r15,
    pub uncertainty_semi_minor_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintySemiMinor_r15,
    pub orientation_major_axis_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15OrientationMajorAxis_r15,
    pub horizontal_confidence_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15HorizontalConfidence_r15,
    pub uncertainty_altitude_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintyAltitude_r15,
    pub vertical_confidence_r15:
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15VerticalConfidence_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15 {
    pub degrees_latitude_r15:
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15DegreesLatitude_r15,
    pub degrees_longitude_r15:
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15DegreesLongitude_r15,
    pub uncertainty_semi_major_r15:
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15UncertaintySemiMajor_r15,
    pub uncertainty_semi_minor_r15:
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15UncertaintySemiMinor_r15,
    pub orientation_major_axis_r15:
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15OrientationMajorAxis_r15,
    pub confidence_r15: HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15Confidence_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HorizontalAccuracy {
    pub accuracy: HorizontalAccuracyAccuracy,
    pub confidence: HorizontalAccuracyConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct HorizontalAccuracyExt_r15 {
    pub accuracy_ext_r15: HorizontalAccuracyExt_r15AccuracyExt_r15,
    pub confidence_r15: HorizontalAccuracyExt_r15Confidence_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocity {
    pub bearing: HorizontalVelocityBearing,
    pub horizontal_speed: HorizontalVelocityHorizontalSpeed,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalVelocityWithUncertainty {
    pub bearing: HorizontalVelocityWithUncertaintyBearing,
    pub horizontal_speed: HorizontalVelocityWithUncertaintyHorizontalSpeed,
    pub uncertainty_speed: HorizontalVelocityWithUncertaintyUncertaintySpeed,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocity {
    pub bearing: HorizontalWithVerticalVelocityBearing,
    pub horizontal_speed: HorizontalWithVerticalVelocityHorizontalSpeed,
    pub vertical_direction: HorizontalWithVerticalVelocityVerticalDirection,
    pub vertical_speed: HorizontalWithVerticalVelocityVerticalSpeed,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct HorizontalWithVerticalVelocityAndUncertainty {
    pub bearing: HorizontalWithVerticalVelocityAndUncertaintyBearing,
    pub horizontal_speed: HorizontalWithVerticalVelocityAndUncertaintyHorizontalSpeed,
    pub vertical_direction: HorizontalWithVerticalVelocityAndUncertaintyVerticalDirection,
    pub vertical_speed: HorizontalWithVerticalVelocityAndUncertaintyVerticalSpeed,
    pub horizontal_uncertainty_speed:
        HorizontalWithVerticalVelocityAndUncertaintyHorizontalUncertaintySpeed,
    pub vertical_uncertainty_speed:
        HorizontalWithVerticalVelocityAndUncertaintyVerticalUncertaintySpeed,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct Initiator(pub u8);
impl Initiator {
    pub const LOCATION_SERVER: u8 = 0u8;
    pub const TARGET_DEVICE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct IntegrityInfo_r17 {
    pub horizontal_protection_level_r17: IntegrityInfo_r17HorizontalProtectionLevel_r17,
    #[asn(optional_idx = 0)]
    pub vertical_protection_level_r17: Option<IntegrityInfo_r17VerticalProtectionLevel_r17>,
    #[asn(optional_idx = 1)]
    pub achievable_target_integrity_risk_r17:
        Option<IntegrityInfo_r17AchievableTargetIntegrityRisk_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct KlobucharModel2Parameter_r16 {
    pub alfa1_r16: KlobucharModel2Parameter_r16Alfa1_r16,
    pub alfa2_r16: KlobucharModel2Parameter_r16Alfa2_r16,
    pub alfa3_r16: KlobucharModel2Parameter_r16Alfa3_r16,
    pub alfa4_r16: KlobucharModel2Parameter_r16Alfa4_r16,
    pub alfa5_r16: KlobucharModel2Parameter_r16Alfa5_r16,
    pub alfa6_r16: KlobucharModel2Parameter_r16Alfa6_r16,
    pub alfa7_r16: KlobucharModel2Parameter_r16Alfa7_r16,
    pub alfa8_r16: KlobucharModel2Parameter_r16Alfa8_r16,
    pub alfa9_r16: KlobucharModel2Parameter_r16Alfa9_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct KlobucharModelParameter {
    pub data_id: KlobucharModelParameterDataID,
    pub alfa0: KlobucharModelParameterAlfa0,
    pub alfa1: KlobucharModelParameterAlfa1,
    pub alfa2: KlobucharModelParameterAlfa2,
    pub alfa3: KlobucharModelParameterAlfa3,
    pub beta0: KlobucharModelParameterBeta0,
    pub beta1: KlobucharModelParameterBeta1,
    pub beta2: KlobucharModelParameterBeta2,
    pub beta3: KlobucharModelParameterBeta3,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct LCS_GCS_TranslationParameter_r16 {
    pub alpha_r16: LCS_GCS_TranslationParameter_r16Alpha_r16,
    #[asn(optional_idx = 0)]
    pub alpha_fine_r16: Option<LCS_GCS_TranslationParameter_r16Alpha_fine_r16>,
    pub beta_r16: LCS_GCS_TranslationParameter_r16Beta_r16,
    #[asn(optional_idx = 1)]
    pub beta_fine_r16: Option<LCS_GCS_TranslationParameter_r16Beta_fine_r16>,
    pub gamma_r16: LCS_GCS_TranslationParameter_r16Gamma_r16,
    #[asn(optional_idx = 2)]
    pub gamma_fine_r16: Option<LCS_GCS_TranslationParameter_r16Gamma_fine_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LOS_NLOS_Indicator_r17 {
    pub indicator_r17: LOS_NLOS_Indicator_r17Indicator_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct LOS_NLOS_IndicatorGranularity1_r17(pub u8);
impl LOS_NLOS_IndicatorGranularity1_r17 {
    pub const TRPSPECIFIC: u8 = 0u8;
    pub const RESOURCESPECIFIC: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct LOS_NLOS_IndicatorGranularity2_r17(pub u8);
impl LOS_NLOS_IndicatorGranularity2_r17 {
    pub const TRPSPECIFIC: u8 = 0u8;
    pub const RESOURCESPECIFIC: u8 = 1u8;
    pub const BOTH: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct LOS_NLOS_IndicatorType1_r17(pub u8);
impl LOS_NLOS_IndicatorType1_r17 {
    pub const HARDVALUE: u8 = 0u8;
    pub const SOFTVALUE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct LOS_NLOS_IndicatorType2_r17(pub u8);
impl LOS_NLOS_IndicatorType2_r17 {
    pub const HARDVALUE: u8 = 0u8;
    pub const HARD_ANDSOFTVALUE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct LPP_Message {
    #[asn(optional_idx = 0)]
    pub transaction_id: Option<LPP_TransactionID>,
    pub end_transaction: LPP_MessageEndTransaction,
    #[asn(optional_idx = 1)]
    pub sequence_number: Option<SequenceNumber>,
    #[asn(optional_idx = 2)]
    pub acknowledgement: Option<Acknowledgement>,
    #[asn(optional_idx = 3)]
    pub lpp_message_body: Option<LPP_MessageBody>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum LPP_MessageBody {
    #[asn(key = 0, extended = false)]
    C1(LPP_MessageBody_c1),
    #[asn(key = 1, extended = false)]
    MessageClassExtension(LPP_MessageBody_messageClassExtension),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LPP_TransactionID {
    pub initiator: Initiator,
    pub transaction_number: TransactionNumber,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Local2dPointWithUncertaintyEllipse_r18 {
    pub local_origin_r18: ReferencePoint_r16,
    pub cartesian_coordinates_units_r18:
        Local2dPointWithUncertaintyEllipse_r18CartesianCoordinatesUnits_r18,
    pub x_value_r18: X_Value_r18,
    pub y_value_r18: Y_Value_r18,
    pub uncertainty_semi_major_r18: Local2dPointWithUncertaintyEllipse_r18UncertaintySemiMajor_r18,
    pub uncertainty_semi_minor_r18: Local2dPointWithUncertaintyEllipse_r18UncertaintySemiMinor_r18,
    pub orientation_major_axis_r18: Local2dPointWithUncertaintyEllipse_r18OrientationMajorAxis_r18,
    pub confidence_r18: Local2dPointWithUncertaintyEllipse_r18Confidence_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct Local3dPointWithUncertaintyEllipsoid_r18 {
    pub local_origin_r18: ReferencePoint_r16,
    pub cartesian_coordinates_units_r18:
        Local3dPointWithUncertaintyEllipsoid_r18CartesianCoordinatesUnits_r18,
    pub x_value_r18: X_Value_r18,
    pub y_value_r18: Y_Value_r18,
    pub z_value_r18: Z_Value_r18,
    pub uncertainty_semi_major_r18:
        Local3dPointWithUncertaintyEllipsoid_r18UncertaintySemiMajor_r18,
    pub uncertainty_semi_minor_r18:
        Local3dPointWithUncertaintyEllipsoid_r18UncertaintySemiMinor_r18,
    pub orientation_major_axis_r18:
        Local3dPointWithUncertaintyEllipsoid_r18OrientationMajorAxis_r18,
    pub uncertainty_altitude_r18: Local3dPointWithUncertaintyEllipsoid_r18UncertaintyAltitude_r18,
    pub confidence_r18: Local3dPointWithUncertaintyEllipsoid_r18Confidence_r18,
    #[asn(optional_idx = 0)]
    pub v_confidence_r18: Option<Local3dPointWithUncertaintyEllipsoid_r18VConfidence_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct LocalOrigin_r18 {
    pub coordinate_id_r18: LocalOrigin_r18CoordinateID_r18,
    #[asn(optional_idx = 0)]
    pub point_r18: Option<HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15>,
    #[asn(optional_idx = 1)]
    pub horiz_axes_orientation_r18: Option<LocalOrigin_r18HorizAxesOrientation_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationCoordinateTypes {
    pub ellipsoid_point: LocationCoordinateTypesEllipsoidPoint,
    pub ellipsoid_point_with_uncertainty_circle:
        LocationCoordinateTypesEllipsoidPointWithUncertaintyCircle,
    pub ellipsoid_point_with_uncertainty_ellipse:
        LocationCoordinateTypesEllipsoidPointWithUncertaintyEllipse,
    pub polygon: LocationCoordinateTypesPolygon,
    pub ellipsoid_point_with_altitude: LocationCoordinateTypesEllipsoidPointWithAltitude,
    pub ellipsoid_point_with_altitude_and_uncertainty_ellipsoid:
        LocationCoordinateTypesEllipsoidPointWithAltitudeAndUncertaintyEllipsoid,
    pub ellipsoid_arc: LocationCoordinateTypesEllipsoidArc,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "6", extensible = true)]
pub enum LocationCoordinates {
    #[asn(key = 0, extended = false)]
    EllipsoidPoint(Ellipsoid_Point),
    #[asn(key = 1, extended = false)]
    EllipsoidPointWithUncertaintyCircle(Ellipsoid_PointWithUncertaintyCircle),
    #[asn(key = 2, extended = false)]
    EllipsoidPointWithUncertaintyEllipse(EllipsoidPointWithUncertaintyEllipse),
    #[asn(key = 3, extended = false)]
    Polygon(Polygon),
    #[asn(key = 4, extended = false)]
    EllipsoidPointWithAltitude(EllipsoidPointWithAltitude),
    #[asn(key = 5, extended = false)]
    EllipsoidPointWithAltitudeAndUncertaintyEllipsoid(
        EllipsoidPointWithAltitudeAndUncertaintyEllipsoid,
    ),
    #[asn(key = 6, extended = false)]
    EllipsoidArc(EllipsoidArc),
    #[asn(key = 0, extended = true)]
    HighAccuracyEllipsoidPointWithUncertaintyEllipse_v1510(
        HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15,
    ),
    #[asn(key = 1, extended = true)]
    HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_v1510(
        HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15,
    ),
    #[asn(key = 2, extended = true)]
    Ha_EllipsoidPointWithScalableUncertaintyEllipse_v1680(
        HA_EllipsoidPointWithScalableUncertaintyEllipse_r16,
    ),
    #[asn(key = 3, extended = true)]
    Ha_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_v1680(
        HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16,
    ),
    #[asn(key = 4, extended = true)]
    Local2dPointWithUncertaintyEllipse_v1800(Local2dPointWithUncertaintyEllipse_r18),
    #[asn(key = 5, extended = true)]
    Local3dPointWithUncertaintyEllipsoid_v1800(Local3dPointWithUncertaintyEllipsoid_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct LocationDataLCI_r14 {
    pub latitude_uncertainty_r14: LocationDataLCI_r14LatitudeUncertainty_r14,
    pub latitude_r14: LocationDataLCI_r14Latitude_r14,
    pub longitude_uncertainty_r14: LocationDataLCI_r14LongitudeUncertainty_r14,
    pub longitude_r14: LocationDataLCI_r14Longitude_r14,
    #[asn(optional_idx = 0)]
    pub altitude_uncertainty_r14: Option<LocationDataLCI_r14AltitudeUncertainty_r14>,
    #[asn(optional_idx = 1)]
    pub altitude_r14: Option<LocationDataLCI_r14Altitude_r14>,
    pub datum_r14: LocationDataLCI_r14Datum_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct LocationError {
    pub locationfailurecause: LocationFailureCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct LocationFailureCause(pub u8);
impl LocationFailureCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const REQUESTED_METHOD_NOT_SUPPORTED: u8 = 1u8;
    pub const POSITION_METHOD_FAILURE: u8 = 2u8;
    pub const PERIODIC_LOCATION_MEASUREMENTS_NOT_AVAILABLE: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct LocationInformationType(pub u8);
impl LocationInformationType {
    pub const LOCATION_ESTIMATE_REQUIRED: u8 = 0u8;
    pub const LOCATION_MEASUREMENTS_REQUIRED: u8 = 1u8;
    pub const LOCATION_ESTIMATE_PREFERRED: u8 = 2u8;
    pub const LOCATION_MEASUREMENTS_PREFERRED: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct LocationSource_r13(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LocationUncertainty_r16 {
    pub horizontal_uncertainty_r16: LocationUncertainty_r16HorizontalUncertainty_r16,
    pub horizontal_confidence_r16: LocationUncertainty_r16HorizontalConfidence_r16,
    pub vertical_uncertainty_r16: LocationUncertainty_r16VerticalUncertainty_r16,
    pub vertical_confidence_r16: LocationUncertainty_r16VerticalConfidence_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct MBS_AcquisitionAssistance_r14 {
    #[asn(optional_idx = 0)]
    pub transmitter_id_r14: Option<MBS_AcquisitionAssistance_r14TransmitterID_r14>,
    #[asn(optional_idx = 1)]
    pub mbs_configuration_r14: Option<MBS_AcquisitionAssistance_r14MbsConfiguration_r14>,
    #[asn(optional_idx = 2)]
    pub pn_code_index_r14: Option<MBS_AcquisitionAssistance_r14PnCodeIndex_r14>,
    #[asn(optional_idx = 3)]
    pub freq_r14: Option<MBS_AcquisitionAssistance_r14Freq_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MBS_AlmanacAssistance_r14 {
    pub transmitter_id_r14: MBS_AlmanacAssistance_r14TransmitterID_r14,
    pub transmitter_latitude_r14: MBS_AlmanacAssistance_r14TransmitterLatitude_r14,
    pub transmitter_longitude_r14: MBS_AlmanacAssistance_r14TransmitterLongitude_r14,
    pub transmitter_altitude_r14: MBS_AlmanacAssistance_r14TransmitterAltitude_r14,
    #[asn(optional_idx = 0)]
    pub time_correction_r14: Option<MBS_AlmanacAssistance_r14TimeCorrection_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MBS_AssistanceDataElement_r14 {
    #[asn(optional_idx = 0)]
    pub mbs_almanac_assistance_r14: Option<MBS_AlmanacAssistance_r14>,
    #[asn(optional_idx = 1)]
    pub mbs_acquisition_assistance_r14: Option<MBS_AcquisitionAssistance_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct MBS_AssistanceDataList_r14(pub Vec<MBS_AssistanceDataElement_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MBS_AssistanceDataSupportList_r14 {
    pub mbs_acquisition_assistance_data_support_r14:
        MBS_AssistanceDataSupportList_r14Mbs_AcquisitionAssistanceDataSupport_r14,
    pub mbs_almanac_assistance_data_support_r14:
        MBS_AssistanceDataSupportList_r14Mbs_AlmanacAssistanceDataSupport_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MBS_BeaconMeasElement_r13 {
    pub transmitter_id_r13: MBS_BeaconMeasElement_r13TransmitterID_r13,
    pub code_phase_r13: MBS_BeaconMeasElement_r13CodePhase_r13,
    pub code_phase_rms_error_r13: MBS_BeaconMeasElement_r13CodePhaseRMSError_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct MBS_BeaconMeasList_r13(pub Vec<MBS_BeaconMeasElement_r13>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct MeasQuantityResults_r16 {
    #[asn(optional_idx = 0)]
    pub nr_rsrp_r16: Option<MeasQuantityResults_r16Nr_RSRP_r16>,
    #[asn(optional_idx = 1)]
    pub nr_rsrq_r16: Option<MeasQuantityResults_r16Nr_RSRQ_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct MeasuredResultsElement {
    pub phys_cell_id: MeasuredResultsElementPhysCellId,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub arfcn_eutra: ARFCN_ValueEUTRA,
    #[asn(optional_idx = 1)]
    pub system_frame_number: Option<MeasuredResultsElementSystemFrameNumber>,
    #[asn(optional_idx = 2)]
    pub rsrp_result: Option<MeasuredResultsElementRsrp_Result>,
    #[asn(optional_idx = 3)]
    pub rsrq_result: Option<MeasuredResultsElementRsrq_Result>,
    #[asn(optional_idx = 4)]
    pub ue_rx_tx_time_diff: Option<MeasuredResultsElementUe_RxTxTimeDiff>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct MeasuredResultsList(pub Vec<MeasuredResultsElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct MeasurementReferenceTime {
    pub gnss_tod_msec: MeasurementReferenceTimeGnss_TOD_msec,
    #[asn(optional_idx = 0)]
    pub gnss_tod_frac: Option<MeasurementReferenceTimeGnss_TOD_frac>,
    #[asn(optional_idx = 1)]
    pub gnss_tod_unc: Option<MeasurementReferenceTimeGnss_TOD_unc>,
    pub gnss_time_id: GNSS_ID,
    #[asn(optional_idx = 2)]
    pub network_time: Option<MeasurementReferenceTimeNetworkTime>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MessageSizeLimitNB_r14 {
    #[asn(optional_idx = 0)]
    pub measurement_limit_r14: Option<MessageSizeLimitNB_r14MeasurementLimit_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct MotionTimeSource_r15 {
    pub time_source_r15: MotionTimeSource_r15TimeSource_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Multi_RTT_MeasCapabilityPerBand_r17 {
    pub freq_band_indicator_nr_r17: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub support_of_dl_prs_first_path_rsrp_r17:
        Option<Multi_RTT_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17>,
    #[asn(optional_idx = 1)]
    pub dl_prs_meas_rrc_inactive_r17:
        Option<Multi_RTT_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NAV_ClockModel {
    pub nav_toc: NAV_ClockModelNavToc,
    pub navaf2: NAV_ClockModelNavaf2,
    pub navaf1: NAV_ClockModelNavaf1,
    pub navaf0: NAV_ClockModelNavaf0,
    pub nav_tgd: NAV_ClockModelNavTgd,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NCGI_r15 {
    pub mcc_r15: NCGI_r15Mcc_r15,
    pub mnc_r15: NCGI_r15Mnc_r15,
    pub nr_cellidentity_r15: NCGI_r15Nr_cellidentity_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct NPRS_Info_r14 {
    pub operation_mode_info_nprs_r14: NPRS_Info_r14OperationModeInfoNPRS_r14,
    #[asn(optional_idx = 0)]
    pub nprs_carrier_r14: Option<CarrierFreq_NB_r14>,
    #[asn(optional_idx = 1)]
    pub nprs_sequence_info_r14: Option<NPRS_Info_r14NprsSequenceInfo_r14>,
    #[asn(optional_idx = 2)]
    pub nprs_id_r14: Option<NPRS_Info_r14NprsID_r14>,
    #[asn(optional_idx = 3)]
    pub part_a_r14: Option<NPRS_Info_r14PartA_r14>,
    #[asn(optional_idx = 4)]
    pub part_b_r14: Option<NPRS_Info_r14PartB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_AdditionalPath_r16 {
    pub nr_relative_time_difference_r16: NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16,
    #[asn(optional_idx = 0)]
    pub nr_path_quality_r16: Option<NR_TimingQuality_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_AdditionalPathList_r16(pub Vec<NR_AdditionalPath_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_AdditionalPathListExt_r17(pub Vec<NR_AdditionalPath_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_AggregatedDL_PRS_ResourceInfo_Element_r18 {
    #[asn(optional_idx = 0)]
    pub aggregated_dl_prs_id_r18:
        Option<NR_AggregatedDL_PRS_ResourceInfo_Element_r18AggregatedDL_PRS_ID_r18>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r18: Option<NR_DL_PRS_ResourceSetID_r16>,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_resource_id_r18: Option<NR_DL_PRS_ResourceID_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_Cell_IDs_r17 {
    #[asn(optional_idx = 0)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    #[asn(optional_idx = 1)]
    pub nr_phys_cell_id_r17: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r17: Option<ARFCN_ValueNR_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_DL_AIML_CapabilityPerBand_r19 {
    pub freq_band_indicator_nr_r19: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub simul_dl_aiml_and_dl_tdoa_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_TDOA_r19>,
    #[asn(optional_idx = 1)]
    pub simul_dl_aiml_and_dl_ao_d_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_AoD_r19>,
    #[asn(optional_idx = 2)]
    pub support_of_dl_prs_bwa_rrc_connected_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Connected_r19>,
    #[asn(optional_idx = 3)]
    pub support_of_dl_prs_bwa_rrc_inactive_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Inactive_r19>,
    #[asn(optional_idx = 4)]
    pub support_of_dl_prs_bwa_rrc_idle_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Idle_r19>,
    #[asn(optional_idx = 5)]
    pub support_of_dl_aiml_pos_rrc_inactive_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Inactive_r19>,
    #[asn(optional_idx = 6)]
    pub support_of_dl_aiml_pos_rrc_idle_r19:
        Option<NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Idle_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_AIML_LocationInformation_r19 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r19:
        Option<NR_DL_AIML_LocationInformation_r19MeasurementReferenceTime_r19>,
    #[asn(optional_idx = 1)]
    pub location_coordinates_r19: Option<LocationCoordinates>,
    #[asn(optional_idx = 2)]
    pub location_source_r19: Option<LocationSource_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_LocationServerErrorCauses_r19 {
    pub cause_r19: NR_DL_AIML_LocationServerErrorCauses_r19Cause_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19 { pub dl_prs_buffer_type_rrc_inactive_r19 : NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19Dl_PRS_BufferType_RRC_Inactive_r19 , pub duration_of_prs_processing_rrc_inactive_r19 : NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19 , pub max_num_of_dl_prs_res_processed_per_slot_rrc_inactive_r19 : NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AIML_PRS_ProcessingCapability_r19 {
    pub nr_dl_aiml_prs_processing_capability_band_list_r19:
        NR_DL_AIML_PRS_ProcessingCapability_r19Nr_dl_aiml_prs_ProcessingCapabilityBandList_r19,
    #[asn(optional_idx = 0)]
    pub multiple_activated_prs_processing_windows_r19:
        Option<NR_DL_AIML_PRS_ProcessingCapability_r19MultipleActivatedPRS_ProcessingWindows_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19 {
    pub supported_bandwidth_prs_r19:
        NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19,
    pub dl_prs_buffer_type_r19: NR_DL_AIML_PRS_ProcessingCapabilityElement_r19Dl_PRS_BufferType_r19,
    pub duration_of_prs_processing_r19:
        NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19,
    pub max_num_of_dl_prs_res_processed_per_slot_r19:
        NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19 {
    pub freq_band_indicator_nr_r19: FreqBandIndicatorNR_r16,
    pub nr_dl_aiml_prs_processing_capability_r19: NR_DL_AIML_PRS_ProcessingCapabilityElement_r19,
    #[asn(optional_idx = 0)]
    pub prs_processing_window_type1_a_r19:
        Option<NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1A_r19>,
    #[asn(optional_idx = 1)]
    pub prs_processing_window_type1_b_r19:
        Option<NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1B_r19>,
    #[asn(optional_idx = 2)]
    pub prs_processing_window_type2_r19:
        Option<NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType2_r19>,
    #[asn(optional_idx = 3)]
    pub prs_processing_capability_outside_m_gin_ppw_r19: Option<
        NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingCapabilityOutsideMGinPPW_r19,
    >,
    #[asn(optional_idx = 4)]
    pub prs_bwa_two_contiguous_intraband_in_mg_rrc_connected_r19:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r19>,
    #[asn(optional_idx = 5)]
    pub prs_bwa_three_contiguous_intraband_in_mg_rrc_connected_r19:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r19>,
    #[asn(optional_idx = 6)]
    pub support_of_prs_bwa_with_two_pfl_combination_r19: Option<
        NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19SupportOfPRS_BWA_WithTwoPFL_Combination_r19,
    >,
    #[asn(optional_idx = 7)]
    pub nr_dl_aiml_prs_processing_capability_rrc_inactive_r19:
        Option<NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19>,
    #[asn(optional_idx = 8)]
    pub prs_bwa_two_contiguous_intraband_rrc_idle_and_inactive_r19:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r19>,
    #[asn(optional_idx = 9)]
    pub prs_bwa_three_contiguous_intraband_rrc_idle_and_inactive_r19:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_AIML_Positioning_Error_r19 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r19(NR_DL_AIML_LocationServerErrorCauses_r19),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r19(NR_DL_AIML_TargetDeviceErrorCauses_r19),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_DL_AIML_ProvideAssistanceData_r19 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_assistance_data_r19: Option<NR_DL_PRS_AssistanceData_r16>,
    #[asn(optional_idx = 1)]
    pub nr_selected_dl_prs_index_list_r19: Option<NR_SelectedDL_PRS_IndexList_r16>,
    #[asn(optional_idx = 2)]
    pub nr_on_demand_dl_prs_configurations_r19: Option<NR_On_Demand_DL_PRS_Configurations_r17>,
    #[asn(optional_idx = 3)]
    pub nr_on_demand_dl_prs_configurations_selected_index_list_r19:
        Option<NR_On_Demand_DL_PRS_Configurations_Selected_IndexList_r17>,
    #[asn(optional_idx = 4)]
    pub assistance_data_validity_area_r19: Option<AreaID_CellList_r17>,
    #[asn(optional_idx = 5)]
    pub nr_position_calculation_assistance_r19: Option<NR_PositionCalculationAssistance_r16>,
    #[asn(optional_idx = 6)]
    pub nr_dl_aiml_positioning_error_r19: Option<NR_DL_AIML_Positioning_Error_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 14)]
pub struct NR_DL_AIML_ProvideCapabilities_r19 {
    #[asn(optional_idx = 0)]
    pub location_coordinate_types_r19: Option<LocationCoordinateTypes>,
    #[asn(optional_idx = 1)]
    pub periodical_reporting_r19: Option<NR_DL_AIML_ProvideCapabilities_r19PeriodicalReporting_r19>,
    #[asn(optional_idx = 2)]
    pub periodic_reporting_interval_ms_support_r19: Option<PeriodicReportingIntervalMsSupport_r18>,
    #[asn(optional_idx = 3)]
    pub ten_ms_unit_response_time_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Ten_ms_unit_ResponseTime_r19>,
    #[asn(optional_idx = 4)]
    pub scheduled_location_request_supported_r19: Option<ScheduledLocationTimeSupport_r17>,
    #[asn(optional_idx = 5)]
    pub nr_pos_calc_assistance_support_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_PosCalcAssistanceSupport_r19>,
    #[asn(optional_idx = 6)]
    pub nr_los_nlos_assistance_data_support_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_los_nlos_AssistanceDataSupport_r19>,
    #[asn(optional_idx = 7)]
    pub nr_dl_prs_expected_ao_d_or_ao_a_sup_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_DL_PRS_ExpectedAoD_or_AoA_Sup_r19>,
    #[asn(optional_idx = 8)]
    pub nr_dl_aiml_on_demand_dl_prs_support_r19: Option<NR_On_Demand_DL_PRS_Support_r17>,
    #[asn(optional_idx = 9)]
    pub nr_dl_aiml_on_demand_dl_prs_for_bwa_support_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_DL_AIML_On_Demand_DL_PRS_ForBWA_Support_r19>,
    #[asn(optional_idx = 10)]
    pub nr_dl_prs_assistance_data_validity_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_dl_prs_AssistanceDataValidity_r19>,
    #[asn(optional_idx = 11)]
    pub multi_location_estimate_in_same_meas_report_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19MultiLocationEstimateInSameMeasReport_r19>,
    #[asn(optional_idx = 12)]
    pub nr_integrity_assistance_support_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_IntegrityAssistanceSupport_r19>,
    #[asn(optional_idx = 13)]
    pub nr_dl_aiml_capability_per_band_list_r19:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_DL_AIML_CapabilityPerBandList_r19>,
    pub nr_dl_aiml_prs_capability_r19: NR_DL_PRS_ResourcesCapability_r16,
    pub nr_dl_aiml_qcl_processing_capability_r19: NR_DL_PRS_QCL_ProcessingCapability_r16,
    pub nr_dl_aiml_prs_processing_capability_r19: NR_DL_AIML_PRS_ProcessingCapability_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_AIML_ProvideLocationInformation_r19 {
    #[asn(optional_idx = 0)]
    pub nr_dl_aiml_location_information_r19: Option<NR_DL_AIML_LocationInformation_r19>,
    #[asn(optional_idx = 1)]
    pub nr_dl_aiml_location_information_instances_r19: Option<
        NR_DL_AIML_ProvideLocationInformation_r19Nr_DL_AIML_LocationInformationInstances_r19,
    >,
    #[asn(optional_idx = 2)]
    pub nr_dl_aiml_positioning_error_r19: Option<NR_DL_AIML_Positioning_Error_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_DL_AIML_RequestAssistanceData_r19 {
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r19: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_assistance_data_req_r19:
        Option<NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_AssistanceDataReq_r19>,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_expected_ao_d_or_ao_a_req_r19:
        Option<NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_ExpectedAoD_or_AoA_Req_r19>,
    #[asn(optional_idx = 3)]
    pub nr_on_demand_dl_prs_req_r19: Option<NR_On_Demand_DL_PRS_Request_r17>,
    #[asn(optional_idx = 4)]
    pub pre_configured_assistance_data_req_r19:
        Option<NR_DL_AIML_RequestAssistanceData_r19Pre_configured_AssistanceDataReq_r19>,
    #[asn(optional_idx = 5)]
    pub nr_position_calculation_assistance_req_r19:
        Option<NR_DL_AIML_RequestAssistanceData_r19Nr_PositionCalculationAssistanceReq_r19>,
    #[asn(optional_idx = 6)]
    pub nr_integrity_assistance_req_r19:
        Option<NR_DL_AIML_RequestAssistanceData_r19Nr_IntegrityAssistanceReq_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_RequestCapabilities_r19 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AIML_RequestLocationInformation_r19 {
    pub nr_assistance_availability_r19:
        NR_DL_AIML_RequestLocationInformation_r19Nr_AssistanceAvailability_r19,
    #[asn(optional_idx = 0)]
    pub multi_location_estimate_in_same_report_r19:
        Option<NR_DL_AIML_RequestLocationInformation_r19MultiLocationEstimateInSameReport_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AIML_TargetDeviceErrorCauses_r19 {
    pub cause_r19: NR_DL_AIML_TargetDeviceErrorCauses_r19Cause_r19,
    #[asn(optional_idx = 0)]
    pub remote_ue_indication_r19:
        Option<NR_DL_AIML_TargetDeviceErrorCauses_r19RemoteUE_Indication_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
    pub nr_dl_prs_rsrp_result_diff_r16:
        NR_DL_AoD_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_rx_beam_index_r16:
        Option<NR_DL_AoD_AdditionalMeasurementElement_r16Nr_DL_PRS_RxBeamIndex_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r17 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_id_r17: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r17: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r17: NR_TimeStamp_r16,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_rsrp_result_diff_r17:
        Option<NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_RSRP_ResultDiff_r17>,
    #[asn(optional_idx = 3)]
    pub nr_dl_prs_rx_beam_index_r17:
        Option<NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_RxBeamIndex_r17>,
    #[asn(optional_idx = 4)]
    pub nr_dl_prs_first_path_rsrp_result_diff_r17:
        Option<NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_FirstPathRSRP_ResultDiff_r17>,
    #[asn(optional_idx = 5)]
    pub nr_los_nlos_indicator_per_resource_r17: Option<LOS_NLOS_Indicator_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "7")]
pub struct NR_DL_AoD_AdditionalMeasurements_r16(
    pub Vec<NR_DL_AoD_AdditionalMeasurementElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "23")]
pub struct NR_DL_AoD_AdditionalMeasurementsExt_r17(
    pub Vec<NR_DL_AoD_AdditionalMeasurementElement_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_AoD_Error_r16 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r16(NR_DL_AoD_LocationServerErrorCauses_r16),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r16(NR_DL_AoD_TargetDeviceErrorCauses_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AoD_LocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r16:
        Option<NR_DL_AoD_LocationInformation_r16MeasurementReferenceTime_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_LocationServerErrorCauses_r16 {
    pub cause_r16: NR_DL_AoD_LocationServerErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_DL_AoD_MeasElement_r16 {
    pub dl_prs_id_r16: NR_DL_AoD_MeasElement_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 4)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
    pub nr_dl_prs_rsrp_result_r16: NR_DL_AoD_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16,
    #[asn(optional_idx = 5)]
    pub nr_dl_prs_rx_beam_index_r16: Option<NR_DL_AoD_MeasElement_r16Nr_DL_PRS_RxBeamIndex_r16>,
    #[asn(optional_idx = 6)]
    pub nr_dl_ao_d_additional_measurements_r16: Option<NR_DL_AoD_AdditionalMeasurements_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_DL_AoD_MeasList_r16(pub Vec<NR_DL_AoD_MeasElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_MeasurementCapability_r16 {
    pub max_dl_prs_rsrp_measurement_fr1_r16:
        NR_DL_AoD_MeasurementCapability_r16MaxDL_PRS_RSRP_MeasurementFR1_r16,
    pub max_dl_prs_rsrp_measurement_fr2_r16:
        NR_DL_AoD_MeasurementCapability_r16MaxDL_PRS_RSRP_MeasurementFR2_r16,
    pub dl_ao_d_meas_capability_band_list_r16:
        NR_DL_AoD_MeasurementCapability_r16Dl_AoD_MeasCapabilityBandList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_AoD_ProvideAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_assistance_data_r16: Option<NR_DL_PRS_AssistanceData_r16>,
    #[asn(optional_idx = 1)]
    pub nr_selected_dl_prs_index_list_r16: Option<NR_SelectedDL_PRS_IndexList_r16>,
    #[asn(optional_idx = 2)]
    pub nr_position_calculation_assistance_r16: Option<NR_PositionCalculationAssistance_r16>,
    #[asn(optional_idx = 3)]
    pub nr_dl_ao_d_error_r16: Option<NR_DL_AoD_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AoD_ProvideCapabilities_r16 {
    pub nr_dl_ao_d_mode_r16: PositioningModes,
    pub nr_dl_ao_d_prs_capability_r16: NR_DL_PRS_ResourcesCapability_r16,
    pub nr_dl_ao_d_measurement_capability_r16: NR_DL_AoD_MeasurementCapability_r16,
    pub nr_dl_prs_qcl_processing_capability_r16: NR_DL_PRS_QCL_ProcessingCapability_r16,
    pub nr_dl_prs_processing_capability_r16: NR_DL_PRS_ProcessingCapability_r16,
    #[asn(optional_idx = 0)]
    pub periodical_reporting_r16: Option<PositioningModes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_AoD_ProvideLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_ao_d_signal_measurement_information_r16:
        Option<NR_DL_AoD_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_ao_d_location_information_r16: Option<NR_DL_AoD_LocationInformation_r16>,
    #[asn(optional_idx = 2)]
    pub nr_dl_ao_d_error_r16: Option<NR_DL_AoD_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AoD_ReportConfig_r16 {
    #[asn(optional_idx = 0)]
    pub max_dl_prs_rsrp_measurements_per_trp_r16:
        Option<NR_DL_AoD_ReportConfig_r16MaxDL_PRS_RSRP_MeasurementsPerTRP_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AoD_RequestAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    pub nr_ad_type_r16: NR_DL_AoD_RequestAssistanceData_r16Nr_AdType_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_RequestCapabilities_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_RequestLocationInformation_r16 {
    pub nr_assistance_availability_r16:
        NR_DL_AoD_RequestLocationInformation_r16Nr_AssistanceAvailability_r16,
    pub nr_dl_ao_d_report_config_r16: NR_DL_AoD_ReportConfig_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_SignalMeasurementInformation_r16 {
    pub nr_dl_ao_d_meas_list_r16: NR_DL_AoD_MeasList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AoD_TargetDeviceErrorCauses_r16 {
    pub cause_r16: NR_DL_AoD_TargetDeviceErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_DL_PRS_AggregationElement_r18 {
    pub nr_dl_prs_frequency_layer_index_r18:
        NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_FrequencyLayerIndex_r18,
    pub nr_dl_prs_trp_index_r18: NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_TRP_Index_r18,
    pub nr_dl_prs_resource_set_index_r18:
        NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_ResourceSetIndex_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_DL_PRS_AggregationInfo_r18(
    pub Vec<NR_linkedDL_PRS_ResourceSetID_PRS_AggregationList_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_PRS_AssistanceData_r16 {
    pub nr_dl_prs_reference_info_r16: DL_PRS_ID_Info_r16,
    pub nr_dl_prs_assistance_data_list_r16:
        NR_DL_PRS_AssistanceData_r16Nr_DL_PRS_AssistanceDataList_r16,
    #[asn(optional_idx = 0)]
    pub nr_ssb_config_r16: Option<NR_DL_PRS_AssistanceData_r16Nr_SSB_Config_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_AssistanceDataPerFreq_r16 {
    pub nr_dl_prs_positioning_frequency_layer_r16: NR_DL_PRS_PositioningFrequencyLayer_r16,
    pub nr_dl_prs_assistance_data_per_freq_r16:
        NR_DL_PRS_AssistanceDataPerFreq_r16Nr_DL_PRS_AssistanceDataPerFreq_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_PRS_AssistanceDataPerTRP_r16 {
    pub dl_prs_id_r16: NR_DL_PRS_AssistanceDataPerTRP_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    pub nr_dl_prs_sfn0_offset_r16: NR_DL_PRS_SFN0_Offset_r16,
    pub nr_dl_prs_expected_rstd_r16: NR_DL_PRS_AssistanceDataPerTRP_r16Nr_DL_PRS_ExpectedRSTD_r16,
    pub nr_dl_prs_expected_rstd_uncertainty_r16:
        NR_DL_PRS_AssistanceDataPerTRP_r16Nr_DL_PRS_ExpectedRSTD_Uncertainty_r16,
    pub nr_dl_prs_info_r16: NR_DL_PRS_Info_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_DL_PRS_BeamInfo_r16(pub Vec<NR_DL_PRS_BeamInfoPerFreqLayer_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_BeamInfoPerFreqLayer_r16(pub Vec<NR_DL_PRS_BeamInfoPerTRP_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_DL_PRS_BeamInfoPerTRP_r16 {
    pub dl_prs_id_r16: NR_DL_PRS_BeamInfoPerTRP_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub associated_dl_prs_id_r16: Option<NR_DL_PRS_BeamInfoPerTRP_r16Associated_DL_PRS_ID_r16>,
    #[asn(optional_idx = 4)]
    pub lcs_gcs_translation_parameter_r16: Option<LCS_GCS_TranslationParameter_r16>,
    #[asn(optional_idx = 5)]
    pub dl_prs_beam_info_set_r16: Option<DL_PRS_BeamInfoSet_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NR_DL_PRS_ExpectedAoD_or_AoA_r17 {
    #[asn(key = 0, extended = false)]
    ExpectedAoD_r17(NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17),
    #[asn(key = 1, extended = false)]
    ExpectedAoA_r17(NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_Assistance_r17(
    pub Vec<NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerFreqLayer_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerFreqLayer_r17(
    pub Vec<NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerResource_r17(pub Vec<LOS_NLOS_Indicator_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17 {
    pub dl_prs_id_r17: NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Dl_PRS_ID_r17,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r17: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r17: Option<ARFCN_ValueNR_r15>,
    pub nr_los_nlos_indicator_r17:
        NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Nr_los_nlos_indicator_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_Info_r16 {
    pub nr_dl_prs_resource_set_list_r16: NR_DL_PRS_Info_r16Nr_DL_PRS_ResourceSetList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfig_r18(
    pub Vec<NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18 {
    pub nr_start_sfn_time_window_r18:
        NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_StartSFN_TimeWindow_r18,
    #[asn(optional_idx = 0)]
    pub nr_periodic_or_one_shot_time_window_r18:
        Option<NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18>,
    #[asn(optional_idx = 1)]
    pub nr_symbol_offset_time_window_r18:
        Option<NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SymbolOffsetTimeWindow_r18>,
    pub nr_duration_time_window_r18:
        NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_DurationTimeWindow_r18,
    #[asn(optional_idx = 2)]
    pub nr_selected_dl_prs_frequency_layer_index_r18: Option<
        NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SelectedDL_PRS_FrequencyLayerIndex_r18,
    >,
    #[asn(optional_idx = 3)]
    pub nr_selected_dl_prs_index_list_per_freq_r18: Option<
        NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SelectedDL_PRS_IndexListPerFreq_r18,
    >,
    #[asn(optional_idx = 4)]
    pub nr_measurements_to_perform_in_time_window_r18: Option<
        NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_MeasurementsToPerformInTimeWindow_r18,
    >,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16 {
    #[asn(key = 0, extended = false)]
    Scs15_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16),
    #[asn(key = 1, extended = false)]
    Scs30_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16),
    #[asn(key = 2, extended = false)]
    Scs60_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16),
    #[asn(key = 3, extended = false)]
    Scs120_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16 {
    pub dl_prs_subcarrier_spacing_r16:
        NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_SubcarrierSpacing_r16,
    pub dl_prs_resource_bandwidth_r16:
        NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_ResourceBandwidth_r16,
    pub dl_prs_start_prb_r16: NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_StartPRB_r16,
    pub dl_prs_point_a_r16: ARFCN_ValueNR_r15,
    pub dl_prs_comb_size_n_r16: NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CombSizeN_r16,
    pub dl_prs_cyclic_prefix_r16: NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CyclicPrefix_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_PRS_ProcessingCapability_r16 {
    pub prs_processing_capability_band_list_r16:
        NR_DL_PRS_ProcessingCapability_r16Prs_ProcessingCapabilityBandList_r16,
    pub max_supported_freq_layers_r16: NR_DL_PRS_ProcessingCapability_r16MaxSupportedFreqLayers_r16,
    #[asn(optional_idx = 0)]
    pub simul_lte_nr_prs_r16: Option<NR_DL_PRS_ProcessingCapability_r16SimulLTE_NR_PRS_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_QCL_ProcessingCapability_r16 {
    pub dl_prs_qcl_processing_capability_band_list_r16:
        NR_DL_PRS_QCL_ProcessingCapability_r16Dl_PRS_QCL_ProcessingCapabilityBandList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_PRS_Resource_r16 {
    pub nr_dl_prs_resource_id_r16: NR_DL_PRS_ResourceID_r16,
    pub dl_prs_sequence_id_r16: NR_DL_PRS_Resource_r16Dl_PRS_SequenceID_r16,
    pub dl_prs_comb_size_n_and_re_offset_r16:
        NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16,
    pub dl_prs_resource_slot_offset_r16: NR_DL_PRS_Resource_r16Dl_PRS_ResourceSlotOffset_r16,
    pub dl_prs_resource_symbol_offset_r16: NR_DL_PRS_Resource_r16Dl_PRS_ResourceSymbolOffset_r16,
    #[asn(optional_idx = 0)]
    pub dl_prs_qcl_info_r16: Option<DL_PRS_QCL_Info_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_ResourceID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_PRS_ResourceSet_r16 {
    pub nr_dl_prs_resource_set_id_r16: NR_DL_PRS_ResourceSetID_r16,
    pub dl_prs_periodicity_and_resource_set_slot_offset_r16:
        NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16,
    #[asn(optional_idx = 0)]
    pub dl_prs_resource_repetition_factor_r16:
        Option<NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceRepetitionFactor_r16>,
    #[asn(optional_idx = 1)]
    pub dl_prs_resource_time_gap_r16: Option<NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceTimeGap_r16>,
    pub dl_prs_num_symbols_r16: NR_DL_PRS_ResourceSet_r16Dl_PRS_NumSymbols_r16,
    #[asn(optional_idx = 2)]
    pub dl_prs_muting_option1_r16: Option<DL_PRS_MutingOption1_r16>,
    #[asn(optional_idx = 3)]
    pub dl_prs_muting_option2_r16: Option<DL_PRS_MutingOption2_r16>,
    pub dl_prs_resource_power_r16: NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourcePower_r16,
    pub dl_prs_resource_list_r16: NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NR_DL_PRS_ResourceSetID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_ResourcesCapability_r16 {
    pub max_nr_of_dl_prs_resource_set_per_trp_per_frequency_layer_r16:
        NR_DL_PRS_ResourcesCapability_r16MaxNrOfDL_PRS_ResourceSetPerTrpPerFrequencyLayer_r16,
    pub max_nr_of_trp_across_freqs_r16: NR_DL_PRS_ResourcesCapability_r16MaxNrOfTRP_AcrossFreqs_r16,
    pub max_nr_of_pos_layer_r16: NR_DL_PRS_ResourcesCapability_r16MaxNrOfPosLayer_r16,
    pub dl_prs_resources_capability_band_list_r16:
        NR_DL_PRS_ResourcesCapability_r16Dl_PRS_ResourcesCapabilityBandList_r16,
    pub dl_prs_resources_band_combination_list_r16: DL_PRS_ResourcesBandCombinationList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_PRS_SFN0_Offset_r16 {
    pub sfn_offset_r16: NR_DL_PRS_SFN0_Offset_r16Sfn_Offset_r16,
    pub integer_subframe_offset_r16: NR_DL_PRS_SFN0_Offset_r16IntegerSubframeOffset_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_DL_PRS_TRP_TEG_Info_r17(pub Vec<NR_DL_PRS_TRP_TEG_InfoPerFreqLayer_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_TRP_TEG_InfoPerFreqLayer_r17(pub Vec<NR_DL_PRS_TRP_TEG_InfoPerTRP_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_PRS_TRP_TEG_InfoPerTRP_r17 {
    pub dl_prs_id_r17: NR_DL_PRS_TRP_TEG_InfoPerTRP_r17Dl_PRS_ID_r17,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r17: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r17: Option<ARFCN_ValueNR_r15>,
    pub dl_prs_teg_info_set_r17: NR_DL_PRS_TRP_TEG_InfoPerTRP_r17Dl_PRS_TEG_InfoSet_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_PRSResourcePriorityItem_r17 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_prio_resource_set_id_r17: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_dl_prs_prio_resource_id_r17: NR_DL_PRS_ResourceID_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
    pub nr_rstd_result_diff_r16: NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16,
    pub nr_timing_quality_r16: NR_TimingQuality_r16,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_rsrp_result_diff_r16:
        Option<NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16>,
    #[asn(optional_idx = 3)]
    pub nr_additional_path_list_r16: Option<NR_AdditionalPathList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_DL_TDOA_AdditionalMeasurements_r16(
    pub Vec<NR_DL_TDOA_AdditionalMeasurementElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "31")]
pub struct NR_DL_TDOA_AdditionalMeasurementsExt_r17(
    pub Vec<NR_DL_TDOA_AdditionalMeasurementElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_TDOA_Error_r16 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r16(NR_DL_TDOA_LocationServerErrorCauses_r16),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r16(NR_DL_TDOA_TargetDeviceErrorCauses_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_TDOA_LocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r16:
        Option<NR_DL_TDOA_LocationInformation_r16MeasurementReferenceTime_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_TDOA_LocationServerErrorCauses_r16 {
    pub cause_r16: NR_DL_TDOA_LocationServerErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct NR_DL_TDOA_MeasElement_r16 {
    pub dl_prs_id_r16: NR_DL_TDOA_MeasElement_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 4)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
    pub nr_rstd_r16: NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16,
    #[asn(optional_idx = 5)]
    pub nr_additional_path_list_r16: Option<NR_AdditionalPathList_r16>,
    pub nr_timing_quality_r16: NR_TimingQuality_r16,
    #[asn(optional_idx = 6)]
    pub nr_dl_prs_rsrp_result_r16: Option<NR_DL_TDOA_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16>,
    #[asn(optional_idx = 7)]
    pub nr_dl_tdoa_additional_measurements_r16: Option<NR_DL_TDOA_AdditionalMeasurements_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_DL_TDOA_MeasList_r16(pub Vec<NR_DL_TDOA_MeasElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_DL_TDOA_MeasurementCapability_r16 {
    pub dl_rstd_measurement_per_pair_of_trp_fr1_r16:
        NR_DL_TDOA_MeasurementCapability_r16Dl_RSTD_MeasurementPerPairOfTRP_FR1_r16,
    pub dl_rstd_measurement_per_pair_of_trp_fr2_r16:
        NR_DL_TDOA_MeasurementCapability_r16Dl_RSTD_MeasurementPerPairOfTRP_FR2_r16,
    #[asn(optional_idx = 0)]
    pub support_of_dl_prs_rsrp_meas_fr1_r16:
        Option<NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR1_r16>,
    #[asn(optional_idx = 1)]
    pub support_of_dl_prs_rsrp_meas_fr2_r16:
        Option<NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR2_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_TDOA_ProvideAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_assistance_data_r16: Option<NR_DL_PRS_AssistanceData_r16>,
    #[asn(optional_idx = 1)]
    pub nr_selected_dl_prs_index_list_r16: Option<NR_SelectedDL_PRS_IndexList_r16>,
    #[asn(optional_idx = 2)]
    pub nr_position_calculation_assistance_r16: Option<NR_PositionCalculationAssistance_r16>,
    #[asn(optional_idx = 3)]
    pub nr_dl_tdoa_error_r16: Option<NR_DL_TDOA_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_DL_TDOA_ProvideCapabilities_r16 {
    pub nr_dl_tdoa_mode_r16: PositioningModes,
    pub nr_dl_tdoa_prs_capability_r16: NR_DL_PRS_ResourcesCapability_r16,
    pub nr_dl_tdoa_measurement_capability_r16: NR_DL_TDOA_MeasurementCapability_r16,
    pub nr_dl_prs_qcl_processing_capability_r16: NR_DL_PRS_QCL_ProcessingCapability_r16,
    pub nr_dl_prs_processing_capability_r16: NR_DL_PRS_ProcessingCapability_r16,
    #[asn(optional_idx = 0)]
    pub additional_paths_report_r16:
        Option<NR_DL_TDOA_ProvideCapabilities_r16AdditionalPathsReport_r16>,
    #[asn(optional_idx = 1)]
    pub periodical_reporting_r16: Option<PositioningModes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_TDOA_ProvideLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_tdoa_signal_measurement_information_r16:
        Option<NR_DL_TDOA_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_tdoa_location_information_r16: Option<NR_DL_TDOA_LocationInformation_r16>,
    #[asn(optional_idx = 2)]
    pub nr_dl_tdoa_error_r16: Option<NR_DL_TDOA_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_DL_TDOA_ReportConfig_r16 {
    #[asn(optional_idx = 0)]
    pub max_dl_prs_rstd_measurements_per_trp_pair_r16:
        Option<NR_DL_TDOA_ReportConfig_r16MaxDL_PRS_RSTD_MeasurementsPerTRP_Pair_r16>,
    #[asn(optional_idx = 1)]
    pub timing_reporting_granularity_factor_r16:
        Option<NR_DL_TDOA_ReportConfig_r16TimingReportingGranularityFactor_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_TDOA_RequestAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    pub nr_ad_type_r16: NR_DL_TDOA_RequestAssistanceData_r16Nr_AdType_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_TDOA_RequestCapabilities_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_DL_TDOA_RequestLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_rstd_measurement_info_request_r16:
        Option<NR_DL_TDOA_RequestLocationInformation_r16Nr_DL_PRS_RstdMeasurementInfoRequest_r16>,
    pub nr_requested_measurements_r16:
        NR_DL_TDOA_RequestLocationInformation_r16Nr_RequestedMeasurements_r16,
    pub nr_assistance_availability_r16:
        NR_DL_TDOA_RequestLocationInformation_r16Nr_AssistanceAvailability_r16,
    #[asn(optional_idx = 1)]
    pub nr_dl_tdoa_report_config_r16: Option<NR_DL_TDOA_ReportConfig_r16>,
    #[asn(optional_idx = 2)]
    pub additional_paths_r16: Option<NR_DL_TDOA_RequestLocationInformation_r16AdditionalPaths_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_TDOA_SignalMeasurementInformation_r16 {
    pub dl_prs_reference_info_r16: DL_PRS_ID_Info_r16,
    pub nr_dl_tdoa_meas_list_r16: NR_DL_TDOA_MeasList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_TDOA_TargetDeviceErrorCauses_r16 {
    pub cause_r16: NR_DL_TDOA_TargetDeviceErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_ECID_Error_r16 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r16(NR_ECID_LocationServerErrorCauses_r16),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r16(NR_ECID_TargetDeviceErrorCauses_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_ECID_LocationServerErrorCauses_r16 {
    pub cause_r16: NR_ECID_LocationServerErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_ECID_ProvideCapabilities_r16 {
    pub nr_ecid_meas_supported_r16: NR_ECID_ProvideCapabilities_r16Nr_ECID_MeasSupported_r16,
    #[asn(optional_idx = 0)]
    pub periodical_reporting_r16: Option<NR_ECID_ProvideCapabilities_r16PeriodicalReporting_r16>,
    #[asn(optional_idx = 1)]
    pub triggered_reporting_r16: Option<NR_ECID_ProvideCapabilities_r16TriggeredReporting_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_ECID_ProvideLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_ecid_signal_measurement_information_r16:
        Option<NR_ECID_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 1)]
    pub nr_ecid_error_r16: Option<NR_ECID_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_ECID_RequestCapabilities_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_ECID_RequestLocationInformation_r16 {
    pub requested_measurements_r16: NR_ECID_RequestLocationInformation_r16RequestedMeasurements_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_ECID_SignalMeasurementInformation_r16 {
    pub nr_primary_cell_measured_results_r16: NR_MeasuredResultsElement_r16,
    #[asn(optional_idx = 0)]
    pub nr_measured_results_list_r16: Option<NR_MeasuredResultsList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_ECID_TargetDeviceErrorCauses_r16 {
    pub cause_r16: NR_ECID_TargetDeviceErrorCauses_r16Cause_r16,
    #[asn(optional_idx = 0)]
    pub ss_rsrp_measurement_not_possible_r16:
        Option<NR_ECID_TargetDeviceErrorCauses_r16Ss_RSRPMeasurementNotPossible_r16>,
    #[asn(optional_idx = 1)]
    pub ss_rsrq_measurement_not_possible_r16:
        Option<NR_ECID_TargetDeviceErrorCauses_r16Ss_RSRQMeasurementNotPossible_r16>,
    #[asn(optional_idx = 2)]
    pub csi_rsrp_measurement_not_possible_r16:
        Option<NR_ECID_TargetDeviceErrorCauses_r16Csi_RSRPMeasurementNotPossible_r16>,
    #[asn(optional_idx = 3)]
    pub csi_rsrq_measurement_not_possible_r16:
        Option<NR_ECID_TargetDeviceErrorCauses_r16Csi_RSRQMeasurementNotPossible_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityBeamInfoBounds_r18 {
    pub mean_azimuth_r18: NR_IntegrityBeamInfoBounds_r18MeanAzimuth_r18,
    pub std_dev_azimuth_r18: NR_IntegrityBeamInfoBounds_r18StdDevAzimuth_r18,
    pub mean_elevation_r18: NR_IntegrityBeamInfoBounds_r18MeanElevation_r18,
    pub std_dev_elevation_r18: NR_IntegrityBeamInfoBounds_r18StdDevElevation_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityBeamPowerBounds_r18 {
    pub mean_beam_power_r18: NR_IntegrityBeamPowerBounds_r18MeanBeamPower_r18,
    pub std_dev_beam_power_r18: NR_IntegrityBeamPowerBounds_r18StdDevBeamPower_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityLocationBounds_r18 {
    pub units_r18: NR_IntegrityLocationBounds_r18Units_r18,
    pub mean_location_error_bound_r18: NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18,
    pub std_dev_location_error_bound_r18:
        NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityParametersDL_PRS_BeamInfo_r18 {
    pub dl_prs_beam_info_error_correlation_time_r18:
        NR_IntegrityParametersDL_PRS_BeamInfo_r18Dl_PRS_BeamInfoErrorCorrelationTime_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityParametersRTD_Info_r18 {
    pub rtd_error_correlation_time_r18:
        NR_IntegrityParametersRTD_Info_r18Rtd_ErrorCorrelationTime_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityParametersTRP_BeamAntennaInfo_r18 {
    pub trp_beam_antenna_info_error_correlation_time_r18:
        NR_IntegrityParametersTRP_BeamAntennaInfo_r18Trp_BeamAntennaInfoErrorCorrelationTime_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_IntegrityParametersTRP_LocationInfo_r18 {
    #[asn(optional_idx = 0)]
    pub trp_error_correlation_time_r18:
        Option<NR_IntegrityParametersTRP_LocationInfo_r18Trp_ErrorCorrelationTime_r18>,
    #[asn(optional_idx = 1)]
    pub dl_prs_resource_set_arp_error_correlation_time_r18: Option<
        NR_IntegrityParametersTRP_LocationInfo_r18Dl_PRS_ResourceSetARP_ErrorCorrelationTime_r18,
    >,
    #[asn(optional_idx = 2)]
    pub dl_prs_resource_arp_error_correlation_time_r18: Option<
        NR_IntegrityParametersTRP_LocationInfo_r18Dl_PRS_ResourceARP_ErrorCorrelationTime_r18,
    >,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityRTD_InfoBounds_r18 {
    pub resolution_r18: NR_IntegrityRTD_InfoBounds_r18Resolution_r18,
    pub mean_rtd_r18: NR_IntegrityRTD_InfoBounds_r18MeanRTD_r18,
    pub std_dev_rtd_r18: NR_IntegrityRTD_InfoBounds_r18StdDevRTD_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityRiskParameters_r18 {
    pub nr_prob_onset_trp_fault_r18: NR_IntegrityRiskParameters_r18Nr_ProbOnsetTRP_Fault_r18,
    pub nr_mean_trp_fault_duration_r18: NR_IntegrityRiskParameters_r18Nr_MeanTRP_FaultDuration_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_IntegrityServiceAlert_r18(pub Vec<NR_TRP_IntegrityServiceAlertPerFreqLayer_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_IntegrityServiceParameters_r18 {
    pub ir_minimum_r18: NR_IntegrityServiceParameters_r18Ir_Minimum_r18,
    pub ir_maximum_r18: NR_IntegrityServiceParameters_r18Ir_Maximum_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_MeasuredResultsElement_r16 {
    pub nr_phys_cell_id_r16: NR_PhysCellID_r16,
    pub nr_arfcn_r16: NR_MeasuredResultsElement_r16Nr_ARFCN_r16,
    #[asn(optional_idx = 0)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 1)]
    pub system_frame_number_r16: Option<NR_MeasuredResultsElement_r16SystemFrameNumber_r16>,
    #[asn(optional_idx = 2)]
    pub results_ssb_cell_r16: Option<MeasQuantityResults_r16>,
    #[asn(optional_idx = 3)]
    pub results_csi_rs_cell_r16: Option<MeasQuantityResults_r16>,
    #[asn(optional_idx = 4)]
    pub results_ssb_indexes_r16: Option<ResultsPerSSB_IndexList_r16>,
    #[asn(optional_idx = 5)]
    pub results_csi_rs_indexes_r16: Option<ResultsPerCSI_RS_IndexList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NR_MeasuredResultsList_r16(pub Vec<NR_MeasuredResultsElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    #[asn(optional_idx = 2)]
    pub nr_dl_prs_rsrp_result_diff_r16:
        Option<NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16>,
    pub nr_ue_rx_tx_time_diff_additional_r16:
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16,
    pub nr_timing_quality_r16: NR_TimingQuality_r16,
    #[asn(optional_idx = 3)]
    pub nr_additional_path_list_r16: Option<NR_AdditionalPathList_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_Multi_RTT_AdditionalMeasurements_r16(
    pub Vec<NR_Multi_RTT_AdditionalMeasurementElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "31")]
pub struct NR_Multi_RTT_AdditionalMeasurementsExt_r17(
    pub Vec<NR_Multi_RTT_AdditionalMeasurementElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_Multi_RTT_Error_r16 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r16(NR_Multi_RTT_LocationServerErrorCauses_r16),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r16(NR_Multi_RTT_TargetDeviceErrorCauses_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_Multi_RTT_LocationServerErrorCauses_r16 {
    pub cause_r16: NR_Multi_RTT_LocationServerErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct NR_Multi_RTT_MeasElement_r16 {
    pub dl_prs_id_r16: NR_Multi_RTT_MeasElement_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub nr_dl_prs_resource_id_r16: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 4)]
    pub nr_dl_prs_resource_set_id_r16: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_ue_rx_tx_time_diff_r16: NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16,
    #[asn(optional_idx = 5)]
    pub nr_additional_path_list_r16: Option<NR_AdditionalPathList_r16>,
    pub nr_time_stamp_r16: NR_TimeStamp_r16,
    pub nr_timing_quality_r16: NR_TimingQuality_r16,
    #[asn(optional_idx = 6)]
    pub nr_dl_prs_rsrp_result_r16: Option<NR_Multi_RTT_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16>,
    #[asn(optional_idx = 7)]
    pub nr_multi_rtt_additional_measurements_r16: Option<NR_Multi_RTT_AdditionalMeasurements_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_Multi_RTT_MeasList_r16(pub Vec<NR_Multi_RTT_MeasElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_Multi_RTT_MeasurementCapability_r16 {
    #[asn(optional_idx = 0)]
    pub max_nr_of_rx_tx_meas_fr1_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16MaxNrOfRx_TX_MeasFR1_r16>,
    #[asn(optional_idx = 1)]
    pub max_nr_of_rx_tx_meas_fr2_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16MaxNrOfRx_TX_MeasFR2_r16>,
    #[asn(optional_idx = 2)]
    pub support_of_rsrp_meas_fr1_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR1_r16>,
    #[asn(optional_idx = 3)]
    pub support_of_rsrp_meas_fr2_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR2_r16>,
    #[asn(optional_idx = 4)]
    pub srs_assoc_prs_multi_layers_fr1_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR1_r16>,
    #[asn(optional_idx = 5)]
    pub srs_assoc_prs_multi_layers_fr2_r16:
        Option<NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR2_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_Multi_RTT_ProvideAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_dl_prs_assistance_data_r16: Option<NR_DL_PRS_AssistanceData_r16>,
    #[asn(optional_idx = 1)]
    pub nr_selected_dl_prs_index_list_r16: Option<NR_SelectedDL_PRS_IndexList_r16>,
    #[asn(optional_idx = 2)]
    pub nr_multi_rtt_error_r16: Option<NR_Multi_RTT_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_Multi_RTT_ProvideCapabilities_r16 {
    pub nr_multi_rtt_prs_capability_r16: NR_DL_PRS_ResourcesCapability_r16,
    pub nr_multi_rtt_measurement_capability_r16: NR_Multi_RTT_MeasurementCapability_r16,
    pub nr_dl_prs_qcl_processing_capability_r16: NR_DL_PRS_QCL_ProcessingCapability_r16,
    pub nr_dl_prs_processing_capability_r16: NR_DL_PRS_ProcessingCapability_r16,
    pub nr_ul_srs_capability_r16: NR_UL_SRS_Capability_r16,
    #[asn(optional_idx = 0)]
    pub additional_paths_report_r16:
        Option<NR_Multi_RTT_ProvideCapabilities_r16AdditionalPathsReport_r16>,
    #[asn(optional_idx = 1)]
    pub periodical_reporting_r16:
        Option<NR_Multi_RTT_ProvideCapabilities_r16PeriodicalReporting_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_Multi_RTT_ProvideLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_multi_rtt_signal_measurement_information_r16:
        Option<NR_Multi_RTT_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 1)]
    pub nr_multi_rtt_error_r16: Option<NR_Multi_RTT_Error_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NR_Multi_RTT_ReportConfig_r16 {
    #[asn(optional_idx = 0)]
    pub max_dl_prs_rx_tx_time_diff_meas_per_trp_r16:
        Option<NR_Multi_RTT_ReportConfig_r16MaxDL_PRS_RxTxTimeDiffMeasPerTRP_r16>,
    #[asn(optional_idx = 1)]
    pub timing_reporting_granularity_factor_r16:
        Option<NR_Multi_RTT_ReportConfig_r16TimingReportingGranularityFactor_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_Multi_RTT_RequestAssistanceData_r16 {
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    pub nr_ad_type_r16: NR_Multi_RTT_RequestAssistanceData_r16Nr_AdType_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_Multi_RTT_RequestCapabilities_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_Multi_RTT_RequestLocationInformation_r16 {
    #[asn(optional_idx = 0)]
    pub nr_ue_rx_tx_time_diff_measurement_info_request_r16: Option<
        NR_Multi_RTT_RequestLocationInformation_r16Nr_UE_RxTxTimeDiffMeasurementInfoRequest_r16,
    >,
    pub nr_requested_measurements_r16:
        NR_Multi_RTT_RequestLocationInformation_r16Nr_RequestedMeasurements_r16,
    pub nr_assistance_availability_r16:
        NR_Multi_RTT_RequestLocationInformation_r16Nr_AssistanceAvailability_r16,
    pub nr_multi_rtt_report_config_r16: NR_Multi_RTT_ReportConfig_r16,
    #[asn(optional_idx = 1)]
    pub additional_paths_r16:
        Option<NR_Multi_RTT_RequestLocationInformation_r16AdditionalPaths_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_Multi_RTT_SignalMeasurementInformation_r16 {
    pub nr_multi_rtt_meas_list_r16: NR_Multi_RTT_MeasList_r16,
    #[asn(optional_idx = 0)]
    pub nr_nta_offset_r16: Option<NR_Multi_RTT_SignalMeasurementInformation_r16Nr_NTA_Offset_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_Multi_RTT_TargetDeviceErrorCauses_r16 {
    pub cause_r16: NR_Multi_RTT_TargetDeviceErrorCauses_r16Cause_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_MutingPattern_r16 {
    #[asn(key = 0, extended = false)]
    Po2_r16(NR_MutingPattern_r16_po2_r16),
    #[asn(key = 1, extended = false)]
    Po4_r16(NR_MutingPattern_r16_po4_r16),
    #[asn(key = 2, extended = false)]
    Po6_r16(NR_MutingPattern_r16_po6_r16),
    #[asn(key = 3, extended = false)]
    Po8_r16(NR_MutingPattern_r16_po8_r16),
    #[asn(key = 4, extended = false)]
    Po16_r16(NR_MutingPattern_r16_po16_r16),
    #[asn(key = 5, extended = false)]
    Po32_r16(NR_MutingPattern_r16_po32_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_NTN_UE_RxTxMeasurements_r18 {
    pub nr_ntn_ue_rx_tx_time_diff_subframe_offset_r18:
        NR_NTN_UE_RxTxMeasurements_r18Nr_NTN_UE_RxTxTimeDiffSubframeOffset_r18,
    pub nr_ntn_dl_timing_drift_r18: NR_NTN_UE_RxTxMeasurements_r18Nr_NTN_DL_TimingDrift_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_On_Demand_DL_PRS_Configurations_Selected_IndexList_r17(
    pub Vec<DL_PRS_Configuration_ID_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_On_Demand_DL_PRS_Configurations_r17 {
    pub on_demand_dl_prs_configuration_list_r17:
        NR_On_Demand_DL_PRS_Configurations_r17On_demand_dl_prs_configuration_list_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_On_Demand_DL_PRS_Information_r17(pub Vec<NR_On_Demand_DL_PRS_PerFreqLayer_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17 {
    pub dl_prs_frequency_range_req_r17:
        NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_FrequencyRangeReq_r17,
    #[asn(optional_idx = 0)]
    pub dl_prs_resource_set_periodicity_req_r17:
        Option<NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceSetPeriodicityReq_r17>,
    #[asn(optional_idx = 1)]
    pub dl_prs_resource_bandwidth_req_r17:
        Option<NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceBandwidthReq_r17>,
    #[asn(optional_idx = 2)]
    pub dl_prs_resource_repetition_factor_req_r17:
        Option<NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceRepetitionFactorReq_r17>,
    #[asn(optional_idx = 3)]
    pub dl_prs_num_symbols_req_r17:
        Option<NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_NumSymbolsReq_r17>,
    #[asn(optional_idx = 4)]
    pub dl_prs_comb_size_n_req_r17:
        Option<NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_CombSizeN_Req_r17>,
    #[asn(optional_idx = 5)]
    pub dl_prs_qcl_information_req_tr_plist_r17: Option<DL_PRS_QCL_InformationReqTRPlist_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_On_Demand_DL_PRS_Request_r17 {
    #[asn(optional_idx = 0)]
    pub dl_prs_start_time_and_duration_r17: Option<DL_PRS_StartTime_and_Duration_r17>,
    #[asn(optional_idx = 1)]
    pub nr_on_demand_dl_prs_information_r17: Option<NR_On_Demand_DL_PRS_Information_r17>,
    #[asn(optional_idx = 2)]
    pub dl_prs_configuration_id_pref_list_r17:
        Option<NR_On_Demand_DL_PRS_Request_r17Dl_prs_configuration_id_PrefList_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_On_Demand_DL_PRS_Support_r17 {
    #[asn(optional_idx = 0)]
    pub nr_on_demand_dl_prs_information_sup_r17:
        Option<NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_InformationSup_r17>,
    #[asn(optional_idx = 1)]
    pub nr_on_demand_dl_prs_configurations_sup_r17:
        Option<NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_ConfigurationsSup_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct NR_OnDemandDL_PRS_AggregationReqElement_r18(
    pub Vec<NR_OnDemandDL_PRS_AggregationReqElement_r18_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_PRU_DL_Info_r18 {
    #[asn(optional_idx = 0)]
    pub nr_pru_location_info_r18: Option<LocationCoordinates>,
    #[asn(optional_idx = 1)]
    pub nr_pru_dl_tdoa_meas_info_r18: Option<NR_DL_TDOA_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 2)]
    pub nr_pru_dl_ao_d_meas_info_r18: Option<NR_DL_AoD_SignalMeasurementInformation_r16>,
    #[asn(optional_idx = 3)]
    pub nr_pru_rscp_meas_info_r18: Option<NR_PRU_RSCP_MeasurementInformation_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18 { # [asn (optional_idx = 0 ,)] pub nr_dl_prs_resource_id_r18 : Option < NR_DL_PRS_ResourceID_r16 > , # [asn (optional_idx = 1 ,)] pub nr_dl_prs_resource_set_id_r18 : Option < NR_DL_PRS_ResourceSetID_r16 > , # [asn (optional_idx = 2 ,)] pub nr_dl_prs_rsrp_result_diff_r18 : Option < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_DL_PRS_RSRP_ResultDiff_r18 > , # [asn (optional_idx = 3 ,)] pub nr_dl_prs_first_path_rsrp_result_diff_r18 : Option < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_DL_PRS_FirstPathRSRP_ResultDiff_r18 > , # [asn (optional_idx = 4 ,)] pub nr_pru_rscp_additional_measurements_list_r18 : Option < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSCP_AdditionalMeasurementsList_r18 > , # [asn (optional_idx = 5 ,)] pub nr_pru_rsrp_diff_additional_measurements_list_r18 : Option < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSRPDiff_AdditionalMeasurementsList_r18 > , # [asn (optional_idx = 6 ,)] pub nr_pru_first_path_rsrp_result_diff_additional_measurements_list_r18 : Option < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AdditionalMeasurementsList_r18 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_PRU_RSCP_AdditionalMeasurements_r18(
    pub Vec<NR_PRU_RSCP_AdditionalMeasurementElement_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 14)]
pub struct NR_PRU_RSCP_MeasElement_r18 {
    pub dl_prs_id_r18: NR_PRU_RSCP_MeasElement_r18Dl_PRS_ID_r18,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r18: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r18: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r18: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub nr_dl_prs_resource_id_r18: Option<NR_DL_PRS_ResourceID_r16>,
    #[asn(optional_idx = 4)]
    pub nr_dl_prs_resource_set_id_r18: Option<NR_DL_PRS_ResourceSetID_r16>,
    pub nr_time_stamp_r18: NR_TimeStamp_r16,
    #[asn(optional_idx = 5)]
    pub nr_los_nlos_indicator_r18: Option<NR_PRU_RSCP_MeasElement_r18Nr_los_nlos_Indicator_r18>,
    #[asn(optional_idx = 6)]
    pub nr_rscp_r18: Option<NR_PRU_RSCP_MeasElement_r18Nr_RSCP_r18>,
    #[asn(optional_idx = 7)]
    pub nr_dl_prs_rsrp_result_r18: Option<NR_PRU_RSCP_MeasElement_r18Nr_DL_PRS_RSRP_Result_r18>,
    #[asn(optional_idx = 8)]
    pub nr_dl_prs_first_path_rsrp_result_r18:
        Option<NR_PRU_RSCP_MeasElement_r18Nr_DL_PRS_FirstPathRSRP_Result_r18>,
    #[asn(optional_idx = 9)]
    pub nr_phase_quality_r18: Option<NR_PhaseQuality_r18>,
    #[asn(optional_idx = 10)]
    pub nr_pru_rscp_add_sample_measurements_r18:
        Option<NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSCP_AddSampleMeasurements_r18>,
    #[asn(optional_idx = 11)]
    pub nr_pru_rsrp_add_sample_measurements_r18:
        Option<NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSRP_AddSampleMeasurements_r18>,
    #[asn(optional_idx = 12)]
    pub nr_pru_first_path_rsrp_result_diff_add_sample_measurements_r18: Option<
        NR_PRU_RSCP_MeasElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AddSampleMeasurements_r18,
    >,
    #[asn(optional_idx = 13)]
    pub nr_pru_rscp_additional_measurements_r18: Option<NR_PRU_RSCP_AdditionalMeasurements_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_PRU_RSCP_MeasurementInformation_r18(pub Vec<NR_PRU_RSCP_MeasElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_PeriodicAssistData_r18 {
    #[asn(optional_idx = 0)]
    pub nr_periodic_pru_dl_info_r18: Option<NR_PeriodicControlParam_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_PeriodicAssistDataReq_r18 {
    #[asn(optional_idx = 0)]
    pub nr_periodic_pru_dl_info_req_r18: Option<NR_PeriodicControlParam_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_PeriodicControlParam_r18 {
    pub delivery_amount_r18: NR_PeriodicControlParam_r18DeliveryAmount_r18,
    pub delivery_interval_r18: NR_PeriodicControlParam_r18DeliveryInterval_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_PhaseQuality_r18 {
    pub phase_quality_index_r18: NR_PhaseQuality_r18PhaseQualityIndex_r18,
    pub phase_quality_resolution_r18: NR_PhaseQuality_r18PhaseQualityResolution_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct NR_PhysCellID_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_PositionCalculationAssistance_r16 {
    #[asn(optional_idx = 0)]
    pub nr_trp_location_info_r16: Option<NR_TRP_LocationInfo_r16>,
    #[asn(optional_idx = 1)]
    pub nr_dl_prs_beam_info_r16: Option<NR_DL_PRS_BeamInfo_r16>,
    #[asn(optional_idx = 2)]
    pub nr_rtd_info_r16: Option<NR_RTD_Info_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_RSCP_AdditionalMeasurements_r18 {
    #[asn(optional_idx = 0)]
    pub nr_rscp_r18: Option<NR_RSCP_AdditionalMeasurements_r18Nr_RSCP_r18>,
    #[asn(optional_idx = 1)]
    pub nr_phase_quality_r18: Option<NR_PhaseQuality_r18>,
    #[asn(optional_idx = 2)]
    pub nr_time_stamp_r18: Option<NR_TimeStamp_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_RSCPD_AdditionalMeasurementSamplesElement_r18 {
    #[asn(optional_idx = 0)]
    pub nr_rscpd_r18: Option<NR_RSCPD_AdditionalMeasurementSamplesElement_r18Nr_RSCPD_r18>,
    #[asn(optional_idx = 1)]
    pub nr_phase_quality_r18: Option<NR_PhaseQuality_r18>,
    #[asn(optional_idx = 2)]
    pub nr_time_stamp_r18: Option<NR_TimeStamp_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_RTD_Info_r16 {
    pub reference_trp_rtd_info_r16: ReferenceTRP_RTD_Info_r16,
    pub rtd_info_list_r16: RTD_InfoList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_SRS_TxTEG_Element_r17 {
    #[asn(optional_idx = 0)]
    pub nr_time_stamp_r17: Option<NR_TimeStamp_r16>,
    pub nr_ue_tx_teg_id_r17: NR_SRS_TxTEG_Element_r17Nr_UE_Tx_TEG_ID_r17,
    #[asn(optional_idx = 1)]
    pub carrier_freq_r17: Option<NR_SRS_TxTEG_Element_r17CarrierFreq_r17>,
    pub srs_pos_resource_list_r17: NR_SRS_TxTEG_Element_r17Srs_PosResourceList_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_SSB_Config_r16 {
    pub nr_phys_cell_id_r16: NR_PhysCellID_r16,
    pub nr_arfcn_r16: ARFCN_ValueNR_r15,
    pub ss_pbch_block_power_r16: NR_SSB_Config_r16Ss_PBCH_BlockPower_r16,
    pub half_frame_index_r16: NR_SSB_Config_r16HalfFrameIndex_r16,
    pub ssb_periodicity_r16: NR_SSB_Config_r16Ssb_periodicity_r16,
    #[asn(optional_idx = 0)]
    pub ssb_positions_in_burst_r16: Option<NR_SSB_Config_r16Ssb_PositionsInBurst_r16>,
    pub ssb_subcarrier_spacing_r16: NR_SSB_Config_r16Ssb_SubcarrierSpacing_r16,
    pub sfn_ssb_offset_r16: NR_SSB_Config_r16Sfn_SSB_Offset_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_SelectedDL_PRS_IndexList_r16(pub Vec<NR_SelectedDL_PRS_PerFreq_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r16 {
    pub nr_selected_trp_index_r16: NR_SelectedDL_PRS_IndexPerTRP_r16Nr_SelectedTRP_Index_r16,
    #[asn(optional_idx = 0)]
    pub dl_selected_prs_resource_set_index_list_r16:
        Option<NR_SelectedDL_PRS_IndexPerTRP_r16Dl_SelectedPRS_ResourceSetIndexList_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r18 {
    pub nr_selected_trp_index_r18: NR_SelectedDL_PRS_IndexPerTRP_r18Nr_SelectedTRP_Index_r18,
    #[asn(optional_idx = 0)]
    pub dl_selected_prs_resource_set_index_list_r18:
        Option<NR_SelectedDL_PRS_IndexPerTRP_r18Dl_SelectedPRS_ResourceSetIndexList_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_SelectedDL_PRS_PerFreq_r16 {
    pub nr_selected_dl_prs_frequency_layer_index_r16:
        NR_SelectedDL_PRS_PerFreq_r16Nr_SelectedDL_PRS_FrequencyLayerIndex_r16,
    #[asn(optional_idx = 0)]
    pub nr_selected_dl_prs_index_list_per_freq_r16:
        Option<NR_SelectedDL_PRS_PerFreq_r16Nr_SelectedDL_PRS_IndexListPerFreq_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "3600"
)]
pub struct NR_TRP_BeamAntennaAngles_r17(pub Vec<NR_TRP_BeamAntennaInfoAzimuthElevation_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_TRP_BeamAntennaInfo_r17(pub Vec<NR_TRP_BeamAntennaInfoPerFreqLayer_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NR_TRP_BeamAntennaInfoAzimuthElevation_r17 {
    #[asn(optional_idx = 0)]
    pub azimuth_r17: Option<NR_TRP_BeamAntennaInfoAzimuthElevation_r17Azimuth_r17>,
    #[asn(optional_idx = 1)]
    pub azimuth_fine_r17: Option<NR_TRP_BeamAntennaInfoAzimuthElevation_r17Azimuth_fine_r17>,
    pub elevation_list_r17: NR_TRP_BeamAntennaInfoAzimuthElevation_r17ElevationList_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_TRP_BeamAntennaInfoPerFreqLayer_r17(pub Vec<NR_TRP_BeamAntennaInfoPerTRP_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct NR_TRP_BeamAntennaInfoPerTRP_r17 {
    pub dl_prs_id_r17: NR_TRP_BeamAntennaInfoPerTRP_r17Dl_PRS_ID_r17,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r17: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r17: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub associated_dl_prs_id_r17: Option<NR_TRP_BeamAntennaInfoPerTRP_r17Associated_DL_PRS_ID_r17>,
    #[asn(optional_idx = 4)]
    pub lcs_gcs_translation_parameter_r17: Option<LCS_GCS_TranslationParameter_r16>,
    #[asn(optional_idx = 5)]
    pub nr_trp_beam_antenna_angles_r17: Option<NR_TRP_BeamAntennaAngles_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct NR_TRP_IntegrityServiceAlertElement_r18 {
    pub dl_prs_id_r18: NR_TRP_IntegrityServiceAlertElement_r18Dl_PRS_ID_r18,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r18: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r18: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r18: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub rtd_do_not_use_r18: Option<NR_TRP_IntegrityServiceAlertElement_r18Rtd_DoNotUse_r18>,
    #[asn(optional_idx = 4)]
    pub trp_location_do_not_use_r18:
        Option<NR_TRP_IntegrityServiceAlertElement_r18Trp_LocationDoNotUse_r18>,
    #[asn(optional_idx = 5)]
    pub beam_info_do_not_use_r18:
        Option<NR_TRP_IntegrityServiceAlertElement_r18BeamInfo_DoNotUse_r18>,
    #[asn(optional_idx = 6)]
    pub beam_antenna_info_do_not_use_r18:
        Option<NR_TRP_IntegrityServiceAlertElement_r18BeamAntennaInfo_DoNotUse_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_TRP_IntegrityServiceAlertPerFreqLayer_r18(
    pub Vec<NR_TRP_IntegrityServiceAlertElement_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_TRP_LocationInfo_Implicit_r19(pub Vec<TRP_LocationInfo_Implicit_Element_r19>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_TRP_LocationInfo_r16(pub Vec<NR_TRP_LocationInfoPerFreqLayer_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_TRP_LocationInfoPerFreqLayer_r16 {
    #[asn(optional_idx = 0)]
    pub reference_point_r16: Option<ReferencePoint_r16>,
    pub trp_location_info_list_r16: NR_TRP_LocationInfoPerFreqLayer_r16Trp_LocationInfoList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_TRP_RequestList_r19(pub Vec<NR_TRP_RequestListPerFreqLayer_r19>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_TRP_RequestListPerFreqLayer_r19(pub Vec<TRP_RequestInfoElement_r19>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_TimeStamp_r16 {
    pub dl_prs_id_r16: NR_TimeStamp_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    pub nr_sfn_r16: NR_TimeStamp_r16Nr_SFN_r16,
    pub nr_slot_r16: NR_TimeStamp_r16Nr_Slot_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_TimingQuality_r16 {
    pub timing_quality_value_r16: NR_TimingQuality_r16TimingQualityValue_r16,
    pub timing_quality_resolution_r16: NR_TimingQuality_r16TimingQualityResolution_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NR_UE_RxTx_TEG_Info_r17 {
    #[asn(key = 0, extended = false)]
    Case1_r17(NR_UE_RxTx_TEG_Info_r17_case1_r17),
    #[asn(key = 1, extended = false)]
    Case2_r17(NR_UE_RxTx_TEG_Info_r17_case2_r17),
    #[asn(key = 2, extended = false)]
    Case3_r17(NR_UE_RxTx_TEG_Info_r17_case3_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_UE_TEG_Capability_r17 {
    #[asn(optional_idx = 0)]
    pub nr_ue_teg_id_capability_band_list_r17:
        Option<NR_UE_TEG_Capability_r17Nr_UE_TEG_ID_CapabilityBandList_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17 {
    pub freq_band_indicator_nr_r17: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub nr_ue_rx_teg_id_max_support_r17:
        Option<NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTEG_ID_MaxSupport_r17>,
    #[asn(optional_idx = 1)]
    pub nr_ue_tx_teg_id_max_support_r17:
        Option<NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_TxTEG_ID_MaxSupport_r17>,
    #[asn(optional_idx = 2)]
    pub nr_ue_rx_tx_teg_id_max_support_r17:
        Option<NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTxTEG_ID_MaxSupport_r17>,
    #[asn(optional_idx = 3)]
    pub measure_same_dl_prs_resource_with_different_rx_te_gs_r17:
        Option<NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGs_r17>,
    #[asn(optional_idx = 4)]
    pub measure_same_dl_prs_resource_with_different_rx_te_gs_simul_r17: Option<
        NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGsSimul_r17,
    >,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_UL_ProvideCapabilities_r16 {
    pub nr_ul_srs_capability_r16: NR_UL_SRS_Capability_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_UL_RequestCapabilities_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct NR_UL_SRS_Capability_r16 {
    pub srs_capability_band_list_r16: NR_UL_SRS_Capability_r16Srs_CapabilityBandList_r16,
    #[asn(optional_idx = 0)]
    pub srs_pos_resource_config_ca_band_list_r16:
        Option<NR_UL_SRS_Capability_r16Srs_PosResourceConfigCA_BandList_r16>,
    #[asn(optional_idx = 1)]
    pub max_number_srs_pos_path_loss_estimate_all_serving_cells_r16:
        Option<NR_UL_SRS_Capability_r16MaxNumberSRS_PosPathLossEstimateAllServingCells_r16>,
    #[asn(optional_idx = 2)]
    pub max_number_srs_pos_spatial_relations_all_serving_cells_r16:
        Option<NR_UL_SRS_Capability_r16MaxNumberSRS_PosSpatialRelationsAllServingCells_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct NR_linkedDL_PRS_ResourceSetID_PRS_AggregationList_r18(
    pub Vec<NR_DL_PRS_AggregationElement_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_CDC_r16 {
    pub navic_clock_bias_correction_r16: NavIC_CDC_r16Navic_ClockBiasCorrection_r16,
    pub navic_clock_drift_correction_r16: NavIC_CDC_r16Navic_ClockDriftCorrection_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_ClockModel_r16 {
    pub navic_toc_r16: NavIC_ClockModel_r16Navic_Toc_r16,
    pub navic_af2_r16: NavIC_ClockModel_r16Navic_af2_r16,
    pub navic_af1_r16: NavIC_ClockModel_r16Navic_af1_r16,
    pub navic_af0_r16: NavIC_ClockModel_r16Navic_af0_r16,
    pub navic_tgd_r16: NavIC_ClockModel_r16Navic_Tgd_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_ClockModel2_r19 {
    pub navic_l1_toec_r19: NavIC_ClockModel2_r19NavicL1_Toec_r19,
    pub navic_l1_af2_r19: NavIC_ClockModel2_r19NavicL1_af2_r19,
    pub navic_l1_af1_r19: NavIC_ClockModel2_r19NavicL1_af1_r19,
    pub navic_l1_af0_r19: NavIC_ClockModel2_r19NavicL1_af0_r19,
    pub navic_l1_tgd_r19: NavIC_ClockModel2_r19NavicL1_Tgd_r19,
    pub navic_l1_isc_l1_por_s_r19: NavIC_ClockModel2_r19NavicL1_iscL1PorS_r19,
    pub navic_l1_isc_l1d_r19: NavIC_ClockModel2_r19NavicL1_iscL1D_r19,
    pub navic_l1_rsf_r19: NavIC_ClockModel2_r19NavicL1_RSF_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_CorrectionElementAutoNav_r16 {
    pub sv_id: SV_ID,
    pub navic_tod_r16: NavIC_CorrectionElementAutoNav_r16Navic_Tod_r16,
    pub navic_iodec_r16: NavIC_CorrectionElementAutoNav_r16Navic_iodec_r16,
    pub navic_udrai_r16: NavIC_CorrectionElementAutoNav_r16Navic_UDRAI_r16,
    pub navic_udr_arate_i_r16: NavIC_CorrectionElementAutoNav_r16Navic_UDRArateI_r16,
    pub navic_edc_r16: NavIC_EDC_r16,
    pub navic_cdc_r16: NavIC_CDC_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NavIC_CorrectionListAutoNav_r16(pub Vec<NavIC_CorrectionElementAutoNav_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_DifferentialCorrections_r16 {
    pub navic_ref_towc_r16: NavIC_DifferentialCorrections_r16Navic_RefTOWC_r16,
    pub navic_correction_list_auto_nav_r16: NavIC_CorrectionListAutoNav_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_DifferentialCorrectionsReq_r16 {
    pub dgnss_signals_req_r16: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_DifferentialCorrectionsSupport_r16 {
    pub gnss_signal_i_ds_r16: GNSS_SignalIDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_EDC_r16 {
    pub navic_alpha_edc_r16: NavIC_EDC_r16Navic_AlphaEDC_r16,
    pub navic_beta_edc_r16: NavIC_EDC_r16Navic_BetaEDC_r16,
    pub navic_gamma_edc_r16: NavIC_EDC_r16Navic_GammaEDC_r16,
    pub navic_ao_icorrection_r16: NavIC_EDC_r16Navic_AoIcorrection_r16,
    pub navic_ao_r_acorrection_r16: NavIC_EDC_r16Navic_AoRAcorrection_r16,
    pub navic_semi_majorcorrection_r16: NavIC_EDC_r16Navic_SemiMajorcorrection_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_GridModelParameter_r16 {
    pub navic_ref_towc_r16: NavIC_GridModelParameter_r16Navic_RefTOWC_r16,
    pub region_masked_r16: NavIC_GridModelParameter_r16RegionMasked_r16,
    pub region_igp_list_r16: RegionIgpList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_GridModelReq_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavIC_GridModelSupport_r16 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModel_BDS_KeplerianSet_r12 {
    pub bds_aode_r12: NavModel_BDS_KeplerianSet_r12BdsAODE_r12,
    pub bds_urai_r12: NavModel_BDS_KeplerianSet_r12BdsURAI_r12,
    pub bds_toe_r12: NavModel_BDS_KeplerianSet_r12BdsToe_r12,
    pub bds_a_power_half_r12: NavModel_BDS_KeplerianSet_r12BdsAPowerHalf_r12,
    pub bds_e_r12: NavModel_BDS_KeplerianSet_r12BdsE_r12,
    pub bds_w_r12: NavModel_BDS_KeplerianSet_r12BdsW_r12,
    pub bds_delta_n_r12: NavModel_BDS_KeplerianSet_r12BdsDeltaN_r12,
    pub bds_m0_r12: NavModel_BDS_KeplerianSet_r12BdsM0_r12,
    pub bds_omega0_r12: NavModel_BDS_KeplerianSet_r12BdsOmega0_r12,
    pub bds_omega_dot_r12: NavModel_BDS_KeplerianSet_r12BdsOmegaDot_r12,
    pub bds_i0_r12: NavModel_BDS_KeplerianSet_r12BdsI0_r12,
    pub bds_i_dot_r12: NavModel_BDS_KeplerianSet_r12BdsIDot_r12,
    pub bds_cuc_r12: NavModel_BDS_KeplerianSet_r12BdsCuc_r12,
    pub bds_cus_r12: NavModel_BDS_KeplerianSet_r12BdsCus_r12,
    pub bds_crc_r12: NavModel_BDS_KeplerianSet_r12BdsCrc_r12,
    pub bds_crs_r12: NavModel_BDS_KeplerianSet_r12BdsCrs_r12,
    pub bds_cic_r12: NavModel_BDS_KeplerianSet_r12BdsCic_r12,
    pub bds_cis_r12: NavModel_BDS_KeplerianSet_r12BdsCis_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModel_BDS_KeplerianSet2_r16 {
    pub bds_iode_r16: NavModel_BDS_KeplerianSet2_r16BdsIODE_r16,
    pub bds_toe_r16: NavModel_BDS_KeplerianSet2_r16BdsToe_r16,
    pub bds_delta_a_r16: NavModel_BDS_KeplerianSet2_r16BdsDeltaA_r16,
    pub bds_adot_r16: NavModel_BDS_KeplerianSet2_r16BdsAdot_r16,
    pub bds_delta_n0_r16: NavModel_BDS_KeplerianSet2_r16BdsDeltaN0_r16,
    pub bds_delta_n0dot_r16: NavModel_BDS_KeplerianSet2_r16BdsDeltaN0dot_r16,
    pub bds_m0_r16: NavModel_BDS_KeplerianSet2_r16BdsM0_r16,
    pub bds_e_r16: NavModel_BDS_KeplerianSet2_r16BdsE_r16,
    pub bds_omega_r16: NavModel_BDS_KeplerianSet2_r16BdsOmega_r16,
    pub bds_omega0_r16: NavModel_BDS_KeplerianSet2_r16BdsOmega0_r16,
    pub bds_i0_r16: NavModel_BDS_KeplerianSet2_r16BdsI0_r16,
    pub bds_omega_dot_r16: NavModel_BDS_KeplerianSet2_r16BdsOmegaDot_r16,
    pub bds_i0_dot_r16: NavModel_BDS_KeplerianSet2_r16BdsI0Dot_r16,
    pub bds_cuc_r16: NavModel_BDS_KeplerianSet2_r16BdsCuc_r16,
    pub bds_cus_r16: NavModel_BDS_KeplerianSet2_r16BdsCus_r16,
    pub bds_crc_r16: NavModel_BDS_KeplerianSet2_r16BdsCrc_r16,
    pub bds_crs_r16: NavModel_BDS_KeplerianSet2_r16BdsCrs_r16,
    pub bds_cic_r16: NavModel_BDS_KeplerianSet2_r16BdsCic_r16,
    pub bds_cis_r16: NavModel_BDS_KeplerianSet2_r16BdsCis_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModel_GLONASS_ECEF {
    pub glo_en: NavModel_GLONASS_ECEFGloEn,
    pub glo_p1: NavModel_GLONASS_ECEFGloP1,
    pub glo_p2: NavModel_GLONASS_ECEFGloP2,
    pub glo_m: NavModel_GLONASS_ECEFGloM,
    pub glo_x: NavModel_GLONASS_ECEFGloX,
    pub glo_xdot: NavModel_GLONASS_ECEFGloXdot,
    pub glo_xdotdot: NavModel_GLONASS_ECEFGloXdotdot,
    pub glo_y: NavModel_GLONASS_ECEFGloY,
    pub glo_ydot: NavModel_GLONASS_ECEFGloYdot,
    pub glo_ydotdot: NavModel_GLONASS_ECEFGloYdotdot,
    pub glo_z: NavModel_GLONASS_ECEFGloZ,
    pub glo_zdot: NavModel_GLONASS_ECEFGloZdot,
    pub glo_zdotdot: NavModel_GLONASS_ECEFGloZdotdot,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModel_NavIC_KeplerianSet_r16 {
    pub navic_toe_r16: NavModel_NavIC_KeplerianSet_r16Navic_Toe_r16,
    pub navic_urai_r16: NavModel_NavIC_KeplerianSet_r16Navic_URAI_r16,
    pub navic_w_r16: NavModel_NavIC_KeplerianSet_r16Navic_W_r16,
    pub navic_delta_n_r16: NavModel_NavIC_KeplerianSet_r16Navic_DeltaN_r16,
    pub navic_m0_r16: NavModel_NavIC_KeplerianSet_r16Navic_M0_r16,
    pub navic_omega_dot_r16: NavModel_NavIC_KeplerianSet_r16Navic_OmegaDot_r16,
    pub navic_e_r16: NavModel_NavIC_KeplerianSet_r16Navic_E_r16,
    pub navic_i_dot_r16: NavModel_NavIC_KeplerianSet_r16Navic_IDot_r16,
    pub navic_a_power_half_r16: NavModel_NavIC_KeplerianSet_r16Navic_APowerHalf_r16,
    pub navic_i0_r16: NavModel_NavIC_KeplerianSet_r16Navic_I0_r16,
    pub navic_omega0_r16: NavModel_NavIC_KeplerianSet_r16Navic_Omega0_r16,
    pub navic_crs_r16: NavModel_NavIC_KeplerianSet_r16Navic_Crs_r16,
    pub navic_cis_r16: NavModel_NavIC_KeplerianSet_r16Navic_Cis_r16,
    pub navic_cus_r16: NavModel_NavIC_KeplerianSet_r16Navic_Cus_r16,
    pub navic_crc_r16: NavModel_NavIC_KeplerianSet_r16Navic_Crc_r16,
    pub navic_cic_r16: NavModel_NavIC_KeplerianSet_r16Navic_Cic_r16,
    pub navic_cuc_r16: NavModel_NavIC_KeplerianSet_r16Navic_Cuc_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModel_NavIC_KeplerianSet2_r19 {
    pub navic_l1_toec_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Toec_r19,
    pub navic_l1_urai_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_URAI_r19,
    pub navic_l1_delta_a_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaA_r19,
    pub navic_l1_adot_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Adot_r19,
    pub navic_l1_delta_n0_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaN0_r19,
    pub navic_l1_delta_ndot_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaNdot_r19,
    pub navic_l1_m0_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_M0_r19,
    pub navic_l1_e_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_E_r19,
    pub navic_l1_w_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_W_r19,
    pub navic_l1_omega0_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Omega0_r19,
    pub navic_l1_omega_dot_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_OmegaDot_r19,
    pub navic_l1_i0_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_I0_r19,
    pub navic_l1_i_dot_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_IDot_r19,
    pub navic_l1_cis_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Cis_r19,
    pub navic_l1_cic_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Cic_r19,
    pub navic_l1_crs_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Crs_r19,
    pub navic_l1_crc_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Crc_r19,
    pub navic_l1_cus_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Cus_r19,
    pub navic_l1_cuc_r19: NavModel_NavIC_KeplerianSet2_r19NavicL1_Cuc_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NavModel_SBAS_ECEF {
    #[asn(optional_idx = 0)]
    pub sbas_to: Option<NavModel_SBAS_ECEFSbasTo>,
    pub sbas_accuracy: NavModel_SBAS_ECEFSbasAccuracy,
    pub sbas_xg: NavModel_SBAS_ECEFSbasXg,
    pub sbas_yg: NavModel_SBAS_ECEFSbasYg,
    pub sbas_zg: NavModel_SBAS_ECEFSbasZg,
    pub sbas_xg_dot: NavModel_SBAS_ECEFSbasXgDot,
    pub sbas_yg_dot: NavModel_SBAS_ECEFSbasYgDot,
    pub sbas_zg_dot: NavModel_SBAS_ECEFSbasZgDot,
    pub sbas_xg_dot_dot: NavModel_SBAS_ECEFSbasXgDotDot,
    pub sbag_yg_dot_dot: NavModel_SBAS_ECEFSbagYgDotDot,
    pub sbas_zg_dot_dot: NavModel_SBAS_ECEFSbasZgDotDot,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModelCNAV_KeplerianSet {
    pub cnav_top: NavModelCNAV_KeplerianSetCnavTop,
    pub cnav_ur_aindex: NavModelCNAV_KeplerianSetCnavURAindex,
    pub cnav_delta_a: NavModelCNAV_KeplerianSetCnavDeltaA,
    pub cnav_adot: NavModelCNAV_KeplerianSetCnavAdot,
    pub cnav_delta_no: NavModelCNAV_KeplerianSetCnavDeltaNo,
    pub cnav_delta_no_dot: NavModelCNAV_KeplerianSetCnavDeltaNoDot,
    pub cnav_mo: NavModelCNAV_KeplerianSetCnavMo,
    pub cnav_e: NavModelCNAV_KeplerianSetCnavE,
    pub cnav_omega: NavModelCNAV_KeplerianSetCnavOmega,
    pub cnav_omega0: NavModelCNAV_KeplerianSetCnavOMEGA0,
    pub cnav_delta_omega_dot: NavModelCNAV_KeplerianSetCnavDeltaOmegaDot,
    pub cnav_io: NavModelCNAV_KeplerianSetCnavIo,
    pub cnav_io_dot: NavModelCNAV_KeplerianSetCnavIoDot,
    pub cnav_cis: NavModelCNAV_KeplerianSetCnavCis,
    pub cnav_cic: NavModelCNAV_KeplerianSetCnavCic,
    pub cnav_crs: NavModelCNAV_KeplerianSetCnavCrs,
    pub cnav_crc: NavModelCNAV_KeplerianSetCnavCrc,
    pub cnav_cus: NavModelCNAV_KeplerianSetCnavCus,
    pub cnav_cuc: NavModelCNAV_KeplerianSetCnavCuc,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NavModelKeplerianSet {
    pub kepler_toe: NavModelKeplerianSetKeplerToe,
    pub kepler_w: NavModelKeplerianSetKeplerW,
    pub kepler_delta_n: NavModelKeplerianSetKeplerDeltaN,
    pub kepler_m0: NavModelKeplerianSetKeplerM0,
    pub kepler_omega_dot: NavModelKeplerianSetKeplerOmegaDot,
    pub kepler_e: NavModelKeplerianSetKeplerE,
    pub kepler_i_dot: NavModelKeplerianSetKeplerIDot,
    pub kepler_a_power_half: NavModelKeplerianSetKeplerAPowerHalf,
    pub kepler_i0: NavModelKeplerianSetKeplerI0,
    pub kepler_omega0: NavModelKeplerianSetKeplerOmega0,
    pub kepler_crs: NavModelKeplerianSetKeplerCrs,
    pub kepler_cis: NavModelKeplerianSetKeplerCis,
    pub kepler_cus: NavModelKeplerianSetKeplerCus,
    pub kepler_crc: NavModelKeplerianSetKeplerCrc,
    pub kepler_cic: NavModelKeplerianSetKeplerCic,
    pub kepler_cuc: NavModelKeplerianSetKeplerCuc,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NavModelNAV_KeplerianSet {
    pub nav_ura: NavModelNAV_KeplerianSetNavURA,
    pub nav_fit_flag: NavModelNAV_KeplerianSetNavFitFlag,
    pub nav_toe: NavModelNAV_KeplerianSetNavToe,
    pub nav_omega: NavModelNAV_KeplerianSetNavOmega,
    pub nav_delta_n: NavModelNAV_KeplerianSetNavDeltaN,
    pub nav_m0: NavModelNAV_KeplerianSetNavM0,
    pub nav_omega_a_dot: NavModelNAV_KeplerianSetNavOmegaADot,
    pub nav_e: NavModelNAV_KeplerianSetNavE,
    pub nav_i_dot: NavModelNAV_KeplerianSetNavIDot,
    pub nav_a_power_half: NavModelNAV_KeplerianSetNavAPowerHalf,
    pub nav_i0: NavModelNAV_KeplerianSetNavI0,
    pub nav_omega_a0: NavModelNAV_KeplerianSetNavOmegaA0,
    pub nav_crs: NavModelNAV_KeplerianSetNavCrs,
    pub nav_cis: NavModelNAV_KeplerianSetNavCis,
    pub nav_cus: NavModelNAV_KeplerianSetNavCus,
    pub nav_crc: NavModelNAV_KeplerianSetNavCrc,
    pub nav_cic: NavModelNAV_KeplerianSetNavCic,
    pub nav_cuc: NavModelNAV_KeplerianSetNavCuc,
    #[asn(optional_idx = 0)]
    pub add_na_vparam: Option<NavModelNAV_KeplerianSetAddNAVparam>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NeQuickModel2Parameter_r19 {
    pub iodn_r19: NeQuickModel2Parameter_r19Iodn_r19,
    pub ne_quick_model2_parameter_list_r19: NeQuickModel2ParameterList_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NeQuickModel2ParameterElement_r19 {
    pub ai0_r19: NeQuickModel2ParameterElement_r19Ai0_r19,
    pub ai1_r19: NeQuickModel2ParameterElement_r19Ai1_r19,
    pub ai2_r19: NeQuickModel2ParameterElement_r19Ai2_r19,
    #[asn(optional_idx = 0)]
    pub iono_disturbance_flag_r19: Option<NeQuickModel2ParameterElement_r19IonoDisturbanceFlag_r19>,
    pub modipmax_r19: NeQuickModel2ParameterElement_r19Modipmax_r19,
    pub modipmin_r19: NeQuickModel2ParameterElement_r19Modipmin_r19,
    pub m_lonmax_r19: NeQuickModel2ParameterElement_r19MLonmax_r19,
    pub m_lonmin_r19: NeQuickModel2ParameterElement_r19MLonmin_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct NeQuickModel2ParameterList_r19(pub Vec<NeQuickModel2ParameterElement_r19>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct NeQuickModelParameter {
    pub ai0: NeQuickModelParameterAi0,
    pub ai1: NeQuickModelParameterAi1,
    pub ai2: NeQuickModelParameterAi2,
    #[asn(optional_idx = 0)]
    pub iono_storm_flag1: Option<NeQuickModelParameterIonoStormFlag1>,
    #[asn(optional_idx = 1)]
    pub iono_storm_flag2: Option<NeQuickModelParameterIonoStormFlag2>,
    #[asn(optional_idx = 2)]
    pub iono_storm_flag3: Option<NeQuickModelParameterIonoStormFlag3>,
    #[asn(optional_idx = 3)]
    pub iono_storm_flag4: Option<NeQuickModelParameterIonoStormFlag4>,
    #[asn(optional_idx = 4)]
    pub iono_storm_flag5: Option<NeQuickModelParameterIonoStormFlag5>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct NeighbourMeasurementElement {
    pub phys_cell_id_neighbour: NeighbourMeasurementElementPhysCellIdNeighbour,
    #[asn(optional_idx = 0)]
    pub cell_global_id_neighbour: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn_neighbour: Option<ARFCN_ValueEUTRA>,
    pub rstd: NeighbourMeasurementElementRstd,
    pub rstd_quality: OTDOA_MeasQuality,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 8)]
pub struct NeighbourMeasurementElement_NB_r14 {
    pub phys_cell_id_neighbour_r14: NeighbourMeasurementElement_NB_r14PhysCellIdNeighbour_r14,
    #[asn(optional_idx = 0)]
    pub cell_global_id_neighbour_r14: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn_neighbour_r14: Option<ARFCN_ValueEUTRA_r14>,
    pub rstd_r14: NeighbourMeasurementElement_NB_r14Rstd_r14,
    pub rstd_quality_r14: OTDOA_MeasQuality,
    #[asn(optional_idx = 2)]
    pub tp_id_neighbour_r14: Option<NeighbourMeasurementElement_NB_r14TpIdNeighbour_r14>,
    #[asn(optional_idx = 3)]
    pub prs_id_neighbour_r14: Option<NeighbourMeasurementElement_NB_r14PrsIdNeighbour_r14>,
    #[asn(optional_idx = 4)]
    pub delta_rstd_r14: Option<NeighbourMeasurementElement_NB_r14Delta_rstd_r14>,
    #[asn(optional_idx = 5)]
    pub additional_paths_neighbour_r14: Option<AdditionalPathList_r14>,
    #[asn(optional_idx = 6)]
    pub nprs_id_neighbour_r14: Option<NeighbourMeasurementElement_NB_r14NprsIdNeighbour_r14>,
    #[asn(optional_idx = 7)]
    pub carrier_freq_offset_nb_neighbour_r14: Option<CarrierFreqOffsetNB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "24")]
pub struct NeighbourMeasurementList(pub Vec<NeighbourMeasurementElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "24")]
pub struct NeighbourMeasurementList_NB_r14(pub Vec<NeighbourMeasurementElement_NB_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTime {
    pub seconds_from_frame_structure_start: NetworkTimeSecondsFromFrameStructureStart,
    pub fractional_seconds_from_frame_structure_start:
        NetworkTimeFractionalSecondsFromFrameStructureStart,
    #[asn(optional_idx = 0)]
    pub frame_drift: Option<NetworkTimeFrameDrift>,
    pub cell_id: NetworkTimeCellID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct OLPC_SRS_Pos_r16 {
    #[asn(optional_idx = 0)]
    pub olpc_srs_pos_based_on_prs_serving_r16:
        Option<OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Serving_r16>,
    #[asn(optional_idx = 1)]
    pub olpc_srs_pos_based_on_ssb_neigh_r16:
        Option<OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnSSB_Neigh_r16>,
    #[asn(optional_idx = 2)]
    pub olpc_srs_pos_based_on_prs_neigh_r16:
        Option<OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Neigh_r16>,
    #[asn(optional_idx = 3)]
    pub max_number_path_loss_estimate_per_serving_r16:
        Option<OLPC_SRS_Pos_r16MaxNumberPathLossEstimatePerServing_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct ORBIT_IntegrityParameters_r17 {
    pub prob_onset_const_fault_r17: ORBIT_IntegrityParameters_r17ProbOnsetConstFault_r17,
    pub mean_const_fault_duration_r17: ORBIT_IntegrityParameters_r17MeanConstFaultDuration_r17,
    pub prob_onset_sat_fault_r17: ORBIT_IntegrityParameters_r17ProbOnsetSatFault_r17,
    pub mean_sat_fault_duration_r17: ORBIT_IntegrityParameters_r17MeanSatFaultDuration_r17,
    #[asn(optional_idx = 0)]
    pub orbit_range_error_correlation_time_r17:
        Option<ORBIT_IntegrityParameters_r17OrbitRangeErrorCorrelationTime_r17>,
    #[asn(optional_idx = 1)]
    pub orbit_range_rate_error_correlation_time_r17:
        Option<ORBIT_IntegrityParameters_r17OrbitRangeRateErrorCorrelationTime_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum OTDOA_Error {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses(OTDOA_LocationServerErrorCauses),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses(OTDOA_TargetDeviceErrorCauses),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_LocationServerErrorCauses {
    pub cause: OTDOA_LocationServerErrorCausesCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct OTDOA_MeasQuality {
    pub error_resolution: OTDOA_MeasQualityError_Resolution,
    pub error_value: OTDOA_MeasQualityError_Value,
    #[asn(optional_idx = 0)]
    pub error_num_samples: Option<OTDOA_MeasQualityError_NumSamples>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct OTDOA_NeighbourCellInfoElement {
    pub phys_cell_id: OTDOA_NeighbourCellInfoElementPhysCellId,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn: Option<ARFCN_ValueEUTRA>,
    #[asn(optional_idx = 2)]
    pub cp_length: Option<OTDOA_NeighbourCellInfoElementCpLength>,
    #[asn(optional_idx = 3)]
    pub prs_info: Option<PRS_Info>,
    #[asn(optional_idx = 4)]
    pub antenna_port_config: Option<OTDOA_NeighbourCellInfoElementAntennaPortConfig>,
    #[asn(optional_idx = 5)]
    pub slot_number_offset: Option<OTDOA_NeighbourCellInfoElementSlotNumberOffset>,
    #[asn(optional_idx = 6)]
    pub prs_subframe_offset: Option<OTDOA_NeighbourCellInfoElementPrs_SubframeOffset>,
    pub expected_rstd: OTDOA_NeighbourCellInfoElementExpectedRSTD,
    pub expected_rstd_uncertainty: OTDOA_NeighbourCellInfoElementExpectedRSTD_Uncertainty,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct OTDOA_NeighbourCellInfoList(pub Vec<OTDOA_NeighbourFreqInfo>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "72")]
pub struct OTDOA_NeighbourCellInfoListNB_r14(pub Vec<OTDOA_NeighbourCellInfoNB_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 13)]
pub struct OTDOA_NeighbourCellInfoNB_r14 {
    #[asn(optional_idx = 0)]
    pub phys_cell_id_nb_r14: Option<OTDOA_NeighbourCellInfoNB_r14PhysCellIdNB_r14>,
    #[asn(optional_idx = 1)]
    pub cell_global_id_nb_r14: Option<ECGI>,
    #[asn(optional_idx = 2)]
    pub carrier_freq_r14: Option<CarrierFreq_NB_r14>,
    #[asn(optional_idx = 3)]
    pub earfcn_r14: Option<ARFCN_ValueEUTRA_r14>,
    #[asn(optional_idx = 4)]
    pub eutra_num_crs_ports_r14: Option<OTDOA_NeighbourCellInfoNB_r14Eutra_NumCRS_Ports_r14>,
    #[asn(optional_idx = 5)]
    pub otdoa_sib1_nb_repetitions_r14:
        Option<OTDOA_NeighbourCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14>,
    #[asn(optional_idx = 6)]
    pub nprs_info_r14: Option<PRS_Info_NB_r14>,
    #[asn(optional_idx = 7)]
    pub nprs_slot_number_offset_r14: Option<OTDOA_NeighbourCellInfoNB_r14Nprs_slotNumberOffset_r14>,
    #[asn(optional_idx = 8)]
    pub nprs_sfn_offset_r14: Option<OTDOA_NeighbourCellInfoNB_r14Nprs_SFN_Offset_r14>,
    #[asn(optional_idx = 9)]
    pub nprs_subframe_offset_r14: Option<OTDOA_NeighbourCellInfoNB_r14Nprs_SubframeOffset_r14>,
    #[asn(optional_idx = 10)]
    pub expected_rstd_r14: Option<OTDOA_NeighbourCellInfoNB_r14ExpectedRSTD_r14>,
    #[asn(optional_idx = 11)]
    pub expected_rstd_uncertainty_r14:
        Option<OTDOA_NeighbourCellInfoNB_r14ExpectedRSTD_Uncertainty_r14>,
    #[asn(optional_idx = 12)]
    pub prs_neighbour_cell_index_r14:
        Option<OTDOA_NeighbourCellInfoNB_r14PrsNeighbourCellIndex_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "24")]
pub struct OTDOA_NeighbourFreqInfo(pub Vec<OTDOA_NeighbourCellInfoElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct OTDOA_ProvideAssistanceData {
    #[asn(optional_idx = 0)]
    pub otdoa_reference_cell_info: Option<OTDOA_ReferenceCellInfo>,
    #[asn(optional_idx = 1)]
    pub otdoa_neighbour_cell_info: Option<OTDOA_NeighbourCellInfoList>,
    #[asn(optional_idx = 2)]
    pub otdoa_error: Option<OTDOA_Error>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_ProvideCapabilities {
    pub otdoa_mode: OTDOA_ProvideCapabilitiesOtdoa_Mode,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct OTDOA_ProvideLocationInformation {
    #[asn(optional_idx = 0)]
    pub otdoa_signal_measurement_information: Option<OTDOA_SignalMeasurementInformation>,
    #[asn(optional_idx = 1)]
    pub otdoa_error: Option<OTDOA_Error>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct OTDOA_ReferenceCellInfo {
    pub phys_cell_id: OTDOA_ReferenceCellInfoPhysCellId,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn_ref: Option<ARFCN_ValueEUTRA>,
    #[asn(optional_idx = 2)]
    pub antenna_port_config: Option<OTDOA_ReferenceCellInfoAntennaPortConfig>,
    pub cp_length: OTDOA_ReferenceCellInfoCpLength,
    #[asn(optional_idx = 3)]
    pub prs_info: Option<PRS_Info>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct OTDOA_ReferenceCellInfoNB_r14 {
    #[asn(optional_idx = 0)]
    pub phys_cell_id_nb_r14: Option<OTDOA_ReferenceCellInfoNB_r14PhysCellIdNB_r14>,
    #[asn(optional_idx = 1)]
    pub cell_global_id_nb_r14: Option<ECGI>,
    #[asn(optional_idx = 2)]
    pub carrier_freq_ref_r14: Option<CarrierFreq_NB_r14>,
    #[asn(optional_idx = 3)]
    pub earfcn_r14: Option<ARFCN_ValueEUTRA_r14>,
    #[asn(optional_idx = 4)]
    pub eutra_num_crs_ports_r14: Option<OTDOA_ReferenceCellInfoNB_r14Eutra_NumCRS_Ports_r14>,
    #[asn(optional_idx = 5)]
    pub otdoa_sib1_nb_repetitions_r14:
        Option<OTDOA_ReferenceCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14>,
    #[asn(optional_idx = 6)]
    pub nprs_info_r14: Option<PRS_Info_NB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_RequestAssistanceData {
    pub phys_cell_id: OTDOA_RequestAssistanceDataPhysCellId,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_RequestCapabilities {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_RequestLocationInformation {
    pub assistance_availability: OTDOA_RequestLocationInformationAssistanceAvailability,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct OTDOA_SignalMeasurementInformation {
    pub system_frame_number: OTDOA_SignalMeasurementInformationSystemFrameNumber,
    pub phys_cell_id_ref: OTDOA_SignalMeasurementInformationPhysCellIdRef,
    #[asn(optional_idx = 0)]
    pub cell_global_id_ref: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn_ref: Option<ARFCN_ValueEUTRA>,
    #[asn(optional_idx = 2)]
    pub reference_quality: Option<OTDOA_MeasQuality>,
    pub neighbour_measurement_list: NeighbourMeasurementList,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 9)]
pub struct OTDOA_SignalMeasurementInformation_NB_r14 {
    pub system_frame_number_r14: OTDOA_SignalMeasurementInformation_NB_r14SystemFrameNumber_r14,
    pub phys_cell_id_ref_r14: OTDOA_SignalMeasurementInformation_NB_r14PhysCellIdRef_r14,
    #[asn(optional_idx = 0)]
    pub cell_global_id_ref_r14: Option<ECGI>,
    #[asn(optional_idx = 1)]
    pub earfcn_ref_r14: Option<ARFCN_ValueEUTRA_r14>,
    #[asn(optional_idx = 2)]
    pub reference_quality_r14: Option<OTDOA_MeasQuality>,
    pub neighbour_measurement_list_r14: NeighbourMeasurementList_NB_r14,
    #[asn(optional_idx = 3)]
    pub tp_id_ref_r14: Option<OTDOA_SignalMeasurementInformation_NB_r14TpIdRef_r14>,
    #[asn(optional_idx = 4)]
    pub prs_id_ref_r14: Option<OTDOA_SignalMeasurementInformation_NB_r14PrsIdRef_r14>,
    #[asn(optional_idx = 5)]
    pub additional_paths_ref_r14: Option<AdditionalPathList_r14>,
    #[asn(optional_idx = 6)]
    pub nprs_id_ref_r14: Option<OTDOA_SignalMeasurementInformation_NB_r14NprsIdRef_r14>,
    #[asn(optional_idx = 7)]
    pub carrier_freq_offset_nb_ref_r14: Option<CarrierFreqOffsetNB_r14>,
    #[asn(optional_idx = 8)]
    pub hyper_sfn_r14: Option<OTDOA_SignalMeasurementInformation_NB_r14HyperSFN_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct OTDOA_TargetDeviceErrorCauses {
    pub cause: OTDOA_TargetDeviceErrorCausesCause,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct On_Demand_DL_PRS_Configuration_r17 {
    pub dl_prs_configuration_id_r17: DL_PRS_Configuration_ID_r17,
    pub nr_dl_prs_positioning_frequency_layer_r17: NR_DL_PRS_PositioningFrequencyLayer_r16,
    pub nr_dl_prs_info_r17: NR_DL_PRS_Info_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct OnDemandDL_PRS_AggregationInfo_r18(pub Vec<DL_PRS_Configuration_ID_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18 {
    #[asn(optional_idx = 0)]
    pub maximum_of_three_aggregated_dl_prs_bandwidth_fr1_r18: Option<
        PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r18,
    >,
    #[asn(optional_idx = 1)]
    pub maximum_of_three_aggregated_dl_prs_bandwidth_fr2_r18: Option<
        PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r18,
    >,
    #[asn(optional_idx = 2)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr1_r18:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18>,
    #[asn(optional_idx = 3)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr2_r18:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18>,
    pub dl_prs_buffer_type_of_bwa_r18:
        PRS_BWA_ThreeContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18,
    #[asn(optional_idx = 4)]
    pub prs_duration_of_three_prs_bwa_processing_r18:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18>,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr1_r18:
        PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr2_r18:
        PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19 {
    #[asn(optional_idx = 0)]
    pub maximum_of_three_aggregated_dl_prs_bandwidth_fr1_r19: Option<
        PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r19,
    >,
    #[asn(optional_idx = 1)]
    pub maximum_of_three_aggregated_dl_prs_bandwidth_fr2_r19: Option<
        PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r19,
    >,
    #[asn(optional_idx = 2)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr1_r19:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19>,
    #[asn(optional_idx = 3)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr2_r19:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19>,
    pub dl_prs_buffer_type_of_bwa_r19:
        PRS_BWA_ThreeContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19,
    #[asn(optional_idx = 4)]
    pub prs_duration_of_three_prs_bwa_processing_r19:
        Option<PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19>,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr1_r19:
        PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr2_r19:
        PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 5)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18 {
    #[asn(optional_idx = 0)]
    pub maximum_of_two_aggregated_dl_prs_bandwidth_fr1_r18: Option<
        PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r18,
    >,
    #[asn(optional_idx = 1)]
    pub maximum_of_two_aggregated_dl_prs_bandwidth_fr2_r18: Option<
        PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r18,
    >,
    #[asn(optional_idx = 2)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr1_r18:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18>,
    #[asn(optional_idx = 3)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr2_r18:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18>,
    pub dl_prs_buffer_type_of_bwa_r18:
        PRS_BWA_TwoContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18,
    #[asn(optional_idx = 4)]
    pub prs_duration_of_two_prs_bwa_processing_r18:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18>,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr1_r18:
        PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr2_r18:
        PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19 {
    #[asn(optional_idx = 0)]
    pub maximum_of_two_aggregated_dl_prs_bandwidth_fr1_r19: Option<
        PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r19,
    >,
    #[asn(optional_idx = 1)]
    pub maximum_of_two_aggregated_dl_prs_bandwidth_fr2_r19: Option<
        PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r19,
    >,
    #[asn(optional_idx = 2)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr1_r19:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19>,
    #[asn(optional_idx = 3)]
    pub maximum_of_dl_prs_bandwidth_per_pfl_fr2_r19:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19>,
    pub dl_prs_buffer_type_of_bwa_r19:
        PRS_BWA_TwoContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19,
    #[asn(optional_idx = 4)]
    pub prs_duration_of_two_prs_bwa_processing_r19:
        Option<PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19>,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr1_r19:
        PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19,
    pub max_num_of_aggregated_dl_prs_resource_per_slot_fr2_r19:
        PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_Info {
    pub prs_bandwidth: PRS_InfoPrs_Bandwidth,
    pub prs_configuration_index: PRS_InfoPrs_ConfigurationIndex,
    pub num_dl_frames: PRS_InfoNumDL_Frames,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "5")]
pub struct PRS_Info_NB_r14(pub Vec<NPRS_Info_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17 {
    pub prs_processing_type_r17:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r17PrsProcessingType_r17,
    pub ppw_dl_prs_buffer_type_r17:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_dl_PRS_BufferType_r17,
    #[asn(optional_idx = 0)]
    pub ppw_duration_of_prs_processing1_r17:
        Option<PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17>,
    #[asn(optional_idx = 1)]
    pub ppw_duration_of_prs_processing2_r17:
        Option<PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17>,
    pub ppw_max_num_of_dl_prs_res_processed_per_slot_r17:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19 {
    pub prs_processing_type_r19:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r19PrsProcessingType_r19,
    pub ppw_dl_prs_buffer_type_r19:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_dl_PRS_BufferType_r19,
    #[asn(optional_idx = 0)]
    pub ppw_duration_of_prs_processing1_r19:
        Option<PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19>,
    #[asn(optional_idx = 1)]
    pub ppw_duration_of_prs_processing2_r19:
        Option<PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19>,
    pub ppw_max_num_of_dl_prs_res_processed_per_slot_r19:
        PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19,
    #[asn(optional_idx = 2)]
    pub ppw_max_num_of_dl_bandwidth_r19:
        Option<PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_ProcessingCapabilityPerBand_r16 {
    pub freq_band_indicator_nr_r16: FreqBandIndicatorNR_r16,
    pub supported_bandwidth_prs_r16: PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16,
    pub dl_prs_buffer_type_r16: PRS_ProcessingCapabilityPerBand_r16Dl_PRS_BufferType_r16,
    pub duration_of_prs_processing_r16:
        PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16,
    pub max_num_of_dl_prs_res_processed_per_slot_r16:
        PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PeriodicAssistanceDataControlParameters_r15 {
    pub periodic_session_id_r15: PeriodicSessionID_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PeriodicReportingIntervalMsSupport_r18 {
    pub min_periodic_reporting_interval_ms_r18:
        PeriodicReportingIntervalMsSupport_r18MinPeriodicReportingIntervalMs_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PeriodicReportingIntervalMsSupportPerMode_r18 {
    #[asn(optional_idx = 0)]
    pub min_periodic_reporting_interval_ms1_supported_r18: Option<PositioningModes>,
    #[asn(optional_idx = 1)]
    pub min_periodic_reporting_interval_ms10_supported_r18: Option<PositioningModes>,
    #[asn(optional_idx = 2)]
    pub min_periodic_reporting_interval_ms100_supported_r18: Option<PositioningModes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PeriodicSessionID_r15 {
    pub periodic_session_initiator_r15: PeriodicSessionID_r15PeriodicSessionInitiator_r15,
    pub periodic_session_number_r15: PeriodicSessionID_r15PeriodicSessionNumber_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct PeriodicalReportingCriteria {
    #[asn(optional_idx = 0)]
    pub reporting_amount: Option<PeriodicalReportingCriteriaReportingAmount>,
    pub reporting_interval: PeriodicalReportingCriteriaReportingInterval,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PeriodicalReportingCriteriaExt_r18 {
    #[asn(optional_idx = 0)]
    pub reporting_amount_r18: Option<PeriodicalReportingCriteriaExt_r18ReportingAmount_r18>,
    pub reporting_interval_ms_r18: PeriodicalReportingCriteriaExt_r18ReportingIntervalMs_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PhysicalReferenceStationInfo_r15 {
    pub physical_reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub physical_arp_ecef_x_r15: PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_X_r15,
    pub physical_arp_ecef_y_r15: PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_Y_r15,
    pub physical_arp_ecef_z_r15: PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_Z_r15,
    #[asn(optional_idx = 0)]
    pub physical_arp_unc_r15: Option<AntennaReferencePointUnc_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "15")]
pub struct Polygon(pub Vec<PolygonPoints>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PolygonPoints {
    pub latitude_sign: PolygonPointsLatitudeSign,
    pub degrees_latitude: PolygonPointsDegreesLatitude,
    pub degrees_longitude: PolygonPointsDegreesLongitude,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18 {
    pub num_of_carriers_intra_band_contiguous_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18,
    #[asn(optional_idx = 0)]
    pub maximum_aggregated_bw_two_carriers_fr1_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18>,
    #[asn(optional_idx = 1)]
    pub maximum_aggregated_bw_two_carriers_fr2_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18>,
    #[asn(optional_idx = 2)]
    pub maximum_aggregated_bw_three_carriers_fr1_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18>,
    #[asn(optional_idx = 3)]
    pub maximum_aggregated_bw_three_carriers_fr2_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18>,
    pub maximum_aggregated_resource_set_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSet_r18,
    pub maximum_aggregated_resource_periodic_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18,
    pub maximum_aggregated_resource_aperiodic_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18,
    pub maximum_aggregated_resource_semi_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18,
    pub maximum_aggregated_resource_periodic_per_slot_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18,
    pub maximum_aggregated_resource_aperiodic_per_slot_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18,
    pub maximum_aggregated_resource_semi_per_slot_r18:
        PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18,
    pub guard_period_r18: PosSRS_BWA_IndependentCA_RRC_Connected_r18GuardPeriod_r18,
    #[asn(optional_idx = 4)]
    pub power_class_for_two_aggregated_carriers_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForTwoAggregatedCarriers_r18>,
    #[asn(optional_idx = 5)]
    pub power_class_for_three_aggregated_carriers_r18:
        Option<PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForThreeAggregatedCarriers_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PosSRS_BWA_RRC_Connected_r18 {
    pub num_of_carriers_intra_band_contiguous_r18:
        PosSRS_BWA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18,
    #[asn(optional_idx = 0)]
    pub maximum_aggregated_bw_two_carriers_fr1_r18:
        Option<PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18>,
    #[asn(optional_idx = 1)]
    pub maximum_aggregated_bw_two_carriers_fr2_r18:
        Option<PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18>,
    #[asn(optional_idx = 2)]
    pub maximum_aggregated_bw_three_carriers_fr1_r18:
        Option<PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18>,
    #[asn(optional_idx = 3)]
    pub maximum_aggregated_bw_three_carriers_fr2_r18:
        Option<PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18>,
    pub maximum_aggregated_resource_set_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSet_r18,
    pub maximum_aggregated_resource_periodic_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18,
    pub maximum_aggregated_resource_aperiodic_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18,
    pub maximum_aggregated_resource_semi_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18,
    pub maximum_aggregated_resource_periodic_per_slot_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18,
    pub maximum_aggregated_resource_aperiodic_per_slot_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18,
    pub maximum_aggregated_resource_semi_per_slot_r18:
        PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct PosSRS_BWA_RRC_Inactive_r18 {
    pub num_of_carriers_intra_band_contiguous_r18:
        PosSRS_BWA_RRC_Inactive_r18NumOfCarriersIntraBandContiguous_r18,
    #[asn(optional_idx = 0)]
    pub maximum_aggregated_bw_two_carriers_fr1_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR1_r18>,
    #[asn(optional_idx = 1)]
    pub maximum_aggregated_bw_two_carriers_fr2_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR2_r18>,
    #[asn(optional_idx = 2)]
    pub maximum_aggregated_bw_three_carriers_fr1_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR1_r18>,
    #[asn(optional_idx = 3)]
    pub maximum_aggregated_bw_three_carriers_fr2_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR2_r18>,
    pub maximum_aggregated_resource_set_r18:
        PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSet_r18,
    pub maximum_aggregated_resource_periodic_r18:
        PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodic_r18,
    pub maximum_aggregated_resource_semi_r18:
        PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemi_r18,
    pub maximum_aggregated_resource_periodic_per_slot_r18:
        PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodicPerSlot_r18,
    pub maximum_aggregated_resource_semi_per_slot_r18:
        PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemiPerSlot_r18,
    pub guard_period_r18: PosSRS_BWA_RRC_Inactive_r18GuardPeriod_r18,
    #[asn(optional_idx = 4)]
    pub power_class_for_two_aggregated_carriers_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18PowerClassForTwoAggregatedCarriers_r18>,
    #[asn(optional_idx = 5)]
    pub power_class_for_three_aggregated_carriers_r18:
        Option<PosSRS_BWA_RRC_Inactive_r18PowerClassForThreeAggregatedCarriers_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 7)]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17 { # [asn (optional_idx = 0 ,)] pub max_num_of_sr_spos_resource_sets_r17 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17 > , # [asn (optional_idx = 1 ,)] pub max_num_of_periodic_and_semi_persistent_sr_spos_resources_r17 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17 > , # [asn (optional_idx = 2 ,)] pub max_num_of_periodic_and_semi_persistent_sr_spos_resources_per_slot_r17 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17 > , # [asn (optional_idx = 3 ,)] pub max_num_of_periodic_sr_spos_resources_r17 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17 > , # [asn (optional_idx = 4 ,)] pub max_num_of_periodic_sr_spos_resources_per_slot_r17 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17 > , # [asn (optional_idx = 5 ,)] pub dummy1 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy1 > , # [asn (optional_idx = 6 ,)] pub dummy2 : Option < PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy2 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 13)]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17 { # [asn (optional_idx = 0 ,)] pub max_sr_spos_bandwidth_for_each_scs_within_cc_fr1_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR1_r17 > , # [asn (optional_idx = 1 ,)] pub max_sr_spos_bandwidth_for_each_scs_within_cc_fr2_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR2_r17 > , # [asn (optional_idx = 2 ,)] pub max_num_of_sr_spos_resource_sets_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17 > , # [asn (optional_idx = 3 ,)] pub max_num_of_periodic_sr_spos_resources_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17 > , # [asn (optional_idx = 4 ,)] pub max_num_of_periodic_sr_spos_resources_per_slot_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17 > , # [asn (optional_idx = 5 ,)] pub different_numerology_between_sr_spos_and_initial_bwp_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentNumerologyBetweenSRSposAndInitialBWP_r17 > , # [asn (optional_idx = 6 ,)] pub srs_pos_without_restriction_on_bwp_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SrsPosWithoutRestrictionOnBWP_r17 > , # [asn (optional_idx = 7 ,)] pub max_num_of_periodic_and_semi_persistent_sr_spos_resources_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17 > , # [asn (optional_idx = 8 ,)] pub max_num_of_periodic_and_semi_persistent_sr_spos_resources_per_slot_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17 > , # [asn (optional_idx = 9 ,)] pub different_center_freq_between_sr_spos_and_initial_bwp_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentCenterFreqBetweenSRSposAndInitialBWP_r17 > , # [asn (optional_idx = 10 ,)] pub max_num_of_semi_persistent_sr_spos_resources_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17 > , # [asn (optional_idx = 11 ,)] pub max_num_of_semi_persistent_sr_spos_resources_per_slot_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17 > , # [asn (optional_idx = 12 ,)] pub switching_time_srs_tx_other_tx_r17 : Option < PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SwitchingTimeSRS_TX_OtherTX_r17 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17 {
    #[asn(optional_idx = 0)]
    pub max_num_of_semi_persistent_sr_spos_resources_r17:
        Option<PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17>,
    #[asn(optional_idx = 1)]
    pub max_num_of_semi_persistent_sr_spos_resources_per_slot_r17: Option<
        PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17,
    >,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18 {
    #[asn(optional_idx = 0)]
    pub maximum_srs_bandwidth_across_all_hops_fr1_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18>,
    #[asn(optional_idx = 1)]
    pub maximum_srs_bandwidth_across_all_hops_fr2_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18>,
    #[asn(optional_idx = 2)]
    pub maximum_tx_fh_hops_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumTxFH_Hops_r18>,
    #[asn(optional_idx = 3)]
    pub rf_tx_retune_time_fr1_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR1_r18>,
    #[asn(optional_idx = 4)]
    pub rf_tx_retune_time_fr2_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR2_r18>,
    #[asn(optional_idx = 5)]
    pub switch_time_between_active_bwp_frequency_hop_r18: Option<
        PosSRS_TxFrequencyHoppingRRC_Connected_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18,
    >,
    #[asn(optional_idx = 6)]
    pub num_of_overlapping_prb_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18NumOfOverlappingPRB_r18>,
    #[asn(optional_idx = 7)]
    pub maximum_srs_resource_periodic_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourcePeriodic_r18>,
    #[asn(optional_idx = 8)]
    pub maximum_srs_resource_aperiodic_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceAperiodic_r18>,
    #[asn(optional_idx = 9)]
    pub maximum_srs_resource_semipersistent_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceSemipersistent_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 10)]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19 { # [asn (optional_idx = 0 ,)] pub maximum_srs_bandwidth_across_all_hops_fr1_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19 > , # [asn (optional_idx = 1 ,)] pub maximum_srs_bandwidth_across_all_hops_fr2_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19 > , # [asn (optional_idx = 2 ,)] pub maximum_tx_fh_hops_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumTxFH_Hops_r19 > , # [asn (optional_idx = 3 ,)] pub rf_tx_retune_time_fr1_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR1_r19 > , # [asn (optional_idx = 4 ,)] pub rf_tx_retune_time_fr2_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR2_r19 > , # [asn (optional_idx = 5 ,)] pub switch_time_between_active_bwp_frequency_hop_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19 > , # [asn (optional_idx = 6 ,)] pub num_of_overlapping_prb_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19NumOfOverlappingPRB_r19 > , # [asn (optional_idx = 7 ,)] pub maximum_srs_resource_periodic_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourcePeriodic_r19 > , # [asn (optional_idx = 8 ,)] pub maximum_srs_resource_aperiodic_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceAperiodic_r19 > , # [asn (optional_idx = 9 ,)] pub maximum_srs_resource_semipersistent_r19 : Option < PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 9)]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18 {
    #[asn(optional_idx = 0)]
    pub maximum_srs_bandwidth_across_all_hops_fr1_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18>,
    #[asn(optional_idx = 1)]
    pub maximum_srs_bandwidth_across_all_hops_fr2_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18>,
    #[asn(optional_idx = 2)]
    pub maximum_tx_fh_hops_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumTxFH_Hops_r18>,
    #[asn(optional_idx = 3)]
    pub rf_tx_retune_time_fr1_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR1_r18>,
    #[asn(optional_idx = 4)]
    pub rf_tx_retune_time_fr2_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR2_r18>,
    #[asn(optional_idx = 5)]
    pub switch_time_between_active_bwp_frequency_hop_r18: Option<
        PosSRS_TxFrequencyHoppingRRC_Inactive_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18,
    >,
    #[asn(optional_idx = 6)]
    pub num_of_overlapping_prb_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18NumOfOverlappingPRB_r18>,
    #[asn(optional_idx = 7)]
    pub maximum_srs_resource_periodic_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourcePeriodic_r18>,
    #[asn(optional_idx = 8)]
    pub maximum_srs_resource_semipersistent_r18:
        Option<PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourceSemipersistent_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 9)]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19 { # [asn (optional_idx = 0 ,)] pub maximum_srs_bandwidth_across_all_hops_fr1_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19 > , # [asn (optional_idx = 1 ,)] pub maximum_srs_bandwidth_across_all_hops_fr2_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19 > , # [asn (optional_idx = 2 ,)] pub maximum_tx_fh_hops_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumTxFH_Hops_r19 > , # [asn (optional_idx = 3 ,)] pub rf_tx_retune_time_fr1_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR1_r19 > , # [asn (optional_idx = 4 ,)] pub rf_tx_retune_time_fr2_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR2_r19 > , # [asn (optional_idx = 5 ,)] pub switch_time_between_active_bwp_frequency_hop_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19 > , # [asn (optional_idx = 6 ,)] pub num_of_overlapping_prb_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19NumOfOverlappingPRB_r19 > , # [asn (optional_idx = 7 ,)] pub maximum_srs_resource_periodic_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourcePeriodic_r19 > , # [asn (optional_idx = 8 ,)] pub maximum_srs_resource_semipersistent_r19 : Option < PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PositioningModes {
    pub pos_modes: PositioningModesPosModes,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PressureValidityArea_v1520 {
    pub center_point_v1520: Ellipsoid_Point,
    pub validity_area_width_v1520: PressureValidityArea_v1520ValidityAreaWidth_v1520,
    pub validity_area_height_v1520: PressureValidityArea_v1520ValidityAreaHeight_v1520,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct PressureValidityPeriod_v1520 {
    pub begin_time_v1520: GNSS_SystemTime,
    #[asn(optional_idx = 0)]
    pub begin_time_alt_v1520: Option<PressureValidityPeriod_v1520BeginTimeAlt_v1520>,
    pub duration_v1520: PressureValidityPeriod_v1520Duration_v1520,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideAssistanceData {
    pub critical_extensions: ProvideAssistanceDataCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ProvideAssistanceData_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_provide_assistance_data: Option<CommonIEsProvideAssistanceData>,
    #[asn(optional_idx = 1)]
    pub a_gnss_provide_assistance_data: Option<A_GNSS_ProvideAssistanceData>,
    #[asn(optional_idx = 2)]
    pub otdoa_provide_assistance_data: Option<OTDOA_ProvideAssistanceData>,
    #[asn(optional_idx = 3)]
    pub epdu_provide_assistance_data: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideCapabilities {
    pub critical_extensions: ProvideCapabilitiesCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ProvideCapabilities_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_provide_capabilities: Option<CommonIEsProvideCapabilities>,
    #[asn(optional_idx = 1)]
    pub a_gnss_provide_capabilities: Option<A_GNSS_ProvideCapabilities>,
    #[asn(optional_idx = 2)]
    pub otdoa_provide_capabilities: Option<OTDOA_ProvideCapabilities>,
    #[asn(optional_idx = 3)]
    pub ecid_provide_capabilities: Option<ECID_ProvideCapabilities>,
    #[asn(optional_idx = 4)]
    pub epdu_provide_capabilities: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideLocationInformation {
    pub critical_extensions: ProvideLocationInformationCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ProvideLocationInformation_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_provide_location_information: Option<CommonIEsProvideLocationInformation>,
    #[asn(optional_idx = 1)]
    pub a_gnss_provide_location_information: Option<A_GNSS_ProvideLocationInformation>,
    #[asn(optional_idx = 2)]
    pub otdoa_provide_location_information: Option<OTDOA_ProvideLocationInformation>,
    #[asn(optional_idx = 3)]
    pub ecid_provide_location_information: Option<ECID_ProvideLocationInformation>,
    #[asn(optional_idx = 4)]
    pub epdu_provide_location_information: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct QoS {
    #[asn(optional_idx = 0)]
    pub horizontal_accuracy: Option<HorizontalAccuracy>,
    pub vertical_coordinate_request: QoSVerticalCoordinateRequest,
    #[asn(optional_idx = 1)]
    pub vertical_accuracy: Option<VerticalAccuracy>,
    #[asn(optional_idx = 2)]
    pub response_time: Option<ResponseTime>,
    pub velocity_request: QoSVelocityRequest,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RAC_OrbitalErrorComponents_r17 {
    pub radial_r17: RAC_OrbitalErrorComponents_r17Radial_r17,
    pub along_track_r17: RAC_OrbitalErrorComponents_r17AlongTrack_r17,
    pub cross_track_r17: RAC_OrbitalErrorComponents_r17CrossTrack_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct RTD_InfoElement_r16 {
    pub dl_prs_id_r16: RTD_InfoElement_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    pub subframe_offset_r16: RTD_InfoElement_r16SubframeOffset_r16,
    pub rtd_quality_r16: NR_TimingQuality_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct RTD_InfoList_r16(pub Vec<RTD_InfoListPerFreqLayer_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct RTD_InfoListPerFreqLayer_r16(pub Vec<RTD_InfoElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RTK_CorrectionDifferencesElement_r15 {
    pub epoch_time_r15: GNSS_SystemTime,
    pub auxiliary_reference_station_id_r15: GNSS_ReferenceStationID_r15,
    pub geometric_ionospheric_corrections_differences_r15:
        Geometric_Ionospheric_Corrections_Differences_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct RTK_CorrectionDifferencesList_r15(pub Vec<RTK_CorrectionDifferencesElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RTK_Residuals_Element_r15 {
    pub sv_id_r15: SV_ID,
    pub s_oc_r15: RTK_Residuals_Element_r15S_oc_r15,
    pub s_od_r15: RTK_Residuals_Element_r15S_od_r15,
    pub s_oh_r15: RTK_Residuals_Element_r15S_oh_r15,
    pub s_lc_r15: RTK_Residuals_Element_r15S_lc_r15,
    pub s_ld_r15: RTK_Residuals_Element_r15S_ld_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct RTK_Residuals_List_r15(pub Vec<RTK_Residuals_Element_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ReferencePoint_r16 {
    pub reference_point_geographic_location_r16:
        ReferencePoint_r16ReferencePointGeographicLocation_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct ReferenceStationList_r16(pub Vec<GNSS_ReferenceStationID_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct ReferenceTRP_RTD_Info_r16 {
    pub dl_prs_id_ref_r16: ReferenceTRP_RTD_Info_r16Dl_PRS_ID_Ref_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_ref_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_ref_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_ref_r16: Option<ARFCN_ValueNR_r15>,
    pub ref_time_r16: ReferenceTRP_RTD_Info_r16RefTime_r16,
    #[asn(optional_idx = 3)]
    pub rtd_ref_quality_r16: Option<NR_TimingQuality_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RegionIgpElement_r16 {
    pub region_id_r16: RegionIgpElement_r16RegionID_r16,
    pub givei1_r16: RegionIgpElement_r16Givei1_r16,
    pub givd1_r16: RegionIgpElement_r16Givd1_r16,
    pub givei2_r16: RegionIgpElement_r16Givei2_r16,
    pub givd2_r16: RegionIgpElement_r16Givd2_r16,
    pub givei3_r16: RegionIgpElement_r16Givei3_r16,
    pub givd3_r16: RegionIgpElement_r16Givd3_r16,
    pub givei4_r16: RegionIgpElement_r16Givei4_r16,
    pub givd4_r16: RegionIgpElement_r16Givd4_r16,
    pub givei5_r16: RegionIgpElement_r16Givei5_r16,
    pub givd5_r16: RegionIgpElement_r16Givd5_r16,
    pub givei6_r16: RegionIgpElement_r16Givei6_r16,
    pub givd6_r16: RegionIgpElement_r16Givd6_r16,
    pub givei7_r16: RegionIgpElement_r16Givei7_r16,
    pub givd7_r16: RegionIgpElement_r16Givd7_r16,
    pub givei8_r16: RegionIgpElement_r16Givei8_r16,
    pub givd8_r16: RegionIgpElement_r16Givd8_r16,
    pub givei9_r16: RegionIgpElement_r16Givei9_r16,
    pub givd9_r16: RegionIgpElement_r16Givd9_r16,
    pub givei10_r16: RegionIgpElement_r16Givei10_r16,
    pub givd10_r16: RegionIgpElement_r16Givd10_r16,
    pub givei11_r16: RegionIgpElement_r16Givei11_r16,
    pub givd11_r16: RegionIgpElement_r16Givd11_r16,
    pub givei12_r16: RegionIgpElement_r16Givei12_r16,
    pub givd12_r16: RegionIgpElement_r16Givd12_r16,
    pub givei13_r16: RegionIgpElement_r16Givei13_r16,
    pub givd13_r16: RegionIgpElement_r16Givd13_r16,
    pub givei14_r16: RegionIgpElement_r16Givei14_r16,
    pub givd14_r16: RegionIgpElement_r16Givd14_r16,
    pub givei15_r16: RegionIgpElement_r16Givei15_r16,
    pub givd15_r16: RegionIgpElement_r16Givd15_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct RegionIgpList_r16(pub Vec<RegionIgpElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelativeCartesianLocation_r18 {
    pub cartesian_coordinates_units_r18: RelativeCartesianLocation_r18CartesianCoordinatesUnits_r18,
    pub x_value_r18: X_Value_r18,
    pub y_value_r18: Y_Value_r18,
    pub z_value_r18: Z_Value_r18,
    #[asn(optional_idx = 0)]
    pub location_unc_r18: Option<LocationUncertainty_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct RelativeLocation_r16 {
    pub milli_arc_second_units_r16: RelativeLocation_r16Milli_arc_second_units_r16,
    pub height_units_r16: RelativeLocation_r16Height_units_r16,
    pub delta_latitude_r16: Delta_Latitude_r16,
    pub delta_longitude_r16: Delta_Longitude_r16,
    pub delta_height_r16: Delta_Height_r16,
    #[asn(optional_idx = 0)]
    pub location_unc_r16: Option<LocationUncertainty_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct RelativeLocationElement_r16 {
    pub delta_latitude_r16: RelativeLocationElement_r16DeltaLatitude_r16,
    pub delta_longitude_r16: RelativeLocationElement_r16DeltaLongitude_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ReportingDuration(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct ReqNavListInfo {
    pub sv_req_list: ReqNavListInfoSvReqList,
    #[asn(optional_idx = 0)]
    pub clock_model_id_pref_list: Option<ReqNavListInfoClockModelID_PrefList>,
    #[asn(optional_idx = 1)]
    pub orbit_model_id_pref_list: Option<ReqNavListInfoOrbitModelID_PrefList>,
    #[asn(optional_idx = 2)]
    pub add_navparam_req: Option<ReqNavListInfoAddNavparamReq>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestAssistanceData {
    pub critical_extensions: RequestAssistanceDataCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct RequestAssistanceData_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_request_assistance_data: Option<CommonIEsRequestAssistanceData>,
    #[asn(optional_idx = 1)]
    pub a_gnss_request_assistance_data: Option<A_GNSS_RequestAssistanceData>,
    #[asn(optional_idx = 2)]
    pub otdoa_request_assistance_data: Option<OTDOA_RequestAssistanceData>,
    #[asn(optional_idx = 3)]
    pub epdu_request_assistance_data: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestCapabilities {
    pub critical_extensions: RequestCapabilitiesCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RequestCapabilities_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_request_capabilities: Option<CommonIEsRequestCapabilities>,
    #[asn(optional_idx = 1)]
    pub a_gnss_request_capabilities: Option<A_GNSS_RequestCapabilities>,
    #[asn(optional_idx = 2)]
    pub otdoa_request_capabilities: Option<OTDOA_RequestCapabilities>,
    #[asn(optional_idx = 3)]
    pub ecid_request_capabilities: Option<ECID_RequestCapabilities>,
    #[asn(optional_idx = 4)]
    pub epdu_request_capabilities: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestLocationInformation {
    pub critical_extensions: RequestLocationInformationCriticalExtensions,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct RequestLocationInformation_r9_IEs {
    #[asn(optional_idx = 0)]
    pub common_i_es_request_location_information: Option<CommonIEsRequestLocationInformation>,
    #[asn(optional_idx = 1)]
    pub a_gnss_request_location_information: Option<A_GNSS_RequestLocationInformation>,
    #[asn(optional_idx = 2)]
    pub otdoa_request_location_information: Option<OTDOA_RequestLocationInformation>,
    #[asn(optional_idx = 3)]
    pub ecid_request_location_information: Option<ECID_RequestLocationInformation>,
    #[asn(optional_idx = 4)]
    pub epdu_request_location_information: Option<EPDU_Sequence>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct ResponseTime {
    pub time: ResponseTimeTime,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct ResponseTimeNB_r14 {
    pub time_nb_r14: ResponseTimeNB_r14TimeNB_r14,
    #[asn(optional_idx = 0)]
    pub response_time_early_fix_nb_r14: Option<ResponseTimeNB_r14ResponseTimeEarlyFixNB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultsPerCSI_RS_Index_r16 {
    pub csi_rs_index_r16: ResultsPerCSI_RS_Index_r16Csi_RS_Index_r16,
    pub csi_rs_results_r16: MeasQuantityResults_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct ResultsPerCSI_RS_IndexList_r16(pub Vec<ResultsPerCSI_RS_Index_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ResultsPerSSB_Index_r16 {
    pub ssb_index_r16: ResultsPerSSB_Index_r16Ssb_Index_r16,
    pub ssb_results_r16: MeasQuantityResults_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct ResultsPerSSB_IndexList_r16(pub Vec<ResultsPerSSB_Index_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "15")]
pub struct RxTxTEG_TimingErrorMargin_r17(pub u8);
impl RxTxTEG_TimingErrorMargin_r17 {
    pub const TC0_5: u8 = 0u8;
    pub const TC1: u8 = 1u8;
    pub const TC2: u8 = 2u8;
    pub const TC4: u8 = 3u8;
    pub const TC8: u8 = 4u8;
    pub const TC12: u8 = 5u8;
    pub const TC16: u8 = 6u8;
    pub const TC20: u8 = 7u8;
    pub const TC24: u8 = 8u8;
    pub const TC32: u8 = 9u8;
    pub const TC40: u8 = 10u8;
    pub const TC48: u8 = 11u8;
    pub const TC64: u8 = 12u8;
    pub const TC80: u8 = 13u8;
    pub const TC96: u8 = 14u8;
    pub const TC128: u8 = 15u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SBAS_ClockModel {
    pub sbas_to: SBAS_ClockModelSbasTo,
    pub sbas_agfo: SBAS_ClockModelSbasAgfo,
    pub sbas_agf1: SBAS_ClockModelSbasAgf1,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SBAS_ID {
    pub sbas_id: SBAS_IDSbas_id,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SBAS_IDs {
    pub sbas_i_ds: SBAS_IDsSbas_IDs,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SFN_r15 {
    pub sfn_r15: SFN_r15Sfn_r15,
    #[asn(optional_idx = 0)]
    pub hyper_sfn_r15: Option<SFN_r15HyperSFN_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SRS_CapabilityPerBand_r16 {
    pub freq_band_indicator_nr_r16: FreqBandIndicatorNR_r16,
    #[asn(optional_idx = 0)]
    pub olpc_srs_pos_r16: Option<OLPC_SRS_Pos_r16>,
    #[asn(optional_idx = 1)]
    pub spatial_relations_srs_pos_r16: Option<SpatialRelationsSRS_Pos_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SRS_PosResourcesPerBand_r16 {
    pub freq_band_indicator_nr_r16: FreqBandIndicatorNR_r16,
    pub max_number_srs_pos_resource_sets_per_bwp_r16:
        SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourceSetsPerBWP_r16,
    pub max_number_srs_pos_resources_per_bwp_r16:
        SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourcesPerBWP_r16,
    pub max_number_periodic_srs_pos_resources_per_bwp_r16:
        SRS_PosResourcesPerBand_r16MaxNumberPeriodicSRS_PosResourcesPerBWP_r16,
    #[asn(optional_idx = 0)]
    pub max_number_ap_srs_pos_resources_per_bwp_r16:
        Option<SRS_PosResourcesPerBand_r16MaxNumberAP_SRS_PosResourcesPerBWP_r16>,
    #[asn(optional_idx = 1)]
    pub max_number_sp_srs_pos_resources_per_bwp_r16:
        Option<SRS_PosResourcesPerBand_r16MaxNumberSP_SRS_PosResourcesPerBWP_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_ClockCorrectionList_r15(pub Vec<SSR_ClockCorrectionSatelliteElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SSR_ClockCorrectionSatelliteElement_r15 {
    pub sv_id_r15: SV_ID,
    pub delta_clock_c0_r15: SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C0_r15,
    #[asn(optional_idx = 0)]
    pub delta_clock_c1_r15: Option<SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C1_r15>,
    #[asn(optional_idx = 1)]
    pub delta_clock_c2_r15: Option<SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C2_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_CodeBiasSatElement_r15 {
    pub sv_id_r15: SV_ID,
    pub ssr_code_bias_signal_list_r15: SSR_CodeBiasSignalList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_CodeBiasSatList_r15(pub Vec<SSR_CodeBiasSatElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_CodeBiasSignalElement_r15 {
    pub signal_and_tracking_mode_id_r15: GNSS_SignalID,
    pub code_bias_r15: SSR_CodeBiasSignalElement_r15CodeBias_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct SSR_CodeBiasSignalList_r15(pub Vec<SSR_CodeBiasSignalElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SSR_GriddedCorrectionIntegrityParameters_r17 {
    pub prob_onset_troposphere_fault_r17:
        SSR_GriddedCorrectionIntegrityParameters_r17ProbOnsetTroposphereFault_r17,
    pub mean_troposphere_fault_duration_r17:
        SSR_GriddedCorrectionIntegrityParameters_r17MeanTroposphereFaultDuration_r17,
    #[asn(optional_idx = 0)]
    pub troposphere_range_error_correlation_time_r17: Option<
        SSR_GriddedCorrectionIntegrityParameters_r17TroposphereRangeErrorCorrelationTime_r17,
    >,
    #[asn(optional_idx = 1)]
    pub troposphere_range_rate_error_correlation_time_r17: Option<
        SSR_GriddedCorrectionIntegrityParameters_r17TroposphereRangeRateErrorCorrelationTime_r17,
    >,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_IntegrityClockBounds_r17 {
    pub mean_clock_r17: SSR_IntegrityClockBounds_r17MeanClock_r17,
    pub std_dev_clock_r17: SSR_IntegrityClockBounds_r17StdDevClock_r17,
    pub mean_clock_rate_r17: SSR_IntegrityClockBounds_r17MeanClockRate_r17,
    pub std_dev_clock_rate_r17: SSR_IntegrityClockBounds_r17StdDevClockRate_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_IntegrityCodeBiasBounds_r17 {
    pub mean_code_bias_r17: SSR_IntegrityCodeBiasBounds_r17MeanCodeBias_r17,
    pub std_dev_code_bias_r17: SSR_IntegrityCodeBiasBounds_r17StdDevCodeBias_r17,
    pub mean_code_bias_rate_r17: SSR_IntegrityCodeBiasBounds_r17MeanCodeBiasRate_r17,
    pub std_dev_code_bias_rate_r17: SSR_IntegrityCodeBiasBounds_r17StdDevCodeBiasRate_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_IntegrityOrbitBounds_r17 {
    pub mean_orbit_error_r17: RAC_OrbitalErrorComponents_r17,
    pub std_dev_orbit_error_r17: RAC_OrbitalErrorComponents_r17,
    pub mean_orbit_rate_error_r17: RAC_OrbitalErrorComponents_r17,
    pub std_dev_orbit_rate_error_r17: RAC_OrbitalErrorComponents_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_IntegrityPhaseBiasBounds_r17 {
    pub mean_phase_bias_r17: SSR_IntegrityPhaseBiasBounds_r17MeanPhaseBias_r17,
    pub std_dev_phase_bias_r17: SSR_IntegrityPhaseBiasBounds_r17StdDevPhaseBias_r17,
    pub mean_phase_bias_rate_r17: SSR_IntegrityPhaseBiasBounds_r17MeanPhaseBiasRate_r17,
    pub std_dev_phase_bias_rate_r17: SSR_IntegrityPhaseBiasBounds_r17StdDevPhaseBiasRate_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_OrbitCorrectionList_r15(pub Vec<SSR_OrbitCorrectionSatelliteElement_r15>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct SSR_OrbitCorrectionSatelliteElement_r15 {
    pub sv_id_r15: SV_ID,
    pub iod_r15: SSR_OrbitCorrectionSatelliteElement_r15Iod_r15,
    pub delta_radial_r15: SSR_OrbitCorrectionSatelliteElement_r15Delta_radial_r15,
    pub delta_along_track_r15: SSR_OrbitCorrectionSatelliteElement_r15Delta_AlongTrack_r15,
    pub delta_cross_track_r15: SSR_OrbitCorrectionSatelliteElement_r15Delta_CrossTrack_r15,
    #[asn(optional_idx = 0)]
    pub dot_delta_radial_r15: Option<SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_radial_r15>,
    #[asn(optional_idx = 1)]
    pub dot_delta_along_track_r15:
        Option<SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_AlongTrack_r15>,
    #[asn(optional_idx = 2)]
    pub dot_delta_cross_track_r15:
        Option<SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_CrossTrack_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_PhaseBiasSatElement_r16 {
    pub sv_id_r16: SV_ID,
    pub ssr_phase_bias_signal_list_r16: SSR_PhaseBiasSignalList_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_PhaseBiasSatList_r16(pub Vec<SSR_PhaseBiasSatElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct SSR_PhaseBiasSignalElement_r16 {
    pub signal_and_tracking_mode_id_r16: GNSS_SignalID,
    pub phase_bias_r16: SSR_PhaseBiasSignalElement_r16PhaseBias_r16,
    pub phase_discontinuity_indicator_r16:
        SSR_PhaseBiasSignalElement_r16PhaseDiscontinuityIndicator_r16,
    #[asn(optional_idx = 0)]
    pub phase_bias_integer_indicator_r16:
        Option<SSR_PhaseBiasSignalElement_r16PhaseBiasIntegerIndicator_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct SSR_PhaseBiasSignalList_r16(pub Vec<SSR_PhaseBiasSignalElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct SSR_PhaseCenterVariationList_r18(pub Vec<SSR_PhaseCenterVariationList_r18_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_SatellitePCV_Element_r18 {
    pub sv_id_r18: SV_ID,
    pub ssr_satellite_pcv_frequency_list_r18: SSR_SatellitePCV_FrequencyList_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_SatellitePCV_FrequencyElement_r18 {
    pub frequency_id_r18: GNSS_FrequencyID_r15,
    pub phase_center_variations_r18: SSR_PhaseCenterVariationList_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SSR_SatellitePCV_FrequencyList_r18(pub Vec<SSR_SatellitePCV_FrequencyElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_SatellitePCV_List_r18(pub Vec<SSR_SatellitePCV_Element_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SSR_URA_SatElement_r16 {
    pub sv_id_r16: SV_ID,
    pub ssr_ura_r16: SSR_URA_SatElement_r16Ssr_URA_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SSR_URA_SatList_r16(pub Vec<SSR_URA_SatElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct STEC_IntegrityErrorBounds_r17 {
    pub mean_ionosphere_r17: STEC_IntegrityErrorBounds_r17MeanIonosphere_r17,
    pub std_dev_ionosphere_r17: STEC_IntegrityErrorBounds_r17StdDevIonosphere_r17,
    pub mean_ionosphere_rate_r17: STEC_IntegrityErrorBounds_r17MeanIonosphereRate_r17,
    pub std_dev_ionosphere_rate_r17: STEC_IntegrityErrorBounds_r17StdDevIonosphereRate_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct STEC_IntegrityParameters_r17 {
    pub prob_onset_iono_fault_r17: STEC_IntegrityParameters_r17ProbOnsetIonoFault_r17,
    pub mean_iono_fault_duration_r17: STEC_IntegrityParameters_r17MeanIonoFaultDuration_r17,
    #[asn(optional_idx = 0)]
    pub iono_range_error_correlation_time_r17:
        Option<STEC_IntegrityParameters_r17IonoRangeErrorCorrelationTime_r17>,
    #[asn(optional_idx = 1)]
    pub iono_range_rate_error_correlation_time_r17:
        Option<STEC_IntegrityParameters_r17IonoRangeRateErrorCorrelationTime_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct STEC_ResidualSatElement_r16 {
    pub sv_id_r16: SV_ID,
    pub stec_residual_correction_r16: STEC_ResidualSatElement_r16StecResidualCorrection_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct STEC_ResidualSatList_r16(pub Vec<STEC_ResidualSatElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct STEC_SatElement_r16 {
    pub sv_id_r16: SV_ID,
    pub stec_quality_indicator_r16: STEC_SatElement_r16StecQualityIndicator_r16,
    pub stec_c00_r16: STEC_SatElement_r16Stec_C00_r16,
    #[asn(optional_idx = 0)]
    pub stec_c01_r16: Option<STEC_SatElement_r16Stec_C01_r16>,
    #[asn(optional_idx = 1)]
    pub stec_c10_r16: Option<STEC_SatElement_r16Stec_C10_r16>,
    #[asn(optional_idx = 2)]
    pub stec_c11_r16: Option<STEC_SatElement_r16Stec_C11_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct STEC_SatList_r16(pub Vec<STEC_SatElement_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SV_ID {
    pub satellite_id: SV_IDSatellite_id,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct SatListElement_r15 {
    pub sv_id_r15: SV_ID,
    pub iod_r15: SatListElement_r15Iod_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct SatListRelatedDataElement {
    pub sv_id: SV_ID,
    pub iod: SatListRelatedDataElementIod,
    #[asn(optional_idx = 0)]
    pub clock_model_id: Option<SatListRelatedDataElementClockModelID>,
    #[asn(optional_idx = 1)]
    pub orbit_model_id: Option<SatListRelatedDataElementOrbitModelID>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct SatListRelatedDataList(pub Vec<SatListRelatedDataElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 4)]
pub struct ScheduledLocationTime_r17 {
    #[asn(optional_idx = 0)]
    pub utc_time_r17: Option<ScheduledLocationTime_r17UtcTime_r17>,
    #[asn(optional_idx = 1)]
    pub gnss_time_r17: Option<ScheduledLocationTime_r17GnssTime_r17>,
    #[asn(optional_idx = 2)]
    pub network_time_r17: Option<ScheduledLocationTime_r17NetworkTime_r17>,
    #[asn(optional_idx = 3)]
    pub relative_time_r17: Option<ScheduledLocationTime_r17RelativeTime_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ScheduledLocationTimeSupport_r17 {
    #[asn(optional_idx = 0)]
    pub utc_time_r17: Option<ScheduledLocationTimeSupport_r17UtcTime_r17>,
    #[asn(optional_idx = 1)]
    pub gnss_time_r17: Option<GNSS_ID_Bitmap>,
    #[asn(optional_idx = 2)]
    pub e_utra_time_r17: Option<ScheduledLocationTimeSupport_r17E_utraTime_r17>,
    #[asn(optional_idx = 3)]
    pub nr_time_r17: Option<ScheduledLocationTimeSupport_r17NrTime_r17>,
    #[asn(optional_idx = 4)]
    pub relative_time_r17: Option<ScheduledLocationTimeSupport_r17RelativeTime_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 5)]
pub struct ScheduledLocationTimeSupportPerMode_r17 {
    #[asn(optional_idx = 0)]
    pub utc_time_r17: Option<PositioningModes>,
    #[asn(optional_idx = 1)]
    pub gnss_time_r17: Option<ScheduledLocationTimeSupportPerMode_r17GnssTime_r17>,
    #[asn(optional_idx = 2)]
    pub e_utra_time_r17: Option<PositioningModes>,
    #[asn(optional_idx = 3)]
    pub nr_time_r17: Option<PositioningModes>,
    #[asn(optional_idx = 4)]
    pub relative_time_r17: Option<PositioningModes>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct SegmentationInfo_r14(pub u8);
impl SegmentationInfo_r14 {
    pub const NO_MORE_MESSAGES: u8 = 0u8;
    pub const MORE_MESSAGES_ON_THE_WAY: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Sensor_AssistanceDataList_r14 {
    pub ref_pressure_r14: Sensor_AssistanceDataList_r14RefPressure_r14,
    #[asn(optional_idx = 0)]
    pub ref_position_r14: Option<EllipsoidPointWithAltitudeAndUncertaintyEllipsoid>,
    #[asn(optional_idx = 1)]
    pub ref_temperature_r14: Option<Sensor_AssistanceDataList_r14RefTemperature_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_AssistanceDataSupportList_r14 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum Sensor_Error_r13 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r13(Sensor_LocationServerErrorCauses_r13),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r13(Sensor_TargetDeviceErrorCauses_r13),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_LocationServerErrorCauses_r13 {
    pub cause_r13: Sensor_LocationServerErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Sensor_MeasurementInformation_r13 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r13:
        Option<Sensor_MeasurementInformation_r13MeasurementReferenceTime_r13>,
    #[asn(optional_idx = 1)]
    pub uncompensated_barometric_pressure_r13:
        Option<Sensor_MeasurementInformation_r13UncompensatedBarometricPressure_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_MotionInformation_r15 {
    pub ref_time_r15: DisplacementTimeStamp_r15,
    pub displacement_info_list_r15: DisplacementInfoList_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Sensor_ProvideAssistanceData_r14 {
    #[asn(optional_idx = 0)]
    pub sensor_assistance_data_list_r14: Option<Sensor_AssistanceDataList_r14>,
    #[asn(optional_idx = 1)]
    pub sensor_error_r14: Option<Sensor_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_ProvideCapabilities_r13 {
    pub sensor_modes_r13: Sensor_ProvideCapabilities_r13Sensor_Modes_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct Sensor_ProvideLocationInformation_r13 {
    #[asn(optional_idx = 0)]
    pub sensor_measurement_information_r13: Option<Sensor_MeasurementInformation_r13>,
    #[asn(optional_idx = 1)]
    pub sensor_error_r13: Option<Sensor_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_RequestAssistanceData_r14 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_RequestCapabilities_r13 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_RequestLocationInformation_r13 {
    pub uncompensated_barometric_pressure_req_r13:
        Sensor_RequestLocationInformation_r13UncompensatedBarometricPressureReq_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct Sensor_TargetDeviceErrorCauses_r13 {
    pub cause_r13: Sensor_TargetDeviceErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SequenceNumber(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct SpatialDelta_r18(pub u8);
impl SpatialDelta_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N10: u8 = 5u8;
    pub const N20: u8 = 6u8;
    pub const N50: u8 = 7u8;
    pub const N100: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct SpatialRelationsSRS_Pos_r16 {
    #[asn(optional_idx = 0)]
    pub spatial_relation_srs_pos_based_on_ssb_serving_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Serving_r16>,
    #[asn(optional_idx = 1)]
    pub spatial_relation_srs_pos_based_on_csi_rs_serving_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnCSI_RS_Serving_r16>,
    #[asn(optional_idx = 2)]
    pub spatial_relation_srs_pos_based_on_prs_serving_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Serving_r16>,
    #[asn(optional_idx = 3)]
    pub spatial_relation_srs_pos_based_on_srs_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSRS_r16>,
    #[asn(optional_idx = 4)]
    pub spatial_relation_srs_pos_based_on_ssb_neigh_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Neigh_r16>,
    #[asn(optional_idx = 5)]
    pub spatial_relation_srs_pos_based_on_prs_neigh_r16:
        Option<SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Neigh_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct StandardClockModelElement {
    pub stan_clock_toc: StandardClockModelElementStanClockToc,
    pub stan_clock_af2: StandardClockModelElementStanClockAF2,
    pub stan_clock_af1: StandardClockModelElementStanClockAF1,
    pub stan_clock_af0: StandardClockModelElementStanClockAF0,
    #[asn(optional_idx = 0)]
    pub stan_clock_tgd: Option<StandardClockModelElementStanClockTgd>,
    pub sisa: StandardClockModelElementSisa,
    #[asn(optional_idx = 1)]
    pub stan_model_id: Option<StandardClockModelElementStanModelID>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct StandardClockModelList(pub Vec<StandardClockModelElement>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct StoredNavListInfo {
    pub gnss_week_or_day: StoredNavListInfoGnss_WeekOrDay,
    pub gnss_toe: StoredNavListInfoGnss_Toe,
    pub t_toe_limit: StoredNavListInfoT_toeLimit,
    #[asn(optional_idx = 0)]
    pub sat_list_related_data_list: Option<SatListRelatedDataList>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedBandEUTRA {
    pub band_eutra: SupportedBandEUTRABandEUTRA,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct SupportedBandEUTRA_v9a0 {
    #[asn(optional_idx = 0)]
    pub band_eutra_v9a0: Option<SupportedBandEUTRA_v9a0BandEUTRA_v9a0>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedChannels_11a_r14 {
    pub ch34_r14: SupportedChannels_11a_r14Ch34_r14,
    pub ch36_r14: SupportedChannels_11a_r14Ch36_r14,
    pub ch38_r14: SupportedChannels_11a_r14Ch38_r14,
    pub ch40_r14: SupportedChannels_11a_r14Ch40_r14,
    pub ch42_r14: SupportedChannels_11a_r14Ch42_r14,
    pub ch44_r14: SupportedChannels_11a_r14Ch44_r14,
    pub ch46_r14: SupportedChannels_11a_r14Ch46_r14,
    pub ch48_r14: SupportedChannels_11a_r14Ch48_r14,
    pub ch52_r14: SupportedChannels_11a_r14Ch52_r14,
    pub ch56_r14: SupportedChannels_11a_r14Ch56_r14,
    pub ch60_r14: SupportedChannels_11a_r14Ch60_r14,
    pub ch64_r14: SupportedChannels_11a_r14Ch64_r14,
    pub ch149_r14: SupportedChannels_11a_r14Ch149_r14,
    pub ch153_r14: SupportedChannels_11a_r14Ch153_r14,
    pub ch157_r14: SupportedChannels_11a_r14Ch157_r14,
    pub ch161_r14: SupportedChannels_11a_r14Ch161_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct SupportedChannels_11bg_r14 {
    pub ch1_r14: SupportedChannels_11bg_r14Ch1_r14,
    pub ch2_r14: SupportedChannels_11bg_r14Ch2_r14,
    pub ch3_r14: SupportedChannels_11bg_r14Ch3_r14,
    pub ch4_r14: SupportedChannels_11bg_r14Ch4_r14,
    pub ch5_r14: SupportedChannels_11bg_r14Ch5_r14,
    pub ch6_r14: SupportedChannels_11bg_r14Ch6_r14,
    pub ch7_r14: SupportedChannels_11bg_r14Ch7_r14,
    pub ch8_r14: SupportedChannels_11bg_r14Ch8_r14,
    pub ch9_r14: SupportedChannels_11bg_r14Ch9_r14,
    pub ch10_r14: SupportedChannels_11bg_r14Ch10_r14,
    pub ch11_r14: SupportedChannels_11bg_r14Ch11_r14,
    pub ch12_r14: SupportedChannels_11bg_r14Ch12_r14,
    pub ch13_r14: SupportedChannels_11bg_r14Ch13_r14,
    pub ch14_r14: SupportedChannels_11bg_r14Ch14_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct TBS_AssistanceDataList_r14 {
    #[asn(optional_idx = 0)]
    pub mbs_assistance_data_list_r14: Option<MBS_AssistanceDataList_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum TBS_Error_r13 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r13(TBS_LocationServerErrorCauses_r13),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r13(TBS_TargetDeviceErrorCauses_r13),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_LocationServerErrorCauses_r13 {
    pub cause_r13: TBS_LocationServerErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TBS_MeasurementInformation_r13 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r13:
        Option<TBS_MeasurementInformation_r13MeasurementReferenceTime_r13>,
    #[asn(optional_idx = 1)]
    pub mbs_sgn_meas_list_r13: Option<MBS_BeaconMeasList_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TBS_ProvideAssistanceData_r14 {
    #[asn(optional_idx = 0)]
    pub tbs_assistance_data_list_r14: Option<TBS_AssistanceDataList_r14>,
    #[asn(optional_idx = 1)]
    pub tbs_error_r14: Option<TBS_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_ProvideCapabilities_r13 {
    pub tbs_modes_r13: TBS_ProvideCapabilities_r13Tbs_Modes_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct TBS_ProvideLocationInformation_r13 {
    #[asn(optional_idx = 0)]
    pub tbs_measurement_information_r13: Option<TBS_MeasurementInformation_r13>,
    #[asn(optional_idx = 1)]
    pub tbs_error_r13: Option<TBS_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_RequestAssistanceData_r14 {
    pub mbs_almanac_assistance_data_req_r14:
        TBS_RequestAssistanceData_r14Mbs_AlmanacAssistanceDataReq_r14,
    pub mbs_acquisition_assistance_data_req_r14:
        TBS_RequestAssistanceData_r14Mbs_AcquisitionAssistanceDataReq_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_RequestCapabilities_r13 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_RequestLocationInformation_r13 {
    pub mbs_sgn_meas_list_req_r13: TBS_RequestLocationInformation_r13MbsSgnMeasListReq_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TBS_TargetDeviceErrorCauses_r13 {
    pub cause_r13: TBS_TargetDeviceErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TDD_Config_v1520 {
    pub subframe_assignment_v1520: TDD_Config_v1520SubframeAssignment_v1520,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "15")]
pub struct TEG_TimingErrorMargin_r17(pub u8);
impl TEG_TimingErrorMargin_r17 {
    pub const TC0: u8 = 0u8;
    pub const TC2: u8 = 1u8;
    pub const TC4: u8 = 2u8;
    pub const TC6: u8 = 3u8;
    pub const TC8: u8 = 4u8;
    pub const TC12: u8 = 5u8;
    pub const TC16: u8 = 6u8;
    pub const TC20: u8 = 7u8;
    pub const TC24: u8 = 8u8;
    pub const TC32: u8 = 9u8;
    pub const TC40: u8 = 10u8;
    pub const TC48: u8 = 11u8;
    pub const TC56: u8 = 12u8;
    pub const TC64: u8 = 13u8;
    pub const TC72: u8 = 14u8;
    pub const TC80: u8 = 15u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TRP_LocationInfo_Implicit_Element_r19 {
    pub nr_cell_global_id_r19: NCGI_r15,
    pub nr_aiml_associated_id_r19: TRP_LocationInfo_Implicit_Element_r19Nr_AIML_AssociatedID_r19,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 6)]
pub struct TRP_LocationInfoElement_r16 {
    pub dl_prs_id_r16: TRP_LocationInfoElement_r16Dl_PRS_ID_r16,
    #[asn(optional_idx = 0)]
    pub nr_phys_cell_id_r16: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 1)]
    pub nr_cell_global_id_r16: Option<NCGI_r15>,
    #[asn(optional_idx = 2)]
    pub nr_arfcn_r16: Option<ARFCN_ValueNR_r15>,
    #[asn(optional_idx = 3)]
    pub associated_dl_prs_id_r16: Option<TRP_LocationInfoElement_r16Associated_DL_PRS_ID_r16>,
    #[asn(optional_idx = 4)]
    pub trp_location_r16: Option<RelativeLocation_r16>,
    #[asn(optional_idx = 5)]
    pub trp_dl_prs_resource_sets_r16:
        Option<TRP_LocationInfoElement_r16Trp_DL_PRS_ResourceSets_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct TRP_RequestInfoElement_r19 {
    #[asn(optional_idx = 0)]
    pub dl_prs_id_r19: Option<TRP_RequestInfoElement_r19Dl_PRS_ID_r19>,
    #[asn(optional_idx = 1)]
    pub nr_phys_cell_id_r19: Option<NR_PhysCellID_r16>,
    #[asn(optional_idx = 2)]
    pub nr_cell_global_id_r19: Option<NCGI_r15>,
    #[asn(optional_idx = 3)]
    pub nr_arfcn_r19: Option<ARFCN_ValueNR_r15>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "10", ub = "90")]
pub struct TargetIntegrityRisk_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TransactionNumber(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TriggeredReportingCriteria {
    pub cell_change: TriggeredReportingCriteriaCellChange,
    pub reporting_duration: ReportingDuration,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TropoDelayIntegrityErrorBounds_r17 {
    pub mean_troposphere_vertical_hydro_static_delay_r17:
        TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalHydroStaticDelay_r17,
    pub std_dev_troposphere_vertical_hydro_static_delay_r17:
        TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalHydroStaticDelay_r17,
    pub mean_troposphere_vertical_wet_delay_r17:
        TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalWetDelay_r17,
    pub std_dev_troposphere_vertical_wet_delay_r17:
        TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalWetDelay_r17,
    pub mean_troposphere_vertical_hydro_static_delay_rate_r17:
        TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalHydroStaticDelayRate_r17,
    pub std_dev_troposphere_vertical_hydro_static_delay_rate_r17:
        TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalHydroStaticDelayRate_r17,
    pub mean_troposphere_vertical_wet_delay_rate_r17:
        TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalWetDelayRate_r17,
    pub std_dev_troposphere_vertical_wet_delay_rate_r17:
        TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalWetDelayRate_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct TropospericDelayCorrection_r16 {
    pub tropo_hydro_static_vertical_delay_r16:
        TropospericDelayCorrection_r16TropoHydroStaticVerticalDelay_r16,
    pub tropo_wet_vertical_delay_r16: TropospericDelayCorrection_r16TropoWetVerticalDelay_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTC_ModelSet1 {
    pub gnss_utc_a1: UTC_ModelSet1Gnss_Utc_A1,
    pub gnss_utc_a0: UTC_ModelSet1Gnss_Utc_A0,
    pub gnss_utc_tot: UTC_ModelSet1Gnss_Utc_Tot,
    pub gnss_utc_w_nt: UTC_ModelSet1Gnss_Utc_WNt,
    pub gnss_utc_delta_tls: UTC_ModelSet1Gnss_Utc_DeltaTls,
    pub gnss_utc_w_nlsf: UTC_ModelSet1Gnss_Utc_WNlsf,
    pub gnss_utc_dn: UTC_ModelSet1Gnss_Utc_DN,
    pub gnss_utc_delta_tlsf: UTC_ModelSet1Gnss_Utc_DeltaTlsf,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTC_ModelSet2 {
    pub utc_a0: UTC_ModelSet2UtcA0,
    pub utc_a1: UTC_ModelSet2UtcA1,
    pub utc_a2: UTC_ModelSet2UtcA2,
    pub utc_delta_tls: UTC_ModelSet2UtcDeltaTls,
    pub utc_tot: UTC_ModelSet2UtcTot,
    pub utc_w_not: UTC_ModelSet2UtcWNot,
    pub utc_w_nlsf: UTC_ModelSet2UtcWNlsf,
    pub utc_dn: UTC_ModelSet2UtcDN,
    pub utc_delta_tlsf: UTC_ModelSet2UtcDeltaTlsf,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct UTC_ModelSet3 {
    pub na: UTC_ModelSet3NA,
    pub tau_c: UTC_ModelSet3TauC,
    #[asn(optional_idx = 0)]
    pub b1: Option<UTC_ModelSet3B1>,
    #[asn(optional_idx = 1)]
    pub b2: Option<UTC_ModelSet3B2>,
    #[asn(optional_idx = 2)]
    pub kp: Option<UTC_ModelSet3Kp>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTC_ModelSet4 {
    pub utc_a1wnt: UTC_ModelSet4UtcA1wnt,
    pub utc_a0wnt: UTC_ModelSet4UtcA0wnt,
    pub utc_tot: UTC_ModelSet4UtcTot,
    pub utc_w_nt: UTC_ModelSet4UtcWNt,
    pub utc_delta_tls: UTC_ModelSet4UtcDeltaTls,
    pub utc_w_nlsf: UTC_ModelSet4UtcWNlsf,
    pub utc_dn: UTC_ModelSet4UtcDN,
    pub utc_delta_tlsf: UTC_ModelSet4UtcDeltaTlsf,
    pub utc_standard_id: UTC_ModelSet4UtcStandardID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTC_ModelSet5_r12 {
    pub utc_a0_r12: UTC_ModelSet5_r12UtcA0_r12,
    pub utc_a1_r12: UTC_ModelSet5_r12UtcA1_r12,
    pub utc_delta_tls_r12: UTC_ModelSet5_r12UtcDeltaTls_r12,
    pub utc_w_nlsf_r12: UTC_ModelSet5_r12UtcWNlsf_r12,
    pub utc_dn_r12: UTC_ModelSet5_r12UtcDN_r12,
    pub utc_delta_tlsf_r12: UTC_ModelSet5_r12UtcDeltaTlsf_r12,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct UTC_Time_r15 {
    pub utc_time_r15: UTC_Time_r15UtcTime_r15,
    pub utc_time_ms_r15: UTC_Time_r15UtcTime_ms_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct UpdateCapabilities_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum Velocity {
    #[asn(key = 0, extended = false)]
    HorizontalVelocity(HorizontalVelocity),
    #[asn(key = 1, extended = false)]
    HorizontalWithVerticalVelocity(HorizontalWithVerticalVelocity),
    #[asn(key = 2, extended = false)]
    HorizontalVelocityWithUncertainty(HorizontalVelocityWithUncertainty),
    #[asn(key = 3, extended = false)]
    HorizontalWithVerticalVelocityAndUncertainty(HorizontalWithVerticalVelocityAndUncertainty),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct VelocityTypes {
    pub horizontal_velocity: VelocityTypesHorizontalVelocity,
    pub horizontal_with_vertical_velocity: VelocityTypesHorizontalWithVerticalVelocity,
    pub horizontal_velocity_with_uncertainty: VelocityTypesHorizontalVelocityWithUncertainty,
    pub horizontal_with_vertical_velocity_and_uncertainty:
        VelocityTypesHorizontalWithVerticalVelocityAndUncertainty,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct VerticalAccuracy {
    pub accuracy: VerticalAccuracyAccuracy,
    pub confidence: VerticalAccuracyConfidence,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct VerticalAccuracyExt_r15 {
    pub accuracy_ext_r15: VerticalAccuracyExt_r15AccuracyExt_r15,
    pub confidence_r15: VerticalAccuracyExt_r15Confidence_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct VerticalGridPoints_r18 {
    pub reference_altitude_coarse_r18: VerticalGridPoints_r18ReferenceAltitudeCoarse_r18,
    pub number_of_steps_down_r18: VerticalGridPoints_r18NumberOfStepsDown_r18,
    pub step_down_r18: SpatialDelta_r18,
    #[asn(optional_idx = 0)]
    pub upper_validity_delta_altitude_r18: Option<SpatialDelta_r18>,
    #[asn(optional_idx = 1)]
    pub lower_validity_delta_altitude_r18: Option<SpatialDelta_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WLAN_AP_Data_r14 {
    pub wlan_ap_identifier_r14: WLAN_AP_Identifier_r13,
    #[asn(optional_idx = 0)]
    pub wlan_ap_location_r14: Option<WLAN_AP_Location_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WLAN_AP_Identifier_r13 {
    pub bssid_r13: WLAN_AP_Identifier_r13Bssid_r13,
    #[asn(optional_idx = 0)]
    pub ssid_r13: Option<WLAN_AP_Identifier_r13Ssid_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WLAN_AP_Location_r14 {
    pub location_data_lci_r14: LocationDataLCI_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_DataSet_r14 {
    pub wlan_ap_list_r14: WLAN_DataSet_r14Wlan_AP_List_r14,
    #[asn(optional_idx = 0)]
    pub supported_channels_11a_r14: Option<SupportedChannels_11a_r14>,
    #[asn(optional_idx = 1)]
    pub supported_channels_11bg_r14: Option<SupportedChannels_11bg_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum WLAN_Error_r13 {
    #[asn(key = 0, extended = false)]
    LocationServerErrorCauses_r13(WLAN_LocationServerErrorCauses_r13),
    #[asn(key = 1, extended = false)]
    TargetDeviceErrorCauses_r13(WLAN_TargetDeviceErrorCauses_r13),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WLAN_LocationServerErrorCauses_r13 {
    pub cause_r13: WLAN_LocationServerErrorCauses_r13Cause_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct WLAN_MeasurementElement_r13 {
    pub wlan_ap_identifier_r13: WLAN_AP_Identifier_r13,
    #[asn(optional_idx = 0)]
    pub rssi_r13: Option<WLAN_MeasurementElement_r13Rssi_r13>,
    #[asn(optional_idx = 1)]
    pub rtt_r13: Option<WLAN_RTT_r13>,
    #[asn(optional_idx = 2)]
    pub ap_channel_frequency_r13: Option<WLAN_MeasurementElement_r13ApChannelFrequency_r13>,
    #[asn(optional_idx = 3)]
    pub serving_flag_r13: Option<WLAN_MeasurementElement_r13ServingFlag_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_MeasurementInformation_r13 {
    #[asn(optional_idx = 0)]
    pub measurement_reference_time_r13:
        Option<WLAN_MeasurementInformation_r13MeasurementReferenceTime_r13>,
    #[asn(optional_idx = 1)]
    pub wlan_measurement_list_r13: Option<WLAN_MeasurementList_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct WLAN_MeasurementList_r13(pub Vec<WLAN_MeasurementElement_r13>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_ProvideAssistanceData_r14 {
    #[asn(optional_idx = 0)]
    pub wlan_data_set_r14: Option<WLAN_ProvideAssistanceData_r14Wlan_DataSet_r14>,
    #[asn(optional_idx = 1)]
    pub wlan_error_r14: Option<WLAN_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WLAN_ProvideCapabilities_r13 {
    pub wlan_modes_r13: WLAN_ProvideCapabilities_r13Wlan_Modes_r13,
    pub wlan_meas_supported_r13: WLAN_ProvideCapabilities_r13Wlan_MeasSupported_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_ProvideLocationInformation_r13 {
    #[asn(optional_idx = 0)]
    pub wlan_measurement_information_r13: Option<WLAN_MeasurementInformation_r13>,
    #[asn(optional_idx = 1)]
    pub wlan_error_r13: Option<WLAN_Error_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct WLAN_RTT_r13 {
    pub rtt_value_r13: WLAN_RTT_r13RttValue_r13,
    pub rtt_units_r13: WLAN_RTT_r13RttUnits_r13,
    #[asn(optional_idx = 0)]
    pub rtt_accuracy_r13: Option<WLAN_RTT_r13RttAccuracy_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_RequestAssistanceData_r14 {
    pub requested_ad_r14: WLAN_RequestAssistanceData_r14RequestedAD_r14,
    #[asn(optional_idx = 0)]
    pub visible_a_ps_r14: Option<WLAN_RequestAssistanceData_r14VisibleAPs_r14>,
    #[asn(optional_idx = 1)]
    pub wlan_ap_stored_data_r14: Option<WLAN_RequestAssistanceData_r14Wlan_AP_StoredData_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WLAN_RequestCapabilities_r13 {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct WLAN_RequestLocationInformation_r13 {
    pub requested_measurements_r13: WLAN_RequestLocationInformation_r13RequestedMeasurements_r13,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct WLAN_TargetDeviceErrorCauses_r13 {
    pub cause_r13: WLAN_TargetDeviceErrorCauses_r13Cause_r13,
    #[asn(optional_idx = 0)]
    pub wlan_ap_rssi_measurement_not_possible_r13:
        Option<WLAN_TargetDeviceErrorCauses_r13Wlan_AP_RSSI_MeasurementNotPossible_r13>,
    #[asn(optional_idx = 1)]
    pub wlan_ap_rtt_measurement_not_possible_r13:
        Option<WLAN_TargetDeviceErrorCauses_r13Wlan_AP_RTT_MeasurementNotPossible_r13>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct X_Value_r18 {
    pub delta_x_r18: X_Value_r18Delta_x_r18,
    #[asn(optional_idx = 0)]
    pub coarse_delta_x_r18: Option<X_Value_r18Coarse_delta_x_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Y_Value_r18 {
    pub delta_y_r18: Y_Value_r18Delta_y_r18,
    #[asn(optional_idx = 0)]
    pub coarse_delta_y_r18: Option<Y_Value_r18Coarse_delta_y_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct Z_Value_r18 {
    pub delta_z_r18: Z_Value_r18Delta_z_r18,
    #[asn(optional_idx = 0)]
    pub coarse_delta_z_r18: Option<Z_Value_r18Coarse_delta_z_r18>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct A_GNSS_RequestCapabilitiesGnss_SupportListReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct A_GNSS_RequestCapabilitiesAssistanceDataSupportListReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct A_GNSS_RequestCapabilitiesLocationVelocityTypesReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct AbortCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct AbortCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct AbortCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum AbortCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    Abort_r9(Abort_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(AbortCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(AbortCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(AbortCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct AbortCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum AbortCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(AbortCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(AbortCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct AccessTypesAccessTypes(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AcknowledgementAckRequested(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-256", ub = "255")]
pub struct AdditionalPath_r14RelativeTimeDifference_r14(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmToa_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmSqrtA_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmE_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmW_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmM0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmOmega0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmOmegaDot_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmDeltaI_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmA0_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacBDS_AlmanacSet_r12BdsAlmA1_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "9", sz_ub = "9")]
pub struct AlmanacBDS_AlmanacSet_r12BdsSvHealth_r12(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmDataID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmHealth(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmXg(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmYg(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-256", ub = "255")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmZg(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4", ub = "3")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmXgdot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4", ub = "3")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmYgDot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8", ub = "7")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmZgDot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct AlmanacECEF_SBAS_AlmanacSetSbasAlmTo(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1461")]
pub struct AlmanacGLONASS_AlmanacSetGloAlm_NA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "24")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmnA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmHA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmLambdaA(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2097151")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmtlambdaA(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmDeltaIa(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmDeltaTA(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmDeltaTdotA(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmEpsilonA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmOmegaA(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmTauA(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmCA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct AlmanacGLONASS_AlmanacSetGloAlmMA(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct AlmanacKeplerianSetKepAlmanacE(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacKeplerianSetKepAlmanacDeltaI(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacKeplerianSetKepAlmanacOmegaDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct AlmanacKeplerianSetKepSV_StatusINAV(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct AlmanacKeplerianSetKepSV_StatusFNAV(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct AlmanacKeplerianSetKepAlmanacAPowerHalf(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacKeplerianSetKepAlmanacOmega0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacKeplerianSetKepAlmanacW(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacKeplerianSetKepAlmanacM0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacKeplerianSetKepAlmanacAF0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct AlmanacKeplerianSetKepAlmanacAF1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct AlmanacMidiAlmanacSetMidiAlmE(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacMidiAlmanacSetMidiAlmDeltaI(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacMidiAlmanacSetMidiAlmOmegaDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct AlmanacMidiAlmanacSetMidiAlmSqrtA(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacMidiAlmanacSetMidiAlmOmega0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacMidiAlmanacSetMidiAlmOmega(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacMidiAlmanacSetMidiAlmMo(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacMidiAlmanacSetMidiAlmaf0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct AlmanacMidiAlmanacSetMidiAlmaf1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacMidiAlmanacSetMidiAlmL1Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacMidiAlmanacSetMidiAlmL2Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacMidiAlmanacSetMidiAlmL5Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct AlmanacNAV_KeplerianSetNavAlmE(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacNAV_KeplerianSetNavAlmDeltaI(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacNAV_KeplerianSetNavAlmOMEGADOT(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct AlmanacNAV_KeplerianSetNavAlmSVHealth(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct AlmanacNAV_KeplerianSetNavAlmSqrtA(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNAV_KeplerianSetNavAlmOMEGAo(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNAV_KeplerianSetNavAlmOmega(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNAV_KeplerianSetNavAlmMo(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacNAV_KeplerianSetNavAlmaf0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacNAV_KeplerianSetNavAlmaf1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmToa_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmE_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmOMEGADOT_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmSqrtA_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmOMEGAo_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmOmega_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_AlmMo_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_Almaf0_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacNavIC_AlmanacSet_r16Navic_Almaf1_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmToa_r19(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1048575")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmE_r19(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_i0_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOMEGADOT_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmSqrtA_r19(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOMEGAo_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmOmega_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_AlmMo_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_Almaf0_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct AlmanacNavIC_AlmanacSet2_r19NavicL1_Almaf1_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct AlmanacReducedKeplerianSetRedAlmDeltaA(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct AlmanacReducedKeplerianSetRedAlmOmega0(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct AlmanacReducedKeplerianSetRedAlmPhi0(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacReducedKeplerianSetRedAlmL1Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacReducedKeplerianSetRedAlmL2Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct AlmanacReducedKeplerianSetRedAlmL5Health(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct AntennaDescription_r15AntennaDescriptor_r15(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct AntennaDescription_r15AntennaSetUpID_r15(pub u8);
impl AntennaDescription_r15AntennaSetUpID_r15 {
    pub const NON_ZERO: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct AntennaReferencePointUnc_r15Uncertainty_X_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct AntennaReferencePointUnc_r15Confidence_X_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct AntennaReferencePointUnc_r15Uncertainty_Y_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct AntennaReferencePointUnc_r15Confidence_Y_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct AntennaReferencePointUnc_r15Uncertainty_Z_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct AntennaReferencePointUnc_r15Confidence_Z_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777215")]
pub struct ArrayOfGridPoints_r18ReferencePointLatitude_r18(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct ArrayOfGridPoints_r18ReferencePointLongitude_r18(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ArrayOfGridPoints_r18NumberOfStepsSouth_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ArrayOfGridPoints_r18NumberOfStepsEast_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog16_r18(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog64_r18(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "BITSTRING",
    sz_extensible = false,
    sz_lb = "256",
    sz_ub = "256"
)]
pub struct ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog256_r18(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum ArrayOfGridPoints_r18BitmaskOfGrids_r18 {
    #[asn(key = 0, extended = false)]
    Bog16_r18(ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog16_r18),
    #[asn(key = 1, extended = false)]
    Bog64_r18(ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog64_r18),
    #[asn(key = 2, extended = false)]
    Bog256_r18(ArrayOfGridPoints_r18BitmaskOfGrids_r18_bog256_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Aux_ARP_Unc_r15HorizontalUncertainty_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Aux_ARP_Unc_r15HorizontalConfidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Aux_ARP_Unc_r15VerticalUncertainty_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Aux_ARP_Unc_r15VerticalConfidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-524288", ub = "524287")]
pub struct AuxiliaryStationElement_r15Aux_master_delta_latitude_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct AuxiliaryStationElement_r15Aux_master_delta_longitude_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4194304", ub = "4194303")]
pub struct AuxiliaryStationElement_r15Aux_master_delta_height_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct BDS_ClockModel_r12BdsAODC_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct BDS_ClockModel_r12BdsToc_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct BDS_ClockModel_r12BdsA0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct BDS_ClockModel_r12BdsA1_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct BDS_ClockModel_r12BdsA2_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct BDS_ClockModel_r12BdsTgd1_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct BDS_ClockModel2_r16BdsToc_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777215")]
pub struct BDS_ClockModel2_r16BdsA0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct BDS_ClockModel2_r16BdsA1_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct BDS_ClockModel2_r16BdsA2_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct BDS_ClockModel2_r16BdsTgdB1Cp_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct BDS_ClockModel2_r16BdsIscB1Cd_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct BDS_DifferentialCorrections_r12Dbds_RefTime_r12(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct BDS_GridModelParameter_r12Bds_RefTime_r12(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct BT_AntElement_r18Polarization_r18(pub u8);
impl BT_AntElement_r18Polarization_r18 {
    pub const M45: u8 = 0u8;
    pub const ZERO: u8 = 1u8;
    pub const P45: u8 = 2u8;
    pub const P90: u8 = 3u8;
    pub const CIRC: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct BT_AntSwitchElement_r18AntElementIndexShort_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct BT_AntSwitchElement_r18AntElementIndexOffset_r18(pub u8);
impl BT_AntSwitchElement_r18AntElementIndexOffset_r18 {
    pub const O16: u8 = 0u8;
    pub const O32: u8 = 1u8;
    pub const O48: u8 = 2u8;
    pub const O64: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "48", sz_ub = "48")]
pub struct BT_AoA_Config_r18Bt_Addr_r18(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct BT_AoA_Config_r18Cte_Status_r18(pub u8);
impl BT_AoA_Config_r18Cte_Status_r18 {
    pub const ENABLED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "32", ub = "16777")]
pub struct BT_AoA_Config_r18PrimaryAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "6", ub = "65535")]
pub struct BT_AoA_Config_r18SecondAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-127", ub = "20")]
pub struct BT_AoA_Config_r18Tx_Power_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "2", ub = "20")]
pub struct BT_AoA_Config_r18Cte_Length_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct BT_AoA_Config_r18Cte_Count_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct BT_AoA_Config_r18Tx_PHY_M2_r18(pub u8);
impl BT_AoA_Config_r18Tx_PHY_M2_r18 {
    pub const M2: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "32", ub = "16777")]
pub struct BT_AoD_TransmConfig_r18PrimaryAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "6", ub = "65535")]
pub struct BT_AoD_TransmConfig_r18SecondAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "2", ub = "20")]
pub struct BT_AoD_TransmConfig_r18Cte_Length_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct BT_AoD_TransmConfig_r18Cte_Count_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct BT_AoD_TransmConfig_r18Cte_Type2us_r18(pub u8);
impl BT_AoD_TransmConfig_r18Cte_Type2us_r18 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct BT_AoD_TransmConfig_r18Tx_PHY_M2_r18(pub u8);
impl BT_AoD_TransmConfig_r18Tx_PHY_M2_r18 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct BT_BeaconInfo_r18Bt_BeaconInfoList_r18(pub Vec<BT_BeaconInfoElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "48", sz_ub = "48")]
pub struct BT_BeaconInfoElement_r18Bt_Addr_r18(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "74")]
pub struct BT_BeaconInfoElement_r18Bt_antElementList_r18(pub Vec<BT_AntElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "74")]
pub struct BT_BeaconInfoElement_r18Bt_antSwitchingPattern_r18(pub Vec<BT_AntSwitchElement_r18>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct BT_LocationServerErrorCauses_r13Cause_r13(pub u8);
impl BT_LocationServerErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "48", sz_ub = "48")]
pub struct BT_MeasurementElement_r13BtAddr_r13(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct BT_MeasurementElement_r13Rssi_r13(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "48", sz_ub = "48")]
pub struct BT_MeasurementElement_r18BtAddr_r18(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct BT_MeasurementElement_r18Bt_azimuth_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct BT_MeasurementElement_r18Bt_elevation_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct BT_MeasurementElement_r18Rssi_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct BT_MeasurementInformation_r13MeasurementReferenceTime_r13(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct BT_ProvideCapabilities_r13Bt_Modes_r13(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct BT_ProvideCapabilities_r13Bt_MeasSupported_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct BT_RequestAssistanceData_r18RequestedAD_r18(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct BT_RequestLocationInformation_r13RequestedMeasurements_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct BT_SuggestedAoA_Config_r18Cte_Status_r18(pub u8);
impl BT_SuggestedAoA_Config_r18Cte_Status_r18 {
    pub const ENABLED: u8 = 0u8;
    pub const DISABLED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "32", ub = "16777")]
pub struct BT_SuggestedAoA_Config_r18PrimaryAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "6", ub = "65535")]
pub struct BT_SuggestedAoA_Config_r18SecondAdvInterval_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-127", ub = "20")]
pub struct BT_SuggestedAoA_Config_r18Tx_Power_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "2", ub = "20")]
pub struct BT_SuggestedAoA_Config_r18Cte_Length_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct BT_SuggestedAoA_Config_r18Cte_Count_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct BT_SuggestedAoA_Config_r18Tx_PHY_M2_r18(pub u8);
impl BT_SuggestedAoA_Config_r18Tx_PHY_M2_r18 {
    pub const M2: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct BT_TargetDeviceErrorCauses_r13Cause_r13(pub u8);
impl BT_TargetDeviceErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
    pub const REQUESTED_MEASUREMENTS_NOT_AVAILABLE: u8 = 1u8;
    pub const NOT_ALLREQUESTED_MEASUREMENTS_POSSIBLE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct BT_TargetDeviceErrorCauses_r13Bt_Beacon_rssiMeasurementNotPossible_r13;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-135", ub = "135")]
pub struct BT_ULA_GenericAntElement_r18DeltaY_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-135", ub = "135")]
pub struct BT_ULA_GenericAntElement_r18DeltaX_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-135", ub = "135")]
pub struct BT_ULA_GenericAntElement_r18DeltaZ_r18(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "2", ub = "74")]
pub struct BT_UniformCircularArray_r18Bt_NoElements_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "30", ub = "130")]
pub struct BT_UniformCircularArray_r18Bt_InterElementDist_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "2", ub = "74")]
pub struct BT_UniformLinearArray_r18Bt_NoElements_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "30", ub = "130")]
pub struct BT_UniformLinearArray_r18Bt_InterElementDist_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "74")]
pub struct BT_UniformRectangularArray_r18Bt_NoElementsY_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "74")]
pub struct BT_UniformRectangularArray_r18Bt_NoElementsZ_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "30", ub = "135")]
pub struct BT_UniformRectangularArray_r18Bt_InterElementDistY_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "30", ub = "135")]
pub struct BT_UniformRectangularArray_r18Bt_InterElementDistZ_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct BeamPowerElement_r17Nr_dl_prs_RelativePower_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct BeamPowerElement_r17Nr_dl_prs_RelativePowerFine_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct CLOCK_IntegrityParameters_r17ClockRangeErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct CLOCK_IntegrityParameters_r17ClockRangeRateErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2015")]
pub struct CNAV_ClockModelCnavToc(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2015")]
pub struct CNAV_ClockModelCnavTop(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct CNAV_ClockModelCnavURA0(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct CNAV_ClockModelCnavURA1(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct CNAV_ClockModelCnavURA2(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct CNAV_ClockModelCnavAf2(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-524288", ub = "524287")]
pub struct CNAV_ClockModelCnavAf1(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct CNAV_ClockModelCnavAf0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavTgd(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl1cp(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl1cd(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl1ca(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl2c(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl5i5(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct CNAV_ClockModelCnavISCl5q5(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMcc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMcc(
    pub Vec<CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMcc_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMnc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMnc(
    pub Vec<CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMnc_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellGlobalIdEUTRA_AndUTRAPlmn_Identity {
    pub mcc: CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMcc,
    pub mnc: CellGlobalIdEUTRA_AndUTRAPlmn_IdentityMnc,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct CellGlobalIdEUTRA_AndUTRACellIdentity_eutra(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct CellGlobalIdEUTRA_AndUTRACellIdentity_utra(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum CellGlobalIdEUTRA_AndUTRACellIdentity {
    #[asn(key = 0, extended = false)]
    Eutra(CellGlobalIdEUTRA_AndUTRACellIdentity_eutra),
    #[asn(key = 1, extended = false)]
    Utra(CellGlobalIdEUTRA_AndUTRACellIdentity_utra),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct CellGlobalIdGERANPlmn_IdentityMcc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct CellGlobalIdGERANPlmn_IdentityMcc(pub Vec<CellGlobalIdGERANPlmn_IdentityMcc_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct CellGlobalIdGERANPlmn_IdentityMnc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct CellGlobalIdGERANPlmn_IdentityMnc(pub Vec<CellGlobalIdGERANPlmn_IdentityMnc_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct CellGlobalIdGERANPlmn_Identity {
    pub mcc: CellGlobalIdGERANPlmn_IdentityMcc,
    pub mnc: CellGlobalIdGERANPlmn_IdentityMnc,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct CellGlobalIdGERANLocationAreaCode(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct CellGlobalIdGERANCellIdentity(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct CommonIEsAbortAbortCause(pub u8);
impl CommonIEsAbortAbortCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const STOP_PERIODIC_REPORTING: u8 = 1u8;
    pub const TARGET_DEVICE_ABORT: u8 = 2u8;
    pub const NETWORK_ABORT: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct CommonIEsErrorErrorCause(pub u8);
impl CommonIEsErrorErrorCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const LPP_MESSAGE_HEADER_ERROR: u8 = 1u8;
    pub const LPP_MESSAGE_BODY_ERROR: u8 = 2u8;
    pub const EPDU_ERROR: u8 = 3u8;
    pub const INCORRECT_DATA_VALUE: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct DBDS_CorrectionElement_r12Bds_UDREI_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct DBDS_CorrectionElement_r12Bds_RURAI_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct DBDS_CorrectionElement_r12Bds_ECC_DeltaT_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct DGNSS_CorrectionsElementIod(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct DGNSS_CorrectionsElementUdre(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2047", ub = "2047")]
pub struct DGNSS_CorrectionsElementPseudoRangeCor(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-127", ub = "127")]
pub struct DGNSS_CorrectionsElementRangeRateCor(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct DGNSS_CorrectionsElementUdreGrowthRate(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct DGNSS_CorrectionsElementUdreValidityTime(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct DGNSS_SgnTypeElementGnss_StatusHealth(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_DL_TDOA_r16(pub u8);
impl DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_DL_TDOA_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_Multi_RTT_r16(pub u8);
impl DL_AoD_MeasCapabilityPerBand_r16Simul_NR_DL_AoD_Multi_RTT_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct DL_PRS_BeamInfoElement_r16Dl_PRS_Azimuth_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct DL_PRS_BeamInfoElement_r16Dl_PRS_Azimuth_fine_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct DL_PRS_BeamInfoElement_r16Dl_PRS_Elevation_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct DL_PRS_BeamInfoElement_r16Dl_PRS_Elevation_fine_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct DL_PRS_Configuration_ID_r17Nr_dl_prs_configuration_id_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DL_PRS_ID_Info_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_PRS_ID_Info_r16Nr_DL_PRS_ResourceID_List_r16(pub Vec<NR_DL_PRS_ResourceID_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR1_r18(
    pub u8,
);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR1_r18 {
    pub const MHZ40: u8 = 0u8;
    pub const MHZ50: u8 = 1u8;
    pub const MHZ80: u8 = 2u8;
    pub const MHZ100: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR2_r18(
    pub u8,
);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumPRS_BandwidthAcrossAllHopsFR2_r18 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumFH_Hops_r18(pub u8);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18MaximumFH_Hops_r18 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N5: u8 = 3u8;
    pub const N6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingPRS_SymbolsDurationN3_r18(
    pub u8,
);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingPRS_SymbolsDurationN3_r18 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingDurationT3_r18(
    pub u8,
);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingDurationT3_r18 {
    pub const MS8: u8 = 0u8;
    pub const MS16: u8 = 1u8;
    pub const MS20: u8 = 2u8;
    pub const MS30: u8 = 3u8;
    pub const MS40: u8 = 4u8;
    pub const MS80: u8 = 5u8;
    pub const MS160: u8 = 6u8;
    pub const MS320: u8 = 7u8;
    pub const MS640: u8 = 8u8;
    pub const MS1280: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18 { pub processing_prs_symbols_duration_n3_r18 : DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingPRS_SymbolsDurationN3_r18 , pub processing_duration_t3_r18 : DL_PRS_MeasurementWithRxFH_RRC_Connected_r18ProcessingDuration_r18ProcessingDurationT3_r18 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR1_r18(pub u8);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR1_r18 {
    pub const N70: u8 = 0u8;
    pub const N140: u8 = 1u8;
    pub const N210: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR2_r18(pub u8);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18Rf_RxRetuneTimeFR2_r18 {
    pub const N35: u8 = 0u8;
    pub const N70: u8 = 1u8;
    pub const N140: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct DL_PRS_MeasurementWithRxFH_RRC_Connected_r18NumOfOverlappingPRB_r18(pub u8);
impl DL_PRS_MeasurementWithRxFH_RRC_Connected_r18NumOfOverlappingPRB_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct DL_PRS_MutingOption1_r16Dl_prs_MutingBitRepetitionFactor_r16(pub u8);
impl DL_PRS_MutingOption1_r16Dl_prs_MutingBitRepetitionFactor_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct DL_PRS_QCL_Info_r16_ssb_r16Ssb_Index_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct DL_PRS_QCL_Info_r16_ssb_r16Rs_Type_r16(pub u8);
impl DL_PRS_QCL_Info_r16_ssb_r16Rs_Type_r16 {
    pub const TYPE_C: u8 = 0u8;
    pub const TYPE_D: u8 = 1u8;
    pub const TYPE_C_PLUS_TYPE_D: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DL_PRS_QCL_Info_r16_ssb_r16 {
    pub pci_r16: NR_PhysCellID_r16,
    pub ssb_index_r16: DL_PRS_QCL_Info_r16_ssb_r16Ssb_Index_r16,
    pub rs_type_r16: DL_PRS_QCL_Info_r16_ssb_r16Rs_Type_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct DL_PRS_QCL_Info_r16_dl_PRS_r16 {
    pub qcl_dl_prs_resource_id_r16: NR_DL_PRS_ResourceID_r16,
    pub qcl_dl_prs_resource_set_id_r16: NR_DL_PRS_ResourceSetID_r16,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct DL_PRS_QCL_InfoReq_r17Dl_prs_QCL_InformationReq_r17_dl_prs_QCL_Info_requested_r17;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum DL_PRS_QCL_InfoReq_r17Dl_prs_QCL_InformationReq_r17 {
    #[asn(key = 0, extended = false)]
    Dl_prs_QCL_InfoRecPerResourceSet_r17(DL_PRS_QCL_Info_r16),
    #[asn(key = 1, extended = false)]
    Dl_prs_QCL_Info_requested_r17(
        DL_PRS_QCL_InfoReq_r17Dl_prs_QCL_InformationReq_r17_dl_prs_QCL_Info_requested_r17,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct DL_PRS_QCL_InformationReqPerTRP_r17Dl_PRS_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct DL_PRS_QCL_InformationReqPerTRP_r17Dl_prs_QCL_InformationReqSet_r17(
    pub Vec<DL_PRS_QCL_InfoReq_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_PRS_QCL_ProcessingCapabilityPerBand_r16Ssb_FromNeighCellAsQCL_r16(pub u8);
impl DL_PRS_QCL_ProcessingCapabilityPerBand_r16Ssb_FromNeighCellAsQCL_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_PRS_QCL_ProcessingCapabilityPerBand_r16Prs_FromServNeighCellAsQCL_r16(pub u8);
impl DL_PRS_QCL_ProcessingCapabilityPerBand_r16Prs_FromServNeighCellAsQCL_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_PRS_ResourceSets_TRP_Element_r16Dl_PRS_Resource_ARP_List_r16(
    pub Vec<DL_PRS_Resource_ARP_Element_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct DL_PRS_ResourcesBandCombination_r16BandList_r16(pub Vec<FreqBandIndicatorNR_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_Only_r16(
    pub u8,
);
impl DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_Only_r16 { pub const N6 : u8 = 0u8 ; pub const N24 : u8 = 1u8 ; pub const N64 : u8 = 2u8 ; pub const N128 : u8 = 3u8 ; pub const N192 : u8 = 4u8 ; pub const N256 : u8 = 5u8 ; pub const N512 : u8 = 6u8 ; pub const N1024 : u8 = 7u8 ; pub const N2048 : u8 = 8u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr2_Only_r16(
    pub u8,
);
impl DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr2_Only_r16 { pub const N24 : u8 = 0u8 ; pub const N64 : u8 = 1u8 ; pub const N96 : u8 = 2u8 ; pub const N128 : u8 = 3u8 ; pub const N192 : u8 = 4u8 ; pub const N256 : u8 = 5u8 ; pub const N512 : u8 = 6u8 ; pub const N1024 : u8 = 7u8 ; pub const N2048 : u8 = 8u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr1_r16(
    pub u8,
);
impl DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr1_r16 { pub const N6 : u8 = 0u8 ; pub const N24 : u8 = 1u8 ; pub const N64 : u8 = 2u8 ; pub const N96 : u8 = 3u8 ; pub const N128 : u8 = 4u8 ; pub const N192 : u8 = 5u8 ; pub const N256 : u8 = 6u8 ; pub const N512 : u8 = 7u8 ; pub const N1024 : u8 = 8u8 ; pub const N2048 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr2_r16(
    pub u8,
);
impl DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr2_r16 { pub const N24 : u8 = 0u8 ; pub const N64 : u8 = 1u8 ; pub const N96 : u8 = 2u8 ; pub const N128 : u8 = 3u8 ; pub const N192 : u8 = 4u8 ; pub const N256 : u8 = 5u8 ; pub const N512 : u8 = 6u8 ; pub const N1024 : u8 = 7u8 ; pub const N2048 : u8 = 8u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16 { pub fr1_r16 : DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr1_r16 , pub fr2_r16 : DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16Fr2_r16 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16 {
    # [asn (key = 0 , extended = false)] Fr1_Only_r16 (DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_Only_r16) , # [asn (key = 1 , extended = false)] Fr2_Only_r16 (DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr2_Only_r16) , # [asn (key = 2 , extended = false)] Fr1_FR2Mix_r16 (DL_PRS_ResourcesBandCombination_r16MaxNrOfDL_PRS_ResourcesAcrossAllFL_TRP_ResourceSet_r16_fr1_FR2Mix_r16) , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerResourceSet_r16(pub u8);
impl DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerResourceSet_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "8")]
pub struct DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerPositioningFrequencylayer_r16(
    pub u8,
);
impl DL_PRS_ResourcesCapabilityPerBand_r16MaxNrOfDL_PRS_ResourcesPerPositioningFrequencylayer_r16 {
    pub const N6: u8 = 0u8;
    pub const N24: u8 = 1u8;
    pub const N32: u8 = 2u8;
    pub const N64: u8 = 3u8;
    pub const N96: u8 = 4u8;
    pub const N128: u8 = 5u8;
    pub const N256: u8 = 6u8;
    pub const N512: u8 = 7u8;
    pub const N1024: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1024")]
pub struct DL_PRS_StartTime_and_Duration_r17Dl_prs_start_time_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "59")]
pub struct DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Seconds_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "59")]
pub struct DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Minutes_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "23")]
pub struct DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Hours_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17 {
    #[asn(optional_idx = 0)]
    pub seconds_r17: Option<DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Seconds_r17>,
    #[asn(optional_idx = 1)]
    pub minutes_r17: Option<DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Minutes_r17>,
    #[asn(optional_idx = 2)]
    pub hours_r17: Option<DL_PRS_StartTime_and_Duration_r17Dl_prs_duration_r17Hours_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct DL_PRS_TEG_InfoElement_r17Dl_prs_trp_Tx_TEG_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct DL_SelectedPRS_ResourceIndex_r16Nr_DL_SelectedPRS_ResourceIdIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct DL_SelectedPRS_ResourceSetIndex_r16Nr_DL_SelectedPRS_ResourceSetIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct DL_SelectedPRS_ResourceSetIndex_r16Dl_SelectedPRS_ResourceIndexList_r16(
    pub Vec<DL_SelectedPRS_ResourceIndex_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_TDOA_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17(pub u8);
impl DL_TDOA_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct DL_TDOA_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17(pub u8);
impl DL_TDOA_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Delta_Height_r16Delta_Height_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Delta_Height_r16Coarse_delta_Height_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Delta_Latitude_r16Delta_Latitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Delta_Latitude_r16Coarse_delta_Latitude_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Delta_Longitude_r16Delta_Longitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Delta_Longitude_r16Coarse_delta_Longitude_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16384")]
pub struct DeltaTime_r15_deltaTimeSec_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4096")]
pub struct DeltaTime_r15_deltaTimeSFN_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct Displacement_r15Bearing_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Displacement_r15BearingUncConfidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct Displacement_r15BearingRef_r15(pub u8);
impl Displacement_r15BearingRef_r15 {
    pub const GEOGRAPHIC_NORTH: u8 = 0u8;
    pub const MAGNETIC_NORTH: u8 = 1u8;
    pub const LOCAL: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct Displacement_r15HorizontalDistance_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Displacement_r15HorizontalDistanceUnc_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Displacement_r15HorizontalUncConfidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Displacement_r15VerticalDirection_r15(pub u8);
impl Displacement_r15VerticalDirection_r15 {
    pub const UPWARD: u8 = 0u8;
    pub const DOWNWARD: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct Displacement_r15VerticalDistance_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct Displacement_r15VerticalDistanceUnc_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Displacement_r15VerticalUncConfidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "9214")]
pub struct DisplacementTimeStamp_r15_measurementSFN_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct ECGIMcc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct ECGIMcc(pub Vec<ECGIMcc_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct ECGIMnc_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct ECGIMnc(pub Vec<ECGIMnc_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "28", sz_ub = "28")]
pub struct ECGICellidentity(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct ECID_LocationServerErrorCausesCause(pub u8);
impl ECID_LocationServerErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ECID_ProvideCapabilitiesEcid_MeasSupported(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ECID_RequestLocationInformationRequestedMeasurements(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct ECID_TargetDeviceErrorCausesCause(pub u8);
impl ECID_TargetDeviceErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const REQUESTED_MEASUREMENT_NOT_AVAILABLE: u8 = 1u8;
    pub const NOT_ALLREQUESTED_MEASUREMENTS_POSSIBLE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ECID_TargetDeviceErrorCausesRsrpMeasurementNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ECID_TargetDeviceErrorCausesRsrqMeasurementNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ECID_TargetDeviceErrorCausesUeRxTxMeasurementNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct ElevationElement_R17Elevation_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct ElevationElement_R17Elevation_fine_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "24")]
pub struct ElevationElement_R17BeamPowerList_r17(pub Vec<BeamPowerElement_r17>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Ellipsoid_PointLatitudeSign(pub u8);
impl Ellipsoid_PointLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct Ellipsoid_PointDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct Ellipsoid_PointDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct Ellipsoid_PointWithUncertaintyCircleLatitudeSign(pub u8);
impl Ellipsoid_PointWithUncertaintyCircleLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct Ellipsoid_PointWithUncertaintyCircleDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct Ellipsoid_PointWithUncertaintyCircleDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Ellipsoid_PointWithUncertaintyCircleUncertainty(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidArcLatitudeSign(pub u8);
impl EllipsoidArcLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct EllipsoidArcDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct EllipsoidArcDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct EllipsoidArcInnerRadius(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidArcUncertaintyRadius(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct EllipsoidArcOffsetAngle(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct EllipsoidArcIncludedAngle(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct EllipsoidArcConfidence(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidPointWithAltitudeLatitudeSign(pub u8);
impl EllipsoidPointWithAltitudeLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct EllipsoidPointWithAltitudeDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct EllipsoidPointWithAltitudeDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidPointWithAltitudeAltitudeDirection(pub u8);
impl EllipsoidPointWithAltitudeAltitudeDirection {
    pub const HEIGHT: u8 = 0u8;
    pub const DEPTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct EllipsoidPointWithAltitudeAltitude(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidLatitudeSign(pub u8);
impl EllipsoidPointWithAltitudeAndUncertaintyEllipsoidLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidAltitudeDirection(pub u8);
impl EllipsoidPointWithAltitudeAndUncertaintyEllipsoidAltitudeDirection {
    pub const HEIGHT: u8 = 0u8;
    pub const DEPTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidAltitude(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintySemiMajor(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintySemiMinor(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidOrientationMajorAxis(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidUncertaintyAltitude(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct EllipsoidPointWithAltitudeAndUncertaintyEllipsoidConfidence(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct EllipsoidPointWithUncertaintyEllipseLatitudeSign(pub u8);
impl EllipsoidPointWithUncertaintyEllipseLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct EllipsoidPointWithUncertaintyEllipseDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct EllipsoidPointWithUncertaintyEllipseDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidPointWithUncertaintyEllipseUncertaintySemiMajor(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct EllipsoidPointWithUncertaintyEllipseUncertaintySemiMinor(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct EllipsoidPointWithUncertaintyEllipseOrientationMajorAxis(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct EllipsoidPointWithUncertaintyEllipseConfidence(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct EqualIntegerAmbiguityLevel_r16_allReferenceStations_r16;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct Error_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct FKP_Gradients_Element_r15Iod_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct FKP_Gradients_Element_r15North_geometric_gradient_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct FKP_Gradients_Element_r15East_geometric_gradient_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct FKP_Gradients_Element_r15North_ionospheric_gradient_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct FKP_Gradients_Element_r15East_ionospheric_gradient_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct GLO_RTK_BiasInformation_r15CpbIndicator_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GLO_RTK_BiasInformation_r15L1_ca_cpBias_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GLO_RTK_BiasInformation_r15L1_p_cpBias_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GLO_RTK_BiasInformation_r15L2_ca_cpBias_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GLO_RTK_BiasInformation_r15L2_p_cpBias_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct GLONASS_ClockModelGloTau(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct GLONASS_ClockModelGloGamma(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct GLONASS_ClockModelGloDeltaTau(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct GNSS_AcquisitionAssistElementDoppler0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct GNSS_AcquisitionAssistElementDoppler1(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4")]
pub struct GNSS_AcquisitionAssistElementDopplerUncertainty(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1022")]
pub struct GNSS_AcquisitionAssistElementCodePhase(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_AcquisitionAssistElementIntCodePhase(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct GNSS_AcquisitionAssistElementCodePhaseSearchWindow(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct GNSS_AcquisitionAssistElementAzimuth(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_AcquisitionAssistElementElevation(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct GNSS_AlmanacWeekNumber(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct GNSS_AlmanacToa(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GNSS_AlmanacIoda(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_AlmanacCompleteAlmanacProvided(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct GNSS_AlmanacReqModelID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_AlmanacSupportAlmanacModel(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct GNSS_DataBitAssistanceGnss_TOD(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "999")]
pub struct GNSS_DataBitAssistanceGnss_TODfrac(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct GNSS_DataBitAssistanceReqGnss_TOD_Req(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "999")]
pub struct GNSS_DataBitAssistanceReqGnss_TOD_FracReq(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_DataBitAssistanceReqDataBitInterval(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1024")]
pub struct GNSS_DataBitsSgnElementGnss_DataBits(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct GNSS_DifferentialCorrectionsDgnss_RefTime(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_DifferentialCorrectionsReqDgnss_ValidityTimeReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_DifferentialCorrectionsSupportDgnss_ValidityTimeSup(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct GNSS_EarthOrientationParametersTeop(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct GNSS_EarthOrientationParametersPmX(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct GNSS_EarthOrientationParametersPmXdot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct GNSS_EarthOrientationParametersPmY(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct GNSS_EarthOrientationParametersPmYdot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1073741824", ub = "1073741823")]
pub struct GNSS_EarthOrientationParametersDeltaUT1(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct GNSS_EarthOrientationParametersDeltaUT1dot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct GNSS_FrequencyID_r15Gnss_FrequencyID_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct GNSS_IDGnss_id(pub u8);
impl GNSS_IDGnss_id {
    pub const GPS: u8 = 0u8;
    pub const SBAS: u8 = 1u8;
    pub const QZSS: u8 = 2u8;
    pub const GALILEO: u8 = 3u8;
    pub const GLONASS: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GNSS_ID_BDS_SatElement_r16SatType_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_ID_BitmapGnss_ids(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-7", ub = "13")]
pub struct GNSS_ID_GLONASS_SatElementChannelNumber(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_Integrity_ServiceAlert_r17IonosphereDoNotUse_r17(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_Integrity_ServiceAlert_r17TroposphereDoNotUse_r17(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct GNSS_Integrity_ServiceParameters_r17IrMinimum_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct GNSS_Integrity_ServiceParameters_r17IrMaximum_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct GNSS_IonosphericModelReqKlobucharModelReq(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct GNSS_IonosphericModelReqNeQuickModelReq;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_IonosphericModelSupportIonoModel(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct GNSS_LOS_InfoElement_r18Los_r18(pub u8);
impl GNSS_LOS_InfoElement_r18Los_r18 {
    pub const TRUE: u8 = 0u8;
    pub const FALSE: u8 = 1u8;
    pub const UNCERTAIN: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_LOS_NLOS_GridPoints_r18GridPointsSetID_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct GNSS_LOS_NLOS_GridPoints_r18ReferenceAltitudeFine_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_LOS_NLOS_GriddedIndications_r18GridPointsSetID_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct GNSS_LOS_NLOS_GriddedIndications_r18ExpirationTime_r18(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18GridPointsSetID_Req_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_inside_r18(pub u8);
impl GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_inside_r18 {
    pub const UPPER_LEFT: u8 = 0u8;
    pub const UPPER_RIGHT: u8 = 1u8;
    pub const LOWER_LEFT: u8 = 2u8;
    pub const LOWER_RIGHT: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_outside_r18(pub u8);
impl GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_outside_r18 {
    pub const NORTH: u8 = 0u8;
    pub const WEST: u8 = 1u8;
    pub const SOUTH: u8 = 2u8;
    pub const EAST: u8 = 3u8;
    pub const ABOVE: u8 = 4u8;
    pub const BELOW: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18 {
    #[asn(key = 0, extended = false)]
    Inside_r18(GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_inside_r18),
    #[asn(key = 1, extended = false)]
    Outside_r18(GNSS_LOS_NLOS_GriddedIndicationsReq_r18RelativeLocationInfo_r18_outside_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18VerticalGridType_r18(pub u8);
impl GNSS_LOS_NLOS_GriddedIndicationsReq_r18VerticalGridType_r18 {
    pub const GROUND_LEVEL: u8 = 0u8;
    pub const THREE_D: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18ReferenceAltitudeFine_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-50", ub = "900")]
pub struct GNSS_LOS_NLOS_GriddedIndicationsReq_r18ReferenceAltitudeCoarse_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct GNSS_LocationServerErrorCausesCause(pub u8);
impl GNSS_LocationServerErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const UNDELIVERED_ASSISTANCE_DATA_IS_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const UNDELIVERED_ASSISTANCE_DATA_IS_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 =
        2u8;
    pub const UNDELIVERED_ASSISTANCE_DATA_IS_PARTLY_NOT_SUPPORTED_AND_PARTLY_NOT_AVAILABLE_BY_SERVER : u8 = 3u8 ;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct GNSS_NavModelSatelliteElementSvHealth(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct GNSS_NavModelSatelliteElementIod(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct GNSS_NavigationModelNonBroadcastIndFlag(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_NavigationModelSupportClockModel(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_NavigationModelSupportOrbitModel(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct GNSS_NetworkID_r15NetworkID_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "32")]
pub struct GNSS_PeriodicControlParam_r15DeliveryAmount_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "64")]
pub struct GNSS_PeriodicControlParam_r15DeliveryInterval_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_PositioningInstructionsFineTimeAssistanceMeasReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_PositioningInstructionsAdrMeasReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_PositioningInstructionsMultiFreqMeasReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_PositioningInstructionsAssistanceAvailability(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GNSS_RTK_CommonObservationInfo_r15ClockSteeringIndicator_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GNSS_RTK_CommonObservationInfo_r15ExternalClockIndicator_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct GNSS_RTK_CommonObservationInfo_r15SmoothingIndicator_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct GNSS_RTK_CommonObservationInfo_r15SmoothingInterval_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ObservationsReq_r15Gnss_RTK_Integer_ms_Req_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ObservationsReq_r15Gnss_RTK_PhaseRangeRateReq_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ObservationsReq_r15Gnss_RTK_CNR_Req_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct GNSS_RTK_ReferenceStationInfo_r15ReferenceStationIndicator_r15(pub u8);
impl GNSS_RTK_ReferenceStationInfo_r15ReferenceStationIndicator_r15 {
    pub const PHYSICAL: u8 = 0u8;
    pub const NON_PHYSICAL: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_X_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_Y_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct GNSS_RTK_ReferenceStationInfo_r15Antenna_reference_point_ECEF_Z_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct GNSS_RTK_ReferenceStationInfo_r15AntennaHeight_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ReferenceStationInfoReq_r15AntennaDescriptionReq_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ReferenceStationInfoReq_r15AntennaHeightReq_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_RTK_ReferenceStationInfoReq_r15PhysicalReferenceStationReq_r15(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_RTK_Residuals_r15N_Refs_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "254")]
pub struct GNSS_RTK_SatelliteDataElement_r15Integer_ms_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct GNSS_RTK_SatelliteDataElement_r15Rough_range_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct GNSS_RTK_SatelliteDataElement_r15Rough_phase_range_rate_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-524288", ub = "524287")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15Fine_PseudoRange_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15Fine_PhaseRange_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15LockTimeIndicator_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "1")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15HalfCycleAmbiguityIndicator_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15Carrier_to_noise_ratio_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct GNSS_RTK_SatelliteSignalDataElement_r15Fine_PhaseRangeRate_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct GNSS_ReferenceStationID_r15ReferenceStationID_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct GNSS_ReferenceStationID_r15ProviderName_r15(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_ReferenceTimeReferenceTimeUnc(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct GNSS_ReferenceTimeGnss_ReferenceTimeForCells(pub Vec<GNSS_ReferenceTimeForOneCell>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_ReferenceTimeForOneCellReferenceTimeUnc(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct GNSS_ReferenceTimeForOneCellBsAlign(pub u8);
impl GNSS_ReferenceTimeForOneCellBsAlign {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_ReferenceTimeReqGnss_TimeReqPrefList(pub Vec<GNSS_ID>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_ReferenceTimeReqGps_TOW_assistReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_ReferenceTimeReqNotOfLeapSecReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16ReferencePointLatitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16ReferencePointLongitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16NumberOfStepsLatitude_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16NumberOfStepsLongitude_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "511")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16StepOfLatitude_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1023")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16StepOfLongitude_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct GNSS_SSR_ArrayOfCorrectionPoints_r16BitmaskOfGrids_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_ClockCorrections_r15SsrUpdateInterval_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_ClockCorrections_r15Iod_ssr_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_ClockCorrectionsSet2_r17RefEph_r17(pub u8);
impl GNSS_SSR_ClockCorrectionsSet2_r17RefEph_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_ClockCorrectionsSet2Req_r17RefEphReq_r17(pub u8);
impl GNSS_SSR_ClockCorrectionsSet2Req_r17RefEphReq_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_CodeBias_r15SsrUpdateInterval_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_CodeBias_r15Iod_ssr_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_SSR_CorrectionPoints_r16CorrectionPointSetID_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum GNSS_SSR_CorrectionPoints_r16CorrectionPoints_r16 {
    #[asn(key = 0, extended = false)]
    ListOfCorrectionPoints_r16(GNSS_SSR_ListOfCorrectionPoints_r16),
    #[asn(key = 1, extended = false)]
    ArrayOfCorrectionPoints_r16(GNSS_SSR_ArrayOfCorrectionPoints_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_SSR_CorrectionPointsReq_r16CorrectionPointSetID_Req_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_GriddedCorrection_r16SsrUpdateInterval_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_GriddedCorrection_r16Iod_ssr_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct GNSS_SSR_GriddedCorrection_r16TroposphericDelayQualityIndicator_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_SSR_GriddedCorrection_r16CorrectionPointSetID_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_IOD_Update_r18SsrUpdateInterval_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_IOD_Update_r18Iod_ssr_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "64")]
pub struct GNSS_SSR_IOD_Update_r18Iod_ssr_PCVResiduals_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct GNSS_SSR_ListOfCorrectionPoints_r16ReferencePointLatitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GNSS_SSR_ListOfCorrectionPoints_r16ReferencePointLongitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "0", sz_ub = "63")]
pub struct GNSS_SSR_ListOfCorrectionPoints_r16RelativeLocationsList_r16(
    pub Vec<RelativeLocationElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_OrbitCorrections_r15SsrUpdateInterval_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct GNSS_SSR_OrbitCorrections_r15SatelliteReferenceDatum_r15(pub u8);
impl GNSS_SSR_OrbitCorrections_r15SatelliteReferenceDatum_r15 {
    pub const ITRF: u8 = 0u8;
    pub const REGIONAL: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_OrbitCorrections_r15Iod_ssr_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_OrbitCorrectionsSet2_r17RefEph_r17(pub u8);
impl GNSS_SSR_OrbitCorrectionsSet2_r17RefEph_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_OrbitCorrectionsSet2Req_r17RefEphReq_r17(pub u8);
impl GNSS_SSR_OrbitCorrectionsSet2Req_r17RefEphReq_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_PhaseBias_r16SsrUpdateInterval_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_PhaseBias_r16Iod_ssr_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct GNSS_SSR_ProviderInfo_r19Ssr_ProviderID_r19(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_ProviderInfo_r19Ssr_SolutionID_r19(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_STEC_Correction_r16SsrUpdateInterval_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_STEC_Correction_r16Iod_ssr_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GNSS_SSR_STEC_Correction_r16CorrectionPointSetID_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "64")]
pub struct GNSS_SSR_SatellitePCVResiduals_r18Iod_ssr_PCVResiduals_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_URA_Set2_r17RefEph_r17(pub u8);
impl GNSS_SSR_URA_Set2_r17RefEph_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct GNSS_SSR_URA_Set2Req_r17RefEphReq_r17(pub u8);
impl GNSS_SSR_URA_Set2Req_r17RefEphReq_r17 {
    pub const B1C: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_URA_r16SsrUpdateInterval_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SSR_URA_r16Iod_ssr_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct GNSS_SatMeasElementCNo(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct GNSS_SatMeasElementMpathDet(pub u8);
impl GNSS_SatMeasElementMpathDet {
    pub const NOT_MEASURED: u8 = 0u8;
    pub const LOW: u8 = 1u8;
    pub const MEDIUM: u8 = 2u8;
    pub const HIGH: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GNSS_SatMeasElementCarrierQualityInd(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2097151")]
pub struct GNSS_SatMeasElementCodePhase(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_SatMeasElementIntegerCodePhase(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct GNSS_SatMeasElementCodePhaseRMSError(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct GNSS_SatMeasElementDoppler(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "33554431")]
pub struct GNSS_SatMeasElementAdr(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct GNSS_SgnMeasElementGnss_CodePhaseAmbiguity(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct GNSS_SignalIDGnss_SignalID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct GNSS_SignalIDsGnss_SignalIDs(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GNSS_SubNetworkID_r15SubNetworkID_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct GNSS_SupportElementFta_MeasSupport {
    pub cell_time: AccessTypes,
    pub mode: PositioningModes,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_SupportElementAdr_Support(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_SupportElementVelocityMeasurementSupport(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct GNSS_SystemTimeGnss_DayNumber(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "86399")]
pub struct GNSS_SystemTimeGnss_TimeOfDay(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "999")]
pub struct GNSS_SystemTimeGnss_TimeOfDayFrac_msec(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct GNSS_SystemTimeNotificationOfLeapSecond(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct GNSS_TargetDeviceErrorCausesCause(pub u8);
impl GNSS_TargetDeviceErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const THERE_WERE_NOT_ENOUGH_SATELLITES_RECEIVED: u8 = 1u8;
    pub const ASSISTANCE_DATA_MISSING: u8 = 2u8;
    pub const NOT_ALL_REQUESTED_MEASUREMENTS_POSSIBLE: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct GNSS_TargetDeviceErrorCausesFineTimeAssistanceMeasurementsNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct GNSS_TargetDeviceErrorCausesAdrMeasurementsNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct GNSS_TargetDeviceErrorCausesMultiFrequencyMeasurementsNotPossible;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct GNSS_TimeModelElementGnss_TimeModelRefTime(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-67108864", ub = "67108863")]
pub struct GNSS_TimeModelElementTA0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct GNSS_TimeModelElementTA1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct GNSS_TimeModelElementTA2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "15")]
pub struct GNSS_TimeModelElementGnss_TO_ID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct GNSS_TimeModelElementWeekNumber(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct GNSS_TimeModelElementDeltaT(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "15")]
pub struct GNSS_TimeModelElementReqGnss_TO_IDsReq(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct GNSS_TimeModelElementReqDeltaTreq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct GNSS_UTC_ModelReqModelID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct GNSS_UTC_ModelSupportUtc_Model(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "64")]
pub struct GPS_TOW_AssistElementSatelliteID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct GPS_TOW_AssistElementTlmWord(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct GPS_TOW_AssistElementAntiSpoof(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct GPS_TOW_AssistElementAlert(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct GPS_TOW_AssistElementTlmRsvdBits(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15AmbiguityStatusFlag_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15Non_synch_count_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15GeometricCarrierPhaseCorrectionDifference_r15(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15Iod_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct Geometric_Ionospheric_Corrections_Differences_Element_r15IonosphericCarrierPhaseCorrectionDifference_r15(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "320")]
pub struct GridIonElement_r12Igp_ID_r12(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct GridIonElement_r12Dt_r12(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct GridIonElement_r12Givei_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16DegreesLatitude_r16(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16DegreesLongitude_r16(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64000", ub = "1280000")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Altitude_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintySemiMajor_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintySemiMinor_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16OrientationMajorAxis_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16HorizontalConfidence_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16UncertaintyAltitude_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16VerticalConfidence_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Ha_HorizontalExtendedRangeUsed_r16(
    pub bool,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct HA_EllipsoidPointWithAltitudeAndScalableUncertaintyEllipsoid_r16Ha_VerticalExtendedRangeUsed_r16(
    pub bool,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16DegreesLatitude_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16DegreesLongitude_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16UncertaintySemiMajor_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16UncertaintySemiMinor_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16OrientationMajorAxis_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16Confidence_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct HA_EllipsoidPointWithScalableUncertaintyEllipse_r16Ha_ExtendedUncertaintyRangeUsed_r16(
    pub bool,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "64")]
pub struct HA_GNSS_Metrics_r17NrOfUsedSatellites_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct HA_GNSS_Metrics_r17Hdopi_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct HA_GNSS_Metrics_r17Pdopi_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "99")]
pub struct HA_GNSS_Metrics_r17Age_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct HA_GNSS_Metrics_r17FixType_r17(pub u8);
impl HA_GNSS_Metrics_r17FixType_r17 {
    pub const CARRIER_PHASE_FLOAT: u8 = 0u8;
    pub const CARRIER_PHASE_FIX: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15DegreesLatitude_r15(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15DegreesLongitude_r15(
    pub i32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64000", ub = "1280000")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15Altitude_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintySemiMajor_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintySemiMinor_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15OrientationMajorAxis_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15HorizontalConfidence_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15UncertaintyAltitude_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15VerticalConfidence_r15(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15DegreesLatitude_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15DegreesLongitude_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15UncertaintySemiMajor_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15UncertaintySemiMinor_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15OrientationMajorAxis_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HighAccuracyEllipsoidPointWithUncertaintyEllipse_r15Confidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct HorizontalAccuracyAccuracy(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HorizontalAccuracyConfidence(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalAccuracyExt_r15AccuracyExt_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct HorizontalAccuracyExt_r15Confidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct HorizontalVelocityBearing(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct HorizontalVelocityHorizontalSpeed(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct HorizontalVelocityWithUncertaintyBearing(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct HorizontalVelocityWithUncertaintyHorizontalSpeed(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalVelocityWithUncertaintyUncertaintySpeed(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct HorizontalWithVerticalVelocityBearing(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct HorizontalWithVerticalVelocityHorizontalSpeed(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct HorizontalWithVerticalVelocityVerticalDirection(pub u8);
impl HorizontalWithVerticalVelocityVerticalDirection {
    pub const UPWARD: u8 = 0u8;
    pub const DOWNWARD: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalWithVerticalVelocityVerticalSpeed(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyBearing(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyHorizontalSpeed(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyVerticalDirection(pub u8);
impl HorizontalWithVerticalVelocityAndUncertaintyVerticalDirection {
    pub const UPWARD: u8 = 0u8;
    pub const DOWNWARD: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyVerticalSpeed(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyHorizontalUncertaintySpeed(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct HorizontalWithVerticalVelocityAndUncertaintyVerticalUncertaintySpeed(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "50000")]
pub struct IntegrityInfo_r17HorizontalProtectionLevel_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "50000")]
pub struct IntegrityInfo_r17VerticalProtectionLevel_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "10", ub = "90")]
pub struct IntegrityInfo_r17AchievableTargetIntegrityRisk_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct KlobucharModel2Parameter_r16Alfa1_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModel2Parameter_r16Alfa2_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct KlobucharModel2Parameter_r16Alfa3_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct KlobucharModel2Parameter_r16Alfa4_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct KlobucharModel2Parameter_r16Alfa5_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModel2Parameter_r16Alfa6_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModel2Parameter_r16Alfa7_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModel2Parameter_r16Alfa8_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModel2Parameter_r16Alfa9_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct KlobucharModelParameterDataID(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterAlfa0(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterAlfa1(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterAlfa2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterAlfa3(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterBeta0(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterBeta1(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterBeta2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct KlobucharModelParameterBeta3(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct LCS_GCS_TranslationParameter_r16Alpha_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct LCS_GCS_TranslationParameter_r16Alpha_fine_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct LCS_GCS_TranslationParameter_r16Beta_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct LCS_GCS_TranslationParameter_r16Beta_fine_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct LCS_GCS_TranslationParameter_r16Gamma_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct LCS_GCS_TranslationParameter_r16Gamma_fine_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10")]
pub struct LOS_NLOS_Indicator_r17Indicator_r17_soft_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LOS_NLOS_Indicator_r17Indicator_r17_hard_r17(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum LOS_NLOS_Indicator_r17Indicator_r17 {
    #[asn(key = 0, extended = false)]
    Soft_r17(LOS_NLOS_Indicator_r17Indicator_r17_soft_r17),
    #[asn(key = 1, extended = false)]
    Hard_r17(LOS_NLOS_Indicator_r17Indicator_r17_hard_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LPP_MessageEndTransaction(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare7;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare6;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare5;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare4;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct LPP_MessageBody_c1_spare0;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "15", extensible = false)]
pub enum LPP_MessageBody_c1 {
    #[asn(key = 0, extended = false)]
    RequestCapabilities(RequestCapabilities),
    #[asn(key = 1, extended = false)]
    ProvideCapabilities(ProvideCapabilities),
    #[asn(key = 2, extended = false)]
    RequestAssistanceData(RequestAssistanceData),
    #[asn(key = 3, extended = false)]
    ProvideAssistanceData(ProvideAssistanceData),
    #[asn(key = 4, extended = false)]
    RequestLocationInformation(RequestLocationInformation),
    #[asn(key = 5, extended = false)]
    ProvideLocationInformation(ProvideLocationInformation),
    #[asn(key = 6, extended = false)]
    Abort(Abort),
    #[asn(key = 7, extended = false)]
    Error(Error),
    #[asn(key = 8, extended = false)]
    Spare7(LPP_MessageBody_c1_spare7),
    #[asn(key = 9, extended = false)]
    Spare6(LPP_MessageBody_c1_spare6),
    #[asn(key = 10, extended = false)]
    Spare5(LPP_MessageBody_c1_spare5),
    #[asn(key = 11, extended = false)]
    Spare4(LPP_MessageBody_c1_spare4),
    #[asn(key = 12, extended = false)]
    Spare3(LPP_MessageBody_c1_spare3),
    #[asn(key = 13, extended = false)]
    Spare2(LPP_MessageBody_c1_spare2),
    #[asn(key = 14, extended = false)]
    Spare1(LPP_MessageBody_c1_spare1),
    #[asn(key = 15, extended = false)]
    Spare0(LPP_MessageBody_c1_spare0),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct LPP_MessageBody_messageClassExtension {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Local2dPointWithUncertaintyEllipse_r18CartesianCoordinatesUnits_r18(pub u8);
impl Local2dPointWithUncertaintyEllipse_r18CartesianCoordinatesUnits_r18 {
    pub const MM: u8 = 0u8;
    pub const CM: u8 = 1u8;
    pub const DM: u8 = 2u8;
    pub const M: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Local2dPointWithUncertaintyEllipse_r18UncertaintySemiMajor_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Local2dPointWithUncertaintyEllipse_r18UncertaintySemiMinor_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct Local2dPointWithUncertaintyEllipse_r18OrientationMajorAxis_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Local2dPointWithUncertaintyEllipse_r18Confidence_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18CartesianCoordinatesUnits_r18(pub u8);
impl Local3dPointWithUncertaintyEllipsoid_r18CartesianCoordinatesUnits_r18 {
    pub const MM: u8 = 0u8;
    pub const CM: u8 = 1u8;
    pub const DM: u8 = 2u8;
    pub const M: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18UncertaintySemiMajor_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18UncertaintySemiMinor_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18OrientationMajorAxis_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18UncertaintyAltitude_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18Confidence_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct Local3dPointWithUncertaintyEllipsoid_r18VConfidence_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "VisibleString",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct LocalOrigin_r18CoordinateID_r18(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct LocalOrigin_r18HorizAxesOrientation_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidPoint(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidPointWithUncertaintyCircle(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidPointWithUncertaintyEllipse(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesPolygon(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidPointWithAltitude(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidPointWithAltitudeAndUncertaintyEllipsoid(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct LocationCoordinateTypesEllipsoidArc(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct LocationDataLCI_r14LatitudeUncertainty_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "34", sz_ub = "34")]
pub struct LocationDataLCI_r14Latitude_r14(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct LocationDataLCI_r14LongitudeUncertainty_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "34", sz_ub = "34")]
pub struct LocationDataLCI_r14Longitude_r14(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct LocationDataLCI_r14AltitudeUncertainty_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "30", sz_ub = "30")]
pub struct LocationDataLCI_r14Altitude_r14(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct LocationDataLCI_r14Datum_r14(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct LocationUncertainty_r16HorizontalUncertainty_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct LocationUncertainty_r16HorizontalConfidence_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct LocationUncertainty_r16VerticalUncertainty_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct LocationUncertainty_r16VerticalConfidence_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct MBS_AcquisitionAssistance_r14TransmitterID_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct MBS_AcquisitionAssistance_r14MbsConfiguration_r14(pub u8);
impl MBS_AcquisitionAssistance_r14MbsConfiguration_r14 {
    pub const TB1: u8 = 0u8;
    pub const TB2: u8 = 1u8;
    pub const TB3: u8 = 2u8;
    pub const TB4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "128")]
pub struct MBS_AcquisitionAssistance_r14PnCodeIndex_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "919750000", ub = "927250000")]
pub struct MBS_AcquisitionAssistance_r14Freq_r14(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct MBS_AlmanacAssistance_r14TransmitterID_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "26", sz_ub = "26")]
pub struct MBS_AlmanacAssistance_r14TransmitterLatitude_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "27", sz_ub = "27")]
pub struct MBS_AlmanacAssistance_r14TransmitterLongitude_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "15", sz_ub = "15")]
pub struct MBS_AlmanacAssistance_r14TransmitterAltitude_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "25")]
pub struct MBS_AlmanacAssistance_r14TimeCorrection_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct MBS_AssistanceDataSupportList_r14Mbs_AcquisitionAssistanceDataSupport_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct MBS_AssistanceDataSupportList_r14Mbs_AlmanacAssistanceDataSupport_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32767")]
pub struct MBS_BeaconMeasElement_r13TransmitterID_r13(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2097151")]
pub struct MBS_BeaconMeasElement_r13CodePhase_r13(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct MBS_BeaconMeasElement_r13CodePhaseRMSError_r13(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct MeasQuantityResults_r16Nr_RSRP_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct MeasQuantityResults_r16Nr_RSRQ_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct MeasuredResultsElementPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MeasuredResultsElementSystemFrameNumber(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "97")]
pub struct MeasuredResultsElementRsrp_Result(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "34")]
pub struct MeasuredResultsElementRsrq_Result(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct MeasuredResultsElementUe_RxTxTimeDiff(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599999")]
pub struct MeasurementReferenceTimeGnss_TOD_msec(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3999")]
pub struct MeasurementReferenceTimeGnss_TOD_frac(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct MeasurementReferenceTimeGnss_TOD_unc(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct MeasurementReferenceTimeNetworkTime_eUTRAPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MeasurementReferenceTimeNetworkTime_eUTRASystemFrameNumber(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MeasurementReferenceTimeNetworkTime_eUTRA {
    pub phys_cell_id: MeasurementReferenceTimeNetworkTime_eUTRAPhysCellId,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub system_frame_number: MeasurementReferenceTimeNetworkTime_eUTRASystemFrameNumber,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct MeasurementReferenceTimeNetworkTime_uTRAMode_fddPrimary_CPICH_Info(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MeasurementReferenceTimeNetworkTime_uTRAMode_fdd {
    pub primary_cpich_info: MeasurementReferenceTimeNetworkTime_uTRAMode_fddPrimary_CPICH_Info,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct MeasurementReferenceTimeNetworkTime_uTRAMode_tddCellParameters(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct MeasurementReferenceTimeNetworkTime_uTRAMode_tdd {
    pub cell_parameters: MeasurementReferenceTimeNetworkTime_uTRAMode_tddCellParameters,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum MeasurementReferenceTimeNetworkTime_uTRAMode {
    #[asn(key = 0, extended = false)]
    Fdd(MeasurementReferenceTimeNetworkTime_uTRAMode_fdd),
    #[asn(key = 1, extended = false)]
    Tdd(MeasurementReferenceTimeNetworkTime_uTRAMode_tdd),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct MeasurementReferenceTimeNetworkTime_uTRAReferenceSystemFrameNumber(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MeasurementReferenceTimeNetworkTime_uTRA {
    pub mode: MeasurementReferenceTimeNetworkTime_uTRAMode,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub reference_system_frame_number:
        MeasurementReferenceTimeNetworkTime_uTRAReferenceSystemFrameNumber,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct MeasurementReferenceTimeNetworkTime_gSMBcchCarrier(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct MeasurementReferenceTimeNetworkTime_gSMBsic(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct MeasurementReferenceTimeNetworkTime_gSMReferenceFrameReferenceFN(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct MeasurementReferenceTimeNetworkTime_gSMReferenceFrameReferenceFNMSB(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MeasurementReferenceTimeNetworkTime_gSMReferenceFrame {
    pub reference_fn: MeasurementReferenceTimeNetworkTime_gSMReferenceFrameReferenceFN,
    #[asn(optional_idx = 0)]
    pub reference_fnmsb:
        Option<MeasurementReferenceTimeNetworkTime_gSMReferenceFrameReferenceFNMSB>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct MeasurementReferenceTimeNetworkTime_gSMDeltaGNSS_TOD(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MeasurementReferenceTimeNetworkTime_gSM {
    pub bcch_carrier: MeasurementReferenceTimeNetworkTime_gSMBcchCarrier,
    pub bsic: MeasurementReferenceTimeNetworkTime_gSMBsic,
    #[asn(optional_idx = 0)]
    pub cell_global_id: Option<CellGlobalIdGERAN>,
    pub reference_frame: MeasurementReferenceTimeNetworkTime_gSMReferenceFrame,
    #[asn(optional_idx = 1)]
    pub delta_gnss_tod: Option<MeasurementReferenceTimeNetworkTime_gSMDeltaGNSS_TOD>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct MeasurementReferenceTimeNetworkTime_nbIoT_r14NbPhysCellId_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MeasurementReferenceTimeNetworkTime_nbIoT_r14Sfn_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MeasurementReferenceTimeNetworkTime_nbIoT_r14HyperSFN_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct MeasurementReferenceTimeNetworkTime_nbIoT_r14 {
    pub nb_phys_cell_id_r14: MeasurementReferenceTimeNetworkTime_nbIoT_r14NbPhysCellId_r14,
    #[asn(optional_idx = 0)]
    pub nb_cell_global_id_r14: Option<ECGI>,
    pub sfn_r14: MeasurementReferenceTimeNetworkTime_nbIoT_r14Sfn_r14,
    #[asn(optional_idx = 1)]
    pub hyper_sfn_r14: Option<MeasurementReferenceTimeNetworkTime_nbIoT_r14HyperSFN_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct MeasurementReferenceTimeNetworkTime_nr_r15NrPhysCellId_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct MeasurementReferenceTimeNetworkTime_nr_r15Nr_sfn_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct MeasurementReferenceTimeNetworkTime_nr_r15 {
    pub nr_phys_cell_id_r15: MeasurementReferenceTimeNetworkTime_nr_r15NrPhysCellId_r15,
    #[asn(optional_idx = 0)]
    pub nr_cell_global_id_r15: Option<NCGI_r15>,
    pub nr_sfn_r15: MeasurementReferenceTimeNetworkTime_nr_r15Nr_sfn_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum MeasurementReferenceTimeNetworkTime {
    #[asn(key = 0, extended = false)]
    EUTRA(MeasurementReferenceTimeNetworkTime_eUTRA),
    #[asn(key = 1, extended = false)]
    UTRA(MeasurementReferenceTimeNetworkTime_uTRA),
    #[asn(key = 2, extended = false)]
    GSM(MeasurementReferenceTimeNetworkTime_gSM),
    #[asn(key = 0, extended = true)]
    NbIoT_r14(MeasurementReferenceTimeNetworkTime_nbIoT_r14),
    #[asn(key = 1, extended = true)]
    Nr_r15(MeasurementReferenceTimeNetworkTime_nr_r15),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "512")]
pub struct MessageSizeLimitNB_r14MeasurementLimit_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct MotionTimeSource_r15TimeSource_r15(pub u8);
impl MotionTimeSource_r15TimeSource_r15 {
    pub const SERVING_CELL: u8 = 0u8;
    pub const REFERENCE_CELL: u8 = 1u8;
    pub const GNSS: u8 = 2u8;
    pub const MIXED: u8 = 3u8;
    pub const OTHER: u8 = 4u8;
    pub const NONE: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct Multi_RTT_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17(pub u8);
impl Multi_RTT_MeasCapabilityPerBand_r17SupportOfDL_PRS_FirstPathRSRP_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct Multi_RTT_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17(pub u8);
impl Multi_RTT_MeasCapabilityPerBand_r17Dl_PRS_MeasRRC_Inactive_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "37799")]
pub struct NAV_ClockModelNavToc(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct NAV_ClockModelNavaf2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NAV_ClockModelNavaf1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct NAV_ClockModelNavaf0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct NAV_ClockModelNavTgd(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NCGI_r15Mcc_r15_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct NCGI_r15Mcc_r15(pub Vec<NCGI_r15Mcc_r15_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NCGI_r15Mnc_r15_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "2", sz_ub = "3")]
pub struct NCGI_r15Mnc_r15(pub Vec<NCGI_r15Mnc_r15_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "36", sz_ub = "36")]
pub struct NCGI_r15Nr_cellidentity_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct NPRS_Info_r14OperationModeInfoNPRS_r14(pub u8);
impl NPRS_Info_r14OperationModeInfoNPRS_r14 {
    pub const INBAND: u8 = 0u8;
    pub const STANDALONE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "174")]
pub struct NPRS_Info_r14NprsSequenceInfo_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NPRS_Info_r14NprsID_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct NPRS_Info_r14PartA_r14NprsBitmap_r14_subframePattern10_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "40", sz_ub = "40")]
pub struct NPRS_Info_r14PartA_r14NprsBitmap_r14_subframePattern40_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NPRS_Info_r14PartA_r14NprsBitmap_r14 {
    #[asn(key = 0, extended = false)]
    SubframePattern10_r14(NPRS_Info_r14PartA_r14NprsBitmap_r14_subframePattern10_r14),
    #[asn(key = 1, extended = false)]
    SubframePattern40_r14(NPRS_Info_r14PartA_r14NprsBitmap_r14_subframePattern40_r14),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po2_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po4_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po8_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po16_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14 {
    #[asn(key = 0, extended = false)]
    Po2_r14(NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po2_r14),
    #[asn(key = 1, extended = false)]
    Po4_r14(NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po4_r14),
    #[asn(key = 2, extended = false)]
    Po8_r14(NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po8_r14),
    #[asn(key = 3, extended = false)]
    Po16_r14(NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14_po16_r14),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NPRS_Info_r14PartA_r14 {
    pub nprs_bitmap_r14: NPRS_Info_r14PartA_r14NprsBitmap_r14,
    #[asn(optional_idx = 0)]
    pub nprs_muting_info_a_r14: Option<NPRS_Info_r14PartA_r14Nprs_MutingInfoA_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NPRS_Info_r14PartB_r14Nprs_Period_r14(pub u8);
impl NPRS_Info_r14PartB_r14Nprs_Period_r14 {
    pub const MS160: u8 = 0u8;
    pub const MS320: u8 = 1u8;
    pub const MS640: u8 = 2u8;
    pub const MS1280: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct NPRS_Info_r14PartB_r14Nprs_startSF_r14(pub u8);
impl NPRS_Info_r14PartB_r14Nprs_startSF_r14 {
    pub const ZERO: u8 = 0u8;
    pub const ONE_EIGHTH: u8 = 1u8;
    pub const TWO_EIGHTHS: u8 = 2u8;
    pub const THREE_EIGHTHS: u8 = 3u8;
    pub const FOUR_EIGHTHS: u8 = 4u8;
    pub const FIVE_EIGHTHS: u8 = 5u8;
    pub const SIX_EIGHTHS: u8 = 6u8;
    pub const SEVEN_EIGHTHS: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct NPRS_Info_r14PartB_r14Nprs_NumSF_r14(pub u8);
impl NPRS_Info_r14PartB_r14Nprs_NumSF_r14 {
    pub const SF10: u8 = 0u8;
    pub const SF20: u8 = 1u8;
    pub const SF40: u8 = 2u8;
    pub const SF80: u8 = 3u8;
    pub const SF160: u8 = 4u8;
    pub const SF320: u8 = 5u8;
    pub const SF640: u8 = 6u8;
    pub const SF1280: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po2_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po4_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po8_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po16_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14 {
    #[asn(key = 0, extended = false)]
    Po2_r14(NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po2_r14),
    #[asn(key = 1, extended = false)]
    Po4_r14(NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po4_r14),
    #[asn(key = 2, extended = false)]
    Po8_r14(NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po8_r14),
    #[asn(key = 3, extended = false)]
    Po16_r14(NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14_po16_r14),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NPRS_Info_r14PartB_r14 {
    pub nprs_period_r14: NPRS_Info_r14PartB_r14Nprs_Period_r14,
    pub nprs_start_sf_r14: NPRS_Info_r14PartB_r14Nprs_startSF_r14,
    pub nprs_num_sf_r14: NPRS_Info_r14PartB_r14Nprs_NumSF_r14,
    #[asn(optional_idx = 0)]
    pub nprs_muting_info_b_r14: Option<NPRS_Info_r14PartB_r14Nprs_MutingInfoB_r14>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16351")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k0_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8176")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k1_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4088")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k2_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2044")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k3_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1022")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k4_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k5_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1046401")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus6_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "523201")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus5_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "261601")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus4_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "130801")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus3_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65401")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus2_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32701")]
pub struct NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus1_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16 {
    #[asn(key = 0, extended = false)]
    K0_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k0_r16),
    #[asn(key = 1, extended = false)]
    K1_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k1_r16),
    #[asn(key = 2, extended = false)]
    K2_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k2_r16),
    #[asn(key = 3, extended = false)]
    K3_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k3_r16),
    #[asn(key = 4, extended = false)]
    K4_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k4_r16),
    #[asn(key = 5, extended = false)]
    K5_r16(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_k5_r16),
    #[asn(key = 0, extended = true)]
    KMinus6_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus6_r18),
    #[asn(key = 1, extended = true)]
    KMinus5_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus5_r18),
    #[asn(key = 2, extended = true)]
    KMinus4_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus4_r18),
    #[asn(key = 3, extended = true)]
    KMinus3_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus3_r18),
    #[asn(key = 4, extended = true)]
    KMinus2_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus2_r18),
    #[asn(key = 5, extended = true)]
    KMinus1_r18(NR_AdditionalPath_r16Nr_RelativeTimeDifference_r16_kMinus1_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_AggregatedDL_PRS_ResourceInfo_Element_r18AggregatedDL_PRS_ID_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_TDOA_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_TDOA_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_AoD_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19Simul_DL_AIML_and_DL_AoD_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Connected_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Connected_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Inactive_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Inactive_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Idle_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_PRS_BWA_RRC_Idle_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Inactive_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Inactive_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Idle_r19(pub u8);
impl NR_DL_AIML_CapabilityPerBand_r19SupportOfDL_AIML_Pos_RRC_Idle_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct NR_DL_AIML_LocationInformation_r19MeasurementReferenceTime_r19_utc_time_r19(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_AIML_LocationInformation_r19MeasurementReferenceTime_r19 {
    #[asn(key = 0, extended = false)]
    SystemFrameNumber_r19(NR_TimeStamp_r16),
    #[asn(key = 1, extended = false)]
    Utc_time_r19(NR_DL_AIML_LocationInformation_r19MeasurementReferenceTime_r19_utc_time_r19),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_DL_AIML_LocationServerErrorCauses_r19Cause_r19(pub u8);
impl NR_DL_AIML_LocationServerErrorCauses_r19Cause_r19 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const ASSISTANCE_DATA_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 2u8;
    pub const NOT_PROVIDED_ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 3u8;
    pub const ON_DEMAND_DL_PRS_NOT_SUPPORTED_BY_SERVER: u8 = 4u8;
    pub const ON_DEMAND_DL_PRS_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19Dl_PRS_BufferType_RRC_Inactive_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19Dl_PRS_BufferType_RRC_Inactive_r19 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbols_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbols_r19 { pub const N_DOT125 : u8 = 0u8 ; pub const N_DOT25 : u8 = 1u8 ; pub const N_DOT5 : u8 = 2u8 ; pub const N1 : u8 = 3u8 ; pub const N2 : u8 = 4u8 ; pub const N4 : u8 = 5u8 ; pub const N6 : u8 = 6u8 ; pub const N8 : u8 = 7u8 ; pub const N12 : u8 = 8u8 ; pub const N16 : u8 = 9u8 ; pub const N20 : u8 = 10u8 ; pub const N25 : u8 = 11u8 ; pub const N30 : u8 = 12u8 ; pub const N32 : u8 = 13u8 ; pub const N35 : u8 = 14u8 ; pub const N40 : u8 = 15u8 ; pub const N45 : u8 = 16u8 ; pub const N50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19 { pub const N8 : u8 = 0u8 ; pub const N16 : u8 = 1u8 ; pub const N20 : u8 = 2u8 ; pub const N30 : u8 = 3u8 ; pub const N40 : u8 = 4u8 ; pub const N80 : u8 = 5u8 ; pub const N160 : u8 = 6u8 ; pub const N320 : u8 = 7u8 ; pub const N640 : u8 = 8u8 ; pub const N1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19 { pub duration_of_prs_processing_symbols_r19 : NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbols_r19 , pub duration_of_prs_processing_symbols_in_every_tms_r19 : NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19DurationOfPRS_Processing_RRC_Inactive_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs15_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs15_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs30_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs30_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs60_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs60_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs120_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs120_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19 { # [asn (optional_idx = 0 ,)] pub scs15_r19 : Option < NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs15_r19 > , # [asn (optional_idx = 1 ,)] pub scs30_r19 : Option < NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs30_r19 > , # [asn (optional_idx = 2 ,)] pub scs60_r19 : Option < NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs60_r19 > , # [asn (optional_idx = 3 ,)] pub scs120_r19 : Option < NR_DL_AIML_PRS_ProcessingCapability_RRC_Inactive_r19MaxNumOfDL_PRS_ResProcessedPerSlot_RRC_Inactive_r19Scs120_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_AIML_PRS_ProcessingCapability_r19Nr_dl_aiml_prs_ProcessingCapabilityBandList_r19(
    pub Vec<NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct NR_DL_AIML_PRS_ProcessingCapability_r19MultipleActivatedPRS_ProcessingWindows_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapability_r19MultipleActivatedPRS_ProcessingWindows_r19 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr1(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr1 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr2(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr2 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19 {
    #[asn(key = 0, extended = false)]
    Fr1(NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr1),
    #[asn(key = 1, extended = false)]
    Fr2(NR_DL_AIML_PRS_ProcessingCapabilityElement_r19SupportedBandwidthPRS_r19_fr2),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19Dl_PRS_BufferType_r19(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19Dl_PRS_BufferType_r19 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbols_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbols_r19 { pub const N_DOT125 : u8 = 0u8 ; pub const N_DOT25 : u8 = 1u8 ; pub const N_DOT5 : u8 = 2u8 ; pub const N1 : u8 = 3u8 ; pub const N2 : u8 = 4u8 ; pub const N4 : u8 = 5u8 ; pub const N6 : u8 = 6u8 ; pub const N8 : u8 = 7u8 ; pub const N12 : u8 = 8u8 ; pub const N16 : u8 = 9u8 ; pub const N20 : u8 = 10u8 ; pub const N25 : u8 = 11u8 ; pub const N30 : u8 = 12u8 ; pub const N32 : u8 = 13u8 ; pub const N35 : u8 = 14u8 ; pub const N40 : u8 = 15u8 ; pub const N45 : u8 = 16u8 ; pub const N50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19 { pub const N8 : u8 = 0u8 ; pub const N16 : u8 = 1u8 ; pub const N20 : u8 = 2u8 ; pub const N30 : u8 = 3u8 ; pub const N40 : u8 = 4u8 ; pub const N80 : u8 = 5u8 ; pub const N160 : u8 = 6u8 ; pub const N320 : u8 = 7u8 ; pub const N640 : u8 = 8u8 ; pub const N1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19 { pub duration_of_prs_processing_symbols_r19 : NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbols_r19 , pub duration_of_prs_processing_symbols_in_every_tms_r19 : NR_DL_AIML_PRS_ProcessingCapabilityElement_r19DurationOfPRS_Processing_r19DurationOfPRS_ProcessingSymbolsInEveryTms_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19(
    pub u8,
);
impl
    NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19 { # [asn (optional_idx = 0 ,)] pub scs15_r19 : Option < NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19 > , # [asn (optional_idx = 1 ,)] pub scs30_r19 : Option < NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19 > , # [asn (optional_idx = 2 ,)] pub scs60_r19 : Option < NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19 > , # [asn (optional_idx = 3 ,)] pub scs120_r19 : Option < NR_DL_AIML_PRS_ProcessingCapabilityElement_r19MaxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1A_r19(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1A_r19 {
    pub const OPTION1: u8 = 0u8;
    pub const OPTION2: u8 = 1u8;
    pub const OPTION3: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1B_r19(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType1B_r19 {
    pub const OPTION1: u8 = 0u8;
    pub const OPTION2: u8 = 1u8;
    pub const OPTION3: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType2_r19(pub u8);
impl NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingWindowType2_r19 {
    pub const OPTION1: u8 = 0u8;
    pub const OPTION2: u8 = 1u8;
    pub const OPTION3: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19Prs_ProcessingCapabilityOutsideMGinPPW_r19(
    pub Vec<PRS_ProcessingCapabilityOutsideMGinPPWperType_r19>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19SupportOfPRS_BWA_WithTwoPFL_Combination_r19(
    pub u8,
);
impl NR_DL_AIML_PRS_ProcessingCapabilityPerBand_r19SupportOfPRS_BWA_WithTwoPFL_Combination_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_ProvideCapabilities_r19PeriodicalReporting_r19(pub u8);
impl NR_DL_AIML_ProvideCapabilities_r19PeriodicalReporting_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Ten_ms_unit_ResponseTime_r19(pub u8);
impl NR_DL_AIML_ProvideCapabilities_r19Ten_ms_unit_ResponseTime_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_PosCalcAssistanceSupport_r19(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_los_nlos_AssistanceDataSupport_r19 {
    pub typ: LOS_NLOS_IndicatorType2_r17,
    pub granularity: LOS_NLOS_IndicatorGranularity2_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_DL_PRS_ExpectedAoD_or_AoA_Sup_r19(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_DL_AIML_On_Demand_DL_PRS_ForBWA_Support_r19(pub u8);
impl NR_DL_AIML_ProvideCapabilities_r19Nr_DL_AIML_On_Demand_DL_PRS_ForBWA_Support_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "16")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_dl_prs_AssistanceDataValidity_r19Area_validity(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_dl_prs_AssistanceDataValidity_r19 {
    #[asn(optional_idx = 0)]
    pub area_validity:
        Option<NR_DL_AIML_ProvideCapabilities_r19Nr_dl_prs_AssistanceDataValidity_r19Area_validity>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_ProvideCapabilities_r19MultiLocationEstimateInSameMeasReport_r19(pub u8);
impl NR_DL_AIML_ProvideCapabilities_r19MultiLocationEstimateInSameMeasReport_r19 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_IntegrityAssistanceSupport_r19(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_AIML_ProvideCapabilities_r19Nr_DL_AIML_CapabilityPerBandList_r19(
    pub Vec<NR_DL_AIML_CapabilityPerBand_r19>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct NR_DL_AIML_ProvideLocationInformation_r19Nr_DL_AIML_LocationInformationInstances_r19(
    pub Vec<NR_DL_AIML_LocationInformation_r19>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_AssistanceDataReq_r19(pub u8);
impl NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_AssistanceDataReq_r19 {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_ExpectedAoD_or_AoA_Req_r19(pub u8);
impl NR_DL_AIML_RequestAssistanceData_r19Nr_DL_PRS_ExpectedAoD_or_AoA_Req_r19 {
    pub const E_AO_D: u8 = 0u8;
    pub const E_AO_A: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_RequestAssistanceData_r19Pre_configured_AssistanceDataReq_r19(pub u8);
impl NR_DL_AIML_RequestAssistanceData_r19Pre_configured_AssistanceDataReq_r19 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AIML_RequestAssistanceData_r19Nr_PositionCalculationAssistanceReq_r19(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AIML_RequestAssistanceData_r19Nr_IntegrityAssistanceReq_r19(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_DL_AIML_RequestLocationInformation_r19Nr_AssistanceAvailability_r19(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_RequestLocationInformation_r19MultiLocationEstimateInSameReport_r19(pub u8);
impl NR_DL_AIML_RequestLocationInformation_r19MultiLocationEstimateInSameReport_r19 {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct NR_DL_AIML_TargetDeviceErrorCauses_r19Cause_r19(pub u8);
impl NR_DL_AIML_TargetDeviceErrorCauses_r19Cause_r19 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_MISSING: u8 = 1u8;
    pub const UNABLE_TO_MEASURE_ANY_TRP: u8 = 2u8;
    pub const ATTEMPTED_BUT_UNABLE_TO_MEASURE_SOME_NEIGHBOUR_TR_PS: u8 = 3u8;
    pub const THERE_WERE_NOT_ENOUGH_SIGNALS_RECEIVED: u8 = 4u8;
    pub const LOCATION_CALCULATION_ASSISTANCE_DATA_MISSING: u8 = 5u8;
    pub const DL_AIML_POSITIONING_NOT_AVAILABLE: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_AIML_TargetDeviceErrorCauses_r19RemoteUE_Indication_r19(pub u8);
impl NR_DL_AIML_TargetDeviceErrorCauses_r19RemoteUE_Indication_r19 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r16Nr_DL_PRS_RxBeamIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_RSRP_ResultDiff_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_RxBeamIndex_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_DL_AoD_AdditionalMeasurementElement_r17Nr_DL_PRS_FirstPathRSRP_ResultDiff_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct NR_DL_AoD_LocationInformation_r16MeasurementReferenceTime_r16_utc_time_r16(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_AoD_LocationInformation_r16MeasurementReferenceTime_r16 {
    #[asn(key = 0, extended = false)]
    Sfn_time_r16(NR_TimeStamp_r16),
    #[asn(key = 1, extended = false)]
    Utc_time_r16(NR_DL_AoD_LocationInformation_r16MeasurementReferenceTime_r16_utc_time_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_DL_AoD_LocationServerErrorCauses_r16Cause_r16(pub u8);
impl NR_DL_AoD_LocationServerErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const ASSISTANCE_DATA_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 2u8;
    pub const NOT_PROVIDED_ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_AoD_MeasElement_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct NR_DL_AoD_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_MeasElement_r16Nr_DL_PRS_RxBeamIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_MeasurementCapability_r16MaxDL_PRS_RSRP_MeasurementFR1_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_MeasurementCapability_r16MaxDL_PRS_RSRP_MeasurementFR2_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_AoD_MeasurementCapability_r16Dl_AoD_MeasCapabilityBandList_r16(
    pub Vec<DL_AoD_MeasCapabilityPerBand_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct NR_DL_AoD_ReportConfig_r16MaxDL_PRS_RSRP_MeasurementsPerTRP_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_AoD_RequestAssistanceData_r16Nr_AdType_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_DL_AoD_RequestLocationInformation_r16Nr_AssistanceAvailability_r16(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_DL_AoD_TargetDeviceErrorCauses_r16Cause_r16(pub u8);
impl NR_DL_AoD_TargetDeviceErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_MISSING: u8 = 1u8;
    pub const UNABLE_TO_MEASURE_ANY_TRP: u8 = 2u8;
    pub const ATTEMPTED_BUT_UNABLE_TO_MEASURE_SOME_NEIGHBOUR_TR_PS: u8 = 3u8;
    pub const THERE_WERE_NOT_ENOUGH_SIGNALS_RECEIVED_FOR_UE_BASED_DL_AO_D: u8 = 4u8;
    pub const LOCATION_CALCULATION_ASSISTANCE_DATA_MISSING: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_FrequencyLayerIndex_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_TRP_Index_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NR_DL_PRS_AggregationElement_r18Nr_DL_PRS_ResourceSetIndex_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_DL_PRS_AssistanceData_r16Nr_DL_PRS_AssistanceDataList_r16(
    pub Vec<NR_DL_PRS_AssistanceDataPerFreq_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "256"
)]
pub struct NR_DL_PRS_AssistanceData_r16Nr_SSB_Config_r16(pub Vec<NR_SSB_Config_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_AssistanceDataPerFreq_r16Nr_DL_PRS_AssistanceDataPerFreq_r16(
    pub Vec<NR_DL_PRS_AssistanceDataPerTRP_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_AssistanceDataPerTRP_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-3841", ub = "3841")]
pub struct NR_DL_PRS_AssistanceDataPerTRP_r16Nr_DL_PRS_ExpectedRSTD_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "246")]
pub struct NR_DL_PRS_AssistanceDataPerTRP_r16Nr_DL_PRS_ExpectedRSTD_Uncertainty_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_BeamInfoPerTRP_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_BeamInfoPerTRP_r16Associated_DL_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_AzimuthAoD_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "60")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_AzimuthAoD_Unc_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_ZenithAoD_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_ZenithAoD_Unc_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17 {
    pub expected_dl_azimuth_ao_d_r17:
        NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_AzimuthAoD_r17,
    #[asn(optional_idx = 0)]
    pub expected_dl_azimuth_ao_d_unc_r17:
        Option<NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_AzimuthAoD_Unc_r17>,
    pub expected_dl_zenith_ao_d_r17:
        NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_ZenithAoD_r17,
    #[asn(optional_idx = 1)]
    pub expected_dl_zenith_ao_d_unc_r17:
        Option<NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoD_r17ExpectedDL_ZenithAoD_Unc_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_AzimuthAoA_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "60")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_AzimuthAoA_Unc_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "180")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_ZenithAoA_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "30")]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_ZenithAoA_Unc_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17 {
    pub expected_dl_azimuth_ao_a_r17:
        NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_AzimuthAoA_r17,
    #[asn(optional_idx = 0)]
    pub expected_dl_azimuth_ao_a_unc_r17:
        Option<NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_AzimuthAoA_Unc_r17>,
    pub expected_dl_zenith_ao_a_r17:
        NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_ZenithAoA_r17,
    #[asn(optional_idx = 1)]
    pub expected_dl_zenith_ao_a_unc_r17:
        Option<NR_DL_PRS_ExpectedAoD_or_AoA_r17_expectedAoA_r17ExpectedDL_ZenithAoA_Unc_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Dl_PRS_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Nr_los_nlos_indicator_r17_perResource_r17(
    pub Vec<NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerResource_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Nr_los_nlos_indicator_r17 {
    #[asn(key = 0, extended = false)]
    PerTrp_r17(LOS_NLOS_Indicator_r17),
    #[asn(key = 1, extended = false)]
    PerResource_r17(
        NR_DL_PRS_ExpectedLOS_NLOS_AssistancePerTRP_r17Nr_los_nlos_indicator_r17_perResource_r17,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_DL_PRS_Info_r16Nr_DL_PRS_ResourceSetList_r16(pub Vec<NR_DL_PRS_ResourceSet_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_StartSFN_TimeWindow_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs15_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "20479")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs30_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "40959")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs60_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "81919")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs120_r18(
    pub u32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18
{
    # [asn (key = 0 , extended = false)] Scs15_r18 (NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs15_r18) , # [asn (key = 1 , extended = false)] Scs30_r18 (NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs30_r18) , # [asn (key = 2 , extended = false)] Scs60_r18 (NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs60_r18) , # [asn (key = 3 , extended = false)] Scs120_r18 (NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18_scs120_r18) , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18 {
    # [asn (key = 0 , extended = false)] Nr_PeriodicityAndSlotOffsetTimeWindow_r18 (NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16) , # [asn (key = 1 , extended = false)] Nr_OneShotSlotOffsetTimeWindow_r18 (NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_PeriodicOrOneShotTimeWindow_r18_nr_OneShotSlotOffsetTimeWindow_r18) , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "13")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SymbolOffsetTimeWindow_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "6")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_DurationTimeWindow_r18(pub u8);
impl NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_DurationTimeWindow_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SelectedDL_PRS_FrequencyLayerIndex_r18(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_SelectedDL_PRS_IndexListPerFreq_r18(
    pub Vec<NR_SelectedDL_PRS_IndexPerTRP_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct NR_DL_PRS_MeasurementTimeWindowsConfigElement_r18Nr_MeasurementsToPerformInTimeWindow_r18(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n4_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n5_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n8_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n10_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n16_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n20_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n32_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n40_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n64_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n80_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "159")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n160_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "319")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n320_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "639")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n640_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n1280_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2559")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n2560_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5119")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n5120_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n10240_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "16", extensible = true)]
pub enum NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16 {
    #[asn(key = 0, extended = false)]
    N4_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n4_r16),
    #[asn(key = 1, extended = false)]
    N5_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n5_r16),
    #[asn(key = 2, extended = false)]
    N8_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n8_r16),
    #[asn(key = 3, extended = false)]
    N10_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n10_r16),
    #[asn(key = 4, extended = false)]
    N16_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n16_r16),
    #[asn(key = 5, extended = false)]
    N20_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n20_r16),
    #[asn(key = 6, extended = false)]
    N32_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n32_r16),
    #[asn(key = 7, extended = false)]
    N40_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n40_r16),
    #[asn(key = 8, extended = false)]
    N64_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n64_r16),
    #[asn(key = 9, extended = false)]
    N80_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n80_r16),
    #[asn(key = 10, extended = false)]
    N160_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n160_r16),
    #[asn(key = 11, extended = false)]
    N320_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n320_r16),
    #[asn(key = 12, extended = false)]
    N640_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n640_r16),
    #[asn(key = 13, extended = false)]
    N1280_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n1280_r16),
    #[asn(key = 14, extended = false)]
    N2560_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n2560_r16),
    #[asn(key = 15, extended = false)]
    N5120_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n5120_r16),
    #[asn(key = 16, extended = false)]
    N10240_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs15_r16_n10240_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n8_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n10_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n16_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n20_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n32_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n40_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n64_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n80_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n128_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "159")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n160_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "319")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n320_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "639")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n640_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n1280_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2559")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n2560_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5119")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n5120_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n10240_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "20479")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n20480_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "16", extensible = true)]
pub enum NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16 {
    #[asn(key = 0, extended = false)]
    N8_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n8_r16),
    #[asn(key = 1, extended = false)]
    N10_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n10_r16),
    #[asn(key = 2, extended = false)]
    N16_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n16_r16),
    #[asn(key = 3, extended = false)]
    N20_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n20_r16),
    #[asn(key = 4, extended = false)]
    N32_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n32_r16),
    #[asn(key = 5, extended = false)]
    N40_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n40_r16),
    #[asn(key = 6, extended = false)]
    N64_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n64_r16),
    #[asn(key = 7, extended = false)]
    N80_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n80_r16),
    #[asn(key = 8, extended = false)]
    N128_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n128_r16),
    #[asn(key = 9, extended = false)]
    N160_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n160_r16),
    #[asn(key = 10, extended = false)]
    N320_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n320_r16),
    #[asn(key = 11, extended = false)]
    N640_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n640_r16),
    #[asn(key = 12, extended = false)]
    N1280_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n1280_r16),
    #[asn(key = 13, extended = false)]
    N2560_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n2560_r16),
    #[asn(key = 14, extended = false)]
    N5120_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n5120_r16),
    #[asn(key = 15, extended = false)]
    N10240_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n10240_r16),
    #[asn(key = 16, extended = false)]
    N20480_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs30_r16_n20480_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n16_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n20_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n32_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n40_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n64_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n80_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n128_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "159")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n160_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n256_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "319")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n320_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "639")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n640_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n1280_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2559")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n2560_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5119")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n5120_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n10240_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "20479")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n20480_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "40959")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n40960_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "16", extensible = true)]
pub enum NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16 {
    #[asn(key = 0, extended = false)]
    N16_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n16_r16),
    #[asn(key = 1, extended = false)]
    N20_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n20_r16),
    #[asn(key = 2, extended = false)]
    N32_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n32_r16),
    #[asn(key = 3, extended = false)]
    N40_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n40_r16),
    #[asn(key = 4, extended = false)]
    N64_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n64_r16),
    #[asn(key = 5, extended = false)]
    N80_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n80_r16),
    #[asn(key = 6, extended = false)]
    N128_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n128_r16),
    #[asn(key = 7, extended = false)]
    N160_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n160_r16),
    #[asn(key = 8, extended = false)]
    N256_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n256_r16),
    #[asn(key = 9, extended = false)]
    N320_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n320_r16),
    #[asn(key = 10, extended = false)]
    N640_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n640_r16),
    #[asn(key = 11, extended = false)]
    N1280_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n1280_r16),
    #[asn(key = 12, extended = false)]
    N2560_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n2560_r16),
    #[asn(key = 13, extended = false)]
    N5120_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n5120_r16),
    #[asn(key = 14, extended = false)]
    N10240_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n10240_r16),
    #[asn(key = 15, extended = false)]
    N20480_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n20480_r16),
    #[asn(key = 16, extended = false)]
    N40960_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs60_r16_n40960_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n32_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n40_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n64_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n80_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n128_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "159")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n160_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n256_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "319")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n320_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n512_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "639")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n640_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n1280_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2559")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n2560_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5119")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n5120_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "10239")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n10240_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "20479")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n20480_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "40959")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n40960_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "81919")]
pub struct NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n81920_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "16", extensible = true)]
pub enum NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16 {
    #[asn(key = 0, extended = false)]
    N32_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n32_r16),
    #[asn(key = 1, extended = false)]
    N40_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n40_r16),
    #[asn(key = 2, extended = false)]
    N64_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n64_r16),
    #[asn(key = 3, extended = false)]
    N80_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n80_r16),
    #[asn(key = 4, extended = false)]
    N128_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n128_r16),
    #[asn(key = 5, extended = false)]
    N160_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n160_r16),
    #[asn(key = 6, extended = false)]
    N256_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n256_r16),
    #[asn(key = 7, extended = false)]
    N320_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n320_r16),
    #[asn(key = 8, extended = false)]
    N512_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n512_r16),
    #[asn(key = 9, extended = false)]
    N640_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n640_r16),
    #[asn(key = 10, extended = false)]
    N1280_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n1280_r16),
    #[asn(key = 11, extended = false)]
    N2560_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n2560_r16),
    #[asn(key = 12, extended = false)]
    N5120_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n5120_r16),
    #[asn(key = 13, extended = false)]
    N10240_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n10240_r16),
    #[asn(key = 14, extended = false)]
    N20480_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n20480_r16),
    #[asn(key = 15, extended = false)]
    N40960_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n40960_r16),
    #[asn(key = 16, extended = false)]
    N81920_r16(NR_DL_PRS_Periodicity_and_ResourceSetSlotOffset_r16_scs120_r16_n81920_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_SubcarrierSpacing_r16(pub u8);
impl NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_SubcarrierSpacing_r16 {
    pub const K_HZ15: u8 = 0u8;
    pub const K_HZ30: u8 = 1u8;
    pub const K_HZ60: u8 = 2u8;
    pub const K_HZ120: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "63")]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_ResourceBandwidth_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2176")]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_StartPRB_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CombSizeN_r16(pub u8);
impl NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CombSizeN_r16 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N12: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CyclicPrefix_r16(pub u8);
impl NR_DL_PRS_PositioningFrequencyLayer_r16Dl_PRS_CyclicPrefix_r16 {
    pub const NORMAL: u8 = 0u8;
    pub const EXTENDED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_PRS_ProcessingCapability_r16Prs_ProcessingCapabilityBandList_r16(
    pub Vec<PRS_ProcessingCapabilityPerBand_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_DL_PRS_ProcessingCapability_r16MaxSupportedFreqLayers_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_PRS_ProcessingCapability_r16SimulLTE_NR_PRS_r16(pub u8);
impl NR_DL_PRS_ProcessingCapability_r16SimulLTE_NR_PRS_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_PRS_QCL_ProcessingCapability_r16Dl_PRS_QCL_ProcessingCapabilityBandList_r16(
    pub Vec<DL_PRS_QCL_ProcessingCapabilityPerBand_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_SequenceID_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n2_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n4_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n6_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "11")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n12_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = true)]
pub enum NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16 {
    #[asn(key = 0, extended = false)]
    N2_r16(NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n2_r16),
    #[asn(key = 1, extended = false)]
    N4_r16(NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n4_r16),
    #[asn(key = 2, extended = false)]
    N6_r16(NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n6_r16),
    #[asn(key = 3, extended = false)]
    N12_r16(NR_DL_PRS_Resource_r16Dl_PRS_CombSizeN_AndReOffset_r16_n12_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_ResourceSlotOffset_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "12")]
pub struct NR_DL_PRS_Resource_r16Dl_PRS_ResourceSymbolOffset_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceRepetitionFactor_r16(pub u8);
impl NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceRepetitionFactor_r16 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceTimeGap_r16(pub u8);
impl NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceTimeGap_r16 {
    pub const S1: u8 = 0u8;
    pub const S2: u8 = 1u8;
    pub const S4: u8 = 2u8;
    pub const S8: u8 = 3u8;
    pub const S16: u8 = 4u8;
    pub const S32: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_DL_PRS_ResourceSet_r16Dl_PRS_NumSymbols_r16(pub u8);
impl NR_DL_PRS_ResourceSet_r16Dl_PRS_NumSymbols_r16 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N12: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-60", ub = "50")]
pub struct NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourcePower_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_DL_PRS_ResourceSet_r16Dl_PRS_ResourceList_r16(pub Vec<NR_DL_PRS_Resource_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "2")]
pub struct NR_DL_PRS_ResourcesCapability_r16MaxNrOfDL_PRS_ResourceSetPerTrpPerFrequencyLayer_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "7")]
pub struct NR_DL_PRS_ResourcesCapability_r16MaxNrOfTRP_AcrossFreqs_r16(pub u8);
impl NR_DL_PRS_ResourcesCapability_r16MaxNrOfTRP_AcrossFreqs_r16 {
    pub const N4: u8 = 0u8;
    pub const N6: u8 = 1u8;
    pub const N12: u8 = 2u8;
    pub const N16: u8 = 3u8;
    pub const N32: u8 = 4u8;
    pub const N64: u8 = 5u8;
    pub const N128: u8 = 6u8;
    pub const N256: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_DL_PRS_ResourcesCapability_r16MaxNrOfPosLayer_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_DL_PRS_ResourcesCapability_r16Dl_PRS_ResourcesCapabilityBandList_r16(
    pub Vec<DL_PRS_ResourcesCapabilityPerBand_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NR_DL_PRS_SFN0_Offset_r16Sfn_Offset_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NR_DL_PRS_SFN0_Offset_r16IntegerSubframeOffset_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_PRS_TRP_TEG_InfoPerTRP_r17Dl_PRS_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_DL_PRS_TRP_TEG_InfoPerTRP_r17Dl_PRS_TEG_InfoSet_r17(
    pub Vec<DL_PRS_TEG_InfoPerResourceSet_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k0_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k1_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k2_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k3_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k4_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k5_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "524224")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus6_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "262112")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus5_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131056")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus4_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65528")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus3_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32764")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus2_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16382")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus1_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16 {
    #[asn(key = 0, extended = false)]
    K0_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k0_r16),
    #[asn(key = 1, extended = false)]
    K1_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k1_r16),
    #[asn(key = 2, extended = false)]
    K2_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k2_r16),
    #[asn(key = 3, extended = false)]
    K3_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k3_r16),
    #[asn(key = 4, extended = false)]
    K4_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k4_r16),
    #[asn(key = 5, extended = false)]
    K5_r16(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_k5_r16),
    #[asn(key = 0, extended = true)]
    KMinus6_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus6_r18),
    #[asn(key = 1, extended = true)]
    KMinus5_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus5_r18),
    #[asn(key = 2, extended = true)]
    KMinus4_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus4_r18),
    #[asn(key = 3, extended = true)]
    KMinus3_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus3_r18),
    #[asn(key = 4, extended = true)]
    KMinus2_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus2_r18),
    #[asn(key = 5, extended = true)]
    KMinus1_r18(NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_RSTD_ResultDiff_r16_kMinus1_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_DL_TDOA_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct NR_DL_TDOA_LocationInformation_r16MeasurementReferenceTime_r16_utc_time_r16(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum NR_DL_TDOA_LocationInformation_r16MeasurementReferenceTime_r16 {
    #[asn(key = 0, extended = false)]
    SystemFrameNumber_r16(NR_TimeStamp_r16),
    #[asn(key = 1, extended = false)]
    Utc_time_r16(NR_DL_TDOA_LocationInformation_r16MeasurementReferenceTime_r16_utc_time_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_DL_TDOA_LocationServerErrorCauses_r16Cause_r16(pub u8);
impl NR_DL_TDOA_LocationServerErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const ASSISTANCE_DATA_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 2u8;
    pub const NOT_PROVIDED_ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_DL_TDOA_MeasElement_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1970049")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k0_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "985025")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k1_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "492513")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k2_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "246257")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k3_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "123129")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k4_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61565")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k5_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126083073")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus6_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63041537")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus5_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31520769")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus4_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15760385")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus3_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7880193")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus2_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3940097")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus1_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16 {
    #[asn(key = 0, extended = false)]
    K0_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k0_r16),
    #[asn(key = 1, extended = false)]
    K1_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k1_r16),
    #[asn(key = 2, extended = false)]
    K2_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k2_r16),
    #[asn(key = 3, extended = false)]
    K3_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k3_r16),
    #[asn(key = 4, extended = false)]
    K4_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k4_r16),
    #[asn(key = 5, extended = false)]
    K5_r16(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_k5_r16),
    #[asn(key = 0, extended = true)]
    KMinus6_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus6_r18),
    #[asn(key = 1, extended = true)]
    KMinus5_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus5_r18),
    #[asn(key = 2, extended = true)]
    KMinus4_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus4_r18),
    #[asn(key = 3, extended = true)]
    KMinus3_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus3_r18),
    #[asn(key = 4, extended = true)]
    KMinus2_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus2_r18),
    #[asn(key = 5, extended = true)]
    KMinus1_r18(NR_DL_TDOA_MeasElement_r16Nr_RSTD_r16_kMinus1_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct NR_DL_TDOA_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_DL_TDOA_MeasurementCapability_r16Dl_RSTD_MeasurementPerPairOfTRP_FR1_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_DL_TDOA_MeasurementCapability_r16Dl_RSTD_MeasurementPerPairOfTRP_FR2_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR1_r16(pub u8);
impl NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR1_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR2_r16(pub u8);
impl NR_DL_TDOA_MeasurementCapability_r16SupportOfDL_PRS_RSRP_MeasFR2_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_TDOA_ProvideCapabilities_r16AdditionalPathsReport_r16(pub u8);
impl NR_DL_TDOA_ProvideCapabilities_r16AdditionalPathsReport_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_DL_TDOA_ReportConfig_r16MaxDL_PRS_RSTD_MeasurementsPerTRP_Pair_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct NR_DL_TDOA_ReportConfig_r16TimingReportingGranularityFactor_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_TDOA_RequestAssistanceData_r16Nr_AdType_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_TDOA_RequestLocationInformation_r16Nr_DL_PRS_RstdMeasurementInfoRequest_r16(
    pub u8,
);
impl NR_DL_TDOA_RequestLocationInformation_r16Nr_DL_PRS_RstdMeasurementInfoRequest_r16 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_DL_TDOA_RequestLocationInformation_r16Nr_RequestedMeasurements_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_DL_TDOA_RequestLocationInformation_r16Nr_AssistanceAvailability_r16(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_DL_TDOA_RequestLocationInformation_r16AdditionalPaths_r16(pub u8);
impl NR_DL_TDOA_RequestLocationInformation_r16AdditionalPaths_r16 {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_DL_TDOA_TargetDeviceErrorCauses_r16Cause_r16(pub u8);
impl NR_DL_TDOA_TargetDeviceErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_MISSING: u8 = 1u8;
    pub const UNABLE_TO_MEASURE_ANY_TRP: u8 = 2u8;
    pub const ATTEMPTED_BUT_UNABLE_TO_MEASURE_SOME_NEIGHBOUR_TR_PS: u8 = 3u8;
    pub const THERE_WERE_NOT_ENOUGH_SIGNALS_RECEIVED_FOR_UE_BASED_DL_TDOA: u8 = 4u8;
    pub const LOCATION_CALCULATION_ASSISTANCE_DATA_MISSING: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct NR_ECID_LocationServerErrorCauses_r16Cause_r16(pub u8);
impl NR_ECID_LocationServerErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_ECID_ProvideCapabilities_r16Nr_ECID_MeasSupported_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_ECID_ProvideCapabilities_r16PeriodicalReporting_r16(pub u8);
impl NR_ECID_ProvideCapabilities_r16PeriodicalReporting_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_ECID_ProvideCapabilities_r16TriggeredReporting_r16(pub u8);
impl NR_ECID_ProvideCapabilities_r16TriggeredReporting_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_ECID_RequestLocationInformation_r16RequestedMeasurements_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct NR_ECID_TargetDeviceErrorCauses_r16Cause_r16(pub u8);
impl NR_ECID_TargetDeviceErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const REQUESTED_MEASUREMENT_NOT_AVAILABLE: u8 = 1u8;
    pub const NOT_ALLREQUESTED_MEASUREMENTS_POSSIBLE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct NR_ECID_TargetDeviceErrorCauses_r16Ss_RSRPMeasurementNotPossible_r16;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct NR_ECID_TargetDeviceErrorCauses_r16Ss_RSRQMeasurementNotPossible_r16;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct NR_ECID_TargetDeviceErrorCauses_r16Csi_RSRPMeasurementNotPossible_r16;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct NR_ECID_TargetDeviceErrorCauses_r16Csi_RSRQMeasurementNotPossible_r16;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityBeamInfoBounds_r18MeanAzimuth_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityBeamInfoBounds_r18StdDevAzimuth_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityBeamInfoBounds_r18MeanElevation_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityBeamInfoBounds_r18StdDevElevation_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NR_IntegrityBeamPowerBounds_r18MeanBeamPower_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NR_IntegrityBeamPowerBounds_r18StdDevBeamPower_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct NR_IntegrityLocationBounds_r18Units_r18(pub u8);
impl NR_IntegrityLocationBounds_r18Units_r18 {
    pub const MM: u8 = 0u8;
    pub const CM: u8 = 1u8;
    pub const M: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18Horizontal_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18Vertical_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18 {
    pub horizontal_r18: NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18Horizontal_r18,
    pub vertical_r18: NR_IntegrityLocationBounds_r18MeanLocationErrorBound_r18Vertical_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18Horizontal_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18Vertical_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18 {
    pub horizontal_r18: NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18Horizontal_r18,
    pub vertical_r18: NR_IntegrityLocationBounds_r18StdDevLocationErrorBound_r18Vertical_r18,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersDL_PRS_BeamInfo_r18Dl_PRS_BeamInfoErrorCorrelationTime_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersRTD_Info_r18Rtd_ErrorCorrelationTime_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersTRP_BeamAntennaInfo_r18Trp_BeamAntennaInfoErrorCorrelationTime_r18(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersTRP_LocationInfo_r18Trp_ErrorCorrelationTime_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersTRP_LocationInfo_r18Dl_PRS_ResourceSetARP_ErrorCorrelationTime_r18(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityParametersTRP_LocationInfo_r18Dl_PRS_ResourceARP_ErrorCorrelationTime_r18(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_IntegrityRTD_InfoBounds_r18Resolution_r18(pub u8);
impl NR_IntegrityRTD_InfoBounds_r18Resolution_r18 {
    pub const MDOT1: u8 = 0u8;
    pub const M1: u8 = 1u8;
    pub const M10: u8 = 2u8;
    pub const M30: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityRTD_InfoBounds_r18MeanRTD_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_IntegrityRTD_InfoBounds_r18StdDevRTD_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityRiskParameters_r18Nr_ProbOnsetTRP_Fault_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "3600")]
pub struct NR_IntegrityRiskParameters_r18Nr_MeanTRP_FaultDuration_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityServiceParameters_r18Ir_Minimum_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_IntegrityServiceParameters_r18Ir_Maximum_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NR_MeasuredResultsElement_r16Nr_ARFCN_r16 {
    #[asn(key = 0, extended = false)]
    Ssb_ARFCN_r16(ARFCN_ValueNR_r15),
    #[asn(key = 1, extended = false)]
    Csi_RS_pointA_r16(ARFCN_ValueNR_r15),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct NR_MeasuredResultsElement_r16SystemFrameNumber_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_DL_PRS_RSRP_ResultDiff_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k0_r16(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k1_r16(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k2_r16(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k3_r16(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k4_r16(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k5_r16(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "524224")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus6_r18(
    pub u32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "262112")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus5_r18(
    pub u32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131056")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus4_r18(
    pub u32,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65528")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus3_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "32764")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus2_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16382")]
pub struct NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus1_r18(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16 {
    #[asn(key = 0, extended = false)]
    K0_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k0_r16),
    #[asn(key = 1, extended = false)]
    K1_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k1_r16),
    #[asn(key = 2, extended = false)]
    K2_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k2_r16),
    #[asn(key = 3, extended = false)]
    K3_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k3_r16),
    #[asn(key = 4, extended = false)]
    K4_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k4_r16),
    #[asn(key = 5, extended = false)]
    K5_r16(NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_k5_r16),
    #[asn(key = 0, extended = true)]
    KMinus6_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus6_r18,
    ),
    #[asn(key = 1, extended = true)]
    KMinus5_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus5_r18,
    ),
    #[asn(key = 2, extended = true)]
    KMinus4_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus4_r18,
    ),
    #[asn(key = 3, extended = true)]
    KMinus3_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus3_r18,
    ),
    #[asn(key = 4, extended = true)]
    KMinus2_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus2_r18,
    ),
    #[asn(key = 5, extended = true)]
    KMinus1_r18(
        NR_Multi_RTT_AdditionalMeasurementElement_r16Nr_UE_RxTxTimeDiffAdditional_r16_kMinus1_r18,
    ),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct NR_Multi_RTT_LocationServerErrorCauses_r16Cause_r16(pub u8);
impl NR_Multi_RTT_LocationServerErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const ASSISTANCE_DATA_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_Multi_RTT_MeasElement_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1970049")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k0_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "985025")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k1_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "492513")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k2_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "246257")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k3_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "123129")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k4_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61565")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k5_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126083073")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus6_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63041537")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus5_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31520769")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus4_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15760385")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus3_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7880193")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus2_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3940097")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus1_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "5", extensible = true)]
pub enum NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16 {
    #[asn(key = 0, extended = false)]
    K0_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k0_r16),
    #[asn(key = 1, extended = false)]
    K1_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k1_r16),
    #[asn(key = 2, extended = false)]
    K2_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k2_r16),
    #[asn(key = 3, extended = false)]
    K3_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k3_r16),
    #[asn(key = 4, extended = false)]
    K4_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k4_r16),
    #[asn(key = 5, extended = false)]
    K5_r16(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_k5_r16),
    #[asn(key = 0, extended = true)]
    KMinus6_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus6_r18),
    #[asn(key = 1, extended = true)]
    KMinus5_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus5_r18),
    #[asn(key = 2, extended = true)]
    KMinus4_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus4_r18),
    #[asn(key = 3, extended = true)]
    KMinus3_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus3_r18),
    #[asn(key = 4, extended = true)]
    KMinus2_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus2_r18),
    #[asn(key = 5, extended = true)]
    KMinus1_r18(NR_Multi_RTT_MeasElement_r16Nr_UE_RxTxTimeDiff_r16_kMinus1_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct NR_Multi_RTT_MeasElement_r16Nr_DL_PRS_RSRP_Result_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_Multi_RTT_MeasurementCapability_r16MaxNrOfRx_TX_MeasFR1_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_Multi_RTT_MeasurementCapability_r16MaxNrOfRx_TX_MeasFR2_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR1_r16(pub u8);
impl NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR1_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR2_r16(pub u8);
impl NR_Multi_RTT_MeasurementCapability_r16SupportOfRSRP_MeasFR2_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR1_r16(pub u8);
impl NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR1_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR2_r16(pub u8);
impl NR_Multi_RTT_MeasurementCapability_r16Srs_AssocPRS_MultiLayersFR2_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_ProvideCapabilities_r16AdditionalPathsReport_r16(pub u8);
impl NR_Multi_RTT_ProvideCapabilities_r16AdditionalPathsReport_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_ProvideCapabilities_r16PeriodicalReporting_r16(pub u8);
impl NR_Multi_RTT_ProvideCapabilities_r16PeriodicalReporting_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_Multi_RTT_ReportConfig_r16MaxDL_PRS_RxTxTimeDiffMeasPerTRP_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct NR_Multi_RTT_ReportConfig_r16TimingReportingGranularityFactor_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_Multi_RTT_RequestAssistanceData_r16Nr_AdType_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_RequestLocationInformation_r16Nr_UE_RxTxTimeDiffMeasurementInfoRequest_r16(
    pub u8,
);
impl NR_Multi_RTT_RequestLocationInformation_r16Nr_UE_RxTxTimeDiffMeasurementInfoRequest_r16 {
    pub const TRUE: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_Multi_RTT_RequestLocationInformation_r16Nr_RequestedMeasurements_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_Multi_RTT_RequestLocationInformation_r16Nr_AssistanceAvailability_r16(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_Multi_RTT_RequestLocationInformation_r16AdditionalPaths_r16(pub u8);
impl NR_Multi_RTT_RequestLocationInformation_r16AdditionalPaths_r16 {
    pub const REQUESTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_Multi_RTT_SignalMeasurementInformation_r16Nr_NTA_Offset_r16(pub u8);
impl NR_Multi_RTT_SignalMeasurementInformation_r16Nr_NTA_Offset_r16 {
    pub const N_TA1: u8 = 0u8;
    pub const N_TA2: u8 = 1u8;
    pub const N_TA3: u8 = 2u8;
    pub const N_TA4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_Multi_RTT_TargetDeviceErrorCauses_r16Cause_r16(pub u8);
impl NR_Multi_RTT_TargetDeviceErrorCauses_r16Cause_r16 {
    pub const UNDEFINED: u8 = 0u8;
    pub const DL_ASSISTANCE_DATA_MISSING: u8 = 1u8;
    pub const UNABLE_TO_MEASURE_ANY_TRP: u8 = 2u8;
    pub const ATTEMPTED_BUT_UNABLE_TO_MEASURE_SOME_NEIGHBOUR_TR_PS: u8 = 3u8;
    pub const UL_SRS_CONFIGURATION_MISSING: u8 = 4u8;
    pub const UNABLE_TO_TRANSMIT_UL_SRS: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NR_MutingPattern_r16_po2_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NR_MutingPattern_r16_po4_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct NR_MutingPattern_r16_po6_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NR_MutingPattern_r16_po8_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "16", sz_ub = "16")]
pub struct NR_MutingPattern_r16_po16_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "32", sz_ub = "32")]
pub struct NR_MutingPattern_r16_po32_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "542")]
pub struct NR_NTN_UE_RxTxMeasurements_r18Nr_NTN_UE_RxTxTimeDiffSubframeOffset_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-265", ub = "265")]
pub struct NR_NTN_UE_RxTxMeasurements_r18Nr_NTN_DL_TimingDrift_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_On_Demand_DL_PRS_Configurations_r17On_demand_dl_prs_configuration_list_r17(
    pub Vec<On_Demand_DL_PRS_Configuration_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_FrequencyRangeReq_r17(pub u8);
impl NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_FrequencyRangeReq_r17 {
    pub const FR1: u8 = 0u8;
    pub const FR2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "19")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceSetPeriodicityReq_r17(pub u8);
impl NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceSetPeriodicityReq_r17 {
    pub const P4: u8 = 0u8;
    pub const P5: u8 = 1u8;
    pub const P8: u8 = 2u8;
    pub const P10: u8 = 3u8;
    pub const P16: u8 = 4u8;
    pub const P20: u8 = 5u8;
    pub const P32: u8 = 6u8;
    pub const P40: u8 = 7u8;
    pub const P64: u8 = 8u8;
    pub const P80: u8 = 9u8;
    pub const P160: u8 = 10u8;
    pub const P320: u8 = 11u8;
    pub const P640: u8 = 12u8;
    pub const P1280: u8 = 13u8;
    pub const P2560: u8 = 14u8;
    pub const P5120: u8 = 15u8;
    pub const P10240: u8 = 16u8;
    pub const P20480: u8 = 17u8;
    pub const P40960: u8 = 18u8;
    pub const P81920: u8 = 19u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "63")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceBandwidthReq_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceRepetitionFactorReq_r17(pub u8);
impl NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_ResourceRepetitionFactorReq_r17 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_NumSymbolsReq_r17(pub u8);
impl NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_NumSymbolsReq_r17 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N12: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_CombSizeN_Req_r17(pub u8);
impl NR_On_Demand_DL_PRS_PerFreqLayer_r17Dl_prs_CombSizeN_Req_r17 {
    pub const N2: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N6: u8 = 2u8;
    pub const N12: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct NR_On_Demand_DL_PRS_Request_r17Dl_prs_configuration_id_PrefList_r17(
    pub Vec<DL_PRS_Configuration_ID_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_InformationSup_r17(pub u8);
impl NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_InformationSup_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_ConfigurationsSup_r17(pub u8);
impl NR_On_Demand_DL_PRS_Support_r17Nr_on_demand_DL_PRS_ConfigurationsSup_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "4")]
pub struct NR_OnDemandDL_PRS_AggregationReqElement_r18_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_DL_PRS_RSRP_ResultDiff_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_DL_PRS_FirstPathRSRP_ResultDiff_r18(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSCP_AdditionalMeasurementsList_r18(
    pub Vec<NR_RSCP_AdditionalMeasurements_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSRPDiff_AdditionalMeasurementsList_r18_Entry(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSRPDiff_AdditionalMeasurementsList_r18 (pub Vec < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_RSRPDiff_AdditionalMeasurementsList_r18_Entry >) ;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AdditionalMeasurementsList_r18_Entry(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "4")]
pub struct NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AdditionalMeasurementsList_r18 (pub Vec < NR_PRU_RSCP_AdditionalMeasurementElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AdditionalMeasurementsList_r18_Entry >) ;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_PRU_RSCP_MeasElement_r18Dl_PRS_ID_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NR_PRU_RSCP_MeasElement_r18Nr_los_nlos_Indicator_r18 {
    #[asn(key = 0, extended = false)]
    PerTRP(LOS_NLOS_Indicator_r17),
    #[asn(key = 1, extended = false)]
    PerResource(LOS_NLOS_Indicator_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_RSCP_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_DL_PRS_RSRP_Result_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "126")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_DL_PRS_FirstPathRSRP_Result_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSCP_AddSampleMeasurements_r18(
    pub Vec<NR_RSCP_AdditionalMeasurements_r18>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSRP_AddSampleMeasurements_r18_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSRP_AddSampleMeasurements_r18(
    pub Vec<NR_PRU_RSCP_MeasElement_r18Nr_PRU_RSRP_AddSampleMeasurements_r18_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "61")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AddSampleMeasurements_r18_Entry(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "3")]
pub struct NR_PRU_RSCP_MeasElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AddSampleMeasurements_r18(
    pub  Vec<
        NR_PRU_RSCP_MeasElement_r18Nr_PRU_FirstPathRSRP_ResultDiff_AddSampleMeasurements_r18_Entry,
    >,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "32")]
pub struct NR_PeriodicControlParam_r18DeliveryAmount_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "4", ub = "81920")]
pub struct NR_PeriodicControlParam_r18DeliveryInterval_r18(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "179")]
pub struct NR_PhaseQuality_r18PhaseQualityIndex_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct NR_PhaseQuality_r18PhaseQualityResolution_r18(pub u8);
impl NR_PhaseQuality_r18PhaseQualityResolution_r18 {
    pub const MDOT1: u8 = 0u8;
    pub const M1: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct NR_RSCP_AdditionalMeasurements_r18Nr_RSCP_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599")]
pub struct NR_RSCPD_AdditionalMeasurementSamplesElement_r18Nr_RSCPD_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct NR_SRS_TxTEG_Element_r17Nr_UE_Tx_TEG_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2199")]
pub struct NR_SRS_TxTEG_Element_r17CarrierFreq_r17OffsetToPointA_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_SRS_TxTEG_Element_r17CarrierFreq_r17 {
    pub absolute_frequency_point_a_r17: ARFCN_ValueNR_r15,
    pub offset_to_point_a_r17: NR_SRS_TxTEG_Element_r17CarrierFreq_r17OffsetToPointA_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_SRS_TxTEG_Element_r17Srs_PosResourceList_r17_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_SRS_TxTEG_Element_r17Srs_PosResourceList_r17(
    pub Vec<NR_SRS_TxTEG_Element_r17Srs_PosResourceList_r17_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-60", ub = "50")]
pub struct NR_SSB_Config_r16Ss_PBCH_BlockPower_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NR_SSB_Config_r16HalfFrameIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct NR_SSB_Config_r16Ssb_periodicity_r16(pub u8);
impl NR_SSB_Config_r16Ssb_periodicity_r16 {
    pub const MS5: u8 = 0u8;
    pub const MS10: u8 = 1u8;
    pub const MS20: u8 = 2u8;
    pub const MS40: u8 = 3u8;
    pub const MS80: u8 = 4u8;
    pub const MS160: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NR_SSB_Config_r16Ssb_PositionsInBurst_r16_shortBitmap_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "8", sz_ub = "8")]
pub struct NR_SSB_Config_r16Ssb_PositionsInBurst_r16_mediumBitmap_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct NR_SSB_Config_r16Ssb_PositionsInBurst_r16_longBitmap_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = false)]
pub enum NR_SSB_Config_r16Ssb_PositionsInBurst_r16 {
    #[asn(key = 0, extended = false)]
    ShortBitmap_r16(NR_SSB_Config_r16Ssb_PositionsInBurst_r16_shortBitmap_r16),
    #[asn(key = 1, extended = false)]
    MediumBitmap_r16(NR_SSB_Config_r16Ssb_PositionsInBurst_r16_mediumBitmap_r16),
    #[asn(key = 2, extended = false)]
    LongBitmap_r16(NR_SSB_Config_r16Ssb_PositionsInBurst_r16_longBitmap_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct NR_SSB_Config_r16Ssb_SubcarrierSpacing_r16(pub u8);
impl NR_SSB_Config_r16Ssb_SubcarrierSpacing_r16 {
    pub const K_HZ15: u8 = 0u8;
    pub const K_HZ30: u8 = 1u8;
    pub const K_HZ60: u8 = 2u8;
    pub const K_HZ120: u8 = 3u8;
    pub const K_HZ240: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NR_SSB_Config_r16Sfn_SSB_Offset_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r16Nr_SelectedTRP_Index_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r16Dl_SelectedPRS_ResourceSetIndexList_r16(
    pub Vec<DL_SelectedPRS_ResourceSetIndex_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r18Nr_SelectedTRP_Index_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r18Dl_SelectedPRS_ResourceSetIndexList_r18_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct NR_SelectedDL_PRS_IndexPerTRP_r18Dl_SelectedPRS_ResourceSetIndexList_r18(
    pub Vec<NR_SelectedDL_PRS_IndexPerTRP_r18Dl_SelectedPRS_ResourceSetIndexList_r18_Entry>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NR_SelectedDL_PRS_PerFreq_r16Nr_SelectedDL_PRS_FrequencyLayerIndex_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_SelectedDL_PRS_PerFreq_r16Nr_SelectedDL_PRS_IndexListPerFreq_r16(
    pub Vec<NR_SelectedDL_PRS_IndexPerTRP_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "359")]
pub struct NR_TRP_BeamAntennaInfoAzimuthElevation_r17Azimuth_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NR_TRP_BeamAntennaInfoAzimuthElevation_r17Azimuth_fine_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1801"
)]
pub struct NR_TRP_BeamAntennaInfoAzimuthElevation_r17ElevationList_r17(
    pub Vec<ElevationElement_R17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_TRP_BeamAntennaInfoPerTRP_r17Dl_PRS_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_TRP_BeamAntennaInfoPerTRP_r17Associated_DL_PRS_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_TRP_IntegrityServiceAlertElement_r18Dl_PRS_ID_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_TRP_IntegrityServiceAlertElement_r18Rtd_DoNotUse_r18(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_TRP_IntegrityServiceAlertElement_r18Trp_LocationDoNotUse_r18(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_TRP_IntegrityServiceAlertElement_r18BeamInfo_DoNotUse_r18(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NR_TRP_IntegrityServiceAlertElement_r18BeamAntennaInfo_DoNotUse_r18(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "64")]
pub struct NR_TRP_LocationInfoPerFreqLayer_r16Trp_LocationInfoList_r16(
    pub Vec<TRP_LocationInfoElement_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_TimeStamp_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NR_TimeStamp_r16Nr_SFN_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct NR_TimeStamp_r16Nr_Slot_r16_scs15_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct NR_TimeStamp_r16Nr_Slot_r16_scs30_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct NR_TimeStamp_r16Nr_Slot_r16_scs60_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct NR_TimeStamp_r16Nr_Slot_r16_scs120_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum NR_TimeStamp_r16Nr_Slot_r16 {
    #[asn(key = 0, extended = false)]
    Scs15_r16(NR_TimeStamp_r16Nr_Slot_r16_scs15_r16),
    #[asn(key = 1, extended = false)]
    Scs30_r16(NR_TimeStamp_r16Nr_Slot_r16_scs30_r16),
    #[asn(key = 2, extended = false)]
    Scs60_r16(NR_TimeStamp_r16Nr_Slot_r16_scs60_r16),
    #[asn(key = 3, extended = false)]
    Scs120_r16(NR_TimeStamp_r16Nr_Slot_r16_scs120_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_TimingQuality_r16TimingQualityValue_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct NR_TimingQuality_r16TimingQualityResolution_r16(pub u8);
impl NR_TimingQuality_r16TimingQualityResolution_r16 {
    pub const MDOT1: u8 = 0u8;
    pub const M1: u8 = 1u8;
    pub const M10: u8 = 2u8;
    pub const M30: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_UE_RxTx_TEG_Info_r17_case1_r17Nr_UE_RxTx_TEG_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_UE_RxTx_TEG_Info_r17_case1_r17 {
    pub nr_ue_rx_tx_teg_id_r17: NR_UE_RxTx_TEG_Info_r17_case1_r17Nr_UE_RxTx_TEG_ID_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NR_UE_RxTx_TEG_Info_r17_case2_r17Nr_UE_RxTx_TEG_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct NR_UE_RxTx_TEG_Info_r17_case2_r17Nr_UE_Tx_TEG_Index_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_UE_RxTx_TEG_Info_r17_case2_r17 {
    pub nr_ue_rx_tx_teg_id_r17: NR_UE_RxTx_TEG_Info_r17_case2_r17Nr_UE_RxTx_TEG_ID_r17,
    pub nr_ue_tx_teg_index_r17: NR_UE_RxTx_TEG_Info_r17_case2_r17Nr_UE_Tx_TEG_Index_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NR_UE_RxTx_TEG_Info_r17_case3_r17Nr_UE_Rx_TEG_ID_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct NR_UE_RxTx_TEG_Info_r17_case3_r17Nr_UE_Tx_TEG_Index_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NR_UE_RxTx_TEG_Info_r17_case3_r17 {
    pub nr_ue_rx_teg_id_r17: NR_UE_RxTx_TEG_Info_r17_case3_r17Nr_UE_Rx_TEG_ID_r17,
    pub nr_ue_tx_teg_index_r17: NR_UE_RxTx_TEG_Info_r17_case3_r17Nr_UE_Tx_TEG_Index_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_UE_TEG_Capability_r17Nr_UE_TEG_ID_CapabilityBandList_r17(
    pub Vec<NR_UE_TEG_ID_CapabilityPerBand_r17>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTEG_ID_MaxSupport_r17(pub u8);
impl NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTEG_ID_MaxSupport_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N6: u8 = 4u8;
    pub const N8: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_TxTEG_ID_MaxSupport_r17(pub u8);
impl NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_TxTEG_ID_MaxSupport_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N6: u8 = 4u8;
    pub const N8: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "11")]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTxTEG_ID_MaxSupport_r17(pub u8);
impl NR_UE_TEG_ID_CapabilityPerBand_r17Nr_UE_RxTxTEG_ID_MaxSupport_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N36: u8 = 9u8;
    pub const N48: u8 = 10u8;
    pub const N64: u8 = 11u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGs_r17(
    pub u8,
);
impl NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGs_r17 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGsSimul_r17(
    pub u8,
);
impl NR_UE_TEG_ID_CapabilityPerBand_r17MeasureSameDL_PRS_ResourceWithDifferentRxTEGsSimul_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N6: u8 = 4u8;
    pub const N8: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "1024"
)]
pub struct NR_UL_SRS_Capability_r16Srs_CapabilityBandList_r16(pub Vec<SRS_CapabilityPerBand_r16>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "16")]
pub struct NR_UL_SRS_Capability_r16Srs_PosResourceConfigCA_BandList_r16(
    pub Vec<SRS_PosResourcesPerBand_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct NR_UL_SRS_Capability_r16MaxNumberSRS_PosPathLossEstimateAllServingCells_r16(pub u8);
impl NR_UL_SRS_Capability_r16MaxNumberSRS_PosPathLossEstimateAllServingCells_r16 {
    pub const N1: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N8: u8 = 2u8;
    pub const N16: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct NR_UL_SRS_Capability_r16MaxNumberSRS_PosSpatialRelationsAllServingCells_r16(pub u8);
impl NR_UL_SRS_Capability_r16MaxNumberSRS_PosSpatialRelationsAllServingCells_r16 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct NavIC_CDC_r16Navic_ClockBiasCorrection_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct NavIC_CDC_r16Navic_ClockDriftCorrection_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NavIC_ClockModel_r16Navic_Toc_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct NavIC_ClockModel_r16Navic_af2_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavIC_ClockModel_r16Navic_af1_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct NavIC_ClockModel_r16Navic_af0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct NavIC_ClockModel_r16Navic_Tgd_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NavIC_ClockModel2_r19NavicL1_Toec_r19(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct NavIC_ClockModel2_r19NavicL1_af2_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct NavIC_ClockModel2_r19NavicL1_af1_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-268435456", ub = "268435455")]
pub struct NavIC_ClockModel2_r19NavicL1_af0_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_ClockModel2_r19NavicL1_Tgd_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_ClockModel2_r19NavicL1_iscL1PorS_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_ClockModel2_r19NavicL1_iscL1D_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NavIC_ClockModel2_r19NavicL1_RSF_r19(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NavIC_CorrectionElementAutoNav_r16Navic_Tod_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NavIC_CorrectionElementAutoNav_r16Navic_iodec_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavIC_CorrectionElementAutoNav_r16Navic_UDRAI_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavIC_CorrectionElementAutoNav_r16Navic_UDRArateI_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "50400")]
pub struct NavIC_DifferentialCorrections_r16Navic_RefTOWC_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavIC_EDC_r16Navic_AlphaEDC_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavIC_EDC_r16Navic_BetaEDC_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct NavIC_EDC_r16Navic_GammaEDC_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_EDC_r16Navic_AoIcorrection_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_EDC_r16Navic_AoRAcorrection_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct NavIC_EDC_r16Navic_SemiMajorcorrection_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "50400")]
pub struct NavIC_GridModelParameter_r16Navic_RefTOWC_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NavIC_GridModelParameter_r16RegionMasked_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NavModel_BDS_KeplerianSet_r12BdsAODE_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NavModel_BDS_KeplerianSet_r12BdsURAI_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsToe_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet_r12BdsAPowerHalf_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet_r12BdsE_r12(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_BDS_KeplerianSet_r12BdsW_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_BDS_KeplerianSet_r12BdsDeltaN_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_BDS_KeplerianSet_r12BdsM0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_BDS_KeplerianSet_r12BdsOmega0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_BDS_KeplerianSet_r12BdsOmegaDot_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_BDS_KeplerianSet_r12BdsI0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavModel_BDS_KeplerianSet_r12BdsIDot_r12(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCuc_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCus_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCrc_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCrs_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCic_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_BDS_KeplerianSet_r12BdsCis_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsIODE_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsToe_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsDeltaA_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777216")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsAdot_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsDeltaN0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4194304", ub = "4194303")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsDeltaN0dot_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsM0_r16(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8589934591")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsE_r16(pub u64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsOmega_r16(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsOmega0_r16(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsI0_r16(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsOmegaDot_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsI0Dot_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCuc_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCus_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCrc_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCrs_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCic_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_BDS_KeplerianSet2_r16BdsCis_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NavModel_GLONASS_ECEFGloEn(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct NavModel_GLONASS_ECEFGloP1(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct NavModel_GLONASS_ECEFGloP2(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NavModel_GLONASS_ECEFGloM(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-67108864", ub = "67108863")]
pub struct NavModel_GLONASS_ECEFGloX(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_GLONASS_ECEFGloXdot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavModel_GLONASS_ECEFGloXdotdot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-67108864", ub = "67108863")]
pub struct NavModel_GLONASS_ECEFGloY(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_GLONASS_ECEFGloYdot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavModel_GLONASS_ECEFGloYdotdot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-67108864", ub = "67108863")]
pub struct NavModel_GLONASS_ECEFGloZ(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_GLONASS_ECEFGloZdot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavModel_GLONASS_ECEFGloZdotdot(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65536")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Toe_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_URAI_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_W_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_DeltaN_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_M0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_OmegaDot_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_E_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_IDot_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_APowerHalf_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_I0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Omega0_r16(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Crs_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Cis_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Cus_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Crc_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Cic_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet_r16Navic_Cuc_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Toec_r19(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_URAI_r19(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaA_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Adot_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaN0_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4194304", ub = "4194303")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_DeltaNdot_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_M0_r19(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8589934591")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_E_r19(pub u64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_W_r19(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Omega0_r19(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777215")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_OmegaDot_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_I0_r19(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_IDot_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Cis_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Cic_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Crs_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Crc_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Cus_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModel_NavIC_KeplerianSet2_r19NavicL1_Cuc_r19(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5399")]
pub struct NavModel_SBAS_ECEFSbasTo(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct NavModel_SBAS_ECEFSbasAccuracy(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-536870912", ub = "536870911")]
pub struct NavModel_SBAS_ECEFSbasXg(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-536870912", ub = "536870911")]
pub struct NavModel_SBAS_ECEFSbasYg(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777215")]
pub struct NavModel_SBAS_ECEFSbasZg(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct NavModel_SBAS_ECEFSbasXgDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct NavModel_SBAS_ECEFSbasYgDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-131072", ub = "131071")]
pub struct NavModel_SBAS_ECEFSbasZgDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct NavModel_SBAS_ECEFSbasXgDotDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct NavModel_SBAS_ECEFSbagYgDotDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct NavModel_SBAS_ECEFSbasZgDotDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2015")]
pub struct NavModelCNAV_KeplerianSetCnavTop(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16", ub = "15")]
pub struct NavModelCNAV_KeplerianSetCnavURAindex(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-33554432", ub = "33554431")]
pub struct NavModelCNAV_KeplerianSetCnavDeltaA(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16777216", ub = "16777215")]
pub struct NavModelCNAV_KeplerianSetCnavAdot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct NavModelCNAV_KeplerianSetCnavDeltaNo(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4194304", ub = "4194303")]
pub struct NavModelCNAV_KeplerianSetCnavDeltaNoDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModelCNAV_KeplerianSetCnavMo(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8589934591")]
pub struct NavModelCNAV_KeplerianSetCnavE(pub u64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModelCNAV_KeplerianSetCnavOmega(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModelCNAV_KeplerianSetCnavOMEGA0(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-65536", ub = "65535")]
pub struct NavModelCNAV_KeplerianSetCnavDeltaOmegaDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4294967296", ub = "4294967295")]
pub struct NavModelCNAV_KeplerianSetCnavIo(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct NavModelCNAV_KeplerianSetCnavIoDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelCNAV_KeplerianSetCnavCis(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelCNAV_KeplerianSetCnavCic(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModelCNAV_KeplerianSetCnavCrs(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModelCNAV_KeplerianSetCnavCrc(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModelCNAV_KeplerianSetCnavCus(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct NavModelCNAV_KeplerianSetCnavCuc(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct NavModelKeplerianSetKeplerToe(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelKeplerianSetKeplerW(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerDeltaN(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelKeplerianSetKeplerM0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModelKeplerianSetKeplerOmegaDot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModelKeplerianSetKeplerE(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavModelKeplerianSetKeplerIDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModelKeplerianSetKeplerAPowerHalf(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelKeplerianSetKeplerI0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelKeplerianSetKeplerOmega0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCrs(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCis(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCus(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCrc(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCic(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelKeplerianSetKeplerCuc(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct NavModelNAV_KeplerianSetNavURA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NavModelNAV_KeplerianSetNavFitFlag(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "37799")]
pub struct NavModelNAV_KeplerianSetNavToe(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelNAV_KeplerianSetNavOmega(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavDeltaN(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelNAV_KeplerianSetNavM0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct NavModelNAV_KeplerianSetNavOmegaADot(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModelNAV_KeplerianSetNavE(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NavModelNAV_KeplerianSetNavIDot(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4294967295")]
pub struct NavModelNAV_KeplerianSetNavAPowerHalf(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelNAV_KeplerianSetNavI0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct NavModelNAV_KeplerianSetNavOmegaA0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCrs(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCis(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCus(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCrc(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCic(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct NavModelNAV_KeplerianSetNavCuc(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemCodeOnL2(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemL2Pflag(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved1(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved2(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved3(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved4(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemSF1Rsvd {
    pub reserved1: NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved1,
    pub reserved2: NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved2,
    pub reserved3: NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved3,
    pub reserved4: NavModelNAV_KeplerianSetAddNAVparamEphemSF1RsvdReserved4,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "31")]
pub struct NavModelNAV_KeplerianSetAddNAVparamEphemAODA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct NavModelNAV_KeplerianSetAddNAVparam {
    pub ephem_code_on_l2: NavModelNAV_KeplerianSetAddNAVparamEphemCodeOnL2,
    pub ephem_l2_pflag: NavModelNAV_KeplerianSetAddNAVparamEphemL2Pflag,
    pub ephem_sf1_rsvd: NavModelNAV_KeplerianSetAddNAVparamEphemSF1Rsvd,
    pub ephem_aoda: NavModelNAV_KeplerianSetAddNAVparamEphemAODA,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct NeQuickModel2Parameter_r19Iodn_r19(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NeQuickModel2ParameterElement_r19Ai0_r19(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct NeQuickModel2ParameterElement_r19Ai1_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NeQuickModel2ParameterElement_r19Ai2_r19(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModel2ParameterElement_r19IonoDisturbanceFlag_r19(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32", ub = "31")]
pub struct NeQuickModel2ParameterElement_r19Modipmax_r19(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32", ub = "31")]
pub struct NeQuickModel2ParameterElement_r19Modipmin_r19(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct NeQuickModel2ParameterElement_r19MLonmax_r19(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct NeQuickModel2ParameterElement_r19MLonmin_r19(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2047")]
pub struct NeQuickModelParameterAi0(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct NeQuickModelParameterAi1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct NeQuickModelParameterAi2(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModelParameterIonoStormFlag1(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModelParameterIonoStormFlag2(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModelParameterIonoStormFlag3(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModelParameterIonoStormFlag4(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct NeQuickModelParameterIonoStormFlag5(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct NeighbourMeasurementElementPhysCellIdNeighbour(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "12711")]
pub struct NeighbourMeasurementElementRstd(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct NeighbourMeasurementElement_NB_r14PhysCellIdNeighbour_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "12711")]
pub struct NeighbourMeasurementElement_NB_r14Rstd_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NeighbourMeasurementElement_NB_r14TpIdNeighbour_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NeighbourMeasurementElement_NB_r14PrsIdNeighbour_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5")]
pub struct NeighbourMeasurementElement_NB_r14Delta_rstd_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct NeighbourMeasurementElement_NB_r14NprsIdNeighbour_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "12533")]
pub struct NetworkTimeSecondsFromFrameStructureStart(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3999999")]
pub struct NetworkTimeFractionalSecondsFromFrameStructureStart(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct NetworkTimeFrameDrift(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct NetworkTimeCellID_eUTRAPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTimeCellID_eUTRA {
    pub phys_cell_id: NetworkTimeCellID_eUTRAPhysCellId,
    #[asn(optional_idx = 0)]
    pub cell_global_id_eutra: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub earfcn: ARFCN_ValueEUTRA,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct NetworkTimeCellID_uTRAMode_fddPrimary_CPICH_Info(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NetworkTimeCellID_uTRAMode_fdd {
    pub primary_cpich_info: NetworkTimeCellID_uTRAMode_fddPrimary_CPICH_Info,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct NetworkTimeCellID_uTRAMode_tddCellParameters(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct NetworkTimeCellID_uTRAMode_tdd {
    pub cell_parameters: NetworkTimeCellID_uTRAMode_tddCellParameters,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum NetworkTimeCellID_uTRAMode {
    #[asn(key = 0, extended = false)]
    Fdd(NetworkTimeCellID_uTRAMode_fdd),
    #[asn(key = 1, extended = false)]
    Tdd(NetworkTimeCellID_uTRAMode_tdd),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTimeCellID_uTRA {
    pub mode: NetworkTimeCellID_uTRAMode,
    #[asn(optional_idx = 0)]
    pub cell_global_id_utra: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub uarfcn: ARFCN_ValueUTRA,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct NetworkTimeCellID_gSMBcchCarrier(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct NetworkTimeCellID_gSMBsic(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTimeCellID_gSM {
    pub bcch_carrier: NetworkTimeCellID_gSMBcchCarrier,
    pub bsic: NetworkTimeCellID_gSMBsic,
    #[asn(optional_idx = 0)]
    pub cell_global_id_geran: Option<CellGlobalIdGERAN>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct NetworkTimeCellID_nBIoT_r14NbPhysCellId_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTimeCellID_nBIoT_r14 {
    pub nb_phys_cell_id_r14: NetworkTimeCellID_nBIoT_r14NbPhysCellId_r14,
    #[asn(optional_idx = 0)]
    pub nb_cell_global_id_r14: Option<ECGI>,
    pub nb_carrier_freq_r14: CarrierFreq_NB_r14,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1007")]
pub struct NetworkTimeCellID_nr_r15NrPhysCellId_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 1)]
pub struct NetworkTimeCellID_nr_r15 {
    pub nr_phys_cell_id_r15: NetworkTimeCellID_nr_r15NrPhysCellId_r15,
    #[asn(optional_idx = 0)]
    pub nr_cell_global_id_r15: Option<NCGI_r15>,
    pub nr_arfcn_r15: ARFCN_ValueNR_r15,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "2", extensible = true)]
pub enum NetworkTimeCellID {
    #[asn(key = 0, extended = false)]
    EUTRA(NetworkTimeCellID_eUTRA),
    #[asn(key = 1, extended = false)]
    UTRA(NetworkTimeCellID_uTRA),
    #[asn(key = 2, extended = false)]
    GSM(NetworkTimeCellID_gSM),
    #[asn(key = 0, extended = true)]
    NBIoT_r14(NetworkTimeCellID_nBIoT_r14),
    #[asn(key = 1, extended = true)]
    Nr_r15(NetworkTimeCellID_nr_r15),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Serving_r16(pub u8);
impl OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Serving_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnSSB_Neigh_r16(pub u8);
impl OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnSSB_Neigh_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Neigh_r16(pub u8);
impl OLPC_SRS_Pos_r16Olpc_SRS_PosBasedOnPRS_Neigh_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct OLPC_SRS_Pos_r16MaxNumberPathLossEstimatePerServing_r16(pub u8);
impl OLPC_SRS_Pos_r16MaxNumberPathLossEstimatePerServing_r16 {
    pub const N1: u8 = 0u8;
    pub const N4: u8 = 1u8;
    pub const N8: u8 = 2u8;
    pub const N16: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ORBIT_IntegrityParameters_r17ProbOnsetConstFault_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "3600")]
pub struct ORBIT_IntegrityParameters_r17MeanConstFaultDuration_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ORBIT_IntegrityParameters_r17ProbOnsetSatFault_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "3600")]
pub struct ORBIT_IntegrityParameters_r17MeanSatFaultDuration_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ORBIT_IntegrityParameters_r17OrbitRangeErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ORBIT_IntegrityParameters_r17OrbitRangeRateErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct OTDOA_LocationServerErrorCausesCause(pub u8);
impl OTDOA_LocationServerErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_NOT_SUPPORTED_BY_SERVER: u8 = 1u8;
    pub const ASSISTANCE_DATA_SUPPORTED_BUT_CURRENTLY_NOT_AVAILABLE_BY_SERVER: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct OTDOA_MeasQualityError_Resolution(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "5", sz_ub = "5")]
pub struct OTDOA_MeasQualityError_Value(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "3", sz_ub = "3")]
pub struct OTDOA_MeasQualityError_NumSamples(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_NeighbourCellInfoElementPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct OTDOA_NeighbourCellInfoElementCpLength(pub u8);
impl OTDOA_NeighbourCellInfoElementCpLength {
    pub const NORMAL: u8 = 0u8;
    pub const EXTENDED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct OTDOA_NeighbourCellInfoElementAntennaPortConfig(pub u8);
impl OTDOA_NeighbourCellInfoElementAntennaPortConfig {
    pub const PORTS_1_OR_2: u8 = 0u8;
    pub const PORTS_4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct OTDOA_NeighbourCellInfoElementSlotNumberOffset(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct OTDOA_NeighbourCellInfoElementPrs_SubframeOffset(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct OTDOA_NeighbourCellInfoElementExpectedRSTD(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct OTDOA_NeighbourCellInfoElementExpectedRSTD_Uncertainty(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_NeighbourCellInfoNB_r14PhysCellIdNB_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct OTDOA_NeighbourCellInfoNB_r14Eutra_NumCRS_Ports_r14(pub u8);
impl OTDOA_NeighbourCellInfoNB_r14Eutra_NumCRS_Ports_r14 {
    pub const PORTS_1_OR_2: u8 = 0u8;
    pub const PORTS_4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct OTDOA_NeighbourCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14(pub u8);
impl OTDOA_NeighbourCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14 {
    pub const R4: u8 = 0u8;
    pub const R8: u8 = 1u8;
    pub const R16: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct OTDOA_NeighbourCellInfoNB_r14Nprs_slotNumberOffset_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct OTDOA_NeighbourCellInfoNB_r14Nprs_SFN_Offset_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1279")]
pub struct OTDOA_NeighbourCellInfoNB_r14Nprs_SubframeOffset_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct OTDOA_NeighbourCellInfoNB_r14ExpectedRSTD_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct OTDOA_NeighbourCellInfoNB_r14ExpectedRSTD_Uncertainty_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "72")]
pub struct OTDOA_NeighbourCellInfoNB_r14PrsNeighbourCellIndex_r14(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct OTDOA_ProvideCapabilitiesOtdoa_Mode(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_ReferenceCellInfoPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct OTDOA_ReferenceCellInfoAntennaPortConfig(pub u8);
impl OTDOA_ReferenceCellInfoAntennaPortConfig {
    pub const PORTS1_OR_2: u8 = 0u8;
    pub const PORTS4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct OTDOA_ReferenceCellInfoCpLength(pub u8);
impl OTDOA_ReferenceCellInfoCpLength {
    pub const NORMAL: u8 = 0u8;
    pub const EXTENDED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_ReferenceCellInfoNB_r14PhysCellIdNB_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct OTDOA_ReferenceCellInfoNB_r14Eutra_NumCRS_Ports_r14(pub u8);
impl OTDOA_ReferenceCellInfoNB_r14Eutra_NumCRS_Ports_r14 {
    pub const PORTS1_OR_2: u8 = 0u8;
    pub const PORTS4: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct OTDOA_ReferenceCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14(pub u8);
impl OTDOA_ReferenceCellInfoNB_r14Otdoa_SIB1_NB_repetitions_r14 {
    pub const R4: u8 = 0u8;
    pub const R8: u8 = 1u8;
    pub const R16: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_RequestAssistanceDataPhysCellId(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct OTDOA_RequestLocationInformationAssistanceAvailability(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct OTDOA_SignalMeasurementInformationSystemFrameNumber(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_SignalMeasurementInformationPhysCellIdRef(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14SystemFrameNumber_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14PhysCellIdRef_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14TpIdRef_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14PrsIdRef_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14NprsIdRef_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct OTDOA_SignalMeasurementInformation_NB_r14HyperSFN_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct OTDOA_TargetDeviceErrorCausesCause(pub u8);
impl OTDOA_TargetDeviceErrorCausesCause {
    pub const UNDEFINED: u8 = 0u8;
    pub const ASSISTANCE_DATA_MISSING: u8 = 1u8;
    pub const UNABLE_TO_MEASURE_REFERENCE_CELL: u8 = 2u8;
    pub const UNABLE_TO_MEASURE_ANY_NEIGHBOUR_CELL: u8 = 3u8;
    pub const ATTEMPTED_BUT_UNABLE_TO_MEASURE_SOME_NEIGHBOUR_CELLS: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "14")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r18 {
    pub const MHZ15: u8 = 0u8;
    pub const MHZ20: u8 = 1u8;
    pub const MHZ30: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ60: u8 = 5u8;
    pub const MHZ80: u8 = 6u8;
    pub const MHZ100: u8 = 7u8;
    pub const MHZ120: u8 = 8u8;
    pub const MHZ140: u8 = 9u8;
    pub const MHZ150: u8 = 10u8;
    pub const MHZ180: u8 = 11u8;
    pub const MHZ200: u8 = 12u8;
    pub const MHZ240: u8 = 13u8;
    pub const MHZ300: u8 = 14u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r18 {
    pub const MHZ150: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ300: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
    pub const MHZ600: u8 = 4u8;
    pub const MHZ800: u8 = 5u8;
    pub const MHZ1000: u8 = 6u8;
    pub const MHZ1200: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r18 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r18 { pub const MS8 : u8 = 0u8 ; pub const MS16 : u8 = 1u8 ; pub const MS20 : u8 = 2u8 ; pub const MS30 : u8 = 3u8 ; pub const MS40 : u8 = 4u8 ; pub const MS80 : u8 = 5u8 ; pub const MS160 : u8 = 6u8 ; pub const MS320 : u8 = 7u8 ; pub const MS640 : u8 = 8u8 ; pub const MS1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18 { pub prs_duration_of_three_prs_bwa_processing_symbols_n_r18 : PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r18 , pub prs_duration_of_three_prs_bwa_processing_symbols_t_r18 : PRS_BWA_ThreeContiguousIntrabandInMG_r18Prs_durationOfThreePRS_BWA_Processing_r18Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r18 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18 { # [asn (optional_idx = 0 ,)] pub scs15_r18 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18 > , # [asn (optional_idx = 1 ,)] pub scs30_r18 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18 > , # [asn (optional_idx = 2 ,)] pub scs60_r18 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18 { # [asn (optional_idx = 0 ,)] pub scs60_r18 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18 > , # [asn (optional_idx = 1 ,)] pub scs120_r18 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "14")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR1_r19 {
    pub const MHZ15: u8 = 0u8;
    pub const MHZ20: u8 = 1u8;
    pub const MHZ30: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ60: u8 = 5u8;
    pub const MHZ80: u8 = 6u8;
    pub const MHZ100: u8 = 7u8;
    pub const MHZ120: u8 = 8u8;
    pub const MHZ140: u8 = 9u8;
    pub const MHZ150: u8 = 10u8;
    pub const MHZ180: u8 = 11u8;
    pub const MHZ200: u8 = 12u8;
    pub const MHZ240: u8 = 13u8;
    pub const MHZ300: u8 = 14u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfThreeAggregatedDL_PRS_Bandwidth_FR2_r19 {
    pub const MHZ150: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ300: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
    pub const MHZ600: u8 = 4u8;
    pub const MHZ800: u8 = 5u8;
    pub const MHZ1000: u8 = 6u8;
    pub const MHZ1200: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19(pub u8);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r19 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r19 { pub const MS8 : u8 = 0u8 ; pub const MS16 : u8 = 1u8 ; pub const MS20 : u8 = 2u8 ; pub const MS30 : u8 = 3u8 ; pub const MS40 : u8 = 4u8 ; pub const MS80 : u8 = 5u8 ; pub const MS160 : u8 = 6u8 ; pub const MS320 : u8 = 7u8 ; pub const MS640 : u8 = 8u8 ; pub const MS1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19 { pub prs_duration_of_three_prs_bwa_processing_symbols_n_r19 : PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsN_r19 , pub prs_duration_of_three_prs_bwa_processing_symbols_t_r19 : PRS_BWA_ThreeContiguousIntrabandInMG_r19Prs_durationOfThreePRS_BWA_Processing_r19Prs_durationOfThreePRS_BWA_ProcessingSymbolsT_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19 { # [asn (optional_idx = 0 ,)] pub scs15_r19 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19 > , # [asn (optional_idx = 1 ,)] pub scs30_r19 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19 > , # [asn (optional_idx = 2 ,)] pub scs60_r19 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19(
    pub u8,
);
impl PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19 { # [asn (optional_idx = 0 ,)] pub scs60_r19 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19 > , # [asn (optional_idx = 1 ,)] pub scs120_r19 : Option < PRS_BWA_ThreeContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r18(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r18 {
    pub const MHZ10: u8 = 0u8;
    pub const MHZ20: u8 = 1u8;
    pub const MHZ40: u8 = 2u8;
    pub const MHZ50: u8 = 3u8;
    pub const MHZ80: u8 = 4u8;
    pub const MHZ100: u8 = 5u8;
    pub const MHZ160: u8 = 6u8;
    pub const MHZ200: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r18(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r18 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
    pub const MHZ800: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR1_r18 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18MaximumOfDL_PRS_BandwidthPerPFL_FR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18Dl_PRS_BufferTypeOfBWA_r18 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r18(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r18 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r18(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r18 { pub const MS8 : u8 = 0u8 ; pub const MS16 : u8 = 1u8 ; pub const MS20 : u8 = 2u8 ; pub const MS30 : u8 = 3u8 ; pub const MS40 : u8 = 4u8 ; pub const MS80 : u8 = 5u8 ; pub const MS160 : u8 = 6u8 ; pub const MS320 : u8 = 7u8 ; pub const MS640 : u8 = 8u8 ; pub const MS1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18 { pub prs_duration_of_two_prs_bwa_processing_symbols_n_r18 : PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r18 , pub prs_duration_of_two_prs_bwa_processing_symbols_t_r18 : PRS_BWA_TwoContiguousIntrabandInMG_r18Prs_durationOfTwoPRS_BWA_Processing_r18Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r18 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 3)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18 { # [asn (optional_idx = 0 ,)] pub scs15_r18 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs15_r18 > , # [asn (optional_idx = 1 ,)] pub scs30_r18 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs30_r18 > , # [asn (optional_idx = 2 ,)] pub scs60_r18 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r18Scs60_r18 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18 { # [asn (optional_idx = 0 ,)] pub scs60_r18 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs60_r18 > , # [asn (optional_idx = 1 ,)] pub scs120_r18 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r18MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r18Scs120_r18 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r19(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR1_r19 {
    pub const MHZ10: u8 = 0u8;
    pub const MHZ20: u8 = 1u8;
    pub const MHZ40: u8 = 2u8;
    pub const MHZ50: u8 = 3u8;
    pub const MHZ80: u8 = 4u8;
    pub const MHZ100: u8 = 5u8;
    pub const MHZ160: u8 = 6u8;
    pub const MHZ200: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r19(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfTwoAggregatedDL_PRS_Bandwidth_FR2_r19 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
    pub const MHZ800: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR1_r19 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19MaximumOfDL_PRS_BandwidthPerPFL_FR2_r19 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19(pub u8);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19Dl_PRS_BufferTypeOfBWA_r19 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r19(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r19 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r19(
    pub u8,
);
impl PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r19 { pub const MS8 : u8 = 0u8 ; pub const MS16 : u8 = 1u8 ; pub const MS20 : u8 = 2u8 ; pub const MS30 : u8 = 3u8 ; pub const MS40 : u8 = 4u8 ; pub const MS80 : u8 = 5u8 ; pub const MS160 : u8 = 6u8 ; pub const MS320 : u8 = 7u8 ; pub const MS640 : u8 = 8u8 ; pub const MS1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19 { pub prs_duration_of_two_prs_bwa_processing_symbols_n_r19 : PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsN_r19 , pub prs_duration_of_two_prs_bwa_processing_symbols_t_r19 : PRS_BWA_TwoContiguousIntrabandInMG_r19Prs_durationOfTwoPRS_BWA_Processing_r19Prs_durationOfTwoPRS_BWA_ProcessingSymbolsT_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 3)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19 { # [asn (optional_idx = 0 ,)] pub scs15_r19 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs15_r19 > , # [asn (optional_idx = 1 ,)] pub scs30_r19 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs30_r19 > , # [asn (optional_idx = 2 ,)] pub scs60_r19 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR1_r19Scs60_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19(
    pub u8,
);
impl
    PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N6: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N12: u8 = 5u8;
    pub const N16: u8 = 6u8;
    pub const N24: u8 = 7u8;
    pub const N32: u8 = 8u8;
    pub const N48: u8 = 9u8;
    pub const N64: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 2)]
pub struct PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19 { # [asn (optional_idx = 0 ,)] pub scs60_r19 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs60_r19 > , # [asn (optional_idx = 1 ,)] pub scs120_r19 : Option < PRS_BWA_TwoContiguousIntrabandInMG_r19MaxNumOfAggregatedDL_PRS_ResourcePerSlot_FR2_r19Scs120_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "5")]
pub struct PRS_InfoPrs_Bandwidth(pub u8);
impl PRS_InfoPrs_Bandwidth {
    pub const N6: u8 = 0u8;
    pub const N15: u8 = 1u8;
    pub const N25: u8 = 2u8;
    pub const N50: u8 = 3u8;
    pub const N75: u8 = 4u8;
    pub const N100: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct PRS_InfoPrs_ConfigurationIndex(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct PRS_InfoNumDL_Frames(pub u8);
impl PRS_InfoNumDL_Frames {
    pub const SF_1: u8 = 0u8;
    pub const SF_2: u8 = 1u8;
    pub const SF_4: u8 = 2u8;
    pub const SF_6: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17PrsProcessingType_r17(pub u8);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17PrsProcessingType_r17 {
    pub const TYPE1_A: u8 = 0u8;
    pub const TYPE1_B: u8 = 1u8;
    pub const TYPE2: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_dl_PRS_BufferType_r17(pub u8);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_dl_PRS_BufferType_r17 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsN_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsN_r17 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "12")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsT_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsT_r17 { pub const MS1 : u8 = 0u8 ; pub const MS2 : u8 = 1u8 ; pub const MS4 : u8 = 2u8 ; pub const MS8 : u8 = 3u8 ; pub const MS16 : u8 = 4u8 ; pub const MS20 : u8 = 5u8 ; pub const MS30 : u8 = 6u8 ; pub const MS40 : u8 = 7u8 ; pub const MS80 : u8 = 8u8 ; pub const MS160 : u8 = 9u8 ; pub const MS320 : u8 = 10u8 ; pub const MS640 : u8 = 11u8 ; pub const MS1280 : u8 = 12u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17 { pub ppw_duration_of_prs_processing_symbols_n_r17 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsN_r17 , pub ppw_duration_of_prs_processing_symbols_t_r17 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing1_r17Ppw_durationOfPRS_ProcessingSymbolsT_r17 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsN2_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsN2_r17 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS3 : u8 = 5u8 ; pub const MS4 : u8 = 6u8 ; pub const MS5 : u8 = 7u8 ; pub const MS6 : u8 = 8u8 ; pub const MS8 : u8 = 9u8 ; pub const MS12 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsT2_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsT2_r17 { pub const MS4 : u8 = 0u8 ; pub const MS5 : u8 = 1u8 ; pub const MS6 : u8 = 2u8 ; pub const MS8 : u8 = 3u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17 { pub ppw_duration_of_prs_processing_symbols_n2_r17 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsN2_r17 , pub ppw_duration_of_prs_processing_symbols_t2_r17 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_durationOfPRS_Processing2_r17Ppw_durationOfPRS_ProcessingSymbolsT2_r17 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs15_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs15_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs30_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs30_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs60_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs60_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs120_r17(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs120_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17 { # [asn (optional_idx = 0 ,)] pub scs15_r17 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs15_r17 > , # [asn (optional_idx = 1 ,)] pub scs30_r17 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs30_r17 > , # [asn (optional_idx = 2 ,)] pub scs60_r17 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs60_r17 > , # [asn (optional_idx = 3 ,)] pub scs120_r17 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r17Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r17Scs120_r17 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19PrsProcessingType_r19(pub u8);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19PrsProcessingType_r19 {
    pub const TYPE1_A: u8 = 0u8;
    pub const TYPE1_B: u8 = 1u8;
    pub const TYPE2: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_dl_PRS_BufferType_r19(pub u8);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_dl_PRS_BufferType_r19 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsN_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsN_r19 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS4 : u8 = 5u8 ; pub const MS6 : u8 = 6u8 ; pub const MS8 : u8 = 7u8 ; pub const MS12 : u8 = 8u8 ; pub const MS16 : u8 = 9u8 ; pub const MS20 : u8 = 10u8 ; pub const MS25 : u8 = 11u8 ; pub const MS30 : u8 = 12u8 ; pub const MS32 : u8 = 13u8 ; pub const MS35 : u8 = 14u8 ; pub const MS40 : u8 = 15u8 ; pub const MS45 : u8 = 16u8 ; pub const MS50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "12")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsT_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsT_r19 { pub const MS1 : u8 = 0u8 ; pub const MS2 : u8 = 1u8 ; pub const MS4 : u8 = 2u8 ; pub const MS8 : u8 = 3u8 ; pub const MS16 : u8 = 4u8 ; pub const MS20 : u8 = 5u8 ; pub const MS30 : u8 = 6u8 ; pub const MS40 : u8 = 7u8 ; pub const MS80 : u8 = 8u8 ; pub const MS160 : u8 = 9u8 ; pub const MS320 : u8 = 10u8 ; pub const MS640 : u8 = 11u8 ; pub const MS1280 : u8 = 12u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19 { pub ppw_duration_of_prs_processing_symbols_n_r19 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsN_r19 , pub ppw_duration_of_prs_processing_symbols_t_r19 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing1_r19Ppw_durationOfPRS_ProcessingSymbolsT_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsN2_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsN2_r19 { pub const MS_DOT125 : u8 = 0u8 ; pub const MS_DOT25 : u8 = 1u8 ; pub const MS_DOT5 : u8 = 2u8 ; pub const MS1 : u8 = 3u8 ; pub const MS2 : u8 = 4u8 ; pub const MS3 : u8 = 5u8 ; pub const MS4 : u8 = 6u8 ; pub const MS5 : u8 = 7u8 ; pub const MS6 : u8 = 8u8 ; pub const MS8 : u8 = 9u8 ; pub const MS12 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsT2_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsT2_r19 { pub const MS4 : u8 = 0u8 ; pub const MS5 : u8 = 1u8 ; pub const MS6 : u8 = 2u8 ; pub const MS8 : u8 = 3u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19 { pub ppw_duration_of_prs_processing_symbols_n2_r19 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsN2_r19 , pub ppw_duration_of_prs_processing_symbols_t2_r19 : PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_durationOfPRS_Processing2_r19Ppw_durationOfPRS_ProcessingSymbolsT2_r19 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N4 : u8 = 2u8 ; pub const N6 : u8 = 3u8 ; pub const N8 : u8 = 4u8 ; pub const N12 : u8 = 5u8 ; pub const N16 : u8 = 6u8 ; pub const N24 : u8 = 7u8 ; pub const N32 : u8 = 8u8 ; pub const N48 : u8 = 9u8 ; pub const N64 : u8 = 10u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19 { # [asn (optional_idx = 0 ,)] pub scs15_r19 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs15_r19 > , # [asn (optional_idx = 1 ,)] pub scs30_r19 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs30_r19 > , # [asn (optional_idx = 2 ,)] pub scs60_r19 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs60_r19 > , # [asn (optional_idx = 3 ,)] pub scs120_r19 : Option < PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_PRS_ResProcessedPerSlot_r19Scs120_r19 > , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr1(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr1 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr2(
    pub u8,
);
impl PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr2 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19 {
    #[asn(key = 0, extended = false)]
    Fr1(PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr1),
    #[asn(key = 1, extended = false)]
    Fr2(PRS_ProcessingCapabilityOutsideMGinPPWperType_r19Ppw_maxNumOfDL_Bandwidth_r19_fr2),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr1(pub u8);
impl PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr1 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ20: u8 = 2u8;
    pub const MHZ40: u8 = 3u8;
    pub const MHZ50: u8 = 4u8;
    pub const MHZ80: u8 = 5u8;
    pub const MHZ100: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr2(pub u8);
impl PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr2 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16 {
    #[asn(key = 0, extended = false)]
    Fr1(PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr1),
    #[asn(key = 1, extended = false)]
    Fr2(PRS_ProcessingCapabilityPerBand_r16SupportedBandwidthPRS_r16_fr2),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PRS_ProcessingCapabilityPerBand_r16Dl_PRS_BufferType_r16(pub u8);
impl PRS_ProcessingCapabilityPerBand_r16Dl_PRS_BufferType_r16 {
    pub const TYPE1: u8 = 0u8;
    pub const TYPE2: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "17")]
pub struct PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbols_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbols_r16 { pub const N_DOT125 : u8 = 0u8 ; pub const N_DOT25 : u8 = 1u8 ; pub const N_DOT5 : u8 = 2u8 ; pub const N1 : u8 = 3u8 ; pub const N2 : u8 = 4u8 ; pub const N4 : u8 = 5u8 ; pub const N6 : u8 = 6u8 ; pub const N8 : u8 = 7u8 ; pub const N12 : u8 = 8u8 ; pub const N16 : u8 = 9u8 ; pub const N20 : u8 = 10u8 ; pub const N25 : u8 = 11u8 ; pub const N30 : u8 = 12u8 ; pub const N32 : u8 = 13u8 ; pub const N35 : u8 = 14u8 ; pub const N40 : u8 = 15u8 ; pub const N45 : u8 = 16u8 ; pub const N50 : u8 = 17u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbolsInEveryTms_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbolsInEveryTms_r16 { pub const N8 : u8 = 0u8 ; pub const N16 : u8 = 1u8 ; pub const N20 : u8 = 2u8 ; pub const N30 : u8 = 3u8 ; pub const N40 : u8 = 4u8 ; pub const N80 : u8 = 5u8 ; pub const N160 : u8 = 6u8 ; pub const N320 : u8 = 7u8 ; pub const N640 : u8 = 8u8 ; pub const N1280 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true)]
pub struct PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16 { pub duration_of_prs_processing_symbols_r16 : PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbols_r16 , pub duration_of_prs_processing_symbols_in_every_tms_r16 : PRS_ProcessingCapabilityPerBand_r16DurationOfPRS_Processing_r16DurationOfPRS_ProcessingSymbolsInEveryTms_r16 , }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs15_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs15_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N24: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N48: u8 = 7u8;
    pub const N64: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs30_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs30_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N24: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N48: u8 = 7u8;
    pub const N64: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs60_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs60_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N24: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N48: u8 = 7u8;
    pub const N64: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs120_r16(
    pub u8,
);
impl PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs120_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N24: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N48: u8 = 7u8;
    pub const N64: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = true, optional_fields = 4)]
pub struct PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16 {
    #[asn(optional_idx = 0)]
    pub scs15_r16:
        Option<PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs15_r16>,
    #[asn(optional_idx = 1)]
    pub scs30_r16:
        Option<PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs30_r16>,
    #[asn(optional_idx = 2)]
    pub scs60_r16:
        Option<PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs60_r16>,
    #[asn(optional_idx = 3)]
    pub scs120_r16:
        Option<PRS_ProcessingCapabilityPerBand_r16MaxNumOfDL_PRS_ResProcessedPerSlot_r16Scs120_r16>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct PeriodicReportingIntervalMsSupport_r18MinPeriodicReportingIntervalMs_r18(pub u8);
impl PeriodicReportingIntervalMsSupport_r18MinPeriodicReportingIntervalMs_r18 {
    pub const MS1: u8 = 0u8;
    pub const MS10: u8 = 1u8;
    pub const MS100: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct PeriodicSessionID_r15PeriodicSessionInitiator_r15(pub u8);
impl PeriodicSessionID_r15PeriodicSessionInitiator_r15 {
    pub const LOCATION_SERVER: u8 = 0u8;
    pub const TARGET_DEVICE: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct PeriodicSessionID_r15PeriodicSessionNumber_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PeriodicalReportingCriteriaReportingAmount(pub u8);
impl PeriodicalReportingCriteriaReportingAmount {
    pub const RA1: u8 = 0u8;
    pub const RA2: u8 = 1u8;
    pub const RA4: u8 = 2u8;
    pub const RA8: u8 = 3u8;
    pub const RA16: u8 = 4u8;
    pub const RA32: u8 = 5u8;
    pub const RA64: u8 = 6u8;
    pub const RA_INFINITY: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PeriodicalReportingCriteriaReportingInterval(pub u8);
impl PeriodicalReportingCriteriaReportingInterval {
    pub const NO_PERIODICAL_REPORTING: u8 = 0u8;
    pub const RI0_25: u8 = 1u8;
    pub const RI0_5: u8 = 2u8;
    pub const RI1: u8 = 3u8;
    pub const RI2: u8 = 4u8;
    pub const RI4: u8 = 5u8;
    pub const RI8: u8 = 6u8;
    pub const RI16: u8 = 7u8;
    pub const RI32: u8 = 8u8;
    pub const RI64: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PeriodicalReportingCriteriaExt_r18ReportingAmount_r18(pub u8);
impl PeriodicalReportingCriteriaExt_r18ReportingAmount_r18 {
    pub const RA2: u8 = 0u8;
    pub const RA4: u8 = 1u8;
    pub const RA8: u8 = 2u8;
    pub const RA16: u8 = 3u8;
    pub const RA32: u8 = 4u8;
    pub const RA64: u8 = 5u8;
    pub const RA_INFINITY: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "999")]
pub struct PeriodicalReportingCriteriaExt_r18ReportingIntervalMs_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_X_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_Y_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-137438953472", ub = "137438953471")]
pub struct PhysicalReferenceStationInfo_r15Physical_ARP_ECEF_Z_r15(pub i64);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PolygonPointsLatitudeSign(pub u8);
impl PolygonPointsLatitudeSign {
    pub const NORTH: u8 = 0u8;
    pub const SOUTH: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8388607")]
pub struct PolygonPointsDegreesLatitude(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct PolygonPointsDegreesLongitude(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18 {
    pub const TWO: u8 = 0u8;
    pub const THREE: u8 = 1u8;
    pub const TWOANDTHREE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18 {
    pub const MHZ20: u8 = 0u8;
    pub const MHZ40: u8 = 1u8;
    pub const MHZ50: u8 = 2u8;
    pub const MHZ80: u8 = 3u8;
    pub const MHZ100: u8 = 4u8;
    pub const MHZ160: u8 = 5u8;
    pub const MHZ180: u8 = 6u8;
    pub const MHZ190: u8 = 7u8;
    pub const MHZ200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
    pub const MHZ600: u8 = 4u8;
    pub const MHZ800: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18 {
    pub const MHZ80: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ160: u8 = 2u8;
    pub const MHZ200: u8 = 3u8;
    pub const MHZ240: u8 = 4u8;
    pub const MHZ300: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ300: u8 = 3u8;
    pub const MHZ400: u8 = 4u8;
    pub const MHZ600: u8 = 5u8;
    pub const MHZ800: u8 = 6u8;
    pub const MHZ1000: u8 = 7u8;
    pub const MHZ1200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSet_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSet_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N3: u8 = 3u8;
    pub const N4: u8 = 4u8;
    pub const N5: u8 = 5u8;
    pub const N6: u8 = 6u8;
    pub const N8: u8 = 7u8;
    pub const N10: u8 = 8u8;
    pub const N12: u8 = 9u8;
    pub const N14: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N3: u8 = 3u8;
    pub const N4: u8 = 4u8;
    pub const N5: u8 = 5u8;
    pub const N6: u8 = 6u8;
    pub const N8: u8 = 7u8;
    pub const N10: u8 = 8u8;
    pub const N12: u8 = 9u8;
    pub const N14: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18GuardPeriod_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18GuardPeriod_r18 {
    pub const N0: u8 = 0u8;
    pub const N30: u8 = 1u8;
    pub const N100: u8 = 2u8;
    pub const N140: u8 = 3u8;
    pub const N200: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForTwoAggregatedCarriers_r18(pub u8);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForTwoAggregatedCarriers_r18 {
    pub const PC2: u8 = 0u8;
    pub const PC3: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForThreeAggregatedCarriers_r18(
    pub u8,
);
impl PosSRS_BWA_IndependentCA_RRC_Connected_r18PowerClassForThreeAggregatedCarriers_r18 {
    pub const PC2: u8 = 0u8;
    pub const PC3: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_BWA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18NumOfCarriersIntraBandContiguous_r18 {
    pub const TWO: u8 = 0u8;
    pub const THREE: u8 = 1u8;
    pub const TWOANDTHREE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR1_r18 {
    pub const MHZ20: u8 = 0u8;
    pub const MHZ40: u8 = 1u8;
    pub const MHZ50: u8 = 2u8;
    pub const MHZ80: u8 = 3u8;
    pub const MHZ100: u8 = 4u8;
    pub const MHZ160: u8 = 5u8;
    pub const MHZ180: u8 = 6u8;
    pub const MHZ190: u8 = 7u8;
    pub const MHZ200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_TwoCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
    pub const MHZ600: u8 = 4u8;
    pub const MHZ800: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR1_r18 {
    pub const MHZ80: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ160: u8 = 2u8;
    pub const MHZ200: u8 = 3u8;
    pub const MHZ240: u8 = 4u8;
    pub const MHZ300: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedBW_ThreeCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ300: u8 = 3u8;
    pub const MHZ400: u8 = 4u8;
    pub const MHZ600: u8 = 5u8;
    pub const MHZ800: u8 = 6u8;
    pub const MHZ1000: u8 = 7u8;
    pub const MHZ1200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSet_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSet_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodic_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodic_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemi_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourcePeriodicPerSlot_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceAperiodicPerSlot_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N3: u8 = 3u8;
    pub const N4: u8 = 4u8;
    pub const N5: u8 = 5u8;
    pub const N6: u8 = 6u8;
    pub const N8: u8 = 7u8;
    pub const N10: u8 = 8u8;
    pub const N12: u8 = 9u8;
    pub const N14: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18(pub u8);
impl PosSRS_BWA_RRC_Connected_r18MaximumAggregatedResourceSemiPerSlot_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N3: u8 = 3u8;
    pub const N4: u8 = 4u8;
    pub const N5: u8 = 5u8;
    pub const N6: u8 = 6u8;
    pub const N8: u8 = 7u8;
    pub const N10: u8 = 8u8;
    pub const N12: u8 = 9u8;
    pub const N14: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_BWA_RRC_Inactive_r18NumOfCarriersIntraBandContiguous_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18NumOfCarriersIntraBandContiguous_r18 {
    pub const TWO: u8 = 0u8;
    pub const THREE: u8 = 1u8;
    pub const TWOANDTHREE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR1_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR1_r18 {
    pub const MHZ20: u8 = 0u8;
    pub const MHZ40: u8 = 1u8;
    pub const MHZ50: u8 = 2u8;
    pub const MHZ80: u8 = 3u8;
    pub const MHZ100: u8 = 4u8;
    pub const MHZ160: u8 = 5u8;
    pub const MHZ180: u8 = 6u8;
    pub const MHZ190: u8 = 7u8;
    pub const MHZ200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR2_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_TwoCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
    pub const MHZ600: u8 = 4u8;
    pub const MHZ800: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR1_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR1_r18 {
    pub const MHZ80: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ160: u8 = 2u8;
    pub const MHZ200: u8 = 3u8;
    pub const MHZ240: u8 = 4u8;
    pub const MHZ300: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "8")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR2_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedBW_ThreeCarriersFR2_r18 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ300: u8 = 3u8;
    pub const MHZ400: u8 = 4u8;
    pub const MHZ600: u8 = 5u8;
    pub const MHZ800: u8 = 6u8;
    pub const MHZ1000: u8 = 7u8;
    pub const MHZ1200: u8 = 8u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSet_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSet_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodic_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodic_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemi_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemi_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodicPerSlot_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourcePeriodicPerSlot_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "10")]
pub struct PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemiPerSlot_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18MaximumAggregatedResourceSemiPerSlot_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N3: u8 = 3u8;
    pub const N4: u8 = 4u8;
    pub const N5: u8 = 5u8;
    pub const N6: u8 = 6u8;
    pub const N8: u8 = 7u8;
    pub const N10: u8 = 8u8;
    pub const N12: u8 = 9u8;
    pub const N14: u8 = 10u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_BWA_RRC_Inactive_r18GuardPeriod_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18GuardPeriod_r18 {
    pub const N0: u8 = 0u8;
    pub const N30: u8 = 1u8;
    pub const N100: u8 = 2u8;
    pub const N140: u8 = 3u8;
    pub const N200: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PosSRS_BWA_RRC_Inactive_r18PowerClassForTwoAggregatedCarriers_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18PowerClassForTwoAggregatedCarriers_r18 {
    pub const PC2: u8 = 0u8;
    pub const PC3: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "1")]
pub struct PosSRS_BWA_RRC_Inactive_r18PowerClassForThreeAggregatedCarriers_r18(pub u8);
impl PosSRS_BWA_RRC_Inactive_r18PowerClassForThreeAggregatedCarriers_r18 {
    pub const PC2: u8 = 0u8;
    pub const PC3: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17(pub u8);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N3 : u8 = 2u8 ; pub const N4 : u8 = 3u8 ; pub const N5 : u8 = 4u8 ; pub const N6 : u8 = 5u8 ; pub const N8 : u8 = 6u8 ; pub const N10 : u8 = 7u8 ; pub const N12 : u8 = 8u8 ; pub const N14 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17(pub u8);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy1(pub u8);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy1 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy2(pub u8);
impl PosSRS_RRC_Inactive_InInitialUL_BWP_r17Dummy2 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "14")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR1_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR1_r17 {
    pub const MHZ5: u8 = 0u8;
    pub const MHZ10: u8 = 1u8;
    pub const MHZ15: u8 = 2u8;
    pub const MHZ20: u8 = 3u8;
    pub const MHZ25: u8 = 4u8;
    pub const MHZ30: u8 = 5u8;
    pub const MHZ35: u8 = 6u8;
    pub const MHZ40: u8 = 7u8;
    pub const MHZ45: u8 = 8u8;
    pub const MHZ50: u8 = 9u8;
    pub const MHZ60: u8 = 10u8;
    pub const MHZ70: u8 = 11u8;
    pub const MHZ80: u8 = 12u8;
    pub const MHZ90: u8 = 13u8;
    pub const MHZ100: u8 = 14u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR2_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxSRSposBandwidthForEachSCS_withinCC_FR2_r17 {
    pub const MHZ50: u8 = 0u8;
    pub const MHZ100: u8 = 1u8;
    pub const MHZ200: u8 = 2u8;
    pub const MHZ400: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17(pub u8);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSRSposResourceSets_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17(pub u8);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResources_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicSRSposResourcesPerSlot_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentNumerologyBetweenSRSposAndInitialBWP_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentNumerologyBetweenSRSposAndInitialBWP_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SrsPosWithoutRestrictionOnBWP_r17(pub u8);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SrsPosWithoutRestrictionOnBWP_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17(
    pub u8,
);
impl
    PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResources_r17
{
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfPeriodicAndSemiPersistentSRSposResourcesPerSlot_r17 { pub const N1 : u8 = 0u8 ; pub const N2 : u8 = 1u8 ; pub const N3 : u8 = 2u8 ; pub const N4 : u8 = 3u8 ; pub const N5 : u8 = 4u8 ; pub const N6 : u8 = 5u8 ; pub const N8 : u8 = 6u8 ; pub const N10 : u8 = 7u8 ; pub const N12 : u8 = 8u8 ; pub const N14 : u8 = 9u8 ; }

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentCenterFreqBetweenSRSposAndInitialBWP_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17DifferentCenterFreqBetweenSRSposAndInitialBWP_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SwitchingTimeSRS_TX_OtherTX_r17(pub u8);
impl PosSRS_RRC_Inactive_OutsideInitialUL_BWP_r17SwitchingTimeSRS_TX_OtherTX_r17 {
    pub const US100: u8 = 0u8;
    pub const US140: u8 = 1u8;
    pub const US200: u8 = 2u8;
    pub const US300: u8 = 3u8;
    pub const US500: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17(
    pub u8,
);
impl PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResources_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "9")]
pub struct PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17(
    pub u8,
);
impl PosSRS_SP_RRC_Inactive_InInitialUL_BWP_r17MaxNumOfSemiPersistentSRSposResourcesPerSlot_r17 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N3: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N5: u8 = 4u8;
    pub const N6: u8 = 5u8;
    pub const N8: u8 = 6u8;
    pub const N10: u8 = 7u8;
    pub const N12: u8 = 8u8;
    pub const N14: u8 = 9u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18 {
    pub const MHZ40: u8 = 0u8;
    pub const MHZ50: u8 = 1u8;
    pub const MHZ80: u8 = 2u8;
    pub const MHZ100: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumTxFH_Hops_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumTxFH_Hops_r18 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N5: u8 = 3u8;
    pub const N6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR1_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR1_r18 {
    pub const N70: u8 = 0u8;
    pub const N140: u8 = 1u8;
    pub const N210: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR2_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18Rf_TxRetuneTimeFR2_r18 {
    pub const N35: u8 = 0u8;
    pub const N70: u8 = 1u8;
    pub const N140: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18 {
    pub const N100: u8 = 0u8;
    pub const N140: u8 = 1u8;
    pub const N200: u8 = 2u8;
    pub const N300: u8 = 3u8;
    pub const N500: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18NumOfOverlappingPRB_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18NumOfOverlappingPRB_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourcePeriodic_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourcePeriodic_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceAperiodic_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceAperiodic_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceSemipersistent_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Connected_r18MaximumSRS_ResourceSemipersistent_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19 {
    pub const MHZ40: u8 = 0u8;
    pub const MHZ50: u8 = 1u8;
    pub const MHZ80: u8 = 2u8;
    pub const MHZ100: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumTxFH_Hops_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumTxFH_Hops_r19 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N5: u8 = 3u8;
    pub const N6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR1_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR1_r19 {
    pub const N0: u8 = 0u8;
    pub const N70: u8 = 1u8;
    pub const N140: u8 = 2u8;
    pub const N210: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR2_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19Rf_TxRetuneTimeFR2_r19 {
    pub const N0: u8 = 0u8;
    pub const N35: u8 = 1u8;
    pub const N70: u8 = 2u8;
    pub const N140: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19(
    pub u8,
);
impl
    PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19
{
    pub const N0: u8 = 0u8;
    pub const N100: u8 = 1u8;
    pub const N140: u8 = 2u8;
    pub const N200: u8 = 3u8;
    pub const N300: u8 = 4u8;
    pub const N500: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19NumOfOverlappingPRB_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19NumOfOverlappingPRB_r19 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourcePeriodic_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourcePeriodic_r19 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceAperiodic_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceAperiodic_r19 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_ConnectedNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR1_r18 {
    pub const MHZ40: u8 = 0u8;
    pub const MHZ50: u8 = 1u8;
    pub const MHZ80: u8 = 2u8;
    pub const MHZ100: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_BandwidthAcrossAllHopsFR2_r18 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumTxFH_Hops_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumTxFH_Hops_r18 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N5: u8 = 3u8;
    pub const N6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR1_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR1_r18 {
    pub const N70: u8 = 0u8;
    pub const N140: u8 = 1u8;
    pub const N210: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR2_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18Rf_TxRetuneTimeFR2_r18 {
    pub const N35: u8 = 0u8;
    pub const N70: u8 = 1u8;
    pub const N140: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18SwitchTimeBetweenActiveBWP_FrequencyHop_r18 {
    pub const N100: u8 = 0u8;
    pub const N140: u8 = 1u8;
    pub const N200: u8 = 2u8;
    pub const N300: u8 = 3u8;
    pub const N500: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18NumOfOverlappingPRB_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18NumOfOverlappingPRB_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourcePeriodic_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourcePeriodic_r18 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourceSemipersistent_r18(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_Inactive_r18MaximumSRS_ResourceSemipersistent_r18 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR1_r19 {
    pub const MHZ40: u8 = 0u8;
    pub const MHZ50: u8 = 1u8;
    pub const MHZ80: u8 = 2u8;
    pub const MHZ100: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "2")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_BandwidthAcrossAllHopsFR2_r19 {
    pub const MHZ100: u8 = 0u8;
    pub const MHZ200: u8 = 1u8;
    pub const MHZ400: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "4")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumTxFH_Hops_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumTxFH_Hops_r19 {
    pub const N2: u8 = 0u8;
    pub const N3: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N5: u8 = 3u8;
    pub const N6: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR1_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR1_r19 {
    pub const N0: u8 = 0u8;
    pub const N70: u8 = 1u8;
    pub const N140: u8 = 2u8;
    pub const N210: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR2_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19Rf_TxRetuneTimeFR2_r19 {
    pub const N0: u8 = 0u8;
    pub const N35: u8 = 1u8;
    pub const N70: u8 = 2u8;
    pub const N140: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19SwitchTimeBetweenActiveBWP_FrequencyHop_r19 {
    pub const N0: u8 = 0u8;
    pub const N100: u8 = 1u8;
    pub const N140: u8 = 2u8;
    pub const N200: u8 = 3u8;
    pub const N300: u8 = 4u8;
    pub const N500: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "3")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19NumOfOverlappingPRB_r19(pub u8);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19NumOfOverlappingPRB_r19 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourcePeriodic_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourcePeriodic_r19 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "7")]
pub struct PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19(
    pub u8,
);
impl PosSRS_TxFrequencyHoppingRRC_InactiveNonRedCap_r19MaximumSRS_ResourceSemipersistent_r19 {
    pub const N0: u8 = 0u8;
    pub const N1: u8 = 1u8;
    pub const N2: u8 = 2u8;
    pub const N4: u8 = 3u8;
    pub const N8: u8 = 4u8;
    pub const N16: u8 = 5u8;
    pub const N32: u8 = 6u8;
    pub const N64: u8 = 7u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct PositioningModesPosModes(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "128")]
pub struct PressureValidityArea_v1520ValidityAreaWidth_v1520(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "128")]
pub struct PressureValidityArea_v1520ValidityAreaHeight_v1520(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "2881")]
pub struct PressureValidityPeriod_v1520BeginTimeAlt_v1520(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "2881")]
pub struct PressureValidityPeriod_v1520Duration_v1520(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideAssistanceDataCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideAssistanceDataCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideAssistanceDataCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ProvideAssistanceDataCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    ProvideAssistanceData_r9(ProvideAssistanceData_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(ProvideAssistanceDataCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(ProvideAssistanceDataCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(ProvideAssistanceDataCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideAssistanceDataCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProvideAssistanceDataCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(ProvideAssistanceDataCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(ProvideAssistanceDataCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideCapabilitiesCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideCapabilitiesCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideCapabilitiesCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ProvideCapabilitiesCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    ProvideCapabilities_r9(ProvideCapabilities_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(ProvideCapabilitiesCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(ProvideCapabilitiesCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(ProvideCapabilitiesCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideCapabilitiesCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProvideCapabilitiesCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(ProvideCapabilitiesCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(ProvideCapabilitiesCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideLocationInformationCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideLocationInformationCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct ProvideLocationInformationCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ProvideLocationInformationCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    ProvideLocationInformation_r9(ProvideLocationInformation_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(ProvideLocationInformationCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(ProvideLocationInformationCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(ProvideLocationInformationCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ProvideLocationInformationCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum ProvideLocationInformationCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(ProvideLocationInformationCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(ProvideLocationInformationCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct QoSVerticalCoordinateRequest(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct QoSVelocityRequest(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RAC_OrbitalErrorComponents_r17Radial_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RAC_OrbitalErrorComponents_r17AlongTrack_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RAC_OrbitalErrorComponents_r17CrossTrack_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RTD_InfoElement_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1966079")]
pub struct RTD_InfoElement_r16SubframeOffset_r16(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct RTK_Residuals_Element_r15S_oc_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RTK_Residuals_Element_r15S_od_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct RTK_Residuals_Element_r15S_oh_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct RTK_Residuals_Element_r15S_lc_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct RTK_Residuals_Element_r15S_ld_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ReferencePoint_r16ReferencePointGeographicLocation_r16 {
    #[asn(key = 0, extended = false)]
    Location3D_r16(EllipsoidPointWithAltitudeAndUncertaintyEllipsoid),
    #[asn(key = 1, extended = false)]
    Ha_location3D_r16(HighAccuracyEllipsoidPointWithAltitudeAndUncertaintyEllipsoid_r15),
    #[asn(key = 0, extended = true)]
    LocalOrigin_v1800(LocalOrigin_r18),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct ReferenceTRP_RTD_Info_r16Dl_PRS_ID_Ref_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct ReferenceTRP_RTD_Info_r16RefTime_r16_systemFrameNumber_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct ReferenceTRP_RTD_Info_r16RefTime_r16_utc_r16(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ReferenceTRP_RTD_Info_r16RefTime_r16 {
    #[asn(key = 0, extended = false)]
    SystemFrameNumber_r16(ReferenceTRP_RTD_Info_r16RefTime_r16_systemFrameNumber_r16),
    #[asn(key = 1, extended = false)]
    Utc_r16(ReferenceTRP_RTD_Info_r16RefTime_r16_utc_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16RegionID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei1_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd1_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei2_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd2_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei3_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd3_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei4_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd4_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei5_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd5_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei6_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd6_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei7_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd7_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei8_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd8_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei9_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd9_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei10_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd10_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei11_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd11_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei12_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd12_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei13_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd13_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei14_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd14_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct RegionIgpElement_r16Givei15_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "511")]
pub struct RegionIgpElement_r16Givd15_r16(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RelativeCartesianLocation_r18CartesianCoordinatesUnits_r18(pub u8);
impl RelativeCartesianLocation_r18CartesianCoordinatesUnits_r18 {
    pub const MM: u8 = 0u8;
    pub const CM: u8 = 1u8;
    pub const DM: u8 = 2u8;
    pub const M: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct RelativeLocation_r16Milli_arc_second_units_r16(pub u8);
impl RelativeLocation_r16Milli_arc_second_units_r16 {
    pub const MAS0_03: u8 = 0u8;
    pub const MAS0_3: u8 = 1u8;
    pub const MAS3: u8 = 2u8;
    pub const MAS30: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct RelativeLocation_r16Height_units_r16(pub u8);
impl RelativeLocation_r16Height_units_r16 {
    pub const MM: u8 = 0u8;
    pub const CM: u8 = 1u8;
    pub const M: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct RelativeLocationElement_r16DeltaLatitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct RelativeLocationElement_r16DeltaLongitude_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "64", sz_ub = "64")]
pub struct ReqNavListInfoSvReqList(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct ReqNavListInfoClockModelID_PrefList_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ReqNavListInfoClockModelID_PrefList(pub Vec<ReqNavListInfoClockModelID_PrefList_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct ReqNavListInfoOrbitModelID_PrefList_Entry(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct ReqNavListInfoOrbitModelID_PrefList(pub Vec<ReqNavListInfoOrbitModelID_PrefList_Entry>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct ReqNavListInfoAddNavparamReq(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestAssistanceDataCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestAssistanceDataCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestAssistanceDataCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum RequestAssistanceDataCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    RequestAssistanceData_r9(RequestAssistanceData_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(RequestAssistanceDataCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(RequestAssistanceDataCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(RequestAssistanceDataCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestAssistanceDataCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum RequestAssistanceDataCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(RequestAssistanceDataCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(RequestAssistanceDataCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestCapabilitiesCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestCapabilitiesCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestCapabilitiesCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum RequestCapabilitiesCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    RequestCapabilities_r9(RequestCapabilities_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(RequestCapabilitiesCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(RequestCapabilitiesCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(RequestCapabilitiesCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestCapabilitiesCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum RequestCapabilitiesCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(RequestCapabilitiesCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(RequestCapabilitiesCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestLocationInformationCriticalExtensions_c1_spare3;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestLocationInformationCriticalExtensions_c1_spare2;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct RequestLocationInformationCriticalExtensions_c1_spare1;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum RequestLocationInformationCriticalExtensions_c1 {
    #[asn(key = 0, extended = false)]
    RequestLocationInformation_r9(RequestLocationInformation_r9_IEs),
    #[asn(key = 1, extended = false)]
    Spare3(RequestLocationInformationCriticalExtensions_c1_spare3),
    #[asn(key = 2, extended = false)]
    Spare2(RequestLocationInformationCriticalExtensions_c1_spare2),
    #[asn(key = 3, extended = false)]
    Spare1(RequestLocationInformationCriticalExtensions_c1_spare1),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct RequestLocationInformationCriticalExtensions_criticalExtensionsFuture {}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum RequestLocationInformationCriticalExtensions {
    #[asn(key = 0, extended = false)]
    C1(RequestLocationInformationCriticalExtensions_c1),
    #[asn(key = 1, extended = false)]
    CriticalExtensionsFuture(RequestLocationInformationCriticalExtensions_criticalExtensionsFuture),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "128")]
pub struct ResponseTimeTime(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "512")]
pub struct ResponseTimeNB_r14TimeNB_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "512")]
pub struct ResponseTimeNB_r14ResponseTimeEarlyFixNB_r14(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "95")]
pub struct ResultsPerCSI_RS_Index_r16Csi_RS_Index_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct ResultsPerSSB_Index_r16Ssb_Index_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "5399")]
pub struct SBAS_ClockModelSbasTo(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct SBAS_ClockModelSbasAgfo(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct SBAS_ClockModelSbasAgf1(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "3")]
pub struct SBAS_IDSbas_id(pub u8);
impl SBAS_IDSbas_id {
    pub const WAAS: u8 = 0u8;
    pub const EGNOS: u8 = 1u8;
    pub const MSAS: u8 = 2u8;
    pub const GAGAN: u8 = 3u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct SBAS_IDsSbas_IDs(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct SFN_r15Sfn_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "10", sz_ub = "10")]
pub struct SFN_r15HyperSFN_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "5")]
pub struct SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourceSetsPerBWP_r16(pub u8);
impl SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourceSetsPerBWP_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N12: u8 = 4u8;
    pub const N16: u8 = 5u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourcesPerBWP_r16(pub u8);
impl SRS_PosResourcesPerBand_r16MaxNumberSRS_PosResourcesPerBWP_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct SRS_PosResourcesPerBand_r16MaxNumberPeriodicSRS_PosResourcesPerBWP_r16(pub u8);
impl SRS_PosResourcesPerBand_r16MaxNumberPeriodicSRS_PosResourcesPerBWP_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct SRS_PosResourcesPerBand_r16MaxNumberAP_SRS_PosResourcesPerBWP_r16(pub u8);
impl SRS_PosResourcesPerBand_r16MaxNumberAP_SRS_PosResourcesPerBWP_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct SRS_PosResourcesPerBand_r16MaxNumberSP_SRS_PosResourcesPerBWP_r16(pub u8);
impl SRS_PosResourcesPerBand_r16MaxNumberSP_SRS_PosResourcesPerBWP_r16 {
    pub const N1: u8 = 0u8;
    pub const N2: u8 = 1u8;
    pub const N4: u8 = 2u8;
    pub const N8: u8 = 3u8;
    pub const N16: u8 = 4u8;
    pub const N32: u8 = 5u8;
    pub const N64: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C0_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C1_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-67108864", ub = "67108863")]
pub struct SSR_ClockCorrectionSatelliteElement_r15Delta_Clock_C2_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct SSR_CodeBiasSignalElement_r15CodeBias_r15(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_GriddedCorrectionIntegrityParameters_r17ProbOnsetTroposphereFault_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct SSR_GriddedCorrectionIntegrityParameters_r17MeanTroposphereFaultDuration_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "255")]
pub struct SSR_GriddedCorrectionIntegrityParameters_r17TroposphereRangeErrorCorrelationTime_r17(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "255")]
pub struct SSR_GriddedCorrectionIntegrityParameters_r17TroposphereRangeRateErrorCorrelationTime_r17(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityClockBounds_r17MeanClock_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityClockBounds_r17StdDevClock_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityClockBounds_r17MeanClockRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityClockBounds_r17StdDevClockRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityCodeBiasBounds_r17MeanCodeBias_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityCodeBiasBounds_r17StdDevCodeBias_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityCodeBiasBounds_r17MeanCodeBiasRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityCodeBiasBounds_r17StdDevCodeBiasRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityPhaseBiasBounds_r17MeanPhaseBias_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityPhaseBiasBounds_r17StdDevPhaseBias_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityPhaseBiasBounds_r17MeanPhaseBiasRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct SSR_IntegrityPhaseBiasBounds_r17StdDevPhaseBiasRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Iod_r15(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2097152", ub = "2097151")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Delta_radial_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-524288", ub = "524287")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Delta_AlongTrack_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-524288", ub = "524287")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Delta_CrossTrack_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_radial_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_AlongTrack_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-262144", ub = "262143")]
pub struct SSR_OrbitCorrectionSatelliteElement_r15Dot_delta_CrossTrack_r15(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-16384", ub = "16383")]
pub struct SSR_PhaseBiasSignalElement_r16PhaseBias_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct SSR_PhaseBiasSignalElement_r16PhaseDiscontinuityIndicator_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3")]
pub struct SSR_PhaseBiasSignalElement_r16PhaseBiasIntegerIndicator_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct SSR_PhaseCenterVariationList_r18_Entry(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct SSR_URA_SatElement_r16Ssr_URA_r16(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct STEC_IntegrityErrorBounds_r17MeanIonosphere_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct STEC_IntegrityErrorBounds_r17StdDevIonosphere_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct STEC_IntegrityErrorBounds_r17MeanIonosphereRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct STEC_IntegrityErrorBounds_r17StdDevIonosphereRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct STEC_IntegrityParameters_r17ProbOnsetIonoFault_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "256")]
pub struct STEC_IntegrityParameters_r17MeanIonoFaultDuration_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "255")]
pub struct STEC_IntegrityParameters_r17IonoRangeErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "255")]
pub struct STEC_IntegrityParameters_r17IonoRangeRateErrorCorrelationTime_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct STEC_ResidualSatElement_r16StecResidualCorrection_r16_b7_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct STEC_ResidualSatElement_r16StecResidualCorrection_r16_b16_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = false)]
pub enum STEC_ResidualSatElement_r16StecResidualCorrection_r16 {
    #[asn(key = 0, extended = false)]
    B7_r16(STEC_ResidualSatElement_r16StecResidualCorrection_r16_b7_r16),
    #[asn(key = 1, extended = false)]
    B16_r16(STEC_ResidualSatElement_r16StecResidualCorrection_r16_b16_r16),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct STEC_SatElement_r16StecQualityIndicator_r16(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8192", ub = "8191")]
pub struct STEC_SatElement_r16Stec_C00_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct STEC_SatElement_r16Stec_C01_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2048", ub = "2047")]
pub struct STEC_SatElement_r16Stec_C10_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct STEC_SatElement_r16Stec_C11_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "63")]
pub struct SV_IDSatellite_id(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct SatListElement_r15Iod_r15(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "11", sz_ub = "11")]
pub struct SatListRelatedDataElementIod(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct SatListRelatedDataElementClockModelID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "8")]
pub struct SatListRelatedDataElementOrbitModelID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct ScheduledLocationTime_r17UtcTime_r17(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "3599999")]
pub struct ScheduledLocationTime_r17GnssTime_r17Gnss_TOD_msec_r17(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledLocationTime_r17GnssTime_r17 {
    pub gnss_tod_msec_r17: ScheduledLocationTime_r17GnssTime_r17Gnss_TOD_msec_r17,
    pub gnss_time_id_r17: GNSS_ID,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "503")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17Lte_PhysCellId_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17Lte_SystemFrameNumber_r17(
    pub u16,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 1)]
pub struct ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17 {
    pub lte_phys_cell_id_r17:
        ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17Lte_PhysCellId_r17,
    pub lte_arfcn_eutra_r17: ARFCN_ValueEUTRA,
    #[asn(optional_idx = 0)]
    pub lte_cell_global_id_r17: Option<CellGlobalIdEUTRA_AndUTRA>,
    pub lte_system_frame_number_r17:
        ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17Lte_SystemFrameNumber_r17,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1023")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_SFN_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "9")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs15_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "19")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs30_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "39")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs60_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "79")]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs120_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "3", extensible = false)]
pub enum ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17 {
    #[asn(key = 0, extended = false)]
    Scs15_r17(ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs15_r17),
    #[asn(key = 1, extended = false)]
    Scs30_r17(ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs30_r17),
    #[asn(key = 2, extended = false)]
    Scs60_r17(ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs60_r17),
    #[asn(key = 3, extended = false)]
    Scs120_r17(ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17_scs120_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false, optional_fields = 2)]
pub struct ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17 {
    pub nr_phys_cell_id_r17: NR_PhysCellID_r16,
    pub nr_arfcn_r17: ARFCN_ValueNR_r15,
    #[asn(optional_idx = 0)]
    pub nr_cell_global_id_r17: Option<NCGI_r15>,
    pub nr_sfn_r17: ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_SFN_r17,
    #[asn(optional_idx = 1)]
    pub nr_slot_r17: Option<ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17Nr_Slot_r17>,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "CHOICE", lb = "0", ub = "1", extensible = true)]
pub enum ScheduledLocationTime_r17NetworkTime_r17 {
    #[asn(key = 0, extended = false)]
    E_utraTime_r17(ScheduledLocationTime_r17NetworkTime_r17_e_utraTime_r17),
    #[asn(key = 1, extended = false)]
    NrTime_r17(ScheduledLocationTime_r17NetworkTime_r17_nrTime_r17),
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1024")]
pub struct ScheduledLocationTime_r17RelativeTime_r17(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct ScheduledLocationTimeSupport_r17UtcTime_r17(pub u8);
impl ScheduledLocationTimeSupport_r17UtcTime_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct ScheduledLocationTimeSupport_r17E_utraTime_r17(pub u8);
impl ScheduledLocationTimeSupport_r17E_utraTime_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct ScheduledLocationTimeSupport_r17NrTime_r17(pub u8);
impl ScheduledLocationTimeSupport_r17NrTime_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct ScheduledLocationTimeSupport_r17RelativeTime_r17(pub u8);
impl ScheduledLocationTimeSupport_r17RelativeTime_r17 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE", extensible = false)]
pub struct ScheduledLocationTimeSupportPerMode_r17GnssTime_r17 {
    pub pos_modes_r17: PositioningModes,
    pub gnss_time_i_ds_r17: GNSS_ID_Bitmap,
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-20000", ub = "10000")]
pub struct Sensor_AssistanceDataList_r14RefPressure_r14(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct Sensor_AssistanceDataList_r14RefTemperature_r14(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Sensor_LocationServerErrorCauses_r13Cause_r13(pub u8);
impl Sensor_LocationServerErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct Sensor_MeasurementInformation_r13MeasurementReferenceTime_r13(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "30000", ub = "115000")]
pub struct Sensor_MeasurementInformation_r13UncompensatedBarometricPressure_r13(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct Sensor_ProvideCapabilities_r13Sensor_Modes_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct Sensor_RequestLocationInformation_r13UncompensatedBarometricPressureReq_r13(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct Sensor_TargetDeviceErrorCauses_r13Cause_r13(pub u8);
impl Sensor_TargetDeviceErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Serving_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Serving_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnCSI_RS_Serving_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnCSI_RS_Serving_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Serving_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Serving_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSRS_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSRS_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Neigh_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnSSB_Neigh_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "0")]
pub struct SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Neigh_r16(pub u8);
impl SpatialRelationsSRS_Pos_r16SpatialRelation_SRS_PosBasedOnPRS_Neigh_r16 {
    pub const SUPPORTED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16383")]
pub struct StandardClockModelElementStanClockToc(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32", ub = "31")]
pub struct StandardClockModelElementStanClockAF2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1048576", ub = "1048575")]
pub struct StandardClockModelElementStanClockAF1(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1073741824", ub = "1073741823")]
pub struct StandardClockModelElementStanClockAF0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct StandardClockModelElementStanClockTgd(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct StandardClockModelElementSisa(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "1")]
pub struct StandardClockModelElementStanModelID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct StoredNavListInfoGnss_WeekOrDay(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct StoredNavListInfoGnss_Toe(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "15")]
pub struct StoredNavListInfoT_toeLimit(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "64")]
pub struct SupportedBandEUTRABandEUTRA(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "65", ub = "256")]
pub struct SupportedBandEUTRA_v9a0BandEUTRA_v9a0(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch34_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch36_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch38_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch40_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch42_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch44_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch46_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch48_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch52_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch56_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch60_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch64_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch149_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch153_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch157_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11a_r14Ch161_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch1_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch2_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch3_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch4_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch5_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch6_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch7_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch8_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch9_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch10_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch11_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch12_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch13_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct SupportedChannels_11bg_r14Ch14_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct TBS_LocationServerErrorCauses_r13Cause_r13(pub u8);
impl TBS_LocationServerErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct TBS_MeasurementInformation_r13MeasurementReferenceTime_r13(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct TBS_ProvideCapabilities_r13Tbs_Modes_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct TBS_RequestAssistanceData_r14Mbs_AlmanacAssistanceDataReq_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct TBS_RequestAssistanceData_r14Mbs_AcquisitionAssistanceDataReq_r14(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct TBS_RequestLocationInformation_r13MbsSgnMeasListReq_r13(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "1")]
pub struct TBS_TargetDeviceErrorCauses_r13Cause_r13(pub u8);
impl TBS_TargetDeviceErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
    pub const THERE_WERE_NOT_ENOUGH_MBS_BEACONS_RECEIVED: u8 = 1u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", lb = "0", ub = "6")]
pub struct TDD_Config_v1520SubframeAssignment_v1520(pub u8);
impl TDD_Config_v1520SubframeAssignment_v1520 {
    pub const SA0: u8 = 0u8;
    pub const SA1: u8 = 1u8;
    pub const SA2: u8 = 2u8;
    pub const SA3: u8 = 3u8;
    pub const SA4: u8 = 4u8;
    pub const SA5: u8 = 5u8;
    pub const SA6: u8 = 6u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TRP_LocationInfo_Implicit_Element_r19Nr_AIML_AssociatedID_r19(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TRP_LocationInfoElement_r16Dl_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TRP_LocationInfoElement_r16Associated_DL_PRS_ID_r16(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "2")]
pub struct TRP_LocationInfoElement_r16Trp_DL_PRS_ResourceSets_r16(
    pub Vec<DL_PRS_ResourceSets_TRP_Element_r16>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TRP_RequestInfoElement_r19Dl_PRS_ID_r19(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct TriggeredReportingCriteriaCellChange(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalHydroStaticDelay_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalHydroStaticDelay_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalWetDelay_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalWetDelay_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalHydroStaticDelayRate_r17(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalHydroStaticDelayRate_r17(
    pub u8,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17MeanTroposphereVerticalWetDelayRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct TropoDelayIntegrityErrorBounds_r17StdDevTroposphereVerticalWetDelayRate_r17(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-256", ub = "255")]
pub struct TropospericDelayCorrection_r16TropoHydroStaticVerticalDelay_r16(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct TropospericDelayCorrection_r16TropoWetVerticalDelay_r16(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct UTC_ModelSet1Gnss_Utc_A1(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct UTC_ModelSet1Gnss_Utc_A0(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet1Gnss_Utc_Tot(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet1Gnss_Utc_WNt(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet1Gnss_Utc_DeltaTls(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet1Gnss_Utc_WNlsf(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet1Gnss_Utc_DN(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet1Gnss_Utc_DeltaTlsf(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-32768", ub = "32767")]
pub struct UTC_ModelSet2UtcA0(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-4096", ub = "4095")]
pub struct UTC_ModelSet2UtcA1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-64", ub = "63")]
pub struct UTC_ModelSet2UtcA2(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet2UtcDeltaTls(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "65535")]
pub struct UTC_ModelSet2UtcTot(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "8191")]
pub struct UTC_ModelSet2UtcWNot(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet2UtcWNlsf(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "4", sz_ub = "4")]
pub struct UTC_ModelSet2UtcDN(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet2UtcDeltaTlsf(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "1461")]
pub struct UTC_ModelSet3NA(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct UTC_ModelSet3TauC(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct UTC_ModelSet3B1(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-512", ub = "511")]
pub struct UTC_ModelSet3B2(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "2", sz_ub = "2")]
pub struct UTC_ModelSet3Kp(pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct UTC_ModelSet4UtcA1wnt(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct UTC_ModelSet4UtcA0wnt(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet4UtcTot(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet4UtcWNt(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet4UtcDeltaTls(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet4UtcWNlsf(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet4UtcDN(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet4UtcDeltaTlsf(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "7")]
pub struct UTC_ModelSet4UtcStandardID(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-2147483648", ub = "2147483647")]
pub struct UTC_ModelSet5_r12UtcA0_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-8388608", ub = "8388607")]
pub struct UTC_ModelSet5_r12UtcA1_r12(pub i32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet5_r12UtcDeltaTls_r12(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet5_r12UtcWNlsf_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct UTC_ModelSet5_r12UtcDN_r12(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-128", ub = "127")]
pub struct UTC_ModelSet5_r12UtcDeltaTlsf_r12(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct UTC_Time_r15UtcTime_r15(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "999")]
pub struct UTC_Time_r15UtcTime_ms_r15(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct VelocityTypesHorizontalVelocity(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct VelocityTypesHorizontalWithVerticalVelocity(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct VelocityTypesHorizontalVelocityWithUncertainty(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct VelocityTypesHorizontalWithVerticalVelocityAndUncertainty(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "127")]
pub struct VerticalAccuracyAccuracy(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct VerticalAccuracyConfidence(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct VerticalAccuracyExt_r15AccuracyExt_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "100")]
pub struct VerticalAccuracyExt_r15Confidence_r15(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-50", ub = "900")]
pub struct VerticalGridPoints_r18ReferenceAltitudeCoarse_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "1", ub = "3")]
pub struct VerticalGridPoints_r18NumberOfStepsDown_r18(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "OCTET-STRING", sz_extensible = false, sz_lb = "6", sz_ub = "6")]
pub struct WLAN_AP_Identifier_r13Bssid_r13(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "OCTET-STRING",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "32"
)]
pub struct WLAN_AP_Identifier_r13Ssid_r13(pub Vec<u8>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "128"
)]
pub struct WLAN_DataSet_r14Wlan_AP_List_r14(pub Vec<WLAN_AP_Data_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "0")]
pub struct WLAN_LocationServerErrorCauses_r13Cause_r13(pub u8);
impl WLAN_LocationServerErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-127", ub = "128")]
pub struct WLAN_MeasurementElement_r13Rssi_r13(pub i8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "256")]
pub struct WLAN_MeasurementElement_r13ApChannelFrequency_r13(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BOOLEAN")]
pub struct WLAN_MeasurementElement_r13ServingFlag_r13(pub bool);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "UTCTime")]
pub struct WLAN_MeasurementInformation_r13MeasurementReferenceTime_r13(pub String);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct WLAN_ProvideAssistanceData_r14Wlan_DataSet_r14(pub Vec<WLAN_DataSet_r14>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct WLAN_ProvideCapabilities_r13Wlan_Modes_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct WLAN_ProvideCapabilities_r13Wlan_MeasSupported_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "16777215")]
pub struct WLAN_RTT_r13RttValue_r13(pub u32);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "4")]
pub struct WLAN_RTT_r13RttUnits_r13(pub u8);
impl WLAN_RTT_r13RttUnits_r13 {
    pub const MICROSECONDS: u8 = 0u8;
    pub const HUNDREDSOFNANOSECONDS: u8 = 1u8;
    pub const TENSOFNANOSECONDS: u8 = 2u8;
    pub const NANOSECONDS: u8 = 3u8;
    pub const TENTHSOFNANOSECONDS: u8 = 4u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "255")]
pub struct WLAN_RTT_r13RttAccuracy_r13(pub u8);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct WLAN_RequestAssistanceData_r14RequestedAD_r14(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "SEQUENCE-OF", sz_extensible = false, sz_lb = "1", sz_ub = "32")]
pub struct WLAN_RequestAssistanceData_r14VisibleAPs_r14(pub Vec<WLAN_AP_Identifier_r13>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(
    type = "SEQUENCE-OF",
    sz_extensible = false,
    sz_lb = "1",
    sz_ub = "2048"
)]
pub struct WLAN_RequestAssistanceData_r14Wlan_AP_StoredData_r14(pub Vec<WLAN_AP_Identifier_r13>);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "BITSTRING", sz_extensible = false, sz_lb = "1", sz_ub = "8")]
pub struct WLAN_RequestLocationInformation_r13RequestedMeasurements_r13(
    pub bitvec::vec::BitVec<u8, bitvec::order::Msb0>,
);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "ENUMERATED", extensible = true, lb = "0", ub = "2")]
pub struct WLAN_TargetDeviceErrorCauses_r13Cause_r13(pub u8);
impl WLAN_TargetDeviceErrorCauses_r13Cause_r13 {
    pub const UNDEFINED: u8 = 0u8;
    pub const REQUESTED_MEASUREMENTS_NOT_AVAILABLE: u8 = 1u8;
    pub const NOT_ALLREQUESTED_MEASUREMENTS_POSSIBLE: u8 = 2u8;
}

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct WLAN_TargetDeviceErrorCauses_r13Wlan_AP_RSSI_MeasurementNotPossible_r13;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "NULL")]
pub struct WLAN_TargetDeviceErrorCauses_r13Wlan_AP_RTT_MeasurementNotPossible_r13;

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct X_Value_r18Delta_x_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct X_Value_r18Coarse_delta_x_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Y_Value_r18Delta_y_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Y_Value_r18Coarse_delta_y_r18(pub u16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "-1024", ub = "1023")]
pub struct Z_Value_r18Delta_z_r18(pub i16);

#[derive(asn1_codecs_derive :: UperCodec, Debug)]
#[asn(type = "INTEGER", lb = "0", ub = "4095")]
pub struct Z_Value_r18Coarse_delta_z_r18(pub u16);
