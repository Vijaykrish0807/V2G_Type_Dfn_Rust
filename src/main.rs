/* SPDX-License-Identifier: Apache-2.0 */
/*
 * Copyright (C) 2022 - 2023 chargebyte GmbH
 * Copyright (C) 2022 - 2023 Contributors to EVerest
 */

/*****************************************************
 *
 * @author
 * @version
 *
 * The Code is generated! Changes may be overwritten.
 *
 *****************************************************/

/**
  * @file iso2_msgDefDatatypes.h
  * @brief Description goes here
  *
  **/
mod init_fns;
mod stack_alloc;
mod iso2_msgDefEncoder;
use stack_alloc::*;
use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use init_fns::*;
use fixedvec::FixedVec;


const EXI_STRING_MAX_LEN: usize= 1000;
const ASCII_EXTRA_CHAR: usize= 1;
const EXI_BYTE_ARRAY_MAX_LEN: usize= 350;
const ISO2Algorithm_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2anyType_BYTES_SIZE: usize= 4;

const ISO2XPath_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2CryptoBinary_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2X509IssuerName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2Id_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2Type_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2URI_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2DigestValueType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2base64Binary_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2X509SubjectName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2ReferenceType_4_ARRAY_SIZE: usize= 4;

const ISO2ServiceName_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;

const ISO2ServiceScope_CHARACTER_SIZE: usize= 64 + ASCII_EXTRA_CHAR;

const ISO2SignatureValueType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2certificateType_4_ARRAY_SIZE: usize= 4;

const ISO2certificateType_BYTES_SIZE: usize= 800;

const ISO2KeyName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2MgmtData_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2Encoding_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2MimeType_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2EnergyTransferModeType_6_ARRAY_SIZE: usize= 6;

const ISO2FaultMsg_CHARACTER_SIZE: usize= 64 + ASCII_EXTRA_CHAR;

const ISO2paymentOptionType_2_ARRAY_SIZE: usize= 2;

const ISO2SelectedServiceType_16_ARRAY_SIZE: usize= 16;

const ISO2CostType_3_ARRAY_SIZE: usize= 3;

const ISO2ConsumptionCostType_3_ARRAY_SIZE: usize= 3;

const ISO2PMaxScheduleEntryType_1024_ARRAY_SIZE: usize= 1024;

const ISO2Name_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2stringValue_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2SalesTariffDescription_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;

const ISO2SalesTariffEntryType_1024_ARRAY_SIZE: usize= 1024;

const ISO2ParameterType_16_ARRAY_SIZE: usize= 16;

const ISO2SAScheduleTupleType_3_ARRAY_SIZE: usize= 3;

const ISO2ParameterSetType_255_ARRAY_SIZE: usize= 255;

const ISO2ProfileEntryType_24_ARRAY_SIZE: usize= 24;

const ISO2X509IssuerSerialType_20_ARRAY_SIZE: usize= 20;

const ISO2ServiceType_8_ARRAY_SIZE: usize= 8;

const ISO2ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2DiffieHellmanPublickeyType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;

const ISO2MeterID_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;

const ISO2sigMeterReadingType_BYTES_SIZE: usize= 64;

const ISO2CONTENT_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;

const ISO2sessionIDType_BYTES_SIZE: usize= 8;

const ISO2EVSEID_CHARACTER_SIZE: usize= 37 + ASCII_EXTRA_CHAR;

const ISO2genChallengeType_BYTES_SIZE: usize= 16;

const ISO2eMAID_CHARACTER_SIZE: usize= 15 + ASCII_EXTRA_CHAR;

const ISO2evccIDType_BYTES_SIZE: usize= 6;


// enum for function numbers
pub enum ISO2generatedFunctionNumbersType{
    ISO2AC_EVChargeParameter,
    ISO2AC_EVSEChargeParameter,
    ISO2AC_EVSEStatus,
    ISO2AuthorizationReq,
    ISO2AuthorizationRes,
    ISO2BodyElement,
    ISO2CableCheckReq,
    ISO2CableCheckRes,
    ISO2CanonicalizationMethod,
    ISO2CertificateInstallationReq,
    ISO2CertificateInstallationRes,
    ISO2CertificateUpdateReq,
    ISO2CertificateUpdateRes,
    ISO2ChargeParameterDiscoveryReq,
    ISO2ChargeParameterDiscoveryRes,
    ISO2ChargingStatusReq,
    ISO2ChargingStatusRes,
    ISO2CurrentDemandReq,
    ISO2CurrentDemandRes,
    ISO2DC_EVChargeParameter,
    ISO2DC_EVPowerDeliveryParameter,
    ISO2DC_EVSEChargeParameter,
    ISO2DC_EVSEStatus,
    ISO2DC_EVStatus,
    ISO2DSAKeyValue,
    ISO2DigestMethod,
    ISO2DigestValue,
    ISO2EVChargeParameter,
    ISO2EVPowerDeliveryParameter,
    ISO2EVSEChargeParameter,
    ISO2EVSEStatus,
    ISO2EVStatus,
    ISO2Entry,
    ISO2KeyInfo,
    ISO2KeyName,
    ISO2KeyValue,
    ISO2Manifest,
    ISO2MeteringReceiptReq,
    ISO2MeteringReceiptRes,
    ISO2MgmtData,
    ISO2Object,
    ISO2PGPData,
    ISO2PMaxScheduleEntry,
    ISO2PaymentDetailsReq,
    ISO2PaymentDetailsRes,
    ISO2PaymentServiceSelectionReq,
    ISO2PaymentServiceSelectionRes,
    ISO2PowerDeliveryReq,
    ISO2PowerDeliveryRes,
    ISO2PreChargeReq,
    ISO2PreChargeRes,
    ISO2RSAKeyValue,
    ISO2Reference,
    ISO2RelativeTimeInterval,
    ISO2RetrievalMethod,
    ISO2SAScheduleList,
    ISO2SASchedules,
    ISO2SPKIData,
    ISO2SalesTariffEntry,
    ISO2ServiceDetailReq,
    ISO2ServiceDetailRes,
    ISO2ServiceDiscoveryReq,
    ISO2ServiceDiscoveryRes,
    ISO2SessionSetupReq,
    ISO2SessionSetupRes,
    ISO2SessionStopReq,
    ISO2SessionStopRes,
    ISO2Signature,
    ISO2SignatureMethod,
    ISO2SignatureProperties,
    ISO2SignatureProperty,
    ISO2SignatureValue,
    ISO2SignedInfo,
    ISO2TimeInterval,
    ISO2Transform,
    ISO2Transforms,
    ISO2V2G_Message,
    ISO2WeldingDetectionReq,
    ISO2WeldingDetectionRes,
    ISO2X509Data,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}costKind; type={urn:iso:15118:2:2013:MsgDataTypes}costKindType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2costKindType{
    ISO2costKindType_relativePricePercentage,
    ISO2costKindType_RenewableGenerationPercentage,
    ISO2costKindType_CarbonDioxideEmission,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}EnergyTransferMode; type={urn:iso:15118:2:2013:MsgDataTypes}EnergyTransferModeType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2EnergyTransferModeType{
    ISO2EnergyTransferModeType_AC_single_phase_core,
    ISO2EnergyTransferModeType_AC_three_phase_core,
    ISO2EnergyTransferModeType_DC_core,
    ISO2EnergyTransferModeType_DC_extended,
    ISO2EnergyTransferModeType_DC_combo_core,
    ISO2EnergyTransferModeType_DC_unique,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}EVSENotification; type={urn:iso:15118:2:2013:MsgDataTypes}EVSENotificationType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2EVSENotificationType{
    ISO2EVSENotificationType_None,
    ISO2EVSENotificationType_StopCharging,
    ISO2EVSENotificationType_ReNegotiation,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEIsolationStatus; type={urn:iso:15118:2:2013:MsgDataTypes}isolationLevelType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2isolationLevelType{
    ISO2isolationLevelType_Invalid,
    ISO2isolationLevelType_Valid,
    ISO2isolationLevelType_Warning,
    ISO2isolationLevelType_Fault,
    ISO2isolationLevelType_No_IMD,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatusCode; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatusCodeType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2DC_EVSEStatusCodeType{
    ISO2DC_EVSEStatusCodeType_EVSE_NotReady,
    ISO2DC_EVSEStatusCodeType_EVSE_Ready,
    ISO2DC_EVSEStatusCodeType_EVSE_Shutdown,
    ISO2DC_EVSEStatusCodeType_EVSE_UtilityInterruptEvent,
    ISO2DC_EVSEStatusCodeType_EVSE_IsolationMonitoringActive,
    ISO2DC_EVSEStatusCodeType_EVSE_EmergencyShutdown,
    ISO2DC_EVSEStatusCodeType_EVSE_Malfunction,
    ISO2DC_EVSEStatusCodeType_Reserved_8,
    ISO2DC_EVSEStatusCodeType_Reserved_9,
    ISO2DC_EVSEStatusCodeType_Reserved_A,
    ISO2DC_EVSEStatusCodeType_Reserved_B,
    ISO2DC_EVSEStatusCodeType_Reserved_C,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}FaultCode; type={urn:iso:15118:2:2013:MsgDataTypes}faultCodeType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2faultCodeType{
    ISO2faultCodeType_ParsingError,
    ISO2faultCodeType_NoTLSRootCertificatAvailable,
    ISO2faultCodeType_UnknownError,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}PaymentOption; type={urn:iso:15118:2:2013:MsgDataTypes}paymentOptionType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2paymentOptionType{
    ISO2paymentOptionType_Contract,
    ISO2paymentOptionType_ExternalPayment,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}EVErrorCode; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVErrorCodeType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2DC_EVErrorCodeType{
    ISO2DC_EVErrorCodeType_NO_ERROR,
    ISO2DC_EVErrorCodeType_FAILED_RESSTemperatureInhibit,
    ISO2DC_EVErrorCodeType_FAILED_EVShiftPosition,
    ISO2DC_EVErrorCodeType_FAILED_ChargerConnectorLockFault,
    ISO2DC_EVErrorCodeType_FAILED_EVRESSMalfunction,
    ISO2DC_EVErrorCodeType_FAILED_ChargingCurrentdifferential,
    ISO2DC_EVErrorCodeType_FAILED_ChargingVoltageOutOfRange,
    ISO2DC_EVErrorCodeType_Reserved_A,
    ISO2DC_EVErrorCodeType_Reserved_B,
    ISO2DC_EVErrorCodeType_Reserved_C,
    ISO2DC_EVErrorCodeType_FAILED_ChargingSystemIncompatibility,
    ISO2DC_EVErrorCodeType_NoData,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}Unit; type={urn:iso:15118:2:2013:MsgDataTypes}unitSymbolType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2unitSymbolType{
    ISO2unitSymbolType_h,
    ISO2unitSymbolType_m,
    ISO2unitSymbolType_s,
    ISO2unitSymbolType_A,
    ISO2unitSymbolType_V,
    ISO2unitSymbolType_W,
    ISO2unitSymbolType_Wh,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgDataTypes}ServiceCategory; type={urn:iso:15118:2:2013:MsgDataTypes}serviceCategoryType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2serviceCategoryType{
    ISO2serviceCategoryType_EVCharging,
    ISO2serviceCategoryType_Internet,
    ISO2serviceCategoryType_ContractCertificate,
    ISO2serviceCategoryType_OtherCustom,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgBody}ResponseCode; type={urn:iso:15118:2:2013:MsgDataTypes}responseCodeType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2responseCodeType{
    ISO2responseCodeType_OK,
    ISO2responseCodeType_OK_NewSessionEstablished,
    ISO2responseCodeType_OK_OldSessionJoined,
    ISO2responseCodeType_OK_CertificateExpiresSoon,
    ISO2responseCodeType_FAILED,
    ISO2responseCodeType_FAILED_SequenceError,
    ISO2responseCodeType_FAILED_ServiceIDInvalid,
    ISO2responseCodeType_FAILED_UnknownSession,
    ISO2responseCodeType_FAILED_ServiceSelectionInvalid,
    ISO2responseCodeType_FAILED_PaymentSelectionInvalid,
    ISO2responseCodeType_FAILED_CertificateExpired,
    ISO2responseCodeType_FAILED_SignatureError,
    ISO2responseCodeType_FAILED_NoCertificateAvailable,
    ISO2responseCodeType_FAILED_CertChainError,
    ISO2responseCodeType_FAILED_ChallengeInvalid,
    ISO2responseCodeType_FAILED_ContractCanceled,
    ISO2responseCodeType_FAILED_WrongChargeParameter,
    ISO2responseCodeType_FAILED_PowerDeliveryNotApplied,
    ISO2responseCodeType_FAILED_TariffSelectionInvalid,
    ISO2responseCodeType_FAILED_ChargingProfileInvalid,
    ISO2responseCodeType_FAILED_MeteringSignatureNotValid,
    ISO2responseCodeType_FAILED_NoChargeServiceSelected,
    ISO2responseCodeType_FAILED_WrongEnergyTransferMode,
    ISO2responseCodeType_FAILED_ContactorError,
    ISO2responseCodeType_FAILED_CertificateNotAllowedAtThisEVSE,
    ISO2responseCodeType_FAILED_CertificateRevoked,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgBody}ChargingSession; type={urn:iso:15118:2:2013:MsgDataTypes}chargingSessionType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2chargingSessionType{
    ISO2chargingSessionType_Terminate,
    ISO2chargingSessionType_Pause,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgBody}ChargeProgress; type={urn:iso:15118:2:2013:MsgDataTypes}chargeProgressType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2chargeProgressType{
    ISO2chargeProgressType_Start,
    ISO2chargeProgressType_Stop,
    ISO2chargeProgressType_Renegotiate,
}

// Element: definition=enum; name={urn:iso:15118:2:2013:MsgBody}EVSEProcessing; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEProcessingType; base type=string; content type=simple;
//          abstract=False; final=False; derivation=restriction;
pub enum ISO2EVSEProcessingType{
    ISO2EVSEProcessingType_Finished,
    ISO2EVSEProcessingType_Ongoing,
    ISO2EVSEProcessingType_Ongoing_WaitingForCustomerInteraction,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Cost; type={urn:iso:15118:2:2013:MsgDataTypes}CostType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: costKind, costKindType (1, 1); amount, unsignedInt (1, 1); amountMultiplier, unitMultiplierType (0, 1);
pub struct ISO2CostType {
    // costKind, costKindType (base: string)
    
    costKind: ISO2costKindType,
    // amount, unsignedInt (base: unsignedLong)
    
    amount: u32,
    // amountMultiplier, unitMultiplierType (base: byte)
    
    amountMultiplier: Option<i8>,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Transform; type={http://www.w3.org/2000/09/xmldsig#}TransformType; base type=; content type=mixed;
//          abstract=False; final=False; choice=True;
// Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1); XPath, string (0, 1);
pub struct ISO2TransformType {




    // Attribute: Algorithm, anyURI
    Algorithm:ArrayString<ISO2Algorithm_CHARACTER_SIZE>, 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE




     // XPath, string      
    XPath: Option<ArrayString<ISO2XPath_CHARACTER_SIZE>>, 

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}TimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}IntervalType; base type=; content type=empty;
//          abstract=True; final=False;
// Particle: 
pub struct ISO2IntervalType {
    _unused: i32,
}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Transforms; type={http://www.w3.org/2000/09/xmldsig#}TransformsType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Transform, TransformType (1, 1);
pub struct ISO2TransformsType {
    // Transform, TransformType
    
    Transform: ISO2TransformType,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}DSAKeyValueType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: P, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); Q, CryptoBinary (0, 1)(was 1, 1)(seq. ['P', 'Q']); G, CryptoBinary (0, 1); Y, CryptoBinary (1, 1); J, CryptoBinary (0, 1); Seed, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']); PgenCounter, CryptoBinary (0, 1)(was 1, 1)(seq. ['Seed', 'PgenCounter']);
pub struct ISO2DSAKeyValueType {
 
P: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE
 
Q: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE
 
G: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE




    // Y, CryptoBinary (base: base64Binary)
Y: ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE
 
J: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE
 
Seed: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE
 
PgenCounter: Option<ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerial; type={http://www.w3.org/2000/09/xmldsig#}X509IssuerSerialType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: X509IssuerName, string (1, 1); X509SerialNumber, integer (1, 1);
pub struct ISO2X509IssuerSerialType {




    // X509IssuerName, string
    X509IssuerName:ArrayString<ISO2X509IssuerName_CHARACTER_SIZE>,    // X509SerialNumber, integer (base: decimal)
    
    X509SerialNumber: i32,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeInterval; type={urn:iso:15118:2:2013:MsgDataTypes}RelativeTimeIntervalType; base type=IntervalType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: start, AnonType (1, 1); duration, AnonType (0, 1);
pub struct ISO2RelativeTimeIntervalType {
    // start, AnonType (base: unsignedInt)
    
    start: u32,
    // duration, AnonType (base: unsignedInt)
    
    duration: Option<u32>,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}DigestMethod; type={http://www.w3.org/2000/09/xmldsig#}DigestMethodType; base type=; content type=mixed;
//          abstract=False; final=False;
// Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
pub struct ISO2DigestMethodType {




    // Attribute: Algorithm, anyURI
    Algorithm:ArrayString<ISO2Algorithm_CHARACTER_SIZE>, 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RSAKeyValue; type={http://www.w3.org/2000/09/xmldsig#}RSAKeyValueType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Modulus, CryptoBinary (1, 1); Exponent, CryptoBinary (1, 1);
pub struct ISO2RSAKeyValueType {




    // Modulus, CryptoBinary (base: base64Binary)
Modulus: ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE




    // Exponent, CryptoBinary (base: base64Binary)
Exponent: ArrayVec<u8,ISO2CryptoBinary_BYTES_SIZE>,//bytes_max_len: ISO2CryptoBinary_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethod; type={http://www.w3.org/2000/09/xmldsig#}CanonicalizationMethodType; base type=; content type=mixed;
//          abstract=False; final=False;
// Particle: Algorithm, anyURI (1, 1); ANY, anyType (0, 1);
pub struct ISO2CanonicalizationMethodType {




    // Attribute: Algorithm, anyURI
    Algorithm:ArrayString<ISO2Algorithm_CHARACTER_SIZE>, 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureMethod; type={http://www.w3.org/2000/09/xmldsig#}SignatureMethodType; base type=; content type=mixed;
//          abstract=False; final=False;
// Particle: Algorithm, anyURI (1, 1); HMACOutputLength, HMACOutputLengthType (0, 1); ANY, anyType (0, 1);
pub struct ISO2SignatureMethodType {




    // Attribute: Algorithm, anyURI
    Algorithm:ArrayString<ISO2Algorithm_CHARACTER_SIZE>,    // HMACOutputLength, HMACOutputLengthType (base: integer)
    
    HMACOutputLength: Option<i32>,
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyValue; type={http://www.w3.org/2000/09/xmldsig#}KeyValueType; base type=; content type=mixed;
//          abstract=False; final=False; choice=True;
// Particle: DSAKeyValue, DSAKeyValueType (0, 1); RSAKeyValue, RSAKeyValueType (0, 1); ANY, anyType (0, 1);
pub struct ISO2KeyValueType {
    // DSAKeyValue, DSAKeyValueType
    DSAKeyValue: Option<ISO2DSAKeyValueType>,
    // RSAKeyValue, RSAKeyValueType
    RSAKeyValue: Option<ISO2RSAKeyValueType>,
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Reference; type={http://www.w3.org/2000/09/xmldsig#}ReferenceType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Id, ID (0, 1); Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1); DigestMethod, DigestMethodType (1, 1); DigestValue, DigestValueType (1, 1);
pub struct ISO2ReferenceType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




     // Attribute: Type, anyURI      
    Type: Option<ArrayString<ISO2Type_CHARACTER_SIZE>>, 




     // Attribute: URI, anyURI      
    URI: Option<ArrayString<ISO2URI_CHARACTER_SIZE>>, 
    // Transforms, TransformsType
    Transforms: Option<ISO2TransformsType>,
    // DigestMethod, DigestMethodType
    
    DigestMethod: ISO2DigestMethodType,




    // DigestValue, DigestValueType (base: base64Binary)
DigestValue: ArrayVec<u8,ISO2DigestValueType_BYTES_SIZE>,//bytes_max_len: ISO2DigestValueType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}RetrievalMethod; type={http://www.w3.org/2000/09/xmldsig#}RetrievalMethodType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Type, anyURI (0, 1); URI, anyURI (0, 1); Transforms, TransformsType (0, 1);
pub struct ISO2RetrievalMethodType {




     // Attribute: Type, anyURI      
    Type: Option<ArrayString<ISO2Type_CHARACTER_SIZE>>, 




     // Attribute: URI, anyURI      
    URI: Option<ArrayString<ISO2URI_CHARACTER_SIZE>>, 
    // Transforms, TransformsType
    Transforms: Option<ISO2TransformsType>,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}X509Data; type={http://www.w3.org/2000/09/xmldsig#}X509DataType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: X509IssuerSerial, X509IssuerSerialType (0, 1); X509SKI, base64Binary (0, 1); X509SubjectName, string (0, 1); X509Certificate, base64Binary (0, 1); X509CRL, base64Binary (0, 1); ANY, anyType (0, 1);
pub struct ISO2X509DataType {
    // X509IssuerSerial, X509IssuerSerialType
    X509IssuerSerial: Option<ISO2X509IssuerSerialType>,
 
X509SKI: Option<ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE




     // X509SubjectName, string      
    X509SubjectName: Option<ArrayString<ISO2X509SubjectName_CHARACTER_SIZE>>, 
 
X509Certificate: Option<ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
X509CRL: Option<ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}



// sequence of choice 1
struct choice_1Struct{




            // PGPKeyID, base64Binary
PGPKeyID: ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
PGPKeyPacket: Option<ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}



// sequence of choice 2
struct choice_2Struct{




            // PGPKeyPacket, base64Binary
PGPKeyPacket: ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

pub enum ISO2PGPDataType{
    choice_1(choice_1Struct),
    choice_2(choice_2Struct),
}
// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SPKIData; type={http://www.w3.org/2000/09/xmldsig#}SPKIDataType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: SPKISexp, base64Binary (1, 1); ANY, anyType (0, 1);
pub struct ISO2SPKIDataType {




    // SPKISexp, base64Binary
SPKISexp: ArrayVec<u8,ISO2base64Binary_BYTES_SIZE>,//bytes_max_len: ISO2base64Binary_BYTES_SIZE
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignedInfo; type={http://www.w3.org/2000/09/xmldsig#}SignedInfoType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Id, ID (0, 1); CanonicalizationMethod, CanonicalizationMethodType (1, 1); SignatureMethod, SignatureMethodType (1, 1); Reference, ReferenceType (1, 4);
pub struct ISO2SignedInfoType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 
    // CanonicalizationMethod, CanonicalizationMethodType
    
    CanonicalizationMethod: ISO2CanonicalizationMethodType,
    // SignatureMethod, SignatureMethodType
    
    SignatureMethod: ISO2SignatureMethodType,
    // Reference, ReferenceType
    

    
    Reference: ArrayVec<ISO2ReferenceType,ISO2ReferenceType_4_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Service; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1);
pub struct ISO2ServiceType {
    // ServiceID, serviceIDType (base: unsignedShort)
    
    ServiceID: u16,




     // ServiceName, serviceNameType (base: string)      
    ServiceName: Option<ArrayString<ISO2ServiceName_CHARACTER_SIZE>>, 
    // ServiceCategory, serviceCategoryType (base: string)
    
    ServiceCategory: ISO2serviceCategoryType,




     // ServiceScope, serviceScopeType (base: string)      
    ServiceScope: Option<ArrayString<ISO2ServiceScope_CHARACTER_SIZE>>, 
    // FreeService, boolean
    
    FreeService: bool,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SelectedService; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ServiceID, serviceIDType (1, 1); ParameterSetID, short (0, 1);
pub struct ISO2SelectedServiceType {
    // ServiceID, serviceIDType (base: unsignedShort)
    
    ServiceID: u16,
    // ParameterSetID, short (base: int)
    
    ParameterSetID: Option<i16>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); RCD, boolean (1, 1);
pub struct ISO2AC_EVSEStatusType {
    // NotificationMaxDelay, unsignedShort (base: unsignedInt)
    
    NotificationMaxDelay: u16,
    // EVSENotification, EVSENotificationType (base: string)
    
    EVSENotification: ISO2EVSENotificationType,
    // RCD, boolean
    
    RCD: bool,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); EVSEIsolationStatus, isolationLevelType (0, 1); EVSEStatusCode, DC_EVSEStatusCodeType (1, 1);
pub struct ISO2DC_EVSEStatusType {
    // NotificationMaxDelay, unsignedShort (base: unsignedInt)
    
    NotificationMaxDelay: u16,
    // EVSENotification, EVSENotificationType (base: string)
    
    EVSENotification: ISO2EVSENotificationType,
    // EVSEIsolationStatus, isolationLevelType (base: string)
    
    EVSEIsolationStatus: Option<ISO2isolationLevelType>,
    // EVSEStatusCode, DC_EVSEStatusCodeType (base: string)
    
    EVSEStatusCode: ISO2DC_EVSEStatusCodeType,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureValue; type={http://www.w3.org/2000/09/xmldsig#}SignatureValueType; base type=base64Binary; content type=simple;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (0, 1); CONTENT, SignatureValueType (1, 1);
pub struct ISO2SignatureValueType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




    // CONTENT, SignatureValueType (base: base64Binary)
CONTENT: ArrayVec<u8,ISO2SignatureValueType_BYTES_SIZE>,//bytes_max_len: ISO2SignatureValueType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SubCertificates; type={urn:iso:15118:2:2013:MsgDataTypes}SubCertificatesType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Certificate, certificateType (1, 4);
pub struct ISO2SubCertificatesType {




    // Certificate, certificateType (base: base64Binary)                  //array_max_len: ISO2certificateType_4_ARRAY_SIZE
Certificate: ArrayVec<ArrayVec<u8,ISO2certificateType_BYTES_SIZE>,ISO2certificateType_4_ARRAY_SIZE>,//bytes_max_len: ISO2certificateType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyInfo; type={http://www.w3.org/2000/09/xmldsig#}KeyInfoType; base type=; content type=mixed;
//          abstract=False; final=False; choice=True;
// Particle: Id, ID (0, 1); KeyName, string (0, 1); KeyValue, KeyValueType (0, 1); RetrievalMethod, RetrievalMethodType (0, 1); X509Data, X509DataType (0, 1); PGPData, PGPDataType (0, 1); SPKIData, SPKIDataType (0, 1); MgmtData, string (0, 1); ANY, anyType (0, 1);
pub struct ISO2KeyInfoType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




     // KeyName, string      
    KeyName: Option<ArrayString<ISO2KeyName_CHARACTER_SIZE>>, 
    // KeyValue, KeyValueType
    KeyValue: Option<ISO2KeyValueType>,
    // RetrievalMethod, RetrievalMethodType
    RetrievalMethod: Option<ISO2RetrievalMethodType>,
    // X509Data, X509DataType
    X509Data: Option<ISO2X509DataType>,
    // PGPData, PGPDataType
    PGPData: Option<ISO2PGPDataType>,
    // SPKIData, SPKIDataType
    SPKIData: Option<ISO2SPKIDataType>,




     // MgmtData, string      
    MgmtData: Option<ArrayString<ISO2MgmtData_CHARACTER_SIZE>>, 
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Object; type={http://www.w3.org/2000/09/xmldsig#}ObjectType; base type=; content type=mixed;
//          abstract=False; final=False;
// Particle: Encoding, anyURI (0, 1); Id, ID (0, 1); MimeType, string (0, 1); ANY, anyType (0, 1)(old 1, 1);
pub struct ISO2ObjectType {




     // Attribute: Encoding, anyURI      
    Encoding: Option<ArrayString<ISO2Encoding_CHARACTER_SIZE>>, 




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




     // Attribute: MimeType, string      
    MimeType: Option<ArrayString<ISO2MimeType_CHARACTER_SIZE>>, 
 
ANY: Option<ArrayVec<u8,ISO2anyType_BYTES_SIZE>>,//bytes_max_len: ISO2anyType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferMode; type={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferModeType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: EnergyTransferMode, EnergyTransferModeType (1, 6);
pub struct ISO2SupportedEnergyTransferModeType {
    // EnergyTransferMode, EnergyTransferModeType (base: string)
    

    // EnergyTransferMode, EnergyTransferModeType (base: string)
    
    EnergyTransferMode: ArrayVec<ISO2EnergyTransferModeType,ISO2EnergyTransferModeType_6_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DC_EVStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVStatusType; base type=EVStatusType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: EVReady, boolean (1, 1); EVErrorCode, DC_EVErrorCodeType (1, 1); EVRESSSOC, percentValueType (1, 1);
pub struct ISO2DC_EVStatusType {
    // EVReady, boolean
    
    EVReady: bool,
    // EVErrorCode, DC_EVErrorCodeType (base: string)
    
    EVErrorCode: ISO2DC_EVErrorCodeType,
    // EVRESSSOC, percentValueType (base: byte)
    
    EVRESSSOC: i8,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}BodyElement; type={urn:iso:15118:2:2013:MsgBody}BodyBaseType; base type=; content type=empty;
//          abstract=True; final=False;
// Particle: 
pub struct ISO2BodyBaseType {
    _unused: i32,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgHeader}Notification; type={urn:iso:15118:2:2013:MsgDataTypes}NotificationType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: FaultCode, faultCodeType (1, 1); FaultMsg, faultMsgType (0, 1);
pub struct ISO2NotificationType {
    // FaultCode, faultCodeType (base: string)
    
    FaultCode: ISO2faultCodeType,




     // FaultMsg, faultMsgType (base: string)      
    FaultMsg: Option<ArrayString<ISO2FaultMsg_CHARACTER_SIZE>>, 

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentOptionList; type={urn:iso:15118:2:2013:MsgDataTypes}PaymentOptionListType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: PaymentOption, paymentOptionType (1, 2);
pub struct ISO2PaymentOptionListType {
    // PaymentOption, paymentOptionType (base: string)
    

    // PaymentOption, paymentOptionType (base: string)
    
    PaymentOption: ArrayVec<ISO2paymentOptionType,ISO2paymentOptionType_2_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SelectedServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceListType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: SelectedService, SelectedServiceType (1, 16);
pub struct ISO2SelectedServiceListType {
    // SelectedService, SelectedServiceType
    

    
    SelectedService: ArrayVec<ISO2SelectedServiceType,ISO2SelectedServiceType_16_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}EVTargetCurrent; type={urn:iso:15118:2:2013:MsgDataTypes}PhysicalValueType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Multiplier, unitMultiplierType (1, 1); Unit, unitSymbolType (1, 1); Value, short (1, 1);
pub struct ISO2PhysicalValueType {
    // Multiplier, unitMultiplierType (base: byte)
    
    Multiplier: i8,
    // Unit, unitSymbolType (base: string)
    
    Unit: ISO2unitSymbolType,
    // Value, short (base: int)
    
    Value: i16,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCost; type={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCostType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: startValue, PhysicalValueType (1, 1); Cost, CostType (1, 3);
pub struct ISO2ConsumptionCostType {
    // startValue, PhysicalValueType
    
    startValue: ISO2PhysicalValueType,
    // Cost, CostType
    

    
    Cost: ArrayVec<ISO2CostType,ISO2CostType_3_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntry; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntryType; base type=EntryType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); PMax, PhysicalValueType (1, 1);
pub struct ISO2PMaxScheduleEntryType {
    // RelativeTimeInterval, RelativeTimeIntervalType (base: IntervalType)
    RelativeTimeInterval: Option<ISO2RelativeTimeIntervalType>,
    // TimeInterval, IntervalType
    TimeInterval: Option<ISO2IntervalType>,
    // PMax, PhysicalValueType
    
    PMax: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntry; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntryType; base type=EntryType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); EPriceLevel, unsignedByte (0, 1); ConsumptionCost, ConsumptionCostType (0, 3);
pub struct ISO2SalesTariffEntryType {
    // RelativeTimeInterval, RelativeTimeIntervalType (base: IntervalType)
    RelativeTimeInterval: Option<ISO2RelativeTimeIntervalType>,
    // TimeInterval, IntervalType
    TimeInterval: Option<ISO2IntervalType>,
    // EPriceLevel, unsignedByte (base: unsignedShort)
    
    EPriceLevel: Option<u8>,
    // ConsumptionCost, ConsumptionCostType
    

    
    ConsumptionCost: ArrayVec<ISO2ConsumptionCostType,ISO2ConsumptionCostType_3_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxSchedule; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: PMaxScheduleEntry, PMaxScheduleEntryType (1, 1024);
pub struct ISO2PMaxScheduleType {
    // PMaxScheduleEntry, PMaxScheduleEntryType (base: EntryType)
    

    
    PMaxScheduleEntry: ArrayVec<ISO2PMaxScheduleEntryType,ISO2PMaxScheduleEntryType_1024_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Parameter; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False; choice=True;
// Particle: Name, string (1, 1); boolValue, boolean (0, 1); byteValue, byte (0, 1); shortValue, short (0, 1); intValue, int (0, 1); physicalValue, PhysicalValueType (0, 1); stringValue, string (0, 1);
pub struct ISO2ParameterType {




    // Attribute: Name, string
    Name:ArrayString<ISO2Name_CHARACTER_SIZE>,    // boolValue, boolean
    
    boolValue: Option<bool>,
    // byteValue, byte (base: short)
    
    byteValue: Option<i8>,
    // shortValue, short (base: int)
    
    shortValue: Option<i16>,
    // intValue, int (base: long)
    
    intValue: Option<i32>,
    // physicalValue, PhysicalValueType
    physicalValue: Option<ISO2PhysicalValueType>,




     // stringValue, string      
    stringValue: Option<ArrayString<ISO2stringValue_CHARACTER_SIZE>>, 

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariff; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Id, ID (0, 1); SalesTariffID, SAIDType (1, 1); SalesTariffDescription, tariffDescriptionType (0, 1); NumEPriceLevels, unsignedByte (0, 1); SalesTariffEntry, SalesTariffEntryType (1, 1024);
pub struct ISO2SalesTariffType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 
    // SalesTariffID, SAIDType (base: unsignedByte)
    
    SalesTariffID: u8,




     // SalesTariffDescription, tariffDescriptionType (base: string)      
    SalesTariffDescription: Option<ArrayString<ISO2SalesTariffDescription_CHARACTER_SIZE>>, 
    // NumEPriceLevels, unsignedByte (base: unsignedShort)
    
    NumEPriceLevels: Option<u8>,
    // SalesTariffEntry, SalesTariffEntryType (base: EntryType)
    

    
    SalesTariffEntry: ArrayVec<ISO2SalesTariffEntryType,ISO2SalesTariffEntryType_1024_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTuple; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTupleType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: SAScheduleTupleID, SAIDType (1, 1); PMaxSchedule, PMaxScheduleType (1, 1); SalesTariff, SalesTariffType (0, 1);
pub struct ISO2SAScheduleTupleType {
    // SAScheduleTupleID, SAIDType (base: unsignedByte)
    
    SAScheduleTupleID: u8,
    // PMaxSchedule, PMaxScheduleType
    
    PMaxSchedule: ISO2PMaxScheduleType,
    // SalesTariff, SalesTariffType
    SalesTariff: Option<ISO2SalesTariffType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ParameterSet; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterSetType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ParameterSetID, short (1, 1); Parameter, ParameterType (1, 16);
pub struct ISO2ParameterSetType {
    // ParameterSetID, short (base: int)
    
    ParameterSetID: i16,
    // Parameter, ParameterType
    

    
    Parameter: ArrayVec<ISO2ParameterType,ISO2ParameterType_16_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntry; type={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntryType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ChargingProfileEntryStart, unsignedInt (1, 1); ChargingProfileEntryMaxPower, PhysicalValueType (1, 1); ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (0, 1);
pub struct ISO2ProfileEntryType {
    // ChargingProfileEntryStart, unsignedInt (base: unsignedLong)
    
    ChargingProfileEntryStart: u32,
    // ChargingProfileEntryMaxPower, PhysicalValueType
    
    ChargingProfileEntryMaxPower: ISO2PhysicalValueType,
    // ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (base: byte)
    
    ChargingProfileEntryMaxNumberOfPhasesInUse: Option<i8>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SAProvisioningCertificateChain; type={urn:iso:15118:2:2013:MsgDataTypes}CertificateChainType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Id, ID (0, 1); Certificate, certificateType (1, 1); SubCertificates, SubCertificatesType (0, 1);
pub struct ISO2CertificateChainType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




    // Certificate, certificateType (base: base64Binary)
Certificate: ArrayVec<u8,ISO2certificateType_BYTES_SIZE>,//bytes_max_len: ISO2certificateType_BYTES_SIZE
    // SubCertificates, SubCertificatesType
    SubCertificates: Option<ISO2SubCertificatesType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatusType; base type=; content type=ELEMENT-ONLY;
//          abstract=True; final=False;
// Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
pub struct ISO2EVSEStatusType {
    // NotificationMaxDelay, unsignedShort (base: unsignedInt)
    
    NotificationMaxDelay: u16,
    // EVSENotification, EVSENotificationType (base: string)
    
    EVSENotification: ISO2EVSENotificationType,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
    
    AC_EVSEStatus: ISO2AC_EVSEStatusType,

}

// Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Signature; type={http://www.w3.org/2000/09/xmldsig#}SignatureType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Id, ID (0, 1); SignedInfo, SignedInfoType (1, 1); SignatureValue, SignatureValueType (1, 1); KeyInfo, KeyInfoType (0, 1); Object, ObjectType (0, 1);
pub struct ISO2SignatureType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 
    // SignedInfo, SignedInfoType
    
    SignedInfo: ISO2SignedInfoType,
    // SignatureValue, SignatureValueType (base: base64Binary)
    
    SignatureValue: ISO2SignatureValueType,
    // KeyInfo, KeyInfoType
    KeyInfo: Option<ISO2KeyInfoType>,
    // Object, ObjectType
    Object: Option<ISO2ObjectType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeService; type={urn:iso:15118:2:2013:MsgDataTypes}ChargeServiceType; base type=ServiceType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1); SupportedEnergyTransferMode, SupportedEnergyTransferModeType (1, 1);
pub struct ISO2ChargeServiceType {
    // ServiceID, serviceIDType (base: unsignedShort)
    
    ServiceID: u16,




     // ServiceName, serviceNameType (base: string)      
    ServiceName: Option<ArrayString<ISO2ServiceName_CHARACTER_SIZE>>, 
    // ServiceCategory, serviceCategoryType (base: string)
    
    ServiceCategory: ISO2serviceCategoryType,




     // ServiceScope, serviceScopeType (base: string)      
    ServiceScope: Option<ArrayString<ISO2ServiceScope_CHARACTER_SIZE>>, 
    // FreeService, boolean
    
    FreeService: bool,
    // SupportedEnergyTransferMode, SupportedEnergyTransferModeType
    
    SupportedEnergyTransferMode: ISO2SupportedEnergyTransferModeType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SASchedules; type={urn:iso:15118:2:2013:MsgDataTypes}SASchedulesType; base type=; content type=empty;
//          abstract=True; final=False;
// Particle: 
pub struct ISO2SASchedulesType {
    _unused: i32,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleList; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleListType; base type=SASchedulesType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: SAScheduleTuple, SAScheduleTupleType (1, 3);
pub struct ISO2SAScheduleListType {
    // SAScheduleTuple, SAScheduleTupleType
    

    
    SAScheduleTuple: ArrayVec<ISO2SAScheduleTupleType,ISO2SAScheduleTupleType_3_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceParameterList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceParameterListType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ParameterSet, ParameterSetType (1, 255);
pub struct ISO2ServiceParameterListType {
    // ParameterSet, ParameterSetType
    

    
    ParameterSet: ArrayVec<ISO2ParameterSetType,ISO2ParameterSetType_255_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DepartureTime, unsignedInt (0, 1); EAmount, PhysicalValueType (1, 1); EVMaxVoltage, PhysicalValueType (1, 1); EVMaxCurrent, PhysicalValueType (1, 1); EVMinCurrent, PhysicalValueType (1, 1);
pub struct ISO2AC_EVChargeParameterType {
    // DepartureTime, unsignedInt (base: unsignedLong)
    
    DepartureTime: Option<u32>,
    // EAmount, PhysicalValueType
    
    EAmount: ISO2PhysicalValueType,
    // EVMaxVoltage, PhysicalValueType
    
    EVMaxVoltage: ISO2PhysicalValueType,
    // EVMaxCurrent, PhysicalValueType
    
    EVMaxCurrent: ISO2PhysicalValueType,
    // EVMinCurrent, PhysicalValueType
    
    EVMinCurrent: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DepartureTime, unsignedInt (0, 1); DC_EVStatus, DC_EVStatusType (1, 1); EVMaximumCurrentLimit, PhysicalValueType (1, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); EVMaximumVoltageLimit, PhysicalValueType (1, 1); EVEnergyCapacity, PhysicalValueType (0, 1); EVEnergyRequest, PhysicalValueType (0, 1); FullSOC, percentValueType (0, 1); BulkSOC, percentValueType (0, 1);
pub struct ISO2DC_EVChargeParameterType {
    // DepartureTime, unsignedInt (base: unsignedLong)
    
    DepartureTime: Option<u32>,
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,
    // EVMaximumCurrentLimit, PhysicalValueType
    
    EVMaximumCurrentLimit: ISO2PhysicalValueType,
    // EVMaximumPowerLimit, PhysicalValueType
    EVMaximumPowerLimit: Option<ISO2PhysicalValueType>,
    // EVMaximumVoltageLimit, PhysicalValueType
    
    EVMaximumVoltageLimit: ISO2PhysicalValueType,
    // EVEnergyCapacity, PhysicalValueType
    EVEnergyCapacity: Option<ISO2PhysicalValueType>,
    // EVEnergyRequest, PhysicalValueType
    EVEnergyRequest: Option<ISO2PhysicalValueType>,
    // FullSOC, percentValueType (base: byte)
    
    FullSOC: Option<i8>,
    // BulkSOC, percentValueType (base: byte)
    
    BulkSOC: Option<i8>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameterType; base type=; content type=ELEMENT-ONLY;
//          abstract=True; final=False;
// Particle: DepartureTime, unsignedInt (0, 1); AC_EVChargeParameter, AC_EVChargeParameterType (1, 1); DC_EVChargeParameter, DC_EVChargeParameterType (1, 1);
pub struct ISO2EVChargeParameterType {
    // DepartureTime, unsignedInt (base: unsignedLong)
    
    DepartureTime: Option<u32>,
    // AC_EVChargeParameter, AC_EVChargeParameterType (base: EVChargeParameterType)
    
    AC_EVChargeParameter: ISO2AC_EVChargeParameterType,
    // DC_EVChargeParameter, DC_EVChargeParameterType (base: EVChargeParameterType)
    
    DC_EVChargeParameter: ISO2DC_EVChargeParameterType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingProfile; type={urn:iso:15118:2:2013:MsgDataTypes}ChargingProfileType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: ProfileEntry, ProfileEntryType (1, 24);
pub struct ISO2ChargingProfileType {
    // ProfileEntry, ProfileEntryType
    

    
    ProfileEntry: ArrayVec<ISO2ProfileEntryType,ISO2ProfileEntryType_24_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ListOfRootCertificateIDs; type={urn:iso:15118:2:2013:MsgDataTypes}ListOfRootCertificateIDsType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: RootCertificateID, X509IssuerSerialType (1, 20);
pub struct ISO2ListOfRootCertificateIDsType {
    // RootCertificateID, X509IssuerSerialType
    

    
    RootCertificateID: ArrayVec<ISO2X509IssuerSerialType,ISO2X509IssuerSerialType_20_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceListType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Service, ServiceType (1, 8);
pub struct ISO2ServiceListType {
    // Service, ServiceType
    

    
    Service: ArrayVec<ISO2ServiceType,ISO2ServiceType_8_ARRAY_SIZE>,
    
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameterType; base type=; content type=empty;
//          abstract=True; final=False;
// Particle: 
pub struct ISO2EVSEChargeParameterType {
    _unused: i32,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: AC_EVSEStatus, AC_EVSEStatusType (1, 1); EVSENominalVoltage, PhysicalValueType (1, 1); EVSEMaxCurrent, PhysicalValueType (1, 1);
pub struct ISO2AC_EVSEChargeParameterType {
    // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
    
    AC_EVSEStatus: ISO2AC_EVSEStatusType,
    // EVSENominalVoltage, PhysicalValueType
    
    EVSENominalVoltage: ISO2PhysicalValueType,
    // EVSEMaxCurrent, PhysicalValueType
    
    EVSEMaxCurrent: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEMaximumCurrentLimit, PhysicalValueType (1, 1); EVSEMaximumPowerLimit, PhysicalValueType (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (1, 1); EVSEMinimumCurrentLimit, PhysicalValueType (1, 1); EVSEMinimumVoltageLimit, PhysicalValueType (1, 1); EVSECurrentRegulationTolerance, PhysicalValueType (0, 1); EVSEPeakCurrentRipple, PhysicalValueType (1, 1); EVSEEnergyToBeDelivered, PhysicalValueType (0, 1);
pub struct ISO2DC_EVSEChargeParameterType {
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // EVSEMaximumCurrentLimit, PhysicalValueType
    
    EVSEMaximumCurrentLimit: ISO2PhysicalValueType,
    // EVSEMaximumPowerLimit, PhysicalValueType
    
    EVSEMaximumPowerLimit: ISO2PhysicalValueType,
    // EVSEMaximumVoltageLimit, PhysicalValueType
    
    EVSEMaximumVoltageLimit: ISO2PhysicalValueType,
    // EVSEMinimumCurrentLimit, PhysicalValueType
    
    EVSEMinimumCurrentLimit: ISO2PhysicalValueType,
    // EVSEMinimumVoltageLimit, PhysicalValueType
    
    EVSEMinimumVoltageLimit: ISO2PhysicalValueType,
    // EVSECurrentRegulationTolerance, PhysicalValueType
    EVSECurrentRegulationTolerance: Option<ISO2PhysicalValueType>,
    // EVSEPeakCurrentRipple, PhysicalValueType
    
    EVSEPeakCurrentRipple: ISO2PhysicalValueType,
    // EVSEEnergyToBeDelivered, PhysicalValueType
    EVSEEnergyToBeDelivered: Option<ISO2PhysicalValueType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ContractSignatureEncryptedPrivateKey; type={urn:iso:15118:2:2013:MsgDataTypes}ContractSignatureEncryptedPrivateKeyType; base type=privateKeyType; content type=simple;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (1, 1); CONTENT, ContractSignatureEncryptedPrivateKeyType (1, 1);
pub struct ISO2ContractSignatureEncryptedPrivateKeyType {




    // Attribute: Id, ID (base: NCName)
    Id:ArrayString<ISO2Id_CHARACTER_SIZE>,



    // CONTENT, ContractSignatureEncryptedPrivateKeyType (base: base64Binary)
CONTENT: ArrayVec<u8,ISO2ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE>,//bytes_max_len: ISO2ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameterType; base type=; content type=empty;
//          abstract=True; final=False;
// Particle: 
pub struct ISO2EVPowerDeliveryParameterType {
    _unused: i32,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameterType; base type=EVPowerDeliveryParameterType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVStatus, DC_EVStatusType (1, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1);
pub struct ISO2DC_EVPowerDeliveryParameterType {
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,
    // BulkChargingComplete, boolean
    
    BulkChargingComplete: Option<bool>,
    // ChargingComplete, boolean
    
    ChargingComplete: bool,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DHpublickey; type={urn:iso:15118:2:2013:MsgDataTypes}DiffieHellmanPublickeyType; base type=dHpublickeyType; content type=simple;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (1, 1); CONTENT, DiffieHellmanPublickeyType (1, 1);
pub struct ISO2DiffieHellmanPublickeyType {




    // Attribute: Id, ID (base: NCName)
    Id:ArrayString<ISO2Id_CHARACTER_SIZE>,



    // CONTENT, DiffieHellmanPublickeyType (base: base64Binary)
CONTENT: ArrayVec<u8,ISO2DiffieHellmanPublickeyType_BYTES_SIZE>,//bytes_max_len: ISO2DiffieHellmanPublickeyType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeterInfo; type={urn:iso:15118:2:2013:MsgDataTypes}MeterInfoType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: MeterID, meterIDType (1, 1); MeterReading, unsignedLong (0, 1); SigMeterReading, sigMeterReadingType (0, 1); MeterStatus, meterStatusType (0, 1); TMeter, long (0, 1);
pub struct ISO2MeterInfoType {




    // MeterID, meterIDType (base: string)
    MeterID:ArrayString<ISO2MeterID_CHARACTER_SIZE>,    // MeterReading, unsignedLong (base: nonNegativeInteger)
    
    MeterReading: Option<u64>,
 
SigMeterReading: Option<ArrayVec<u8,ISO2sigMeterReadingType_BYTES_SIZE>>,//bytes_max_len: ISO2sigMeterReadingType_BYTES_SIZE
    // MeterStatus, meterStatusType (base: short)
    
    MeterStatus: Option<i16>,
    // TMeter, long (base: integer)
    
    TMeter: Option<i64>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}eMAID; type={urn:iso:15118:2:2013:MsgDataTypes}EMAIDType; base type=eMAIDType; content type=simple;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (1, 1); CONTENT, EMAIDType (1, 1);
pub struct ISO2EMAIDType {




    // Attribute: Id, ID (base: NCName)
    Id:ArrayString<ISO2Id_CHARACTER_SIZE>,



    // CONTENT, EMAIDType (base: string)
    CONTENT:ArrayString<ISO2CONTENT_CHARACTER_SIZE>,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Header; type={urn:iso:15118:2:2013:MsgHeader}MessageHeaderType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: SessionID, sessionIDType (1, 1); Notification, NotificationType (0, 1); Signature, SignatureType (0, 1);
pub struct ISO2MessageHeaderType {




    // SessionID, sessionIDType (base: hexBinary)
SessionID: ArrayVec<u8,ISO2sessionIDType_BYTES_SIZE>,//bytes_max_len: ISO2sessionIDType_BYTES_SIZE
    // Notification, NotificationType
    Notification: Option<ISO2NotificationType>,
    // Signature, SignatureType
    Signature: Option<ISO2SignatureType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupRes; type={urn:iso:15118:2:2013:MsgBody}SessionSetupResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); EVSETimeStamp, long (0, 1);
pub struct ISO2SessionSetupResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,




    // EVSEID, evseIDType (base: string)
    EVSEID:ArrayString<ISO2EVSEID_CHARACTER_SIZE>,    // EVSETimeStamp, long (base: integer)
    
    EVSETimeStamp: Option<i64>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); PaymentOptionList, PaymentOptionListType (1, 1); ChargeService, ChargeServiceType (1, 1); ServiceList, ServiceListType (0, 1);
pub struct ISO2ServiceDiscoveryResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // PaymentOptionList, PaymentOptionListType
    
    PaymentOptionList: ISO2PaymentOptionListType,
    // ChargeService, ChargeServiceType (base: ServiceType)
    
    ChargeService: ISO2ChargeServiceType,
    // ServiceList, ServiceListType
    ServiceList: Option<ISO2ServiceListType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReq; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: SelectedPaymentOption, paymentOptionType (1, 1); SelectedServiceList, SelectedServiceListType (1, 1);
pub struct ISO2PaymentServiceSelectionReqType {
    // SelectedPaymentOption, paymentOptionType (base: string)
    
    SelectedPaymentOption: ISO2paymentOptionType,
    // SelectedServiceList, SelectedServiceListType
    
    SelectedServiceList: ISO2SelectedServiceListType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionRes; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1);
pub struct ISO2PaymentServiceSelectionResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReq; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVStatus, DC_EVStatusType (1, 1);
pub struct ISO2WeldingDetectionReqType {
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandReq; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1); EVMaximumVoltageLimit, PhysicalValueType (0, 1); EVMaximumCurrentLimit, PhysicalValueType (0, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1); RemainingTimeToFullSoC, PhysicalValueType (0, 1); RemainingTimeToBulkSoC, PhysicalValueType (0, 1); EVTargetVoltage, PhysicalValueType (1, 1);
pub struct ISO2CurrentDemandReqType {
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,
    // EVTargetCurrent, PhysicalValueType
    
    EVTargetCurrent: ISO2PhysicalValueType,
    // EVMaximumVoltageLimit, PhysicalValueType
    EVMaximumVoltageLimit: Option<ISO2PhysicalValueType>,
    // EVMaximumCurrentLimit, PhysicalValueType
    EVMaximumCurrentLimit: Option<ISO2PhysicalValueType>,
    // EVMaximumPowerLimit, PhysicalValueType
    EVMaximumPowerLimit: Option<ISO2PhysicalValueType>,
    // BulkChargingComplete, boolean
    
    BulkChargingComplete: Option<bool>,
    // ChargingComplete, boolean
    
    ChargingComplete: bool,
    // RemainingTimeToFullSoC, PhysicalValueType
    RemainingTimeToFullSoC: Option<ISO2PhysicalValueType>,
    // RemainingTimeToBulkSoC, PhysicalValueType
    RemainingTimeToBulkSoC: Option<ISO2PhysicalValueType>,
    // EVTargetVoltage, PhysicalValueType
    
    EVTargetVoltage: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeReq; type={urn:iso:15118:2:2013:MsgBody}PreChargeReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetVoltage, PhysicalValueType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1);
pub struct ISO2PreChargeReqType {
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,
    // EVTargetVoltage, PhysicalValueType
    
    EVTargetVoltage: ISO2PhysicalValueType,
    // EVTargetCurrent, PhysicalValueType
    
    EVTargetCurrent: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopReq; type={urn:iso:15118:2:2013:MsgBody}SessionStopReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ChargingSession, chargingSessionType (1, 1);
pub struct ISO2SessionStopReqType {
    // ChargingSession, chargingSessionType (base: string)
    
    ChargingSession: ISO2chargingSessionType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1); SAScheduleList, SAScheduleListType (0, 1); SASchedules, SASchedulesType (0, 1); AC_EVSEChargeParameter, AC_EVSEChargeParameterType (0, 1); DC_EVSEChargeParameter, DC_EVSEChargeParameterType (0, 1); EVSEChargeParameter, EVSEChargeParameterType (0, 1);
pub struct ISO2ChargeParameterDiscoveryResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // EVSEProcessing, EVSEProcessingType (base: string)
    
    EVSEProcessing: ISO2EVSEProcessingType,
    // SAScheduleList, SAScheduleListType (base: SASchedulesType)
    SAScheduleList: Option<ISO2SAScheduleListType>,
    // SASchedules, SASchedulesType
    SASchedules: Option<ISO2SASchedulesType>,
    // AC_EVSEChargeParameter, AC_EVSEChargeParameterType (base: EVSEChargeParameterType)
    AC_EVSEChargeParameter: Option<ISO2AC_EVSEChargeParameterType>,
    // DC_EVSEChargeParameter, DC_EVSEChargeParameterType (base: EVSEChargeParameterType)
    DC_EVSEChargeParameter: Option<ISO2DC_EVSEChargeParameterType>,
    // EVSEChargeParameter, EVSEChargeParameterType
    EVSEChargeParameter: Option<ISO2EVSEChargeParameterType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ServiceScope, serviceScopeType (0, 1); ServiceCategory, serviceCategoryType (0, 1);
pub struct ISO2ServiceDiscoveryReqType {




     // ServiceScope, serviceScopeType (base: string)      
    ServiceScope: Option<ArrayString<ISO2ServiceScope_CHARACTER_SIZE>>, 
    // ServiceCategory, serviceCategoryType (base: string)
    
    ServiceCategory: Option<ISO2serviceCategoryType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusReq; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusReqType; base type=BodyBaseType; content type=empty;
//          abstract=False; final=False; derivation=extension;
// Particle: 
pub struct ISO2ChargingStatusReqType {
    _unused: i32,
}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); ServiceID, serviceIDType (1, 1); ServiceParameterList, ServiceParameterListType (0, 1);
pub struct ISO2ServiceDetailResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // ServiceID, serviceIDType (base: unsignedShort)
    
    ServiceID: u16,
    // ServiceParameterList, ServiceParameterListType
    ServiceParameterList: Option<ISO2ServiceParameterListType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationRes; type={urn:iso:15118:2:2013:MsgBody}AuthorizationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
pub struct ISO2AuthorizationResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // EVSEProcessing, EVSEProcessingType (base: string)
    
    EVSEProcessing: ISO2EVSEProcessingType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationRes; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1);
pub struct ISO2CertificateInstallationResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // SAProvisioningCertificateChain, CertificateChainType
    
    SAProvisioningCertificateChain: ISO2CertificateChainType,
    // ContractSignatureCertChain, CertificateChainType
    
    ContractSignatureCertChain: ISO2CertificateChainType,
    // ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (base: privateKeyType)
    
    ContractSignatureEncryptedPrivateKey: ISO2ContractSignatureEncryptedPrivateKeyType,
    // DHpublickey, DiffieHellmanPublickeyType (base: dHpublickeyType)
    
    DHpublickey: ISO2DiffieHellmanPublickeyType,
    // eMAID, EMAIDType (base: eMAIDType)
    
    eMAID: ISO2EMAIDType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsRes; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); GenChallenge, genChallengeType (1, 1); EVSETimeStamp, long (1, 1);
pub struct ISO2PaymentDetailsResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,




    // GenChallenge, genChallengeType (base: base64Binary)
GenChallenge: ArrayVec<u8,ISO2genChallengeType_BYTES_SIZE>,//bytes_max_len: ISO2genChallengeType_BYTES_SIZE
    // EVSETimeStamp, long (base: integer)
    
    EVSETimeStamp: i64,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReq; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: eMAID, eMAIDType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1);
pub struct ISO2PaymentDetailsReqType {




    // eMAID, eMAIDType (base: string)
    eMAID:ArrayString<ISO2eMAID_CHARACTER_SIZE>,    // ContractSignatureCertChain, CertificateChainType
    
    ContractSignatureCertChain: ISO2CertificateChainType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ServiceID, serviceIDType (1, 1);
pub struct ISO2ServiceDetailReqType {
    // ServiceID, serviceIDType (base: unsignedShort)
    
    ServiceID: u16,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusRes; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); EVSEMaxCurrent, PhysicalValueType (0, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
pub struct ISO2ChargingStatusResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,




    // EVSEID, evseIDType (base: string)
    EVSEID:ArrayString<ISO2EVSEID_CHARACTER_SIZE>,    // SAScheduleTupleID, SAIDType (base: unsignedByte)
    
    SAScheduleTupleID: u8,
    // EVSEMaxCurrent, PhysicalValueType
    EVSEMaxCurrent: Option<ISO2PhysicalValueType>,
    // MeterInfo, MeterInfoType
    MeterInfo: Option<ISO2MeterInfoType>,
    // ReceiptRequired, boolean
    
    ReceiptRequired: Option<bool>,
    // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
    
    AC_EVSEStatus: ISO2AC_EVSEStatusType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateRes; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1); RetryCounter, short (0, 1);
pub struct ISO2CertificateUpdateResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // SAProvisioningCertificateChain, CertificateChainType
    
    SAProvisioningCertificateChain: ISO2CertificateChainType,
    // ContractSignatureCertChain, CertificateChainType
    
    ContractSignatureCertChain: ISO2CertificateChainType,
    // ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (base: privateKeyType)
    
    ContractSignatureEncryptedPrivateKey: ISO2ContractSignatureEncryptedPrivateKeyType,
    // DHpublickey, DiffieHellmanPublickeyType (base: dHpublickeyType)
    
    DHpublickey: ISO2DiffieHellmanPublickeyType,
    // eMAID, EMAIDType (base: eMAIDType)
    
    eMAID: ISO2EMAIDType,
    // RetryCounter, short (base: int)
    
    RetryCounter: Option<i16>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: MaxEntriesSAScheduleTuple, unsignedShort (0, 1); RequestedEnergyTransferMode, EnergyTransferModeType (1, 1); AC_EVChargeParameter, AC_EVChargeParameterType (0, 1); DC_EVChargeParameter, DC_EVChargeParameterType (0, 1); EVChargeParameter, EVChargeParameterType (0, 1);
pub struct ISO2ChargeParameterDiscoveryReqType {
    // MaxEntriesSAScheduleTuple, unsignedShort (base: unsignedInt)
    
    MaxEntriesSAScheduleTuple: Option<u16>,
    // RequestedEnergyTransferMode, EnergyTransferModeType (base: string)
    
    RequestedEnergyTransferMode: ISO2EnergyTransferModeType,
    // AC_EVChargeParameter, AC_EVChargeParameterType (base: EVChargeParameterType)
    AC_EVChargeParameter: Option<ISO2AC_EVChargeParameterType>,
    // DC_EVChargeParameter, DC_EVChargeParameterType (base: EVChargeParameterType)
    DC_EVChargeParameter: Option<ISO2DC_EVChargeParameterType>,
    // EVChargeParameter, EVChargeParameterType
    EVChargeParameter: Option<ISO2EVChargeParameterType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReq; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ChargeProgress, chargeProgressType (1, 1); SAScheduleTupleID, SAIDType (1, 1); ChargingProfile, ChargingProfileType (0, 1); DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (0, 1); EVPowerDeliveryParameter, EVPowerDeliveryParameterType (0, 1);
pub struct ISO2PowerDeliveryReqType {
    // ChargeProgress, chargeProgressType (base: string)
    
    ChargeProgress: ISO2chargeProgressType,
    // SAScheduleTupleID, SAIDType (base: unsignedByte)
    
    SAScheduleTupleID: u8,
    // ChargingProfile, ChargingProfileType
    ChargingProfile: Option<ISO2ChargingProfileType>,
    // DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (base: EVPowerDeliveryParameterType)
    DC_EVPowerDeliveryParameter: Option<ISO2DC_EVPowerDeliveryParameterType>,
    // EVPowerDeliveryParameter, EVPowerDeliveryParameterType
    EVPowerDeliveryParameter: Option<ISO2EVPowerDeliveryParameterType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeRes; type={urn:iso:15118:2:2013:MsgBody}PreChargeResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
pub struct ISO2PreChargeResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // EVSEPresentVoltage, PhysicalValueType
    
    EVSEPresentVoltage: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationReq; type={urn:iso:15118:2:2013:MsgBody}AuthorizationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (0, 1); GenChallenge, genChallengeType (0, 1);
pub struct ISO2AuthorizationReqType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 
 
GenChallenge: Option<ArrayVec<u8,ISO2genChallengeType_BYTES_SIZE>>,//bytes_max_len: ISO2genChallengeType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryRes; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
pub struct ISO2PowerDeliveryResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
    AC_EVSEStatus: Option<ISO2AC_EVSEStatusType>,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    DC_EVSEStatus: Option<ISO2DC_EVSEStatusType>,
    // EVSEStatus, EVSEStatusType
    EVSEStatus: Option<ISO2EVSEStatusType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopRes; type={urn:iso:15118:2:2013:MsgBody}SessionStopResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1);
pub struct ISO2SessionStopResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReq; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); eMAID, eMAIDType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
pub struct ISO2CertificateUpdateReqType {




    // Attribute: Id, ID (base: NCName)
    Id:ArrayString<ISO2Id_CHARACTER_SIZE>,    // ContractSignatureCertChain, CertificateChainType
    
    ContractSignatureCertChain: ISO2CertificateChainType,




    // eMAID, eMAIDType (base: string)
    eMAID:ArrayString<ISO2eMAID_CHARACTER_SIZE>,    // ListOfRootCertificateIDs, ListOfRootCertificateIDsType
    
    ListOfRootCertificateIDs: ISO2ListOfRootCertificateIDsType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckReq; type={urn:iso:15118:2:2013:MsgBody}CableCheckReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: DC_EVStatus, DC_EVStatusType (1, 1);
pub struct ISO2CableCheckReqType {
    // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
    
    DC_EVStatus: ISO2DC_EVStatusType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReq; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (0, 1); SessionID, sessionIDType (1, 1); SAScheduleTupleID, SAIDType (0, 1); MeterInfo, MeterInfoType (1, 1);
pub struct ISO2MeteringReceiptReqType {




     // Attribute: Id, ID (base: NCName)      
    Id: Option<ArrayString<ISO2Id_CHARACTER_SIZE>>, 




    // SessionID, sessionIDType (base: hexBinary)
SessionID: ArrayVec<u8,ISO2sessionIDType_BYTES_SIZE>,//bytes_max_len: ISO2sessionIDType_BYTES_SIZE
    // SAScheduleTupleID, SAIDType (base: unsignedByte)
    
    SAScheduleTupleID: Option<u8>,
    // MeterInfo, MeterInfoType
    
    MeterInfo: ISO2MeterInfoType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionRes; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
pub struct ISO2WeldingDetectionResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // EVSEPresentVoltage, PhysicalValueType
    
    EVSEPresentVoltage: ISO2PhysicalValueType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupReq; type={urn:iso:15118:2:2013:MsgBody}SessionSetupReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: EVCCID, evccIDType (1, 1);
pub struct ISO2SessionSetupReqType {




    // EVCCID, evccIDType (base: hexBinary)
EVCCID: ArrayVec<u8,ISO2evccIDType_BYTES_SIZE>,//bytes_max_len: ISO2evccIDType_BYTES_SIZE

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandRes; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1); EVSEPresentCurrent, PhysicalValueType (1, 1); EVSECurrentLimitAchieved, boolean (1, 1); EVSEVoltageLimitAchieved, boolean (1, 1); EVSEPowerLimitAchieved, boolean (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (0, 1); EVSEMaximumCurrentLimit, PhysicalValueType (0, 1); EVSEMaximumPowerLimit, PhysicalValueType (0, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1);
pub struct ISO2CurrentDemandResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // EVSEPresentVoltage, PhysicalValueType
    
    EVSEPresentVoltage: ISO2PhysicalValueType,
    // EVSEPresentCurrent, PhysicalValueType
    
    EVSEPresentCurrent: ISO2PhysicalValueType,
    // EVSECurrentLimitAchieved, boolean
    
    EVSECurrentLimitAchieved: bool,
    // EVSEVoltageLimitAchieved, boolean
    
    EVSEVoltageLimitAchieved: bool,
    // EVSEPowerLimitAchieved, boolean
    
    EVSEPowerLimitAchieved: bool,
    // EVSEMaximumVoltageLimit, PhysicalValueType
    EVSEMaximumVoltageLimit: Option<ISO2PhysicalValueType>,
    // EVSEMaximumCurrentLimit, PhysicalValueType
    EVSEMaximumCurrentLimit: Option<ISO2PhysicalValueType>,
    // EVSEMaximumPowerLimit, PhysicalValueType
    EVSEMaximumPowerLimit: Option<ISO2PhysicalValueType>,




    // EVSEID, evseIDType (base: string)
    EVSEID:ArrayString<ISO2EVSEID_CHARACTER_SIZE>,    // SAScheduleTupleID, SAIDType (base: unsignedByte)
    
    SAScheduleTupleID: u8,
    // MeterInfo, MeterInfoType
    MeterInfo: Option<ISO2MeterInfoType>,
    // ReceiptRequired, boolean
    
    ReceiptRequired: Option<bool>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptRes; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
pub struct ISO2MeteringReceiptResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
    AC_EVSEStatus: Option<ISO2AC_EVSEStatusType>,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    DC_EVSEStatus: Option<ISO2DC_EVSEStatusType>,
    // EVSEStatus, EVSEStatusType
    EVSEStatus: Option<ISO2EVSEStatusType>,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckRes; type={urn:iso:15118:2:2013:MsgBody}CableCheckResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
pub struct ISO2CableCheckResType {
    // ResponseCode, responseCodeType (base: string)
    
    ResponseCode: ISO2responseCodeType,
    // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
    
    DC_EVSEStatus: ISO2DC_EVSEStatusType,
    // EVSEProcessing, EVSEProcessingType (base: string)
    
    EVSEProcessing: ISO2EVSEProcessingType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReq; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
//          abstract=False; final=False; derivation=extension;
// Particle: Id, ID (1, 1); OEMProvisioningCert, certificateType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
pub struct ISO2CertificateInstallationReqType {




    // Attribute: Id, ID (base: NCName)
    Id:ArrayString<ISO2Id_CHARACTER_SIZE>,



    // OEMProvisioningCert, certificateType (base: base64Binary)
OEMProvisioningCert: ArrayVec<u8,ISO2certificateType_BYTES_SIZE>,//bytes_max_len: ISO2certificateType_BYTES_SIZE
    // ListOfRootCertificateIDs, ListOfRootCertificateIDsType
    
    ListOfRootCertificateIDs: ISO2ListOfRootCertificateIDsType,

}

// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Body; type={urn:iso:15118:2:2013:MsgBody}BodyType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: AuthorizationReq, AuthorizationReqType (0, 1); AuthorizationRes, AuthorizationResType (0, 1); BodyElement, BodyBaseType (0, 1); CableCheckReq, CableCheckReqType (0, 1); CableCheckRes, CableCheckResType (0, 1); CertificateInstallationReq, CertificateInstallationReqType (0, 1); CertificateInstallationRes, CertificateInstallationResType (0, 1); CertificateUpdateReq, CertificateUpdateReqType (0, 1); CertificateUpdateRes, CertificateUpdateResType (0, 1); ChargeParameterDiscoveryReq, ChargeParameterDiscoveryReqType (0, 1); ChargeParameterDiscoveryRes, ChargeParameterDiscoveryResType (0, 1); ChargingStatusReq, ChargingStatusReqType (0, 1); ChargingStatusRes, ChargingStatusResType (0, 1); CurrentDemandReq, CurrentDemandReqType (0, 1); CurrentDemandRes, CurrentDemandResType (0, 1); MeteringReceiptReq, MeteringReceiptReqType (0, 1); MeteringReceiptRes, MeteringReceiptResType (0, 1); PaymentDetailsReq, PaymentDetailsReqType (0, 1); PaymentDetailsRes, PaymentDetailsResType (0, 1); PaymentServiceSelectionReq, PaymentServiceSelectionReqType (0, 1); PaymentServiceSelectionRes, PaymentServiceSelectionResType (0, 1); PowerDeliveryReq, PowerDeliveryReqType (0, 1); PowerDeliveryRes, PowerDeliveryResType (0, 1); PreChargeReq, PreChargeReqType (0, 1); PreChargeRes, PreChargeResType (0, 1); ServiceDetailReq, ServiceDetailReqType (0, 1); ServiceDetailRes, ServiceDetailResType (0, 1); ServiceDiscoveryReq, ServiceDiscoveryReqType (0, 1); ServiceDiscoveryRes, ServiceDiscoveryResType (0, 1); SessionSetupReq, SessionSetupReqType (0, 1); SessionSetupRes, SessionSetupResType (0, 1); SessionStopReq, SessionStopReqType (0, 1); SessionStopRes, SessionStopResType (0, 1); WeldingDetectionReq, WeldingDetectionReqType (0, 1); WeldingDetectionRes, WeldingDetectionResType (0, 1);
pub enum ISO2BodyType {
    
    AuthorizationReq(ISO2AuthorizationReqType),
    AuthorizationRes(ISO2AuthorizationResType),
    BodyElement(ISO2BodyBaseType),
    CableCheckReq(ISO2CableCheckReqType),
    CableCheckRes(ISO2CableCheckResType),
    CertificateInstallationReq(ISO2CertificateInstallationReqType),
    CertificateInstallationRes(ISO2CertificateInstallationResType),
    CertificateUpdateReq(ISO2CertificateUpdateReqType),
    CertificateUpdateRes(ISO2CertificateUpdateResType),
    ChargeParameterDiscoveryReq(ISO2ChargeParameterDiscoveryReqType),
    ChargeParameterDiscoveryRes(ISO2ChargeParameterDiscoveryResType),
    ChargingStatusReq(ISO2ChargingStatusReqType),
    ChargingStatusRes(ISO2ChargingStatusResType),
    CurrentDemandReq(ISO2CurrentDemandReqType),
    CurrentDemandRes(ISO2CurrentDemandResType),
    MeteringReceiptReq(ISO2MeteringReceiptReqType),
    MeteringReceiptRes(ISO2MeteringReceiptResType),
    PaymentDetailsReq(ISO2PaymentDetailsReqType),
    PaymentDetailsRes(ISO2PaymentDetailsResType),
    PaymentServiceSelectionReq(ISO2PaymentServiceSelectionReqType),
    PaymentServiceSelectionRes(ISO2PaymentServiceSelectionResType),
    PowerDeliveryReq(ISO2PowerDeliveryReqType),
    PowerDeliveryRes(ISO2PowerDeliveryResType),
    PreChargeReq(ISO2PreChargeReqType),
    PreChargeRes(ISO2PreChargeResType),
    ServiceDetailReq(ISO2ServiceDetailReqType),
    ServiceDetailRes(ISO2ServiceDetailResType),
    ServiceDiscoveryReq(ISO2ServiceDiscoveryReqType),
    ServiceDiscoveryRes(ISO2ServiceDiscoveryResType),
    SessionSetupReq(ISO2SessionSetupReqType),
    SessionSetupRes(ISO2SessionSetupResType),
    SessionStopReq(ISO2SessionStopReqType),
    SessionStopRes(ISO2SessionStopResType),
    WeldingDetectionReq(ISO2WeldingDetectionReqType),
    WeldingDetectionRes(ISO2WeldingDetectionResType),
    

}
// Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}V2G_Message; type=AnonymousType; base type=; content type=ELEMENT-ONLY;
//          abstract=False; final=False;
// Particle: Header, MessageHeaderType (1, 1); Body, BodyType (1, 1);
pub struct ISO2V2G_Message {
    // Header, MessageHeaderType
    
    Header: ISO2MessageHeaderType,
    // Body, BodyType
    
    Body: ISO2BodyType,

}



// root elements of EXI doc
pub struct ISO2exiDocument{
    V2G_Message: ISO2V2G_Message,
}


fn main() {
    print!("hi");
}


