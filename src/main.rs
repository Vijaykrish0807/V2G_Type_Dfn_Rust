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

use arrayvec::ArrayString;
use arrayvec::ArrayVec;
use init_fns::*;

 const EXI_STRING_MAX_LEN: usize=  1000;
  const ASCII_EXTRA_CHAR: usize= 1;

  const iso2_Algorithm_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_anyType_BYTES_SIZE: usize= 4;
  
  const iso2_XPath_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const EXI_BYTE_ARRAY_MAX_LEN: usize = 350;
const iso2_CryptoBinary_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_X509IssuerName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_Id_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_Type_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_URI_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_DigestValueType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_base64Binary_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_X509SubjectName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_ReferenceType_4_ARRAY_SIZE: usize= 4;
  
  const iso2_ServiceName_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;
  
  const iso2_ServiceScope_CHARACTER_SIZE: usize= 64 + ASCII_EXTRA_CHAR;
  
  const iso2_SignatureValueType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_certificateType_4_ARRAY_SIZE: usize= 4;
  
  const iso2_certificateType_BYTES_SIZE: usize= 800;
  
  const iso2_CostType_3_ARRAY_SIZE: usize= 3;
  
  const iso2_ConsumptionCostType_3_ARRAY_SIZE: usize= 3;
  
  const iso2_PMaxScheduleEntryType_12_ARRAY_SIZE: usize= 12;
  
  const iso2_Name_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_stringValue_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_SalesTariffDescription_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;
  
  const iso2_SalesTariffEntryType_12_ARRAY_SIZE: usize= 12;
  
  const iso2_ParameterType_16_ARRAY_SIZE: usize= 16;
  
  const iso2_KeyName_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_MgmtData_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_Encoding_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_MimeType_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_EnergyTransferModeType_6_ARRAY_SIZE: usize= 6;
  
  const iso2_FaultMsg_CHARACTER_SIZE: usize= 64 + ASCII_EXTRA_CHAR;
  
  const iso2_paymentOptionType_2_ARRAY_SIZE: usize= 2;
  
  const iso2_SelectedServiceType_16_ARRAY_SIZE: usize= 16;
  
  const iso2_X509IssuerSerialType_5_ARRAY_SIZE: usize= 5;
  
  const iso2_ProfileEntryType_24_ARRAY_SIZE: usize= 24;
  
  const iso2_SAScheduleTupleType_3_ARRAY_SIZE: usize= 3;
  
  const iso2_ParameterSetType_5_ARRAY_SIZE: usize= 5;
  
  const iso2_ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_ServiceType_8_ARRAY_SIZE: usize= 8;
  
  const iso2_DiffieHellmanPublickeyType_BYTES_SIZE: usize= EXI_BYTE_ARRAY_MAX_LEN;
  
  const iso2_MeterID_CHARACTER_SIZE: usize= 32 + ASCII_EXTRA_CHAR;
  
  const iso2_sigMeterReadingType_BYTES_SIZE: usize= 64;
  
  const iso2_CONTENT_CHARACTER_SIZE: usize= EXI_STRING_MAX_LEN + ASCII_EXTRA_CHAR;
  
  const iso2_sessionIDType_BYTES_SIZE: usize= 8;
  
  const iso2_genChallengeType_BYTES_SIZE: usize= 16;
  
  const iso2_eMAID_CHARACTER_SIZE: usize= 15 + ASCII_EXTRA_CHAR;
  
  const iso2_EVSEID_CHARACTER_SIZE: usize= 37 + ASCII_EXTRA_CHAR;
  
  const iso2_evccIDType_BYTES_SIZE: usize= 6;
  
  
  // enum for function numbers
  pub enum iso2_generatedFunctionNumbersType{
      iso2_AC_EVChargeParameter,
      iso2_AC_EVSEChargeParameter,
      iso2_AC_EVSEStatus,
      iso2_AuthorizationReq,
      iso2_AuthorizationRes,
      iso2_BodyElement,
      iso2_CableCheckReq,
      iso2_CableCheckRes,
      iso2_CanonicalizationMethod,
      iso2_CertificateInstallationReq,
      iso2_CertificateInstallationRes,
      iso2_CertificateUpdateReq,
      iso2_CertificateUpdateRes,
      iso2_ChargeParameterDiscoveryReq,
      iso2_ChargeParameterDiscoveryRes,
      iso2_ChargingStatusReq,
      iso2_ChargingStatusRes,
      iso2_CurrentDemandReq,
      iso2_CurrentDemandRes,
      iso2_DC_EVChargeParameter,
      iso2_DC_EVPowerDeliveryParameter,
      iso2_DC_EVSEChargeParameter,
      iso2_DC_EVSEStatus,
      iso2_DC_EVStatus,
      iso2_DSAKeyValue,
      iso2_DigestMethod,
      iso2_DigestValue,
      iso2_EVChargeParameter,
      iso2_EVPowerDeliveryParameter,
      iso2_EVSEChargeParameter,
      iso2_EVSEStatus,
      iso2_EVStatus,
      iso2_Entry,
      iso2_KeyInfo,
      iso2_KeyName,
      iso2_KeyValue,
      iso2_Manifest,
      iso2_MeteringReceiptReq,
      iso2_MeteringReceiptRes,
      iso2_MgmtData,
      iso2_Object,
      iso2_PGPData,
      iso2_PMaxScheduleEntry,
      iso2_PaymentDetailsReq,
      iso2_PaymentDetailsRes,
      iso2_PaymentServiceSelectionReq,
      iso2_PaymentServiceSelectionRes,
      iso2_PowerDeliveryReq,
      iso2_PowerDeliveryRes,
      iso2_PreChargeReq,
      iso2_PreChargeRes,
      iso2_RSAKeyValue,
      iso2_Reference,
      iso2_RelativeTimeInterval,
      iso2_RetrievalMethod,
      iso2_SAScheduleList,
      iso2_SASchedules,
      iso2_SPKIData,
      iso2_SalesTariffEntry,
      iso2_ServiceDetailReq,
      iso2_ServiceDetailRes,
      iso2_ServiceDiscoveryReq,
      iso2_ServiceDiscoveryRes,
      iso2_SessionSetupReq,
      iso2_SessionSetupRes,
      iso2_SessionStopReq,
      iso2_SessionStopRes,
      iso2_Signature,
      iso2_SignatureMethod,
      iso2_SignatureProperties,
      iso2_SignatureProperty,
      iso2_SignatureValue,
      iso2_SignedInfo,
      iso2_TimeInterval,
      iso2_Transform,
      iso2_Transforms,
      iso2_V2G_Message,
      iso2_WeldingDetectionReq,
      iso2_WeldingDetectionRes,
      iso2_X509Data,
  }
  
   
  //ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  
  // sequence of choice 1
  struct choice_1Struct{
  
  
  
  
              // PGPKeyID, base64Binary
  PGPKeyID: ArrayVec<u8,iso2_base64Binary_BYTES_SIZE>,//bytes_max_len: iso2_base64Binary_BYTES_SIZE
   
  PGPKeyPacket: Option<ArrayVec<u8,iso2_base64Binary_BYTES_SIZE>>,//bytes_max_len: iso2_base64Binary_BYTES_SIZE
   
  ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  }
  
  
  
  // sequence of choice 2
  struct choice_2Struct{
  
  
  
  
              // PGPKeyPacket, base64Binary
  PGPKeyPacket: ArrayVec<u8,iso2_base64Binary_BYTES_SIZE>,//bytes_max_len: iso2_base64Binary_BYTES_SIZE
   
  ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  }
  
  pub enum iso2_PGPDataType{
      choice_1(choice_1Struct),
      choice_2(choice_2Struct),
  }
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SPKIData; type={http://www.w3.org/2000/09/xmldsig#}SPKIDataType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SPKISexp, base64Binary (1, 1); ANY, anyType (0, 1);
  pub struct iso2_SPKIDataType {
  
  
  
  
      // SPKISexp, base64Binary
  SPKISexp: ArrayVec<u8,iso2_base64Binary_BYTES_SIZE>,//bytes_max_len: iso2_base64Binary_BYTES_SIZE
   
  ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignedInfo; type={http://www.w3.org/2000/09/xmldsig#}SignedInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); CanonicalizationMethod, CanonicalizationMethodType (1, 1); SignatureMethod, SignatureMethodType (1, 1); Reference, ReferenceType (1, 4);
  pub struct iso2_SignedInfoType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
      // CanonicalizationMethod, CanonicalizationMethodType
      
      CanonicalizationMethod: iso2_CanonicalizationMethodType,
      // SignatureMethod, SignatureMethodType
      
      SignatureMethod: iso2_SignatureMethodType,
      // Reference, ReferenceType
      
  
      Reference: [iso2_ReferenceType;iso2_ReferenceType_4_ARRAY_SIZE],
      ReferenceArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Service; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1);
  pub struct iso2_ServiceType {
      // ServiceID, serviceIDType (base: unsignedShort)
      
      ServiceID: u16,
  
  
  
  
       // ServiceName, serviceNameType (base: string)      
      ServiceName: Option<ArrayString<iso2_ServiceName_CHARACTER_SIZE>>, 
      // ServiceCategory, serviceCategoryType (base: string)
      
      ServiceCategory: iso2_serviceCategoryType,
  
  
  
  
       // ServiceScope, serviceScopeType (base: string)      
      ServiceScope: Option<ArrayString<iso2_ServiceScope_CHARACTER_SIZE>>, 
      // FreeService, boolean
      
      FreeService: bool,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SelectedService; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ServiceID, serviceIDType (1, 1); ParameterSetID, short (0, 1);
  pub struct iso2_SelectedServiceType {
      // ServiceID, serviceIDType (base: unsignedShort)
      
      ServiceID: u16,
      // ParameterSetID, short (base: int)
      
      ParameterSetID: Option<i16>,
  
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}SignatureValue; type={http://www.w3.org/2000/09/xmldsig#}SignatureValueType; base type=base64Binary; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); CONTENT, SignatureValueType (1, 1);
  pub struct iso2_SignatureValueType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
  
  
  
  
      // CONTENT, SignatureValueType (base: base64Binary)
  CONTENT: ArrayVec<u8,iso2_SignatureValueType_BYTES_SIZE>,//bytes_max_len: iso2_SignatureValueType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SubCertificates; type={urn:iso:15118:2:2013:MsgDataTypes}SubCertificatesType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Certificate, certificateType (1, 4);
  pub struct iso2_SubCertificatesType {
  
  
  
  
      // Certificate, certificateType (base: base64Binary)                  //array_max_len: iso2_certificateType_4_ARRAY_SIZE
  Certificate: ArrayVec<ArrayVec<u8,iso2_certificateType_BYTES_SIZE>,iso2_certificateType_4_ARRAY_SIZE>,//bytes_max_len: iso2_certificateType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EAmount; type={urn:iso:15118:2:2013:MsgDataTypes}PhysicalValueType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Multiplier, unitMultiplierType (1, 1); Unit, unitSymbolType (1, 1); Value, short (1, 1);
  pub struct iso2_PhysicalValueType {
      // Multiplier, unitMultiplierType (base: byte)
      
      Multiplier: i8,
      // Unit, unitSymbolType (base: string)
      
      Unit: iso2_unitSymbolType,
      // Value, short (base: int)
      
      Value: i16,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCost; type={urn:iso:15118:2:2013:MsgDataTypes}ConsumptionCostType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: startValue, PhysicalValueType (1, 1); Cost, CostType (1, 3);
  pub struct iso2_ConsumptionCostType {
      // startValue, PhysicalValueType
      
      startValue: iso2_PhysicalValueType,
      // Cost, CostType
      
  
      Cost: [iso2_CostType;iso2_CostType_3_ARRAY_SIZE],
      CostArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntry; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); PMax, PhysicalValueType (1, 1);
  pub struct iso2_PMaxScheduleEntryType {
      // RelativeTimeInterval, RelativeTimeIntervalType (base: IntervalType)
      RelativeTimeInterval: Option<iso2_RelativeTimeIntervalType>,
      // TimeInterval, IntervalType
      TimeInterval: Option<iso2_IntervalType>,
      // PMax, PhysicalValueType
      
      PMax: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntry; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffEntryType; base type=EntryType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: RelativeTimeInterval, RelativeTimeIntervalType (0, 1); TimeInterval, IntervalType (0, 1); EPriceLevel, unsignedByte (0, 1); ConsumptionCost, ConsumptionCostType (0, 3);
  pub struct iso2_SalesTariffEntryType {
      // RelativeTimeInterval, RelativeTimeIntervalType (base: IntervalType)
      RelativeTimeInterval: Option<iso2_RelativeTimeIntervalType>,
      // TimeInterval, IntervalType
      TimeInterval: Option<iso2_IntervalType>,
      // EPriceLevel, unsignedByte (base: unsignedShort)
      
      EPriceLevel: Option<u8>,
      // ConsumptionCost, ConsumptionCostType
      
  
      ConsumptionCost: [iso2_ConsumptionCostType;iso2_ConsumptionCostType_3_ARRAY_SIZE],
      ConsumptionCostArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}PMaxSchedule; type={urn:iso:15118:2:2013:MsgDataTypes}PMaxScheduleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PMaxScheduleEntry, PMaxScheduleEntryType (1, 12);
  pub struct iso2_PMaxScheduleType {
      // PMaxScheduleEntry, PMaxScheduleEntryType (base: EntryType)
      
  
      PMaxScheduleEntry: [iso2_PMaxScheduleEntryType;iso2_PMaxScheduleEntryType_12_ARRAY_SIZE],
      PMaxScheduleEntryArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}Parameter; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; choice=True;
  // Particle: Name, string (1, 1); boolValue, boolean (0, 1); byteValue, byte (0, 1); shortValue, short (0, 1); intValue, int (0, 1); physicalValue, PhysicalValueType (0, 1); stringValue, string (0, 1);
  pub struct iso2_ParameterType {
  
  
  
  
      // Attribute: Name, string
      Name:ArrayString<iso2_Name_CHARACTER_SIZE>,    // boolValue, boolean
      
      boolValue: Option<bool>,
      // byteValue, byte (base: short)
      
      byteValue: Option<i8>,
      // shortValue, short (base: int)
      
      shortValue: Option<i16>,
      // intValue, int (base: long)
      
      intValue: Option<i32>,
      // physicalValue, PhysicalValueType
      physicalValue: Option<iso2_PhysicalValueType>,
  
  
  
  
       // stringValue, string      
      stringValue: Option<ArrayString<iso2_stringValue_CHARACTER_SIZE>>, 
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SalesTariff; type={urn:iso:15118:2:2013:MsgDataTypes}SalesTariffType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SalesTariffID, SAIDType (1, 1); SalesTariffDescription, tariffDescriptionType (0, 1); NumEPriceLevels, unsignedByte (0, 1); SalesTariffEntry, SalesTariffEntryType (1, 12);
  pub struct iso2_SalesTariffType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
      // SalesTariffID, SAIDType (base: unsignedByte)
      
      SalesTariffID: u8,
  
  
  
  
       // SalesTariffDescription, tariffDescriptionType (base: string)      
      SalesTariffDescription: Option<ArrayString<iso2_SalesTariffDescription_CHARACTER_SIZE>>, 
      // NumEPriceLevels, unsignedByte (base: unsignedShort)
      
      NumEPriceLevels: Option<u8>,
      // SalesTariffEntry, SalesTariffEntryType (base: EntryType)
      
  
      SalesTariffEntry: [iso2_SalesTariffEntryType;iso2_SalesTariffEntryType_12_ARRAY_SIZE],
      SalesTariffEntryArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntry; type={urn:iso:15118:2:2013:MsgDataTypes}ProfileEntryType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ChargingProfileEntryStart, unsignedInt (1, 1); ChargingProfileEntryMaxPower, PhysicalValueType (1, 1); ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (0, 1);
  pub struct iso2_ProfileEntryType {
      // ChargingProfileEntryStart, unsignedInt (base: unsignedLong)
      
      ChargingProfileEntryStart: u32,
      // ChargingProfileEntryMaxPower, PhysicalValueType
      
      ChargingProfileEntryMaxPower: iso2_PhysicalValueType,
      // ChargingProfileEntryMaxNumberOfPhasesInUse, maxNumPhasesType (base: byte)
      
      ChargingProfileEntryMaxNumberOfPhasesInUse: Option<i8>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTuple; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleTupleType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SAScheduleTupleID, SAIDType (1, 1); PMaxSchedule, PMaxScheduleType (1, 1); SalesTariff, SalesTariffType (0, 1);
  pub struct iso2_SAScheduleTupleType {
      // SAScheduleTupleID, SAIDType (base: unsignedByte)
      
      SAScheduleTupleID: u8,
      // PMaxSchedule, PMaxScheduleType
      
      PMaxSchedule: iso2_PMaxScheduleType,
      // SalesTariff, SalesTariffType
      SalesTariff: Option<iso2_SalesTariffType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}ParameterSet; type={urn:iso:15118:2:2013:MsgDataTypes}ParameterSetType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSetID, short (1, 1); Parameter, ParameterType (1, 16);
  pub struct iso2_ParameterSetType {
      // ParameterSetID, short (base: int)
      
      ParameterSetID: i16,
      // Parameter, ParameterType
      
  
      Parameter: [iso2_ParameterType;iso2_ParameterType_16_ARRAY_SIZE],
      ParameterArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVStatusType; base type=EVStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVReady, boolean (1, 1); EVErrorCode, DC_EVErrorCodeType (1, 1); EVRESSSOC, percentValueType (1, 1);
  pub struct iso2_DC_EVStatusType {
      // EVReady, boolean
      
      EVReady: bool,
      // EVErrorCode, DC_EVErrorCodeType (base: string)
      
      EVErrorCode: iso2_DC_EVErrorCodeType,
      // EVRESSSOC, percentValueType (base: byte)
      
      EVRESSSOC: i8,
  
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}KeyInfo; type={http://www.w3.org/2000/09/xmldsig#}KeyInfoType; base type=; content type=mixed;
  //          abstract=False; final=False; choice=True;
  // Particle: Id, ID (0, 1); KeyName, string (0, 1); KeyValue, KeyValueType (0, 1); RetrievalMethod, RetrievalMethodType (0, 1); X509Data, X509DataType (0, 1); PGPData, PGPDataType (0, 1); SPKIData, SPKIDataType (0, 1); MgmtData, string (0, 1); ANY, anyType (0, 1);
  pub struct iso2_KeyInfoType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
  
  
  
  
       // KeyName, string      
      KeyName: Option<ArrayString<iso2_KeyName_CHARACTER_SIZE>>, 
      // KeyValue, KeyValueType
      KeyValue: Option<iso2_KeyValueType>,
      // RetrievalMethod, RetrievalMethodType
      RetrievalMethod: Option<iso2_RetrievalMethodType>,
      // X509Data, X509DataType
      X509Data: Option<iso2_X509DataType>,
      // PGPData, PGPDataType
      PGPData: Option<iso2_PGPDataType>,
      // SPKIData, SPKIDataType
      SPKIData: Option<iso2_SPKIDataType>,
  
  
  
  
       // MgmtData, string      
      MgmtData: Option<ArrayString<iso2_MgmtData_CHARACTER_SIZE>>, 
   
  ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Object; type={http://www.w3.org/2000/09/xmldsig#}ObjectType; base type=; content type=mixed;
  //          abstract=False; final=False;
  // Particle: Encoding, anyURI (0, 1); Id, ID (0, 1); MimeType, string (0, 1); ANY, anyType (0, 1)(old 1, 1);
  pub struct iso2_ObjectType {
  
  
  
  
       // Attribute: Encoding, anyURI      
      Encoding: Option<ArrayString<iso2_Encoding_CHARACTER_SIZE>>, 
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
  
  
  
  
       // Attribute: MimeType, string      
      MimeType: Option<ArrayString<iso2_MimeType_CHARACTER_SIZE>>, 
   
  ANY: Option<ArrayVec<u8,iso2_anyType_BYTES_SIZE>>,//bytes_max_len: iso2_anyType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferMode; type={urn:iso:15118:2:2013:MsgDataTypes}SupportedEnergyTransferModeType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: EnergyTransferMode, EnergyTransferModeType (1, 6);
  pub struct iso2_SupportedEnergyTransferModeType {
      
  
      // EnergyTransferMode, EnergyTransferModeType (base: string)
      EnergyTransferMode: [iso2_EnergyTransferModeType;iso2_EnergyTransferModeType_6_ARRAY_SIZE],
      EnergyTransferModeArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}BodyElement; type={urn:iso:15118:2:2013:MsgBody}BodyBaseType; base type=; content type=empty;
  //          abstract=True; final=False;
  // Particle: 
  pub struct iso2_BodyBaseType {
      _unused: i32,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgHeader}Notification; type={urn:iso:15118:2:2013:MsgDataTypes}NotificationType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: FaultCode, faultCodeType (1, 1); FaultMsg, faultMsgType (0, 1);
  pub struct iso2_NotificationType {
      // FaultCode, faultCodeType (base: string)
      
      FaultCode: iso2_faultCodeType,
  
  
  
  
       // FaultMsg, faultMsgType (base: string)      
      FaultMsg: Option<ArrayString<iso2_FaultMsg_CHARACTER_SIZE>>, 
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); RCD, boolean (1, 1);
  pub struct iso2_AC_EVSEStatusType {
      // NotificationMaxDelay, unsignedShort (base: unsignedInt)
      
      NotificationMaxDelay: u16,
      // EVSENotification, EVSENotificationType (base: string)
      
      EVSENotification: iso2_EVSENotificationType,
      // RCD, boolean
      
      RCD: bool,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEStatusType; base type=EVSEStatusType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); EVSEIsolationStatus, isolationLevelType (0, 1); EVSEStatusCode, DC_EVSEStatusCodeType (1, 1);
  pub struct iso2_DC_EVSEStatusType {
      // NotificationMaxDelay, unsignedShort (base: unsignedInt)
      
      NotificationMaxDelay: u16,
      // EVSENotification, EVSENotificationType (base: string)
      
      EVSENotification: iso2_EVSENotificationType,
      // EVSEIsolationStatus, isolationLevelType (base: string)
      
      EVSEIsolationStatus: Option<iso2_isolationLevelType>,
      // EVSEStatusCode, DC_EVSEStatusCodeType (base: string)
      
      EVSEStatusCode: iso2_DC_EVSEStatusCodeType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatus; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEStatusType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: NotificationMaxDelay, unsignedShort (1, 1); EVSENotification, EVSENotificationType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1);
  pub struct iso2_EVSEStatusType {
      // NotificationMaxDelay, unsignedShort (base: unsignedInt)
      
      NotificationMaxDelay: u16,
      // EVSENotification, EVSENotificationType (base: string)
      
      EVSENotification: iso2_EVSENotificationType,
      // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
      
      AC_EVSEStatus: iso2_AC_EVSEStatusType,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SAProvisioningCertificateChain; type={urn:iso:15118:2:2013:MsgDataTypes}CertificateChainType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); Certificate, certificateType (1, 1); SubCertificates, SubCertificatesType (0, 1);
  pub struct iso2_CertificateChainType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
  
  
  
  
      // Certificate, certificateType (base: base64Binary)
  Certificate: ArrayVec<u8,iso2_certificateType_BYTES_SIZE>,//bytes_max_len: iso2_certificateType_BYTES_SIZE
      // SubCertificates, SubCertificatesType
      SubCertificates: Option<iso2_SubCertificatesType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentOptionList; type={urn:iso:15118:2:2013:MsgDataTypes}PaymentOptionListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: PaymentOption, paymentOptionType (1, 2);
  pub struct iso2_PaymentOptionListType {
      
  
      // PaymentOption, paymentOptionType (base: string)
      PaymentOption: [iso2_paymentOptionType;iso2_paymentOptionType_2_ARRAY_SIZE],
      PaymentOptionArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SelectedServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}SelectedServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SelectedService, SelectedServiceType (1, 16);
  pub struct iso2_SelectedServiceListType {
      // SelectedService, SelectedServiceType
      
  
      SelectedService: [iso2_SelectedServiceType;iso2_SelectedServiceType_16_ARRAY_SIZE],
      SelectedServiceArrayLen: u16,
  }
  
  // Element: definition=complex; name={http://www.w3.org/2000/09/xmldsig#}Signature; type={http://www.w3.org/2000/09/xmldsig#}SignatureType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Id, ID (0, 1); SignedInfo, SignedInfoType (1, 1); SignatureValue, SignatureValueType (1, 1); KeyInfo, KeyInfoType (0, 1); Object, ObjectType (0, 1);
  pub struct iso2_SignatureType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
      // SignedInfo, SignedInfoType
      
      SignedInfo: iso2_SignedInfoType,
      // SignatureValue, SignatureValueType (base: base64Binary)
      
      SignatureValue: iso2_SignatureValueType,
      // KeyInfo, KeyInfoType
      KeyInfo: Option<iso2_KeyInfoType>,
      // Object, ObjectType
      Object: Option<iso2_ObjectType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); EAmount, PhysicalValueType (1, 1); EVMaxVoltage, PhysicalValueType (1, 1); EVMaxCurrent, PhysicalValueType (1, 1); EVMinCurrent, PhysicalValueType (1, 1);
  pub struct iso2_AC_EVChargeParameterType {
      // DepartureTime, unsignedInt (base: unsignedLong)
      
      DepartureTime: Option<u32>,
      // EAmount, PhysicalValueType
      
      EAmount: iso2_PhysicalValueType,
      // EVMaxVoltage, PhysicalValueType
      
      EVMaxVoltage: iso2_PhysicalValueType,
      // EVMaxCurrent, PhysicalValueType
      
      EVMaxCurrent: iso2_PhysicalValueType,
      // EVMinCurrent, PhysicalValueType
      
      EVMinCurrent: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVChargeParameterType; base type=EVChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DepartureTime, unsignedInt (0, 1); DC_EVStatus, DC_EVStatusType (1, 1); EVMaximumCurrentLimit, PhysicalValueType (1, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); EVMaximumVoltageLimit, PhysicalValueType (1, 1); EVEnergyCapacity, PhysicalValueType (0, 1); EVEnergyRequest, PhysicalValueType (0, 1); FullSOC, percentValueType (0, 1); BulkSOC, percentValueType (0, 1);
  pub struct iso2_DC_EVChargeParameterType {
      // DepartureTime, unsignedInt (base: unsignedLong)
      
      DepartureTime: Option<u32>,
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
      // EVMaximumCurrentLimit, PhysicalValueType
      
      EVMaximumCurrentLimit: iso2_PhysicalValueType,
      // EVMaximumPowerLimit, PhysicalValueType
      EVMaximumPowerLimit: Option<iso2_PhysicalValueType>,
      // EVMaximumVoltageLimit, PhysicalValueType
      
      EVMaximumVoltageLimit: iso2_PhysicalValueType,
      // EVEnergyCapacity, PhysicalValueType
      EVEnergyCapacity: Option<iso2_PhysicalValueType>,
      // EVEnergyRequest, PhysicalValueType
      EVEnergyRequest: Option<iso2_PhysicalValueType>,
      // FullSOC, percentValueType (base: byte)
      
      FullSOC: Option<i8>,
      // BulkSOC, percentValueType (base: byte)
      
      BulkSOC: Option<i8>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVChargeParameterType; base type=; content type=ELEMENT-ONLY;
  //          abstract=True; final=False;
  // Particle: DepartureTime, unsignedInt (0, 1); AC_EVChargeParameter, AC_EVChargeParameterType (1, 1); DC_EVChargeParameter, DC_EVChargeParameterType (1, 1);
  pub struct iso2_EVChargeParameterType {
      // DepartureTime, unsignedInt (base: unsignedLong)
      
      DepartureTime: Option<u32>,
      // AC_EVChargeParameter, AC_EVChargeParameterType (base: EVChargeParameterType)
      
      AC_EVChargeParameter: iso2_AC_EVChargeParameterType,
      // DC_EVChargeParameter, DC_EVChargeParameterType (base: EVChargeParameterType)
      
      DC_EVChargeParameter: iso2_DC_EVChargeParameterType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeService; type={urn:iso:15118:2:2013:MsgDataTypes}ChargeServiceType; base type=ServiceType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1); ServiceName, serviceNameType (0, 1); ServiceCategory, serviceCategoryType (1, 1); ServiceScope, serviceScopeType (0, 1); FreeService, boolean (1, 1); SupportedEnergyTransferMode, SupportedEnergyTransferModeType (1, 1);
  pub struct iso2_ChargeServiceType {
      // ServiceID, serviceIDType (base: unsignedShort)
      
      ServiceID: u16,
  
  
  
  
       // ServiceName, serviceNameType (base: string)      
      ServiceName: Option<ArrayString<iso2_ServiceName_CHARACTER_SIZE>>, 
      // ServiceCategory, serviceCategoryType (base: string)
      
      ServiceCategory: iso2_serviceCategoryType,
  
  
  
  
       // ServiceScope, serviceScopeType (base: string)      
      ServiceScope: Option<ArrayString<iso2_ServiceScope_CHARACTER_SIZE>>, 
      // FreeService, boolean
      
      FreeService: bool,
      // SupportedEnergyTransferMode, SupportedEnergyTransferModeType
      
      SupportedEnergyTransferMode: iso2_SupportedEnergyTransferModeType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ListOfRootCertificateIDs; type={urn:iso:15118:2:2013:MsgDataTypes}ListOfRootCertificateIDsType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: RootCertificateID, X509IssuerSerialType (1, 5);
  pub struct iso2_ListOfRootCertificateIDsType {
      // RootCertificateID, X509IssuerSerialType
      
  
      RootCertificateID: [iso2_X509IssuerSerialType;iso2_X509IssuerSerialType_5_ARRAY_SIZE],
      RootCertificateIDArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingProfile; type={urn:iso:15118:2:2013:MsgDataTypes}ChargingProfileType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ProfileEntry, ProfileEntryType (1, 24);
  pub struct iso2_ChargingProfileType {
      // ProfileEntry, ProfileEntryType
      
  
      ProfileEntry: [iso2_ProfileEntryType;iso2_ProfileEntryType_24_ARRAY_SIZE],
      ProfileEntryArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SASchedules; type={urn:iso:15118:2:2013:MsgDataTypes}SASchedulesType; base type=; content type=empty;
  //          abstract=True; final=False;
  // Particle: 
  pub struct iso2_SASchedulesType {
      _unused: i32,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleList; type={urn:iso:15118:2:2013:MsgDataTypes}SAScheduleListType; base type=SASchedulesType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: SAScheduleTuple, SAScheduleTupleType (1, 3);
  pub struct iso2_SAScheduleListType {
      // SAScheduleTuple, SAScheduleTupleType
      
  
      SAScheduleTuple: [iso2_SAScheduleTupleType;iso2_SAScheduleTupleType_3_ARRAY_SIZE],
      SAScheduleTupleArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceParameterList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceParameterListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: ParameterSet, ParameterSetType (1, 5);
  pub struct iso2_ServiceParameterListType {
      // ParameterSet, ParameterSetType
      
  
      ParameterSet: [iso2_ParameterSetType;iso2_ParameterSetType_5_ARRAY_SIZE],
      ParameterSetArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ContractSignatureEncryptedPrivateKey; type={urn:iso:15118:2:2013:MsgDataTypes}ContractSignatureEncryptedPrivateKeyType; base type=privateKeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, ContractSignatureEncryptedPrivateKeyType (1, 1);
  pub struct iso2_ContractSignatureEncryptedPrivateKeyType {
  
  
  
  
      // Attribute: Id, ID (base: NCName)
      Id:ArrayString<iso2_Id_CHARACTER_SIZE>,
  
  
  
      // CONTENT, ContractSignatureEncryptedPrivateKeyType (base: base64Binary)
  CONTENT: ArrayVec<u8,iso2_ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE>,//bytes_max_len: iso2_ContractSignatureEncryptedPrivateKeyType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceList; type={urn:iso:15118:2:2013:MsgDataTypes}ServiceListType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Service, ServiceType (1, 8);
  pub struct iso2_ServiceListType {
      // Service, ServiceType
      
  
      Service: [iso2_ServiceType;iso2_ServiceType_8_ARRAY_SIZE],
      ServiceArrayLen: u16,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVPowerDeliveryParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  // Particle: 
  pub struct iso2_EVPowerDeliveryParameterType {
      _unused: i32,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVPowerDeliveryParameterType; base type=EVPowerDeliveryParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1);
  pub struct iso2_DC_EVPowerDeliveryParameterType {
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
      // BulkChargingComplete, boolean
      
      BulkChargingComplete: Option<bool>,
      // ChargingComplete, boolean
      
      ChargingComplete: bool,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}EVSEChargeParameterType; base type=; content type=empty;
  //          abstract=True; final=False;
  // Particle: 
  pub struct iso2_EVSEChargeParameterType {
      _unused: i32,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}AC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: AC_EVSEStatus, AC_EVSEStatusType (1, 1); EVSENominalVoltage, PhysicalValueType (1, 1); EVSEMaxCurrent, PhysicalValueType (1, 1);
  pub struct iso2_AC_EVSEChargeParameterType {
      // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
      
      AC_EVSEStatus: iso2_AC_EVSEStatusType,
      // EVSENominalVoltage, PhysicalValueType
      
      EVSENominalVoltage: iso2_PhysicalValueType,
      // EVSEMaxCurrent, PhysicalValueType
      
      EVSEMaxCurrent: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameter; type={urn:iso:15118:2:2013:MsgDataTypes}DC_EVSEChargeParameterType; base type=EVSEChargeParameterType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEMaximumCurrentLimit, PhysicalValueType (1, 1); EVSEMaximumPowerLimit, PhysicalValueType (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (1, 1); EVSEMinimumCurrentLimit, PhysicalValueType (1, 1); EVSEMinimumVoltageLimit, PhysicalValueType (1, 1); EVSECurrentRegulationTolerance, PhysicalValueType (0, 1); EVSEPeakCurrentRipple, PhysicalValueType (1, 1); EVSEEnergyToBeDelivered, PhysicalValueType (0, 1);
  pub struct iso2_DC_EVSEChargeParameterType {
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
      // EVSEMaximumCurrentLimit, PhysicalValueType
      
      EVSEMaximumCurrentLimit: iso2_PhysicalValueType,
      // EVSEMaximumPowerLimit, PhysicalValueType
      
      EVSEMaximumPowerLimit: iso2_PhysicalValueType,
      // EVSEMaximumVoltageLimit, PhysicalValueType
      
      EVSEMaximumVoltageLimit: iso2_PhysicalValueType,
      // EVSEMinimumCurrentLimit, PhysicalValueType
      
      EVSEMinimumCurrentLimit: iso2_PhysicalValueType,
      // EVSEMinimumVoltageLimit, PhysicalValueType
      
      EVSEMinimumVoltageLimit: iso2_PhysicalValueType,
      // EVSECurrentRegulationTolerance, PhysicalValueType
      EVSECurrentRegulationTolerance: Option<iso2_PhysicalValueType>,
      // EVSEPeakCurrentRipple, PhysicalValueType
      
      EVSEPeakCurrentRipple: iso2_PhysicalValueType,
      // EVSEEnergyToBeDelivered, PhysicalValueType
      EVSEEnergyToBeDelivered: Option<iso2_PhysicalValueType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}DHpublickey; type={urn:iso:15118:2:2013:MsgDataTypes}DiffieHellmanPublickeyType; base type=dHpublickeyType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, DiffieHellmanPublickeyType (1, 1);
  pub struct iso2_DiffieHellmanPublickeyType {
  
  
  
  
      // Attribute: Id, ID (base: NCName)
      Id:ArrayString<iso2_Id_CHARACTER_SIZE>,
  
  
  
      // CONTENT, DiffieHellmanPublickeyType (base: base64Binary)
  CONTENT: ArrayVec<u8,iso2_DiffieHellmanPublickeyType_BYTES_SIZE>,//bytes_max_len: iso2_DiffieHellmanPublickeyType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeterInfo; type={urn:iso:15118:2:2013:MsgDataTypes}MeterInfoType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: MeterID, meterIDType (1, 1); MeterReading, unsignedLong (0, 1); SigMeterReading, sigMeterReadingType (0, 1); MeterStatus, meterStatusType (0, 1); TMeter, long (0, 1);
  pub struct iso2_MeterInfoType {
  
  
  
  
      // MeterID, meterIDType (base: string)
      MeterID:ArrayString<iso2_MeterID_CHARACTER_SIZE>,    // MeterReading, unsignedLong (base: nonNegativeInteger)
      
      MeterReading: Option<u64>,
   
  SigMeterReading: Option<ArrayVec<u8,iso2_sigMeterReadingType_BYTES_SIZE>>,//bytes_max_len: iso2_sigMeterReadingType_BYTES_SIZE
      // MeterStatus, meterStatusType (base: short)
      
      MeterStatus: Option<i16>,
      // TMeter, long (base: integer)
      
      TMeter: Option<i64>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}eMAID; type={urn:iso:15118:2:2013:MsgDataTypes}EMAIDType; base type=eMAIDType; content type=simple;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); CONTENT, EMAIDType (1, 1);
  pub struct iso2_EMAIDType {
  
  
  
  
      // Attribute: Id, ID (base: NCName)
      Id:ArrayString<iso2_Id_CHARACTER_SIZE>,
  
  
  
      // CONTENT, EMAIDType (base: string)
      CONTENT:ArrayString<iso2_CONTENT_CHARACTER_SIZE>,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Header; type={urn:iso:15118:2:2013:MsgHeader}MessageHeaderType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: SessionID, sessionIDType (1, 1); Notification, NotificationType (0, 1); Signature, SignatureType (0, 1);
  pub struct iso2_MessageHeaderType {
  
  
  
  
      // SessionID, sessionIDType (base: hexBinary)
  SessionID: ArrayVec<u8,iso2_sessionIDType_BYTES_SIZE>,//bytes_max_len: iso2_sessionIDType_BYTES_SIZE
      // Notification, NotificationType
      Notification: Option<iso2_NotificationType>,
      // Signature, SignatureType
      Signature: Option<iso2_SignatureType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptRes; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub struct iso2_MeteringReceiptResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
      AC_EVSEStatus: Option<iso2_AC_EVSEStatusType>,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      DC_EVSEStatus: Option<iso2_DC_EVSEStatusType>,
      // EVSEStatus, EVSEStatusType
      EVSEStatus: Option<iso2_EVSEStatusType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateRes; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1); RetryCounter, short (0, 1);
  pub struct iso2_CertificateUpdateResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // SAProvisioningCertificateChain, CertificateChainType
      
      SAProvisioningCertificateChain: iso2_CertificateChainType,
      // ContractSignatureCertChain, CertificateChainType
      
      ContractSignatureCertChain: iso2_CertificateChainType,
      // ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (base: privateKeyType)
      
      ContractSignatureEncryptedPrivateKey: iso2_ContractSignatureEncryptedPrivateKeyType,
      // DHpublickey, DiffieHellmanPublickeyType (base: dHpublickeyType)
      
      DHpublickey: iso2_DiffieHellmanPublickeyType,
      // eMAID, EMAIDType (base: eMAIDType)
      
      eMAID: iso2_EMAIDType,
      // RetryCounter, short (base: int)
      
      RetryCounter: Option<i16>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: MaxEntriesSAScheduleTuple, unsignedShort (0, 1); RequestedEnergyTransferMode, EnergyTransferModeType (1, 1); AC_EVChargeParameter, AC_EVChargeParameterType (0, 1); DC_EVChargeParameter, DC_EVChargeParameterType (0, 1); EVChargeParameter, EVChargeParameterType (0, 1);
  pub struct iso2_ChargeParameterDiscoveryReqType {
      // MaxEntriesSAScheduleTuple, unsignedShort (base: unsignedInt)
      
      MaxEntriesSAScheduleTuple: Option<u16>,
      // RequestedEnergyTransferMode, EnergyTransferModeType (base: string)
      
      RequestedEnergyTransferMode: iso2_EnergyTransferModeType,
      // AC_EVChargeParameter, AC_EVChargeParameterType (base: EVChargeParameterType)
      AC_EVChargeParameter: Option<iso2_AC_EVChargeParameterType>,
      // DC_EVChargeParameter, DC_EVChargeParameterType (base: EVChargeParameterType)
      DC_EVChargeParameter: Option<iso2_DC_EVChargeParameterType>,
      // EVChargeParameter, EVChargeParameterType
      EVChargeParameter: Option<iso2_EVChargeParameterType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceID, serviceIDType (1, 1);
  pub struct iso2_ServiceDetailReqType {
      // ServiceID, serviceIDType (base: unsignedShort)
      
      ServiceID: u16,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationReq; type={urn:iso:15118:2:2013:MsgBody}AuthorizationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); GenChallenge, genChallengeType (0, 1);
  pub struct iso2_AuthorizationReqType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
   
  GenChallenge: Option<ArrayVec<u8,iso2_genChallengeType_BYTES_SIZE>>,//bytes_max_len: iso2_genChallengeType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReq; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: eMAID, eMAIDType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1);
  pub struct iso2_PaymentDetailsReqType {
  
  
  
  
      // eMAID, eMAIDType (base: string)
      eMAID:ArrayString<iso2_eMAID_CHARACTER_SIZE>,    // ContractSignatureCertChain, CertificateChainType
      
      ContractSignatureCertChain: iso2_CertificateChainType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionRes; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub struct iso2_PaymentServiceSelectionResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); PaymentOptionList, PaymentOptionListType (1, 1); ChargeService, ChargeServiceType (1, 1); ServiceList, ServiceListType (0, 1);
  pub struct iso2_ServiceDiscoveryResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // PaymentOptionList, PaymentOptionListType
      
      PaymentOptionList: iso2_PaymentOptionListType,
      // ChargeService, ChargeServiceType (base: ServiceType)
      
      ChargeService: iso2_ChargeServiceType,
      // ServiceList, ServiceListType
      ServiceList: Option<iso2_ServiceListType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReq; type={urn:iso:15118:2:2013:MsgBody}CertificateUpdateReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); eMAID, eMAIDType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub struct iso2_CertificateUpdateReqType {
  
  
  
  
      // Attribute: Id, ID (base: NCName)
      Id:ArrayString<iso2_Id_CHARACTER_SIZE>,    // ContractSignatureCertChain, CertificateChainType
      
      ContractSignatureCertChain: iso2_CertificateChainType,
  
  
  
  
      // eMAID, eMAIDType (base: string)
      eMAID:ArrayString<iso2_eMAID_CHARACTER_SIZE>,    // ListOfRootCertificateIDs, ListOfRootCertificateIDsType
      
      ListOfRootCertificateIDs: iso2_ListOfRootCertificateIDsType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopRes; type={urn:iso:15118:2:2013:MsgBody}SessionStopResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1);
  pub struct iso2_SessionStopResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusReq; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusReqType; base type=BodyBaseType; content type=empty;
  //          abstract=False; final=False; derivation=extension;
  // Particle: 
  pub struct iso2_ChargingStatusReqType {
      _unused: i32,
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargingStatusRes; type={urn:iso:15118:2:2013:MsgBody}ChargingStatusResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); EVSEMaxCurrent, PhysicalValueType (0, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1); AC_EVSEStatus, AC_EVSEStatusType (1, 1);
  pub struct iso2_ChargingStatusResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
  
  
  
  
      // EVSEID, evseIDType (base: string)
      EVSEID:ArrayString<iso2_EVSEID_CHARACTER_SIZE>,    // SAScheduleTupleID, SAIDType (base: unsignedByte)
      
      SAScheduleTupleID: u8,
      // EVSEMaxCurrent, PhysicalValueType
      EVSEMaxCurrent: Option<iso2_PhysicalValueType>,
      // MeterInfo, MeterInfoType
      MeterInfo: Option<iso2_MeterInfoType>,
      // ReceiptRequired, boolean
      
      ReceiptRequired: Option<bool>,
      // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
      
      AC_EVSEStatus: iso2_AC_EVSEStatusType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReq; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargeProgress, chargeProgressType (1, 1); SAScheduleTupleID, SAIDType (1, 1); ChargingProfile, ChargingProfileType (0, 1); DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (0, 1); EVPowerDeliveryParameter, EVPowerDeliveryParameterType (0, 1);
  pub struct iso2_PowerDeliveryReqType {
      // ChargeProgress, chargeProgressType (base: string)
      
      ChargeProgress: iso2_chargeProgressType,
      // SAScheduleTupleID, SAIDType (base: unsignedByte)
      
      SAScheduleTupleID: u8,
      // ChargingProfile, ChargingProfileType
      ChargingProfile: Option<iso2_ChargingProfileType>,
      // DC_EVPowerDeliveryParameter, DC_EVPowerDeliveryParameterType (base: EVPowerDeliveryParameterType)
      DC_EVPowerDeliveryParameter: Option<iso2_DC_EVPowerDeliveryParameterType>,
      // EVPowerDeliveryParameter, EVPowerDeliveryParameterType
      EVPowerDeliveryParameter: Option<iso2_EVPowerDeliveryParameterType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionStopReq; type={urn:iso:15118:2:2013:MsgBody}SessionStopReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ChargingSession, chargingSessionType (1, 1);
  pub struct iso2_SessionStopReqType {
      // ChargingSession, chargingSessionType (base: string)
      
      ChargingSession: iso2_chargingSessionType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentDetailsRes; type={urn:iso:15118:2:2013:MsgBody}PaymentDetailsResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); GenChallenge, genChallengeType (1, 1); EVSETimeStamp, long (1, 1);
  pub struct iso2_PaymentDetailsResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
  
  
  
  
      // GenChallenge, genChallengeType (base: base64Binary)
  GenChallenge: ArrayVec<u8,iso2_genChallengeType_BYTES_SIZE>,//bytes_max_len: iso2_genChallengeType_BYTES_SIZE
      // EVSETimeStamp, long (base: integer)
      
      EVSETimeStamp: i64,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReq; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (1, 1); OEMProvisioningCert, certificateType (1, 1); ListOfRootCertificateIDs, ListOfRootCertificateIDsType (1, 1);
  pub struct iso2_CertificateInstallationReqType {
  
  
  
  
      // Attribute: Id, ID (base: NCName)
      Id:ArrayString<iso2_Id_CHARACTER_SIZE>,
  
  
  
      // OEMProvisioningCert, certificateType (base: base64Binary)
  OEMProvisioningCert: ArrayVec<u8,iso2_certificateType_BYTES_SIZE>,//bytes_max_len: iso2_certificateType_BYTES_SIZE
      // ListOfRootCertificateIDs, ListOfRootCertificateIDsType
      
      ListOfRootCertificateIDs: iso2_ListOfRootCertificateIDsType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryRes; type={urn:iso:15118:2:2013:MsgBody}ChargeParameterDiscoveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1); SAScheduleList, SAScheduleListType (0, 1); SASchedules, SASchedulesType (0, 1); AC_EVSEChargeParameter, AC_EVSEChargeParameterType (0, 1); DC_EVSEChargeParameter, DC_EVSEChargeParameterType (0, 1); EVSEChargeParameter, EVSEChargeParameterType (0, 1);
  pub struct iso2_ChargeParameterDiscoveryResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // EVSEProcessing, EVSEProcessingType (base: string)
      
      EVSEProcessing: iso2_EVSEProcessingType,
      // SAScheduleList, SAScheduleListType (base: SASchedulesType)
      SAScheduleList: Option<iso2_SAScheduleListType>,
      // SASchedules, SASchedulesType
      SASchedules: Option<iso2_SASchedulesType>,
      // AC_EVSEChargeParameter, AC_EVSEChargeParameterType (base: EVSEChargeParameterType)
      AC_EVSEChargeParameter: Option<iso2_AC_EVSEChargeParameterType>,
      // DC_EVSEChargeParameter, DC_EVSEChargeParameterType (base: EVSEChargeParameterType)
      DC_EVSEChargeParameter: Option<iso2_DC_EVSEChargeParameterType>,
      // EVSEChargeParameter, EVSEChargeParameterType
      EVSEChargeParameter: Option<iso2_EVSEChargeParameterType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CertificateInstallationRes; type={urn:iso:15118:2:2013:MsgBody}CertificateInstallationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); SAProvisioningCertificateChain, CertificateChainType (1, 1); ContractSignatureCertChain, CertificateChainType (1, 1); ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (1, 1); DHpublickey, DiffieHellmanPublickeyType (1, 1); eMAID, EMAIDType (1, 1);
  pub struct iso2_CertificateInstallationResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // SAProvisioningCertificateChain, CertificateChainType
      
      SAProvisioningCertificateChain: iso2_CertificateChainType,
      // ContractSignatureCertChain, CertificateChainType
      
      ContractSignatureCertChain: iso2_CertificateChainType,
      // ContractSignatureEncryptedPrivateKey, ContractSignatureEncryptedPrivateKeyType (base: privateKeyType)
      
      ContractSignatureEncryptedPrivateKey: iso2_ContractSignatureEncryptedPrivateKeyType,
      // DHpublickey, DiffieHellmanPublickeyType (base: dHpublickeyType)
      
      DHpublickey: iso2_DiffieHellmanPublickeyType,
      // eMAID, EMAIDType (base: eMAIDType)
      
      eMAID: iso2_EMAIDType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}AuthorizationRes; type={urn:iso:15118:2:2013:MsgBody}AuthorizationResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub struct iso2_AuthorizationResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // EVSEProcessing, EVSEProcessingType (base: string)
      
      EVSEProcessing: iso2_EVSEProcessingType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckReq; type={urn:iso:15118:2:2013:MsgBody}CableCheckReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub struct iso2_CableCheckReqType {
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeReq; type={urn:iso:15118:2:2013:MsgBody}PreChargeReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetVoltage, PhysicalValueType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1);
  pub struct iso2_PreChargeReqType {
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
      // EVTargetVoltage, PhysicalValueType
      
      EVTargetVoltage: iso2_PhysicalValueType,
      // EVTargetCurrent, PhysicalValueType
      
      EVTargetCurrent: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PowerDeliveryRes; type={urn:iso:15118:2:2013:MsgBody}PowerDeliveryResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); AC_EVSEStatus, AC_EVSEStatusType (0, 1); DC_EVSEStatus, DC_EVSEStatusType (0, 1); EVSEStatus, EVSEStatusType (0, 1);
  pub struct iso2_PowerDeliveryResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // AC_EVSEStatus, AC_EVSEStatusType (base: EVSEStatusType)
      AC_EVSEStatus: Option<iso2_AC_EVSEStatusType>,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      DC_EVSEStatus: Option<iso2_DC_EVSEStatusType>,
      // EVSEStatus, EVSEStatusType
      EVSEStatus: Option<iso2_EVSEStatusType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupRes; type={urn:iso:15118:2:2013:MsgBody}SessionSetupResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); EVSEID, evseIDType (1, 1); EVSETimeStamp, long (0, 1);
  pub struct iso2_SessionSetupResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
  
  
  
  
      // EVSEID, evseIDType (base: string)
      EVSEID:ArrayString<iso2_EVSEID_CHARACTER_SIZE>,    // EVSETimeStamp, long (base: integer)
      
      EVSETimeStamp: Option<i64>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CableCheckRes; type={urn:iso:15118:2:2013:MsgBody}CableCheckResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEProcessing, EVSEProcessingType (1, 1);
  pub struct iso2_CableCheckResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
      // EVSEProcessing, EVSEProcessingType (base: string)
      
      EVSEProcessing: iso2_EVSEProcessingType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReq; type={urn:iso:15118:2:2013:MsgBody}MeteringReceiptReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: Id, ID (0, 1); SessionID, sessionIDType (1, 1); SAScheduleTupleID, SAIDType (0, 1); MeterInfo, MeterInfoType (1, 1);
  pub struct iso2_MeteringReceiptReqType {
  
  
  
  
       // Attribute: Id, ID (base: NCName)      
      Id: Option<ArrayString<iso2_Id_CHARACTER_SIZE>>, 
  
  
  
  
      // SessionID, sessionIDType (base: hexBinary)
  SessionID: ArrayVec<u8,iso2_sessionIDType_BYTES_SIZE>,//bytes_max_len: iso2_sessionIDType_BYTES_SIZE
      // SAScheduleTupleID, SAIDType (base: unsignedByte)
      
      SAScheduleTupleID: Option<u8>,
      // MeterInfo, MeterInfoType
      
      MeterInfo: iso2_MeterInfoType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PreChargeRes; type={urn:iso:15118:2:2013:MsgBody}PreChargeResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub struct iso2_PreChargeResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
      // EVSEPresentVoltage, PhysicalValueType
      
      EVSEPresentVoltage: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandRes; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1); EVSEPresentCurrent, PhysicalValueType (1, 1); EVSECurrentLimitAchieved, boolean (1, 1); EVSEVoltageLimitAchieved, boolean (1, 1); EVSEPowerLimitAchieved, boolean (1, 1); EVSEMaximumVoltageLimit, PhysicalValueType (0, 1); EVSEMaximumCurrentLimit, PhysicalValueType (0, 1); EVSEMaximumPowerLimit, PhysicalValueType (0, 1); EVSEID, evseIDType (1, 1); SAScheduleTupleID, SAIDType (1, 1); MeterInfo, MeterInfoType (0, 1); ReceiptRequired, boolean (0, 1);
  pub struct iso2_CurrentDemandResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
      // EVSEPresentVoltage, PhysicalValueType
      
      EVSEPresentVoltage: iso2_PhysicalValueType,
      // EVSEPresentCurrent, PhysicalValueType
      
      EVSEPresentCurrent: iso2_PhysicalValueType,
      // EVSECurrentLimitAchieved, boolean
      
      EVSECurrentLimitAchieved: bool,
      // EVSEVoltageLimitAchieved, boolean
      
      EVSEVoltageLimitAchieved: bool,
      // EVSEPowerLimitAchieved, boolean
      
      EVSEPowerLimitAchieved: bool,
      // EVSEMaximumVoltageLimit, PhysicalValueType
      EVSEMaximumVoltageLimit: Option<iso2_PhysicalValueType>,
      // EVSEMaximumCurrentLimit, PhysicalValueType
      EVSEMaximumCurrentLimit: Option<iso2_PhysicalValueType>,
      // EVSEMaximumPowerLimit, PhysicalValueType
      EVSEMaximumPowerLimit: Option<iso2_PhysicalValueType>,
  
  
  
  
      // EVSEID, evseIDType (base: string)
      EVSEID:ArrayString<iso2_EVSEID_CHARACTER_SIZE>,    // SAScheduleTupleID, SAIDType (base: unsignedByte)
      
      SAScheduleTupleID: u8,
      // MeterInfo, MeterInfoType
      MeterInfo: Option<iso2_MeterInfoType>,
      // ReceiptRequired, boolean
      
      ReceiptRequired: Option<bool>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}SessionSetupReq; type={urn:iso:15118:2:2013:MsgBody}SessionSetupReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: EVCCID, evccIDType (1, 1);
  pub struct iso2_SessionSetupReqType {
  
  
  
  
      // EVCCID, evccIDType (base: hexBinary)
  EVCCID: ArrayVec<u8,iso2_evccIDType_BYTES_SIZE>,//bytes_max_len: iso2_evccIDType_BYTES_SIZE
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionRes; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); DC_EVSEStatus, DC_EVSEStatusType (1, 1); EVSEPresentVoltage, PhysicalValueType (1, 1);
  pub struct iso2_WeldingDetectionResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // DC_EVSEStatus, DC_EVSEStatusType (base: EVSEStatusType)
      
      DC_EVSEStatus: iso2_DC_EVSEStatusType,
      // EVSEPresentVoltage, PhysicalValueType
      
      EVSEPresentVoltage: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReq; type={urn:iso:15118:2:2013:MsgBody}ServiceDiscoveryReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ServiceScope, serviceScopeType (0, 1); ServiceCategory, serviceCategoryType (0, 1);
  pub struct iso2_ServiceDiscoveryReqType {
  
  
  
  
       // ServiceScope, serviceScopeType (base: string)      
      ServiceScope: Option<ArrayString<iso2_ServiceScope_CHARACTER_SIZE>>, 
      // ServiceCategory, serviceCategoryType (base: string)
      
      ServiceCategory: Option<iso2_serviceCategoryType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}CurrentDemandReq; type={urn:iso:15118:2:2013:MsgBody}CurrentDemandReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1); EVTargetCurrent, PhysicalValueType (1, 1); EVMaximumVoltageLimit, PhysicalValueType (0, 1); EVMaximumCurrentLimit, PhysicalValueType (0, 1); EVMaximumPowerLimit, PhysicalValueType (0, 1); BulkChargingComplete, boolean (0, 1); ChargingComplete, boolean (1, 1); RemainingTimeToFullSoC, PhysicalValueType (0, 1); RemainingTimeToBulkSoC, PhysicalValueType (0, 1); EVTargetVoltage, PhysicalValueType (1, 1);
  pub struct iso2_CurrentDemandReqType {
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
      // EVTargetCurrent, PhysicalValueType
      
      EVTargetCurrent: iso2_PhysicalValueType,
      // EVMaximumVoltageLimit, PhysicalValueType
      EVMaximumVoltageLimit: Option<iso2_PhysicalValueType>,
      // EVMaximumCurrentLimit, PhysicalValueType
      EVMaximumCurrentLimit: Option<iso2_PhysicalValueType>,
      // EVMaximumPowerLimit, PhysicalValueType
      EVMaximumPowerLimit: Option<iso2_PhysicalValueType>,
      // BulkChargingComplete, boolean
      
      BulkChargingComplete: Option<bool>,
      // ChargingComplete, boolean
      
      ChargingComplete: bool,
      // RemainingTimeToFullSoC, PhysicalValueType
      RemainingTimeToFullSoC: Option<iso2_PhysicalValueType>,
      // RemainingTimeToBulkSoC, PhysicalValueType
      RemainingTimeToBulkSoC: Option<iso2_PhysicalValueType>,
      // EVTargetVoltage, PhysicalValueType
      
      EVTargetVoltage: iso2_PhysicalValueType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReq; type={urn:iso:15118:2:2013:MsgBody}PaymentServiceSelectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: SelectedPaymentOption, paymentOptionType (1, 1); SelectedServiceList, SelectedServiceListType (1, 1);
  pub struct iso2_PaymentServiceSelectionReqType {
      // SelectedPaymentOption, paymentOptionType (base: string)
      
      SelectedPaymentOption: iso2_paymentOptionType,
      // SelectedServiceList, SelectedServiceListType
      
      SelectedServiceList: iso2_SelectedServiceListType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReq; type={urn:iso:15118:2:2013:MsgBody}WeldingDetectionReqType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: DC_EVStatus, DC_EVStatusType (1, 1);
  pub struct iso2_WeldingDetectionReqType {
      // DC_EVStatus, DC_EVStatusType (base: EVStatusType)
      
      DC_EVStatus: iso2_DC_EVStatusType,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgBody}ServiceDetailRes; type={urn:iso:15118:2:2013:MsgBody}ServiceDetailResType; base type=BodyBaseType; content type=ELEMENT-ONLY;
  //          abstract=False; final=False; derivation=extension;
  // Particle: ResponseCode, responseCodeType (1, 1); ServiceID, serviceIDType (1, 1); ServiceParameterList, ServiceParameterListType (0, 1);
  pub struct iso2_ServiceDetailResType {
      // ResponseCode, responseCodeType (base: string)
      
      ResponseCode: iso2_responseCodeType,
      // ServiceID, serviceIDType (base: unsignedShort)
      
      ServiceID: u16,
      // ServiceParameterList, ServiceParameterListType
      ServiceParameterList: Option<iso2_ServiceParameterListType>,
  
  }
  
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}Body; type={urn:iso:15118:2:2013:MsgBody}BodyType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: AuthorizationReq, AuthorizationReqType (0, 1); AuthorizationRes, AuthorizationResType (0, 1); BodyElement, BodyBaseType (0, 1); CableCheckReq, CableCheckReqType (0, 1); CableCheckRes, CableCheckResType (0, 1); CertificateInstallationReq, CertificateInstallationReqType (0, 1); CertificateInstallationRes, CertificateInstallationResType (0, 1); CertificateUpdateReq, CertificateUpdateReqType (0, 1); CertificateUpdateRes, CertificateUpdateResType (0, 1); ChargeParameterDiscoveryReq, ChargeParameterDiscoveryReqType (0, 1); ChargeParameterDiscoveryRes, ChargeParameterDiscoveryResType (0, 1); ChargingStatusReq, ChargingStatusReqType (0, 1); ChargingStatusRes, ChargingStatusResType (0, 1); CurrentDemandReq, CurrentDemandReqType (0, 1); CurrentDemandRes, CurrentDemandResType (0, 1); MeteringReceiptReq, MeteringReceiptReqType (0, 1); MeteringReceiptRes, MeteringReceiptResType (0, 1); PaymentDetailsReq, PaymentDetailsReqType (0, 1); PaymentDetailsRes, PaymentDetailsResType (0, 1); PaymentServiceSelectionReq, PaymentServiceSelectionReqType (0, 1); PaymentServiceSelectionRes, PaymentServiceSelectionResType (0, 1); PowerDeliveryReq, PowerDeliveryReqType (0, 1); PowerDeliveryRes, PowerDeliveryResType (0, 1); PreChargeReq, PreChargeReqType (0, 1); PreChargeRes, PreChargeResType (0, 1); ServiceDetailReq, ServiceDetailReqType (0, 1); ServiceDetailRes, ServiceDetailResType (0, 1); ServiceDiscoveryReq, ServiceDiscoveryReqType (0, 1); ServiceDiscoveryRes, ServiceDiscoveryResType (0, 1); SessionSetupReq, SessionSetupReqType (0, 1); SessionSetupRes, SessionSetupResType (0, 1); SessionStopReq, SessionStopReqType (0, 1); SessionStopRes, SessionStopResType (0, 1); WeldingDetectionReq, WeldingDetectionReqType (0, 1); WeldingDetectionRes, WeldingDetectionResType (0, 1);
  pub enum iso2_BodyType {
      
      AuthorizationReq(iso2_AuthorizationReqType),
      AuthorizationRes(iso2_AuthorizationResType),
      BodyElement(iso2_BodyBaseType),
      CableCheckReq(iso2_CableCheckReqType),
      CableCheckRes(iso2_CableCheckResType),
      CertificateInstallationReq(iso2_CertificateInstallationReqType),
      CertificateInstallationRes(iso2_CertificateInstallationResType),
      CertificateUpdateReq(iso2_CertificateUpdateReqType),
      CertificateUpdateRes(iso2_CertificateUpdateResType),
      ChargeParameterDiscoveryReq(iso2_ChargeParameterDiscoveryReqType),
      ChargeParameterDiscoveryRes(iso2_ChargeParameterDiscoveryResType),
      ChargingStatusReq(iso2_ChargingStatusReqType),
      ChargingStatusRes(iso2_ChargingStatusResType),
      CurrentDemandReq(iso2_CurrentDemandReqType),
      CurrentDemandRes(iso2_CurrentDemandResType),
      MeteringReceiptReq(iso2_MeteringReceiptReqType),
      MeteringReceiptRes(iso2_MeteringReceiptResType),
      PaymentDetailsReq(iso2_PaymentDetailsReqType),
      PaymentDetailsRes(iso2_PaymentDetailsResType),
      PaymentServiceSelectionReq(iso2_PaymentServiceSelectionReqType),
      PaymentServiceSelectionRes(iso2_PaymentServiceSelectionResType),
      PowerDeliveryReq(iso2_PowerDeliveryReqType),
      PowerDeliveryRes(iso2_PowerDeliveryResType),
      PreChargeReq(iso2_PreChargeReqType),
      PreChargeRes(iso2_PreChargeResType),
      ServiceDetailReq(iso2_ServiceDetailReqType),
      ServiceDetailRes(iso2_ServiceDetailResType),
      ServiceDiscoveryReq(iso2_ServiceDiscoveryReqType),
      ServiceDiscoveryRes(iso2_ServiceDiscoveryResType),
      SessionSetupReq(iso2_SessionSetupReqType),
      SessionSetupRes(iso2_SessionSetupResType),
      SessionStopReq(iso2_SessionStopReqType),
      SessionStopRes(iso2_SessionStopResType),
      WeldingDetectionReq(iso2_WeldingDetectionReqType),
      WeldingDetectionRes(iso2_WeldingDetectionResType),
      
  
  }
  // Element: definition=complex; name={urn:iso:15118:2:2013:MsgDef}V2G_Message; type=AnonymousType; base type=; content type=ELEMENT-ONLY;
  //          abstract=False; final=False;
  // Particle: Header, MessageHeaderType (1, 1); Body, BodyType (1, 1);
  pub struct iso2_V2G_Message {
      // Header, MessageHeaderType
      
      Header: iso2_MessageHeaderType,
      // Body, BodyType
      
      Body: iso2_BodyType,
  
  }
  
  
  
  // root elements of EXI doc
  pub struct iso2_exiDocument{
      V2G_Message: iso2_V2G_Message,
  }
  
  // elements of EXI fragment
  
  
  enum iso2_exiFragmentEnum{
      AuthorizationReq(iso2_AuthorizationReqType),
      CertificateInstallationReq(iso2_CertificateInstallationReqType),
      CertificateInstallationRes(iso2_CertificateInstallationResType),
      CertificateUpdateReq(iso2_CertificateUpdateReqType),
      CertificateUpdateRes(iso2_CertificateUpdateResType),
      ChargeParameterDiscoveryRes(iso2_ChargeParameterDiscoveryResType),
      MeteringReceiptReq(iso2_MeteringReceiptReqType),
      SignedInfo(iso2_SignedInfoType),
      
  }
  struct iso2_exiFragment{
      value: iso2_exiFragmentEnum, 
  }
  
  // elements of xmldsig fragment
  
  
  enum iso2_xmldsigFragmentEnum{
      CanonicalizationMethod(iso2_CanonicalizationMethodType),
      DSAKeyValue(iso2_DSAKeyValueType),
      DigestMethod(iso2_DigestMethodType),
      KeyInfo(iso2_KeyInfoType),
      KeyValue(iso2_KeyValueType),
      Object(iso2_ObjectType),
      PGPData(iso2_PGPDataType),
      RSAKeyValue(iso2_RSAKeyValueType),
      Reference(iso2_ReferenceType),
      RetrievalMethod(iso2_RetrievalMethodType),
      SPKIData(iso2_SPKIDataType),
      Signature(iso2_SignatureType),
      SignatureMethod(iso2_SignatureMethodType),
      SignatureValue(iso2_SignatureValueType),
      SignedInfo(iso2_SignedInfoType),
      Transform(iso2_TransformType),
      Transforms(iso2_TransformsType),
      X509Data(iso2_X509DataType),
      X509IssuerSerial(iso2_X509IssuerSerialType),
      
  }
  struct iso2_xmldsigFragment{
      value: iso2_xmldsigFragmentEnum, 
  }
  
  fn main(){
    let mut a = iso2_SalesTariffEntryType{ RelativeTimeInterval: todo!(), TimeInterval: todo!(), EPriceLevel: todo!(), ConsumptionCost: todo!(), ConsumptionCostArrayLen: todo!() };
    init_iso2_SalesTariffEntryType(&mut a);
  }
  
  
  
  
  