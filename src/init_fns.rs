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
  * @file iso2_msgDefDatatypes.c
  * @brief Description goes here
  *
  **/

  use crate::{iso2_PMaxScheduleType,*};


  // root elements of EXI doc
  fn init_iso2_exiDocument(exiDoc: &mut iso2_exiDocument) {
      let _= exiDoc;
  }
  pub fn init_iso2_CostType(CostType: &mut iso2_CostType){
      
      CostType.amountMultiplier = None;
  }
  
  pub fn init_iso2_TransformType(TransformType: &mut iso2_TransformType){
      
      TransformType.ANY = None;
      TransformType.XPath = None;
  }
  
  pub fn init_iso2_IntervalType(IntervalType: &mut iso2_IntervalType){
      
  }
  
  pub fn init_iso2_ConsumptionCostType(ConsumptionCostType: &mut iso2_ConsumptionCostType){
      
  }
  
  pub fn init_iso2_TransformsType(TransformsType: &mut iso2_TransformsType){
      
  }
  
  pub fn init_iso2_DSAKeyValueType(DSAKeyValueType: &mut iso2_DSAKeyValueType){
      
      DSAKeyValueType.P = None;
      DSAKeyValueType.Q = None;
      DSAKeyValueType.G = None;
      DSAKeyValueType.J = None;
      DSAKeyValueType.Seed = None;
      DSAKeyValueType.PgenCounter = None;
  }
  
  pub fn init_iso2_X509IssuerSerialType(X509IssuerSerialType: &mut iso2_X509IssuerSerialType){
      
  }
  
  pub fn init_iso2_RelativeTimeIntervalType(RelativeTimeIntervalType: &mut iso2_RelativeTimeIntervalType){
      
      RelativeTimeIntervalType.duration = None;
  }
  
  pub fn init_iso2_PMaxScheduleEntryType(PMaxScheduleEntryType: &mut iso2_PMaxScheduleEntryType){
      
      PMaxScheduleEntryType.RelativeTimeInterval = None;
      PMaxScheduleEntryType.TimeInterval = None;
  }
  
  pub fn init_iso2_DigestMethodType(DigestMethodType: &mut iso2_DigestMethodType){
      
      DigestMethodType.ANY = None;
  }
  
  pub fn init_iso2_RSAKeyValueType(RSAKeyValueType: &mut iso2_RSAKeyValueType){
      
  }
  
  pub fn init_iso2_SalesTariffEntryType(SalesTariffEntryType: &mut iso2_SalesTariffEntryType){
      
      SalesTariffEntryType.RelativeTimeInterval = None;
      SalesTariffEntryType.TimeInterval = None;
      SalesTariffEntryType.EPriceLevel = None;
  }
  
  pub fn init_iso2_CanonicalizationMethodType(CanonicalizationMethodType: &mut iso2_CanonicalizationMethodType){
      
      CanonicalizationMethodType.ANY = None;
  }
  
  pub fn init_iso2_SignatureMethodType(SignatureMethodType: &mut iso2_SignatureMethodType){
      
      SignatureMethodType.HMACOutputLength = None;
      SignatureMethodType.ANY = None;
  }
  
  pub fn init_iso2_KeyValueType(KeyValueType: &mut iso2_KeyValueType){
      
      KeyValueType.DSAKeyValue = None;
      KeyValueType.RSAKeyValue = None;
      KeyValueType.ANY = None;
  }
  
  pub fn init_iso2_PMaxScheduleType(PMaxScheduleType: &mut iso2_PMaxScheduleType){
      
  }
  
  pub fn init_iso2_ParameterType(ParameterType: &mut iso2_ParameterType){
      
      ParameterType.boolValue = None;
      ParameterType.byteValue = None;
      ParameterType.shortValue = None;
      ParameterType.intValue = None;
      ParameterType.physicalValue = None;
      ParameterType.stringValue = None;
  }
  
  pub fn init_iso2_ReferenceType(ReferenceType: &mut iso2_ReferenceType){
      
      ReferenceType.Id = None;
      ReferenceType.Type = None;
      ReferenceType.URI = None;
      ReferenceType.Transforms = None;
  }
  
  pub fn init_iso2_RetrievalMethodType(RetrievalMethodType: &mut iso2_RetrievalMethodType){
      
      RetrievalMethodType.Type = None;
      RetrievalMethodType.URI = None;
      RetrievalMethodType.Transforms = None;
  }
  
  pub fn init_iso2_SalesTariffType(SalesTariffType: &mut iso2_SalesTariffType){
      
      SalesTariffType.Id = None;
      SalesTariffType.SalesTariffDescription = None;
      SalesTariffType.NumEPriceLevels = None;
  }
  
  pub fn init_iso2_X509DataType(X509DataType: &mut iso2_X509DataType){
      
      X509DataType.X509IssuerSerial = None;
      X509DataType.X509SKI = None;
      X509DataType.X509SubjectName = None;
      X509DataType.X509Certificate = None;
      X509DataType.X509CRL = None;
      X509DataType.ANY = None;
  }
  
//   pub fn init_iso2_PGPDataType(PGPDataType: &mut iso2_PGPDataType){
      
//       PGPDataType.choice_1 = None;
//       PGPDataType.choice_2 = None;
//   }
  
  pub fn init_iso2_SPKIDataType(SPKIDataType: &mut iso2_SPKIDataType){
      
      SPKIDataType.ANY = None;
  }
  
  pub fn init_iso2_SignedInfoType(SignedInfoType: &mut iso2_SignedInfoType){
      
      SignedInfoType.Id = None;
  }
  
  pub fn init_iso2_ServiceType(ServiceType: &mut iso2_ServiceType){
      
      ServiceType.ServiceName = None;
      ServiceType.ServiceScope = None;
  }
  
  pub fn init_iso2_ProfileEntryType(ProfileEntryType: &mut iso2_ProfileEntryType){
      
      ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse = None;
  }
  
  pub fn init_iso2_SAScheduleTupleType(SAScheduleTupleType: &mut iso2_SAScheduleTupleType){
      
      SAScheduleTupleType.SalesTariff = None;
  }
  
  pub fn init_iso2_SelectedServiceType(SelectedServiceType: &mut iso2_SelectedServiceType){
      
      SelectedServiceType.ParameterSetID = None;
  }
  
  pub fn init_iso2_ParameterSetType(ParameterSetType: &mut iso2_ParameterSetType){
      
  }
  
  pub fn init_iso2_SignatureValueType(SignatureValueType: &mut iso2_SignatureValueType){
      
      SignatureValueType.Id = None;
  }
  
  pub fn init_iso2_SubCertificatesType(SubCertificatesType: &mut iso2_SubCertificatesType){
      
  }
  
  pub fn init_iso2_PhysicalValueType(PhysicalValueType: &mut iso2_PhysicalValueType){
      
  }
  
  pub fn init_iso2_DC_EVStatusType(DC_EVStatusType: &mut iso2_DC_EVStatusType){
      
  }
  
  pub fn init_iso2_KeyInfoType(KeyInfoType: &mut iso2_KeyInfoType){
      
      KeyInfoType.Id = None;
      KeyInfoType.KeyName = None;
      KeyInfoType.KeyValue = None;
      KeyInfoType.RetrievalMethod = None;
      KeyInfoType.X509Data = None;
      KeyInfoType.PGPData = None;
      KeyInfoType.SPKIData = None;
      KeyInfoType.MgmtData = None;
      KeyInfoType.ANY = None;
  }
  
  pub fn init_iso2_ObjectType(ObjectType: &mut iso2_ObjectType){
      
      ObjectType.Encoding = None;
      ObjectType.Id = None;
      ObjectType.MimeType = None;
      ObjectType.ANY = None;
  }
  
  pub fn init_iso2_SupportedEnergyTransferModeType(SupportedEnergyTransferModeType: &mut iso2_SupportedEnergyTransferModeType){
      
  }
  
  pub fn init_iso2_BodyBaseType(BodyBaseType: &mut iso2_BodyBaseType){
      
  }
  
  pub fn init_iso2_NotificationType(NotificationType: &mut iso2_NotificationType){
      
      NotificationType.FaultMsg = None;
  }
  
  pub fn init_iso2_EVSEStatusType(EVSEStatusType: &mut iso2_EVSEStatusType){
      
  }
  
  pub fn init_iso2_AC_EVSEStatusType(AC_EVSEStatusType: &mut iso2_AC_EVSEStatusType){
      
  }
  
  pub fn init_iso2_DC_EVSEStatusType(DC_EVSEStatusType: &mut iso2_DC_EVSEStatusType){
      
      DC_EVSEStatusType.EVSEIsolationStatus = None;
  }
  
  pub fn init_iso2_CertificateChainType(CertificateChainType: &mut iso2_CertificateChainType){
      
      CertificateChainType.Id = None;
      CertificateChainType.SubCertificates = None;
  }
  
  pub fn init_iso2_PaymentOptionListType(PaymentOptionListType: &mut iso2_PaymentOptionListType){
      
  }
  
  pub fn init_iso2_SelectedServiceListType(SelectedServiceListType: &mut iso2_SelectedServiceListType){
      
  }
  
  pub fn init_iso2_SignatureType(SignatureType: &mut iso2_SignatureType){
      
      SignatureType.Id = None;
      SignatureType.KeyInfo = None;
      SignatureType.Object = None;
  }
  
  pub fn init_iso2_EVChargeParameterType(EVChargeParameterType: &mut iso2_EVChargeParameterType){
      
      EVChargeParameterType.DepartureTime = None;
  }
  
  pub fn init_iso2_AC_EVChargeParameterType(AC_EVChargeParameterType: &mut iso2_AC_EVChargeParameterType){
      
      AC_EVChargeParameterType.DepartureTime = None;
  }
  
  pub fn init_iso2_DC_EVChargeParameterType(DC_EVChargeParameterType: &mut iso2_DC_EVChargeParameterType){
      
      DC_EVChargeParameterType.DepartureTime = None;
      DC_EVChargeParameterType.EVMaximumPowerLimit = None;
      DC_EVChargeParameterType.EVEnergyCapacity = None;
      DC_EVChargeParameterType.EVEnergyRequest = None;
      DC_EVChargeParameterType.FullSOC = None;
      DC_EVChargeParameterType.BulkSOC = None;
  }
  
  pub fn init_iso2_ChargeServiceType(ChargeServiceType: &mut iso2_ChargeServiceType){
      
      ChargeServiceType.ServiceName = None;
      ChargeServiceType.ServiceScope = None;
  }
  
  pub fn init_iso2_ListOfRootCertificateIDsType(ListOfRootCertificateIDsType: &mut iso2_ListOfRootCertificateIDsType){
      
  }
  
  pub fn init_iso2_ChargingProfileType(ChargingProfileType: &mut iso2_ChargingProfileType){
      
  }
  
  pub fn init_iso2_SASchedulesType(SASchedulesType: &mut iso2_SASchedulesType){
      
  }
  
  pub fn init_iso2_SAScheduleListType(SAScheduleListType: &mut iso2_SAScheduleListType){
      
  }
  
  pub fn init_iso2_ServiceParameterListType(ServiceParameterListType: &mut iso2_ServiceParameterListType){
      
  }
  
  pub fn init_iso2_ContractSignatureEncryptedPrivateKeyType(ContractSignatureEncryptedPrivateKeyType: &mut iso2_ContractSignatureEncryptedPrivateKeyType){
      
  }
  
  pub fn init_iso2_ServiceListType(ServiceListType: &mut iso2_ServiceListType){
      
  }
  
  pub fn init_iso2_EVPowerDeliveryParameterType(EVPowerDeliveryParameterType: &mut iso2_EVPowerDeliveryParameterType){
      
  }
  
  pub fn init_iso2_DC_EVPowerDeliveryParameterType(DC_EVPowerDeliveryParameterType: &mut iso2_DC_EVPowerDeliveryParameterType){
      
      DC_EVPowerDeliveryParameterType.BulkChargingComplete = None;
  }
  
  pub fn init_iso2_EVSEChargeParameterType(EVSEChargeParameterType: &mut iso2_EVSEChargeParameterType){
      
  }
  
  pub fn init_iso2_AC_EVSEChargeParameterType(AC_EVSEChargeParameterType: &mut iso2_AC_EVSEChargeParameterType){
      
  }
  
  pub fn init_iso2_DC_EVSEChargeParameterType(DC_EVSEChargeParameterType: &mut iso2_DC_EVSEChargeParameterType){
      
      DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance = None;
      DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered = None;
  }
  
  pub fn init_iso2_DiffieHellmanPublickeyType(DiffieHellmanPublickeyType: &mut iso2_DiffieHellmanPublickeyType){
      
  }
  
  pub fn init_iso2_MeterInfoType(MeterInfoType: &mut iso2_MeterInfoType){
      
      MeterInfoType.MeterReading = None;
      MeterInfoType.SigMeterReading = None;
      MeterInfoType.MeterStatus = None;
      MeterInfoType.TMeter = None;
  }
  
  pub fn init_iso2_EMAIDType(EMAIDType: &mut iso2_EMAIDType){
      
  }
  
  pub fn init_iso2_MessageHeaderType(MessageHeaderType: &mut iso2_MessageHeaderType){
      
      MessageHeaderType.Notification = None;
      MessageHeaderType.Signature = None;
  }
  
  pub fn init_iso2_MeteringReceiptResType(MeteringReceiptResType: &mut iso2_MeteringReceiptResType){
      
      MeteringReceiptResType.AC_EVSEStatus = None;
      MeteringReceiptResType.DC_EVSEStatus = None;
      MeteringReceiptResType.EVSEStatus = None;
  }
  
  pub fn init_iso2_CertificateUpdateResType(CertificateUpdateResType: &mut iso2_CertificateUpdateResType){
      
      CertificateUpdateResType.RetryCounter = None;
  }
  
  pub fn init_iso2_ChargeParameterDiscoveryReqType(ChargeParameterDiscoveryReqType: &mut iso2_ChargeParameterDiscoveryReqType){
      
      ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple = None;
      ChargeParameterDiscoveryReqType.AC_EVChargeParameter = None;
      ChargeParameterDiscoveryReqType.DC_EVChargeParameter = None;
      ChargeParameterDiscoveryReqType.EVChargeParameter = None;
  }
  
  pub fn init_iso2_ServiceDetailReqType(ServiceDetailReqType: &mut iso2_ServiceDetailReqType){
      
  }
  
  pub fn init_iso2_AuthorizationReqType(AuthorizationReqType: &mut iso2_AuthorizationReqType){
      
      AuthorizationReqType.Id = None;
      AuthorizationReqType.GenChallenge = None;
  }
  
  pub fn init_iso2_PaymentDetailsReqType(PaymentDetailsReqType: &mut iso2_PaymentDetailsReqType){
      
  }
  
  pub fn init_iso2_PaymentServiceSelectionResType(PaymentServiceSelectionResType: &mut iso2_PaymentServiceSelectionResType){
      
  }
  
  pub fn init_iso2_ServiceDiscoveryResType(ServiceDiscoveryResType: &mut iso2_ServiceDiscoveryResType){
      
      ServiceDiscoveryResType.ServiceList = None;
  }
  
  pub fn init_iso2_CertificateUpdateReqType(CertificateUpdateReqType: &mut iso2_CertificateUpdateReqType){
      
  }
  
  pub fn init_iso2_SessionStopResType(SessionStopResType: &mut iso2_SessionStopResType){
      
  }
  
  pub fn init_iso2_ChargingStatusReqType(ChargingStatusReqType: &mut iso2_ChargingStatusReqType){
      
  }
  
  pub fn init_iso2_ChargingStatusResType(ChargingStatusResType: &mut iso2_ChargingStatusResType){
      
      ChargingStatusResType.EVSEMaxCurrent = None;
      ChargingStatusResType.MeterInfo = None;
      ChargingStatusResType.ReceiptRequired = None;
  }
  
  pub fn init_iso2_PowerDeliveryReqType(PowerDeliveryReqType: &mut iso2_PowerDeliveryReqType){
      
      PowerDeliveryReqType.ChargingProfile = None;
      PowerDeliveryReqType.DC_EVPowerDeliveryParameter = None;
      PowerDeliveryReqType.EVPowerDeliveryParameter = None;
  }
  
  pub fn init_iso2_SessionStopReqType(SessionStopReqType: &mut iso2_SessionStopReqType){
      
  }
  
  pub fn init_iso2_PaymentDetailsResType(PaymentDetailsResType: &mut iso2_PaymentDetailsResType){
      
  }
  
  pub fn init_iso2_CertificateInstallationReqType(CertificateInstallationReqType: &mut iso2_CertificateInstallationReqType){
      
  }
  
  pub fn init_iso2_ChargeParameterDiscoveryResType(ChargeParameterDiscoveryResType: &mut iso2_ChargeParameterDiscoveryResType){
      
      ChargeParameterDiscoveryResType.SAScheduleList = None;
      ChargeParameterDiscoveryResType.SASchedules = None;
      ChargeParameterDiscoveryResType.AC_EVSEChargeParameter = None;
      ChargeParameterDiscoveryResType.DC_EVSEChargeParameter = None;
      ChargeParameterDiscoveryResType.EVSEChargeParameter = None;
  }
  
  pub fn init_iso2_CertificateInstallationResType(CertificateInstallationResType: &mut iso2_CertificateInstallationResType){
      
  }
  
  pub fn init_iso2_AuthorizationResType(AuthorizationResType: &mut iso2_AuthorizationResType){
      
  }
  
  pub fn init_iso2_CableCheckReqType(CableCheckReqType: &mut iso2_CableCheckReqType){
      
  }
  
  pub fn init_iso2_PreChargeReqType(PreChargeReqType: &mut iso2_PreChargeReqType){
      
  }
  
  pub fn init_iso2_PowerDeliveryResType(PowerDeliveryResType: &mut iso2_PowerDeliveryResType){
      
      PowerDeliveryResType.AC_EVSEStatus = None;
      PowerDeliveryResType.DC_EVSEStatus = None;
      PowerDeliveryResType.EVSEStatus = None;
  }
  
  pub fn init_iso2_SessionSetupResType(SessionSetupResType: &mut iso2_SessionSetupResType){
      
      SessionSetupResType.EVSETimeStamp = None;
  }
  
  pub fn init_iso2_CableCheckResType(CableCheckResType: &mut iso2_CableCheckResType){
      
  }
  
  pub fn init_iso2_MeteringReceiptReqType(MeteringReceiptReqType: &mut iso2_MeteringReceiptReqType){
      
      MeteringReceiptReqType.Id = None;
      MeteringReceiptReqType.SAScheduleTupleID = None;
  }
  
  pub fn init_iso2_PreChargeResType(PreChargeResType: &mut iso2_PreChargeResType){
      
  }
  
  pub fn init_iso2_CurrentDemandResType(CurrentDemandResType: &mut iso2_CurrentDemandResType){
      
      CurrentDemandResType.EVSEMaximumVoltageLimit = None;
      CurrentDemandResType.EVSEMaximumCurrentLimit = None;
      CurrentDemandResType.EVSEMaximumPowerLimit = None;
      CurrentDemandResType.MeterInfo = None;
      CurrentDemandResType.ReceiptRequired = None;
  }
  
  pub fn init_iso2_SessionSetupReqType(SessionSetupReqType: &mut iso2_SessionSetupReqType){
      
  }
  
  pub fn init_iso2_WeldingDetectionResType(WeldingDetectionResType: &mut iso2_WeldingDetectionResType){
      
  }
  
  pub fn init_iso2_ServiceDiscoveryReqType(ServiceDiscoveryReqType: &mut iso2_ServiceDiscoveryReqType){
      
      ServiceDiscoveryReqType.ServiceScope = None;
      ServiceDiscoveryReqType.ServiceCategory = None;
  }
  
  pub fn init_iso2_CurrentDemandReqType(CurrentDemandReqType: &mut iso2_CurrentDemandReqType){
      
      CurrentDemandReqType.EVMaximumVoltageLimit = None;
      CurrentDemandReqType.EVMaximumCurrentLimit = None;
      CurrentDemandReqType.EVMaximumPowerLimit = None;
      CurrentDemandReqType.BulkChargingComplete = None;
      CurrentDemandReqType.RemainingTimeToFullSoC = None;
      CurrentDemandReqType.RemainingTimeToBulkSoC = None;
  }
  
  pub fn init_iso2_PaymentServiceSelectionReqType(PaymentServiceSelectionReqType: &mut iso2_PaymentServiceSelectionReqType){
      
  }
  
  pub fn init_iso2_WeldingDetectionReqType(WeldingDetectionReqType: &mut iso2_WeldingDetectionReqType){
      
  }
  
  pub fn init_iso2_ServiceDetailResType(ServiceDetailResType: &mut iso2_ServiceDetailResType){
      
      ServiceDetailResType.ServiceParameterList = None;
  }
  
//   pub fn init_iso2_BodyType(BodyType: &mut iso2_BodyType){
      
      
//       BodyType.AuthorizationReq = None;
//       BodyType.AuthorizationRes = None;
//       BodyType.BodyElement = None;
//       BodyType.CableCheckReq = None;
//       BodyType.CableCheckRes = None;
//       BodyType.CertificateInstallationReq = None;
//       BodyType.CertificateInstallationRes = None;
//       BodyType.CertificateUpdateReq = None;
//       BodyType.CertificateUpdateRes = None;
//       BodyType.ChargeParameterDiscoveryReq = None;
//       BodyType.ChargeParameterDiscoveryRes = None;
//       BodyType.ChargingStatusReq = None;
//       BodyType.ChargingStatusRes = None;
//       BodyType.CurrentDemandReq = None;
//       BodyType.CurrentDemandRes = None;
//       BodyType.MeteringReceiptReq = None;
//       BodyType.MeteringReceiptRes = None;
//       BodyType.PaymentDetailsReq = None;
//       BodyType.PaymentDetailsRes = None;
//       BodyType.PaymentServiceSelectionReq = None;
//       BodyType.PaymentServiceSelectionRes = None;
//       BodyType.PowerDeliveryReq = None;
//       BodyType.PowerDeliveryRes = None;
//       BodyType.PreChargeReq = None;
//       BodyType.PreChargeRes = None;
//       BodyType.ServiceDetailReq = None;
//       BodyType.ServiceDetailRes = None;
//       BodyType.ServiceDiscoveryReq = None;
//       BodyType.ServiceDiscoveryRes = None;
//       BodyType.SessionSetupReq = None;
//       BodyType.SessionSetupRes = None;
//       BodyType.SessionStopReq = None;
//       BodyType.SessionStopRes = None;
//       BodyType.WeldingDetectionReq = None;
//       BodyType.WeldingDetectionRes = None;
//   }
  
//   pub fn init_iso2_V2G_Message(V2G_Message: &mut iso2_V2G_Message){
      
//   }
  
//   // init for fragment
//   pub fn init_iso2_exiFragment(exiFrag: &mut iso2_exiFragment){
      
//       exiFrag.AuthorizationReq = None;
//       exiFrag.CertificateInstallationReq = None;
//       exiFrag.CertificateInstallationRes = None;
//       exiFrag.CertificateUpdateReq = None;
//       exiFrag.CertificateUpdateRes = None;
//       exiFrag.ChargeParameterDiscoveryRes = None;
//       exiFrag.MeteringReceiptReq = None;
//       exiFrag.SignedInfo = None;
//   }
  
//   // init for xmldsig fragment
//   pub fn init_iso2_xmldsigFragment(xmldsigFrag: &mut iso2_xmldsigFragment){
      
//       xmldsigFrag.CanonicalizationMethod = None;
//       xmldsigFrag.DSAKeyValue = None;
//       xmldsigFrag.DigestMethod = None;
//       xmldsigFrag.KeyInfo = None;
//       xmldsigFrag.KeyValue = None;
//       xmldsigFrag.Object = None;
//       xmldsigFrag.PGPData = None;
//       xmldsigFrag.RSAKeyValue = None;
//       xmldsigFrag.Reference = None;
//       xmldsigFrag.RetrievalMethod = None;
//       xmldsigFrag.SPKIData = None;
//       xmldsigFrag.Signature = None;
//       xmldsigFrag.SignatureMethod = None;
//       xmldsigFrag.SignatureValue = None;
//       xmldsigFrag.SignedInfo = None;
//       xmldsigFrag.Transform = None;
//       xmldsigFrag.Transforms = None;
//       xmldsigFrag.X509Data = None;
//       xmldsigFrag.X509IssuerSerial = None;
//   }
  
  
  



