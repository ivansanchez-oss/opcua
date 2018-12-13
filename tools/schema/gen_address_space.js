var _ = require("lodash");
var fs = require("fs");
var xml2js = require("xml2js");

var settings = require("./settings");

var trace = false;

// THIS file will generate the address space

var node_set =
    [
        {
            name: "Opc.Ua.NodeSet2.Part3.xml", module: "nodeset_part_3"
        },
        {
            name: "Opc.Ua.NodeSet2.Part4.xml", module: "nodeset_part_4"
        },
        {
            name: "Opc.Ua.NodeSet2.Part5.xml", module: "nodeset_part_5"
        },
        {
            name: "Opc.Ua.NodeSet2.Part8.xml", module: "nodeset_part_8"
        },
        {
            name: "Opc.Ua.NodeSet2.Part9.xml", module: "nodeset_part_9"
        },
        {
            name: "Opc.Ua.NodeSet2.Part10.xml", module: "nodeset_part_10"
        },
        {
            name: "Opc.Ua.NodeSet2.Part11.xml", module: "nodeset_part_11"
        },
        {
            name: "Opc.Ua.NodeSet2.Part12.xml", module: "nodeset_part_12"
        },
        {
            name: "Opc.Ua.NodeSet2.Part13.xml", module: "nodeset_part_13"
        },
        {
            name: "Opc.Ua.NodeSet2.Part14.xml", module: "nodeset_part_14"
        },
        {
            name: "Opc.Ua.NodeSet2.Part999.xml", module: "nodeset_part_999"
        }
    ];

///////////////////////////////////////////////////////////////////////////////
// Parse all XML inputs into data and place it on the node sets above

var parser = new xml2js.Parser();
_.each(node_set, function (ns) {
    fs.readFile(`${settings.schema_dir}/${ns.name}`, function (err, data) {
        parser.parseString(data, function (err, result) {
            ns.data = result;
            console.log(`Generating code for module ${ns.module}`);
            generate_node_set(ns);
        });
    });
});

///////////////////////////////////////////////////////////////////////////////
// All files to be created under server/src/address_space/generated/
function generate_node_set(ns) {
    var contents = `// This file was autogenerated from ${ns.name} by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::*;
use opcua_types::node_ids::*;
#[allow(unused_imports)]
use crate::address_space::types::*;

`;

// Parse the xml
// Create a file with rs
//   in that file, create a populate_address_space method

    var idx = 1;
    var nodes = ns.data["UANodeSet"];
    if (_.has(nodes, "UAObject")) {
        _.each(nodes["UAObject"], function (node) {
            contents += insert_node(idx++, "Object", node, "Object::new(&node_id, browse_name, display_name, description)");
        });
    }
    if (_.has(nodes, "UAObjectType")) {
        _.each(nodes["UAObjectType"], function (value) {
            var is_abstract = _.has(value["$"], "IsAbstract") && value["$"]["IsAbstract"] === "true";
            contents += insert_node(idx++, "ObjectType", value, `ObjectType::new(&node_id, browse_name, display_name, description, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UADataType")) {
        _.each(nodes["UADataType"], function (node) {
            var is_abstract = _.has(node["$"], "IsAbstract") && node["$"]["IsAbstract"] === "true";
            contents += insert_node(idx++, "DataType", node, `DataType::new(&node_id, browse_name, display_name, description, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UAReferenceType")) {
        _.each(nodes["UAReferenceType"], function (node) {
            var is_abstract = _.has(node["$"], "IsAbstract") && node["$"]["IsAbstract"] === "true";
            var inverse_name = _.has(node, "InverseName") ? `Some(LocalizedText::new("", "${node["InverseName"][0]}"))` : "None";
            var symmetric = _.has(node["$"], "Symmetric") && node["$"]["Symmetric"] === "true";
            contents += insert_node(idx++, "DataType", node, `ReferenceType::new(&node_id, browse_name, display_name, description, ${inverse_name}, ${symmetric}, ${is_abstract})`);
        });
    }
    if (_.has(nodes, "UAVariable")) {
        _.each(nodes["UAVariable"], function (node) {
            var data_type = "DataTypeId::Boolean";
            if (_.has(node["$"], "DataType")) {
                data_type = node["$"]["DataType"];
                if (data_type.startsWith("i=")) {
                    data_type = `DataTypeId::from_u32(${data_type.substr(2)}u32).unwrap()`;
                } else {
                    data_type = `DataTypeId::${data_type}`;
                }
            } else {
                console.log("UAVariable has no data type???");
            }
            contents += insert_node(idx++, "Variable", node, `Variable::new_data_value(&node_id, browse_name, display_name, description, ${data_type}, data_value)`);
        });
    }
    if (_.has(nodes, "UAVariableType")) {
        _.each(nodes["UAVariableType"], function (node) {
            var is_abstract = _.has(node["$"], "IsAbstract") && node["$"]["IsAbstract"] === "true";
            var value_rank = _.has(node["$"], "ValueRank") ? node["$"]["ValueRank"] : -1;
            contents += insert_node(idx++, "VariableType", node, `VariableType::new(&node_id, browse_name, display_name, description, ${is_abstract}, ${value_rank})`);
        });
    }
    if (_.has(nodes, "UAMethod")) {
        _.each(nodes["UAMethod"], function (node) {
            var is_abstract = _.has(node["$"], "IsAbstract") && node["$"]["IsAbstract"] === "true";
            var executable = false; // TODO
            var user_executable = false; // TODO
            contents += insert_node(idx++, "Method", node, `Method::new(&node_id, browse_name, display_name, description, ${is_abstract}, ${executable}, ${user_executable})`);
        });
    }

    contents += "#[allow(unused_variables)]\n";
    contents += `pub fn populate_address_space(address_space: &mut AddressSpace) {\n`;
    if (trace) {
        contents += `    trace!("Populating address space with node set ${ns.name}");\n`
    }
    for (var i = 1; i < idx; i++) {
        contents += `    add_${i}(address_space);\n`
    }
    contents += `}\n`;

    settings.write_to_file(`${settings.rs_address_space_dir}/${ns.module}.rs`, contents);
}

///////////////////////////////////////////////////////////////////////////////
// Create the mod.rs

var mod_contents = `// This file was autogenerated by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

use crate::address_space::types::AddressSpace;

`;

// use each part
_.each(node_set, function (ns) {
    mod_contents += `mod ${ns.module};\n`
});
mod_contents += `\n`;

// in a populate_address_space method
mod_contents += `/// Populates the address space with all defined node sets
pub fn populate_address_space(address_space: &mut AddressSpace) {\n`;

_.each(node_set, function (ns) {
    mod_contents += `    ${ns.module}::populate_address_space(address_space);\n`
});

mod_contents += `}\n`;

settings.write_to_file(`${settings.rs_address_space_dir}/mod.rs`, mod_contents);

function node_id_ctor(snippet) {
    // This turns a snippet like "i=2015" into a node id
    return `NodeId::new(0, ${snippet.substr(2)})`;
}

function insert_node(idx, node_type, node, node_ctor) {
    var contents = `fn add_${idx}(address_space: &mut AddressSpace) {\n`;
    var indent = "    ";

    contents += `${indent}// ${node_type}\n`;

    // Process values
    if (node_type === "Variable") {
        var data_value_is_set = false;
        if (_.has(node, "Value")) {
            var value = node["Value"][0];

            if (_.has(value, "ListOfLocalizedText")) {
                // TODO process ListOfLocalizedText
            }

            if (_.has(value, "ListOfExtensionObject")) {

                // Process ListOfExtensionObject looking for Argument types
                var list = value["ListOfExtensionObject"][0];

                var var_arguments = [];
                _.each(list["ExtensionObject"], function (extension_object) {
                    // Create a value consisting an array of extension objects
                    var node_id = (extension_object["TypeId"][0])["Identifier"][0];
                    var body = extension_object["Body"][0];

                    // InputArguments and OutputArguments will have one of these
                    if (_.has(body, "Argument")) {
                        // console.log("node_id=" + node_id);
                        // console.log("body=" + JSON.stringify(body));

                        // Example Argument payload
                        /*
                            <TypeId>
                                <Identifier>i=297</Identifier>
                            </TypeId>
                            <Body>
                                <Argument>
                                    <Name>FileHandle</Name>
                                    <DataType>
                                        <Identifier>i=7</Identifier>
                                    </DataType>
                                    <ValueRank>-1</ValueRank>
                                    <ArrayDimensions />
                                    <Description p5:nil="true" xmlns:p5="http://www.w3.org/2001/XMLSchema-instance" />
                                </Argument>
                            </Body>
                        */

                        var argument = body["Argument"][0];
                        var name = argument["Name"][0];
                        var data_type = (argument["DataType"][0])["Identifier"][0];
                        var value_rank = argument["ValueRank"][0];
                        var array_dimensions = "None";
                        if (value_rank > 1) {
                            console.log("ERROR: Unsupported array dimensions arg");
                        } else if (value_rank == 1) {
                            console.log("ArrayDimensions is not read - setting dimensions to 0 which means variable length");
                            array_dimensions = "Some(vec![0])"
                        }
                        // var description = argument["Description"][0];
                        var_arguments.push({
                            node_id: node_id,
                            name: name,
                            data_type: data_type,
                            value_rank: value_rank,
                            array_dimensions: array_dimensions,
                        });
                    }
                });


                if (var_arguments.length > 0) {
                    contents += `${indent}let data_value = DataValue::new(vec![\n`;
                    _.each(var_arguments, function (a) {
                        contents += `${indent}    Variant::from(ExtensionObject::from_encodable(\n`;
                        contents += `${indent}        ${node_id_ctor(a.node_id)}, &Argument {\n`
                        contents += `${indent}            name: UAString::from("${a.name}"),\n`
                        contents += `${indent}            data_type: ${node_id_ctor(a.data_type)},\n`
                        contents += `${indent}            value_rank: ${a.value_rank},\n`
                        contents += `${indent}            array_dimensions: ${a.array_dimensions},\n`
                        contents += `${indent}            description: LocalizedText::new("", ""),\n`
                        contents += `${indent}        })),\n`
                    });
                    contents += `${indent}]);\n`;
                    data_value_is_set = true;
                }

                // Turn the array of variants into a variant itself and set as the datavalue
            }
        }
        if (!data_value_is_set) {
            contents += `${indent}let data_value = DataValue::null();\n`
        }
    }

    var browse_name = _.has(node["$"], "BrowseName") ? node["$"]["BrowseName"] : "";
    contents += `${indent}let browse_name = "${browse_name}";\n`;
    var display_name = _.has(node, "DisplayName") ? node["DisplayName"][0] : "";
    contents += `${indent}let display_name = "${display_name}";\n`;
    var description = _.has(node, "Description") ? node["Description"][0] : "";
    contents += `${indent}let description = "${description}";\n`;

    var node_id = node["$"]["NodeId"];
    contents += `${indent}let node_id = ${node_id_ctor(node_id)};\n`;

    if (trace) {
        contents += `${indent}trace!("Inserting node id ${node_id}of type ${node_type}");\n`;
    }

    contents += `${indent}let node = ${node_ctor};\n`;
    contents += `${indent}address_space.insert(node, `;

    var node_references = [];
    // Organizes reference
    if (_.has(node["$"], "ParentNodeId")) {
        var parent_node_id = node_id_ctor(node["$"]["ParentNodeId"]);
        node_references.push({
            node_other: parent_node_id,
            reference_type: "ReferenceTypeId::Organizes",
            reference_direction: "ReferenceDirection::Inverse",
        })
    }

    // Process other references
    if (_.has(node, "References")) {
        contents += insert_references(indent, node["References"][0], node_references)
    }

    if (node_references.length > 0) {
        contents += "Some(&[\n";
        _.each(node_references, function (r) {
            contents += `${indent}    (&${r.node_other}, ${r.reference_type}, ${r.reference_direction}),\n`;
        });
        contents += `${indent}]));\n`;
    } else {
        contents += "None);\n";
    }

    // Process definitions
    if (_.has(node, "Definition")) {
        // TODO process Fields
    }

    // Process InverseName
    indent = indent.substr(0, indent.length - 4);
    contents += `}\n\n`;

    return contents;
}

function insert_references(indent, reference_element, node_references) {
    var contents = "";
    if (_.has(reference_element, "Reference")) {
        _.each(reference_element["Reference"], function (reference) {
            // Test if the reference is forward or reverse
            var is_forward = !_.has(reference["$"], "IsForward") || reference["$"]["IsForward"] === "true";

            var node_other = node_id_ctor(reference["_"]);
            var reference_type = reference["$"]["ReferenceType"];
            var reference_direction = is_forward ? "ReferenceDirection::Forward" : "ReferenceDirection::Inverse";

            if (reference_type.startsWith("i=")) {
                // TODO
            } else {
                node_references.push({
                    node_other: node_other,
                    reference_type: `ReferenceTypeId::${reference_type}`,
                    reference_direction: reference_direction
                })
            }
        });

    }
    return contents;
}