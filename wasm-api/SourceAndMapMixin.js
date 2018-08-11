"use strict";

const SourceNode = require("./wasm-source-map").SourceNode;
const SourceListMap = require("./wasm-source-list-map").SourceListMap;
const Types = require("./Types");

module.exports = function mixinSourceAndMap(proto) {
    proto.node = function(options) {
        let node = new SourceNode(Types.Null);
        options = options || {};
        node.ptr = this._node_bool_bool(
            !(options.columns === false),
            !(options.module === false)
        ).ptr;
        return node;
    };

    proto.listMap = function(options) {
        let map = new SourceListMap(Types.Null);
        options = options || {};
        map.ptr = this._list_map_bool_bool(
            !(options.columns === false),
            !(options.module === false)
        ).ptr;
        return map;
    };

    proto.map = function(options) {
        options = options || {};
        if (options.columns === false) {
            let listMap = this.listMap(options);
            let ret = listMap.toStringWithSourceMap({
                file: "x"
            }).map;
            if (listMap.free) {
                listMap.free();
            }
            return ret;
        } else {
            let node = this.node(options);
            let ret = node.toStringWithSourceMap({
                file: "x"
            }).map;
            if (node.free) {
                node.free();
            }
            return ret;
        }
    };

    proto.sourceAndMap = function(options) {
        options = options || {};
        if (options.columns === false) {
            let listMap = this.listMap(options);
            let ret = listMap.toStringWithSourceMap({
                file: "x"
            });
            if (listMap.free) {
                listMap.free();
            }
            return ret;
        } else {
            let node = this.node(options);
            let ret = node.toStringWithSourceMap({
                file: "x"
            });
            if (node.free) {
                node.free();
            }
            return ret;
        }
    };
};
