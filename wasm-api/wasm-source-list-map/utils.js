"use strict";

const Types = require("./Types");
const wasm = require("../build/webpack_sources");

exports.StringVec = function StringVec(strs) {
    let stringVec = wasm.StringVec.new();
    for (let i = 0; i < strs.length; i++) {
        if (typeof strs[i] === "string") stringVec.push_string(strs[i]);
    }
    return stringVec;
};

exports.NodeVec = function NodeVec(nodes) {
    let nodeVec = wasm.NodeVec.new();
    for (let i = 0; i < nodes.length; i++) {
        if (typeof nodes[i] === "string") {
            nodeVec.push_string(nodes[i]);
        } else {
            switch (nodes[i].type) {
                case Types.CodeNode:
                    nodeVec.push_codenode(nodes[i]);
                    break;
                case Types.SourceNode:
                    nodeVec.push_sourcenode(nodes[i]);
                    break;
                case Types.SingleLineNode:
                    nodeVec.push_singlelinenode(nodes[i]);
                    break;
                case Types.SourceListMap:
                    nodeVec.push_sourcelistmap(nodes[i]);
                    break;
                default:
                    throw new TypeError("Invalid node type");
            }
        }
    }
    return nodeVec;
};
