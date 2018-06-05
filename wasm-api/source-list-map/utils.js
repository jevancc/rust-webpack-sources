"use strict";

var wasm = require("../build/webpack_sources");

exports.StringVec = function StringVec(strs) {
    var stringVec = wasm.StringVec.new();
    for (var i = 0; i < strs.length; i++) {
        if (typeof strs[i] === "string") stringVec.push_string(strs[i]);
    }
    return stringVec;
};

exports.NodeVec = function NodeVec(nodes) {
    var nodeVec = wasm.NodeVec.new();
    for (var i = 0; i < nodes.length; i++) {
        if (typeof nodes[i] === "string") nodeVec.push_string(nodes[i]);
        else if (nodes[i].isCodeNode) nodeVec.push_codenode(nodes[i]);
        else if (nodes[i].isSourceNode)
            nodeVec.push_sourcenode(nodes[i]);
        else if (nodes[i].isSingleLineNode)
            nodeVec.push_singlelinenode(nodes[i]);
        else if (nodes[i].isSourceListMap)
            nodeVec.push_sourcelistmap(nodes[i]);
        else throw "Invalid node type";
    }
    return nodeVec;
};
