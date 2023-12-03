
use fixedvec::{FixedVec, alloc_stack};
use crate::*;
enum MemoryAlloc {
    Reference([iso2_ReferenceType;iso2_ReferenceType_4_ARRAY_SIZE]),
    Cost=[iso2_CostType;iso2_CostType_3_ARRAY_SIZE],
    ConsumptionCost([iso2_ConsumptionCostType;iso2_ConsumptionCostType_3_ARRAY_SIZE]),
    PMaxScheduleEntry([iso2_PMaxScheduleEntryType;iso2_PMaxScheduleEntryType_12_ARRAY_SIZE]),
    SalesTariffEntry([iso2_SalesTariffEntryType;iso2_SalesTariffEntryType_12_ARRAY_SIZE]),
    Parameter([iso2_ParameterType;iso2_ParameterType_16_ARRAY_SIZE]),
    EnergyTransferMode([iso2_EnergyTransferModeType;iso2_EnergyTransferModeType_6_ARRAY_SIZE]),
    PaymentOption([iso2_paymentOptionType;iso2_paymentOptionType_2_ARRAY_SIZE]),
    SelectedService([iso2_SelectedServiceType;iso2_SelectedServiceType_16_ARRAY_SIZE]),
    RootCertificateID([iso2_X509IssuerSerialType;iso2_X509IssuerSerialType_5_ARRAY_SIZE]),
    ProfileEntry([iso2_ProfileEntryType;iso2_ProfileEntryType_24_ARRAY_SIZE]),
    SAScheduleTuple([iso2_SAScheduleTupleType;iso2_SAScheduleTupleType_3_ARRAY_SIZE]),
    ParameterSet([iso2_ParameterSetType;iso2_ParameterSetType_5_ARRAY_SIZE]),
    Service([iso2_ServiceType;iso2_ServiceType_8_ARRAY_SIZE]),
}
fn main(){
    // let mut reference_alloc = alloc_stack!([iso2_ReferenceType;iso2_ReferenceType_4_ARRAY_SIZE]);
    // let mut cost_alloc= alloc_stack!([iso2_CostType;iso2_CostType_3_ARRAY_SIZE]);
    let mut mem= alloc_stack!(MemoryAlloc::PMaxScheduleEntry([iso2_PMaxScheduleEntryType;iso2_PMaxScheduleEntryType_12_ARRAY_SIZE]));
    let sample= iso2_PMaxScheduleType{ PMaxScheduleEntry: FixedVec::new(memory) };
}