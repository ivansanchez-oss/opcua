// This file was autogenerated from Opc.Ua.NodeSet2.Part8.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::*;
use opcua_types::node_ids::*;
#[allow(unused_imports)]
use crate::address_space::types::*;

fn add_1(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 885);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 884), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8873), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_2(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 888);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 887), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8876), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_3(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 12173);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12171), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12175), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_4(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 12174);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12172), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12178), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_5(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 12081);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12079), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12083), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_6(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default XML";
    let display_name = "Default XML";
    let description = "";
    let node_id = NodeId::new(0, 12082);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12080), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12086), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_7(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 886);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 884), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8238), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_8(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 889);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 887), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8241), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_9(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 12181);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12171), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12183), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_10(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 12182);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12172), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12186), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_11(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 12089);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12079), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12091), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_12(address_space: &mut AddressSpace) {
    // Object
    let browse_name = "Default Binary";
    let display_name = "Default Binary";
    let description = "";
    let node_id = NodeId::new(0, 12090);
    let node = Object::new(&node_id, browse_name, display_name, description);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12080), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12094), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_13(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "Range";
    let display_name = "Range";
    let description = "";
    let node_id = NodeId::new(0, 884);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_14(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "EUInformation";
    let display_name = "EUInformation";
    let description = "";
    let node_id = NodeId::new(0, 887);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_15(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "AxisScaleEnumeration";
    let display_name = "AxisScaleEnumeration";
    let description = "";
    let node_id = NodeId::new(0, 12077);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12078), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 29), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_16(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "ComplexNumberType";
    let display_name = "ComplexNumberType";
    let description = "";
    let node_id = NodeId::new(0, 12171);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_17(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "DoubleComplexNumberType";
    let display_name = "DoubleComplexNumberType";
    let description = "";
    let node_id = NodeId::new(0, 12172);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_18(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "AxisInformation";
    let display_name = "AxisInformation";
    let description = "";
    let node_id = NodeId::new(0, 12079);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_19(address_space: &mut AddressSpace) {
    // DataType
    let browse_name = "XVType";
    let display_name = "XVType";
    let description = "";
    let node_id = NodeId::new(0, 12080);
    let node = DataType::new(&node_id, browse_name, display_name, description, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_20(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "Definition";
    let display_name = "Definition";
    let description = "A vendor-specific, human readable string that specifies how the value of this DataItem is calculated.";
    let node_id = NodeId::new(0, 2366);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::String, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_21(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "ValuePrecision";
    let display_name = "ValuePrecision";
    let description = "The maximum precision that the server can maintain for the item based on restrictions in the target environment.";
    let node_id = NodeId::new(0, 2367);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::Double, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_22(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "InstrumentRange";
    let display_name = "InstrumentRange";
    let description = "";
    let node_id = NodeId::new(0, 2370);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(884u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_23(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EURange";
    let display_name = "EURange";
    let description = "";
    let node_id = NodeId::new(0, 2369);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(884u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_24(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EngineeringUnits";
    let display_name = "EngineeringUnits";
    let description = "";
    let node_id = NodeId::new(0, 2371);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(887u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_25(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "FalseState";
    let display_name = "FalseState";
    let description = "";
    let node_id = NodeId::new(0, 2374);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2373), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2373), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_26(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "TrueState";
    let display_name = "TrueState";
    let description = "";
    let node_id = NodeId::new(0, 2375);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2373), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2373), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_27(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EnumStrings";
    let display_name = "EnumStrings";
    let description = "";
    let node_id = NodeId::new(0, 2377);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2376), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2376), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_28(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EnumValues";
    let display_name = "EnumValues";
    let description = "";
    let node_id = NodeId::new(0, 11241);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(7594u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11238), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11238), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_29(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "ValueAsText";
    let display_name = "ValueAsText";
    let description = "";
    let node_id = NodeId::new(0, 11461);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11238), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11238), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_30(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "InstrumentRange";
    let display_name = "InstrumentRange";
    let description = "";
    let node_id = NodeId::new(0, 12024);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(884u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_31(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EURange";
    let display_name = "EURange";
    let description = "";
    let node_id = NodeId::new(0, 12025);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(884u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_32(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EngineeringUnits";
    let display_name = "EngineeringUnits";
    let description = "";
    let node_id = NodeId::new(0, 12026);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(887u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_33(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "Title";
    let display_name = "Title";
    let description = "";
    let node_id = NodeId::new(0, 12027);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_34(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "AxisScaleType";
    let display_name = "AxisScaleType";
    let description = "";
    let node_id = NodeId::new(0, 12028);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12077u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_35(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "XAxisDefinition";
    let display_name = "XAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12037);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12029), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12029), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_36(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "XAxisDefinition";
    let display_name = "XAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12046);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12038), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12038), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_37(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "XAxisDefinition";
    let display_name = "XAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12055);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12047), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12047), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_38(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "YAxisDefinition";
    let display_name = "YAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12056);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12047), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12047), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_39(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "XAxisDefinition";
    let display_name = "XAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12065);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_40(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "YAxisDefinition";
    let display_name = "YAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12066);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_41(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "ZAxisDefinition";
    let display_name = "ZAxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12067);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_42(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "AxisDefinition";
    let display_name = "AxisDefinition";
    let description = "";
    let node_id = NodeId::new(0, 12076);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::from_u32(12079u32).unwrap(), data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12068), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12068), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_43(address_space: &mut AddressSpace) {
    // Variable
    let data_value = DataValue::null();
    let browse_name = "EnumStrings";
    let display_name = "EnumStrings";
    let description = "";
    let node_id = NodeId::new(0, 12078);
    let node = Variable::new_data_value(&node_id, browse_name, display_name, description, DataTypeId::LocalizedText, data_value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12077), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12077), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_44(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "DataItemType";
    let display_name = "DataItemType";
    let description = "A variable that contains live automation data.";
    let node_id = NodeId::new(0, 2365);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2366), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2367), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 63), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_45(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "AnalogItemType";
    let display_name = "AnalogItemType";
    let description = "";
    let node_id = NodeId::new(0, 2368);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2370), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2369), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2371), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_46(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "DiscreteItemType";
    let display_name = "DiscreteItemType";
    let description = "";
    let node_id = NodeId::new(0, 2372);
    let node = VariableType::new(&node_id, browse_name, display_name, description, true, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_47(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "TwoStateDiscreteType";
    let display_name = "TwoStateDiscreteType";
    let description = "";
    let node_id = NodeId::new(0, 2373);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2374), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2375), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_48(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "MultiStateDiscreteType";
    let display_name = "MultiStateDiscreteType";
    let description = "";
    let node_id = NodeId::new(0, 2376);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2377), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_49(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "MultiStateValueDiscreteType";
    let display_name = "MultiStateValueDiscreteType";
    let description = "";
    let node_id = NodeId::new(0, 11238);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11241), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11461), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_50(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "ArrayItemType";
    let display_name = "ArrayItemType";
    let description = "";
    let node_id = NodeId::new(0, 12021);
    let node = VariableType::new(&node_id, browse_name, display_name, description, true, 0);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12024), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12025), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12026), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12027), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12028), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_51(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "YArrayItemType";
    let display_name = "YArrayItemType";
    let description = "";
    let node_id = NodeId::new(0, 12029);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, 1);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12037), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_52(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "XYArrayItemType";
    let display_name = "XYArrayItemType";
    let description = "";
    let node_id = NodeId::new(0, 12038);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, 1);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12046), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_53(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "ImageItemType";
    let display_name = "ImageItemType";
    let description = "";
    let node_id = NodeId::new(0, 12047);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, 2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12055), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12056), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_54(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "CubeItemType";
    let display_name = "CubeItemType";
    let description = "";
    let node_id = NodeId::new(0, 12057);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, 3);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12065), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12066), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12067), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_55(address_space: &mut AddressSpace) {
    // VariableType
    let browse_name = "NDimensionArrayItemType";
    let display_name = "NDimensionArrayItemType";
    let description = "";
    let node_id = NodeId::new(0, 12068);
    let node = VariableType::new(&node_id, browse_name, display_name, description, false, 0);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12076), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_1(address_space);
    add_2(address_space);
    add_3(address_space);
    add_4(address_space);
    add_5(address_space);
    add_6(address_space);
    add_7(address_space);
    add_8(address_space);
    add_9(address_space);
    add_10(address_space);
    add_11(address_space);
    add_12(address_space);
    add_13(address_space);
    add_14(address_space);
    add_15(address_space);
    add_16(address_space);
    add_17(address_space);
    add_18(address_space);
    add_19(address_space);
    add_20(address_space);
    add_21(address_space);
    add_22(address_space);
    add_23(address_space);
    add_24(address_space);
    add_25(address_space);
    add_26(address_space);
    add_27(address_space);
    add_28(address_space);
    add_29(address_space);
    add_30(address_space);
    add_31(address_space);
    add_32(address_space);
    add_33(address_space);
    add_34(address_space);
    add_35(address_space);
    add_36(address_space);
    add_37(address_space);
    add_38(address_space);
    add_39(address_space);
    add_40(address_space);
    add_41(address_space);
    add_42(address_space);
    add_43(address_space);
    add_44(address_space);
    add_45(address_space);
    add_46(address_space);
    add_47(address_space);
    add_48(address_space);
    add_49(address_space);
    add_50(address_space);
    add_51(address_space);
    add_52(address_space);
    add_53(address_space);
    add_54(address_space);
    add_55(address_space);
}
