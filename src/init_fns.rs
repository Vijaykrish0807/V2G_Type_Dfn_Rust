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

use crate::*;




  // root elements of EXI doc
  fn init_ISO2exiDocument(exiDoc: &ISO2exiDocument) {
      let _= exiDoc;
  }
  pub fn init_ISO2CostType(CostType: &mut ISO2CostType){
      
      CostType.amountMultiplier = None;
  }
  
  pub fn init_ISO2TransformType(TransformType: &mut ISO2TransformType){
      
      TransformType.ANY = None;
      TransformType.XPath = None;
  }
  
//   pub fn init_ISO2IntervalType(IntervalType: &mut ISO2IntervalType){
      
//   }
  
//   pub fn init_ISO2ConsumptionCostType(ConsumptionCostType: &mut ISO2ConsumptionCostType){
      
//   }
  
//   pub fn init_ISO2TransformsType(TransformsType: &mut ISO2TransformsType){
      
//   }
  
  pub fn init_ISO2DSAKeyValueType(DSAKeyValueType: &mut ISO2DSAKeyValueType){
      
      DSAKeyValueType.P = None;
      DSAKeyValueType.Q = None;
      DSAKeyValueType.G = None;
      DSAKeyValueType.J = None;
      DSAKeyValueType.Seed = None;
      DSAKeyValueType.PgenCounter = None;
  }
  
  pub fn init_ISO2X509IssuerSerialType(X509IssuerSerialType: &mut ISO2X509IssuerSerialType){
      
  }
  
  pub fn init_ISO2RelativeTimeIntervalType(RelativeTimeIntervalType: &mut ISO2RelativeTimeIntervalType){
      
      RelativeTimeIntervalType.duration = None;
  }
  
  pub fn init_ISO2PMaxScheduleEntryType(PMaxScheduleEntryType: &mut ISO2PMaxScheduleEntryType){
      
      PMaxScheduleEntryType.RelativeTimeInterval = None;
      PMaxScheduleEntryType.TimeInterval = None;
  }
  
  pub fn init_ISO2DigestMethodType(DigestMethodType: &mut ISO2DigestMethodType){
      
      DigestMethodType.ANY = None;
  }
  
  pub fn init_ISO2RSAKeyValueType(RSAKeyValueType: &mut ISO2RSAKeyValueType){
      
  }
  
  pub fn init_ISO2SalesTariffEntryType(SalesTariffEntryType: &mut ISO2SalesTariffEntryType){
      
      SalesTariffEntryType.RelativeTimeInterval = None;
      SalesTariffEntryType.TimeInterval = None;
      SalesTariffEntryType.EPriceLevel = None;
  }
  
  pub fn init_ISO2CanonicalizationMethodType(CanonicalizationMethodType: &mut ISO2CanonicalizationMethodType){
      
      CanonicalizationMethodType.ANY = None;
  }
  
  pub fn init_ISO2SignatureMethodType(SignatureMethodType: &mut ISO2SignatureMethodType){
      
      SignatureMethodType.HMACOutputLength = None;
      SignatureMethodType.ANY = None;
  }
  
  pub fn init_ISO2KeyValueType(KeyValueType: &mut ISO2KeyValueType){
      
      KeyValueType.DSAKeyValue = None;
      KeyValueType.RSAKeyValue = None;
      KeyValueType.ANY = None;
  }
  
  pub fn init_ISO2PMaxScheduleType(PMaxScheduleType: &mut ISO2PMaxScheduleType){
      
  }
  
  pub fn init_ISO2ParameterType(ParameterType: &mut ISO2ParameterType){
      
      ParameterType.boolValue = None;
      ParameterType.byteValue = None;
      ParameterType.shortValue = None;
      ParameterType.intValue = None;
      ParameterType.physicalValue = None;
      ParameterType.stringValue = None;
  }
  
  pub fn init_ISO2ReferenceType(ReferenceType: &mut ISO2ReferenceType){
      
      ReferenceType.Id = None;
      ReferenceType.Type = None;
      ReferenceType.URI = None;
      ReferenceType.Transforms = None;
  }
  
  pub fn init_ISO2RetrievalMethodType(RetrievalMethodType: &mut ISO2RetrievalMethodType){
      
      RetrievalMethodType.Type = None;
      RetrievalMethodType.URI = None;
      RetrievalMethodType.Transforms = None;
  }
  
  pub fn init_ISO2SalesTariffType(SalesTariffType: &mut ISO2SalesTariffType){
      
      SalesTariffType.Id = None;
      SalesTariffType.SalesTariffDescription = None;
      SalesTariffType.NumEPriceLevels = None;
  }
  
  pub fn init_ISO2X509DataType(X509DataType: &mut ISO2X509DataType){
      
      X509DataType.X509IssuerSerial = None;
      X509DataType.X509SKI = None;
      X509DataType.X509SubjectName = None;
      X509DataType.X509Certificate = None;
      X509DataType.X509CRL = None;
      X509DataType.ANY = None;
  }
  
//   pub fn init_ISO2PGPDataType(PGPDataType: &mut ISO2PGPDataType){
      
//       PGPDataType.choice_1 = None;
//       PGPDataType.choice_2 = None;
//   }
  
  pub fn init_ISO2SPKIDataType(SPKIDataType: &mut ISO2SPKIDataType){
      
      SPKIDataType.ANY = None;
  }
  
  pub fn init_ISO2SignedInfoType(SignedInfoType: &mut ISO2SignedInfoType){
      
      SignedInfoType.Id = None;
  }
  
  pub fn init_ISO2ServiceType(ServiceType: &mut ISO2ServiceType){
      
      ServiceType.ServiceName = None;
      ServiceType.ServiceScope = None;
  }
  
  pub fn init_ISO2SelectedServiceType(SelectedServiceType: &mut ISO2SelectedServiceType){
      
      SelectedServiceType.ParameterSetID = None;
  }
  
  pub fn init_ISO2SAScheduleTupleType(SAScheduleTupleType: &mut ISO2SAScheduleTupleType){
      
      SAScheduleTupleType.SalesTariff = None;
  }
  
  pub fn init_ISO2AC_EVSEStatusType(AC_EVSEStatusType: &mut ISO2AC_EVSEStatusType){
      
  }
  
  pub fn init_ISO2DC_EVSEStatusType(DC_EVSEStatusType: &mut ISO2DC_EVSEStatusType){
      
      DC_EVSEStatusType.EVSEIsolationStatus = None;
  }
  
  pub fn init_ISO2ParameterSetType(ParameterSetType: &mut ISO2ParameterSetType){
      
  }
  
  pub fn init_ISO2ProfileEntryType(ProfileEntryType: &mut ISO2ProfileEntryType){
      
      ProfileEntryType.ChargingProfileEntryMaxNumberOfPhasesInUse = None;
  }
  
  pub fn init_ISO2SignatureValueType(SignatureValueType: &mut ISO2SignatureValueType){
      
      SignatureValueType.Id = None;
  }
  
  pub fn init_ISO2SubCertificatesType(SubCertificatesType: &mut ISO2SubCertificatesType){
      
  }
  
  pub fn init_ISO2KeyInfoType(KeyInfoType: &mut ISO2KeyInfoType){
      
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
  
  pub fn init_ISO2ObjectType(ObjectType: &mut ISO2ObjectType){
      
      ObjectType.Encoding = None;
      ObjectType.Id = None;
      ObjectType.MimeType = None;
      ObjectType.ANY = None;
  }
  
  pub fn init_ISO2SupportedEnergyTransferModeType(SupportedEnergyTransferModeType: &mut ISO2SupportedEnergyTransferModeType){
      
  }
  
  pub fn init_ISO2DC_EVStatusType(DC_EVStatusType: &mut ISO2DC_EVStatusType){
      
  }
  
  pub fn init_ISO2BodyBaseType(BodyBaseType: &mut ISO2BodyBaseType){
      
  }
  
  pub fn init_ISO2NotificationType(NotificationType: &mut ISO2NotificationType){
      
      NotificationType.FaultMsg = None;
  }
  
  pub fn init_ISO2PaymentOptionListType(PaymentOptionListType: &mut ISO2PaymentOptionListType){
      
  }
  
  pub fn init_ISO2SelectedServiceListType(SelectedServiceListType: &mut ISO2SelectedServiceListType){
      
  }
  
  pub fn init_ISO2PhysicalValueType(PhysicalValueType: &mut ISO2PhysicalValueType){
      
  }
  
  pub fn init_ISO2CertificateChainType(CertificateChainType: &mut ISO2CertificateChainType){
      
      CertificateChainType.Id = None;
      CertificateChainType.SubCertificates = None;
  }
  
  pub fn init_ISO2EVSEStatusType(EVSEStatusType: &mut ISO2EVSEStatusType){
      
  }
  
  pub fn init_ISO2SignatureType(SignatureType: &mut ISO2SignatureType){
      
      SignatureType.Id = None;
      SignatureType.KeyInfo = None;
      SignatureType.Object = None;
  }
  
  pub fn init_ISO2ChargeServiceType(ChargeServiceType: &mut ISO2ChargeServiceType){
      
      ChargeServiceType.ServiceName = None;
      ChargeServiceType.ServiceScope = None;
  }
  
  pub fn init_ISO2SASchedulesType(SASchedulesType: &mut ISO2SASchedulesType){
      
  }
  
  pub fn init_ISO2SAScheduleListType(SAScheduleListType: &mut ISO2SAScheduleListType){
      
  }
  
  pub fn init_ISO2ServiceParameterListType(ServiceParameterListType: &mut ISO2ServiceParameterListType){
      
  }
  
  pub fn init_ISO2EVChargeParameterType(EVChargeParameterType: &mut ISO2EVChargeParameterType){
      
      EVChargeParameterType.DepartureTime = None;
  }
  
  pub fn init_ISO2AC_EVChargeParameterType(AC_EVChargeParameterType: &mut ISO2AC_EVChargeParameterType){
      
      AC_EVChargeParameterType.DepartureTime = None;
  }
  
  pub fn init_ISO2DC_EVChargeParameterType(DC_EVChargeParameterType: &mut ISO2DC_EVChargeParameterType){
      
      DC_EVChargeParameterType.DepartureTime = None;
      DC_EVChargeParameterType.EVMaximumPowerLimit = None;
      DC_EVChargeParameterType.EVEnergyCapacity = None;
      DC_EVChargeParameterType.EVEnergyRequest = None;
      DC_EVChargeParameterType.FullSOC = None;
      DC_EVChargeParameterType.BulkSOC = None;
  }
  
  pub fn init_ISO2ChargingProfileType(ChargingProfileType: &mut ISO2ChargingProfileType){
      
  }
  
  pub fn init_ISO2ListOfRootCertificateIDsType(ListOfRootCertificateIDsType: &mut ISO2ListOfRootCertificateIDsType){
      
  }
  
  pub fn init_ISO2ServiceListType(ServiceListType: &mut ISO2ServiceListType){
      
  }
  
  pub fn init_ISO2EVSEChargeParameterType(EVSEChargeParameterType: &mut ISO2EVSEChargeParameterType){
      
  }
  
  pub fn init_ISO2AC_EVSEChargeParameterType(AC_EVSEChargeParameterType: &mut ISO2AC_EVSEChargeParameterType){
      
  }
  
  pub fn init_ISO2DC_EVSEChargeParameterType(DC_EVSEChargeParameterType: &mut ISO2DC_EVSEChargeParameterType){
      
      DC_EVSEChargeParameterType.EVSECurrentRegulationTolerance = None;
      DC_EVSEChargeParameterType.EVSEEnergyToBeDelivered = None;
  }
  
  pub fn init_ISO2ContractSignatureEncryptedPrivateKeyType(ContractSignatureEncryptedPrivateKeyType: &mut ISO2ContractSignatureEncryptedPrivateKeyType){
      
  }
  
  pub fn init_ISO2EVPowerDeliveryParameterType(EVPowerDeliveryParameterType: &mut ISO2EVPowerDeliveryParameterType){
      
  }
  
  pub fn init_ISO2DC_EVPowerDeliveryParameterType(DC_EVPowerDeliveryParameterType: &mut ISO2DC_EVPowerDeliveryParameterType){
      
      DC_EVPowerDeliveryParameterType.BulkChargingComplete = None;
  }
  
  pub fn init_ISO2DiffieHellmanPublickeyType(DiffieHellmanPublickeyType: &mut ISO2DiffieHellmanPublickeyType){
      
  }
  
  pub fn init_ISO2MeterInfoType(MeterInfoType: &mut ISO2MeterInfoType){
      
      MeterInfoType.MeterReading = None;
      MeterInfoType.SigMeterReading = None;
      MeterInfoType.MeterStatus = None;
      MeterInfoType.TMeter = None;
  }
  
  pub fn init_ISO2EMAIDType(EMAIDType: &mut ISO2EMAIDType){
      
  }
  
  pub fn init_ISO2MessageHeaderType(MessageHeaderType: &mut ISO2MessageHeaderType){
      
      MessageHeaderType.Notification = None;
      MessageHeaderType.Signature = None;
  }
  
  pub fn init_ISO2SessionSetupResType(SessionSetupResType: &mut ISO2SessionSetupResType){
      
      SessionSetupResType.EVSETimeStamp = None;
  }
  
  pub fn init_ISO2ServiceDiscoveryResType(ServiceDiscoveryResType: &mut ISO2ServiceDiscoveryResType){
      
      ServiceDiscoveryResType.ServiceList = None;
  }
  
  pub fn init_ISO2PaymentServiceSelectionReqType(PaymentServiceSelectionReqType: &mut ISO2PaymentServiceSelectionReqType){
      
  }
  
  pub fn init_ISO2PaymentServiceSelectionResType(PaymentServiceSelectionResType: &mut ISO2PaymentServiceSelectionResType){
      
  }
  
  pub fn init_ISO2WeldingDetectionReqType(WeldingDetectionReqType: &mut ISO2WeldingDetectionReqType){
      
  }
  
  pub fn init_ISO2CurrentDemandReqType(CurrentDemandReqType: &mut ISO2CurrentDemandReqType){
      
      CurrentDemandReqType.EVMaximumVoltageLimit = None;
      CurrentDemandReqType.EVMaximumCurrentLimit = None;
      CurrentDemandReqType.EVMaximumPowerLimit = None;
      CurrentDemandReqType.BulkChargingComplete = None;
      CurrentDemandReqType.RemainingTimeToFullSoC = None;
      CurrentDemandReqType.RemainingTimeToBulkSoC = None;
  }
  
  pub fn init_ISO2PreChargeReqType(PreChargeReqType: &mut ISO2PreChargeReqType){
      
  }
  
  pub fn init_ISO2SessionStopReqType(SessionStopReqType: &mut ISO2SessionStopReqType){
      
  }
  
  pub fn init_ISO2ChargeParameterDiscoveryResType(ChargeParameterDiscoveryResType: &mut ISO2ChargeParameterDiscoveryResType){
      
      ChargeParameterDiscoveryResType.SAScheduleList = None;
      ChargeParameterDiscoveryResType.SASchedules = None;
      ChargeParameterDiscoveryResType.AC_EVSEChargeParameter = None;
      ChargeParameterDiscoveryResType.DC_EVSEChargeParameter = None;
      ChargeParameterDiscoveryResType.EVSEChargeParameter = None;
  }
  
  pub fn init_ISO2ServiceDiscoveryReqType(ServiceDiscoveryReqType: &mut ISO2ServiceDiscoveryReqType){
      
      ServiceDiscoveryReqType.ServiceScope = None;
      ServiceDiscoveryReqType.ServiceCategory = None;
  }
  
  pub fn init_ISO2ChargingStatusReqType(ChargingStatusReqType: &mut ISO2ChargingStatusReqType){
      
  }
  
  pub fn init_ISO2ServiceDetailResType(ServiceDetailResType: &mut ISO2ServiceDetailResType){
      
      ServiceDetailResType.ServiceParameterList = None;
  }
  
  pub fn init_ISO2AuthorizationResType(AuthorizationResType: &mut ISO2AuthorizationResType){
      
  }
  
  pub fn init_ISO2CertificateInstallationResType(CertificateInstallationResType: &mut ISO2CertificateInstallationResType){
      
  }
  
  pub fn init_ISO2PaymentDetailsResType(PaymentDetailsResType: &mut ISO2PaymentDetailsResType){
      
  }
  
  pub fn init_ISO2PaymentDetailsReqType(PaymentDetailsReqType: &mut ISO2PaymentDetailsReqType){
      
  }
  
  pub fn init_ISO2ServiceDetailReqType(ServiceDetailReqType: &mut ISO2ServiceDetailReqType){
      
  }
  
  pub fn init_ISO2ChargingStatusResType(ChargingStatusResType: &mut ISO2ChargingStatusResType){
      
      ChargingStatusResType.EVSEMaxCurrent = None;
      ChargingStatusResType.MeterInfo = None;
      ChargingStatusResType.ReceiptRequired = None;
  }
  
  pub fn init_ISO2CertificateUpdateResType(CertificateUpdateResType: &mut ISO2CertificateUpdateResType){
      
      CertificateUpdateResType.RetryCounter = None;
  }
  
  pub fn init_ISO2ChargeParameterDiscoveryReqType(ChargeParameterDiscoveryReqType: &mut ISO2ChargeParameterDiscoveryReqType){
      
      ChargeParameterDiscoveryReqType.MaxEntriesSAScheduleTuple = None;
      ChargeParameterDiscoveryReqType.AC_EVChargeParameter = None;
      ChargeParameterDiscoveryReqType.DC_EVChargeParameter = None;
      ChargeParameterDiscoveryReqType.EVChargeParameter = None;
  }
  
  pub fn init_ISO2PowerDeliveryReqType(PowerDeliveryReqType: &mut ISO2PowerDeliveryReqType){
      
      PowerDeliveryReqType.ChargingProfile = None;
      PowerDeliveryReqType.DC_EVPowerDeliveryParameter = None;
      PowerDeliveryReqType.EVPowerDeliveryParameter = None;
  }
  
  pub fn init_ISO2PreChargeResType(PreChargeResType: &mut ISO2PreChargeResType){
      
  }
  
  pub fn init_ISO2AuthorizationReqType(AuthorizationReqType: &mut ISO2AuthorizationReqType){
      
      AuthorizationReqType.Id = None;
      AuthorizationReqType.GenChallenge = None;
  }
  
  pub fn init_ISO2PowerDeliveryResType(PowerDeliveryResType: &mut ISO2PowerDeliveryResType){
      
      PowerDeliveryResType.AC_EVSEStatus = None;
      PowerDeliveryResType.DC_EVSEStatus = None;
      PowerDeliveryResType.EVSEStatus = None;
  }
  
  pub fn init_ISO2SessionStopResType(SessionStopResType: &mut ISO2SessionStopResType){
      
  }
  
  pub fn init_ISO2CertificateUpdateReqType(CertificateUpdateReqType: &mut ISO2CertificateUpdateReqType){
      
  }
  
  pub fn init_ISO2CableCheckReqType(CableCheckReqType: &mut ISO2CableCheckReqType){
      
  }
  
  pub fn init_ISO2MeteringReceiptReqType(MeteringReceiptReqType: &mut ISO2MeteringReceiptReqType){
      
      MeteringReceiptReqType.Id = None;
      MeteringReceiptReqType.SAScheduleTupleID = None;
  }
  
  pub fn init_ISO2WeldingDetectionResType(WeldingDetectionResType: &mut ISO2WeldingDetectionResType){
      
  }
  
  pub fn init_ISO2SessionSetupReqType(SessionSetupReqType: &mut ISO2SessionSetupReqType){
      
  }
  
  pub fn init_ISO2CurrentDemandResType(CurrentDemandResType: &mut ISO2CurrentDemandResType){
      
      CurrentDemandResType.EVSEMaximumVoltageLimit = None;
      CurrentDemandResType.EVSEMaximumCurrentLimit = None;
      CurrentDemandResType.EVSEMaximumPowerLimit = None;
      CurrentDemandResType.MeterInfo = None;
      CurrentDemandResType.ReceiptRequired = None;
  }
  
  pub fn init_ISO2MeteringReceiptResType(MeteringReceiptResType: &mut ISO2MeteringReceiptResType){
      
      MeteringReceiptResType.AC_EVSEStatus = None;
      MeteringReceiptResType.DC_EVSEStatus = None;
      MeteringReceiptResType.EVSEStatus = None;
  }
  
//   pub fn init_ISO2CableCheckResType(CableCheckResType: &mut ISO2CableCheckResType){
      
//   }
  
//   pub fn init_ISO2CertificateInstallationReqType(CertificateInstallationReqType: &mut ISO2CertificateInstallationReqType){
      
//   }
  
//   pub fn init_ISO2BodyType(BodyType: &mut ISO2BodyType){
      
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
  
//   pub fn init_ISO2V2G_Message(V2G_Message: &mut ISO2V2G_Message){
      
//   }
  
  
  