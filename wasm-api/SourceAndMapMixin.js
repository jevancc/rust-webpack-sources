/*
	MIT License http://www.opensource.org/licenses/mit-license.php
	Author Tobias Koppers @sokra
*/
"use strict";

module.exports = function mixinSourceAndMap(proto) {
    proto.map = function(options) {
        options = options || {};
        if (options.columns === false) {
            var listMap = this.listMap(options);
            var ret = listMap.toStringWithSourceMap({
                file: "x"
            }).map;
            if (listMap.free) {
                listMap.free();
            }
            return ret;
        }
        var StringWithSourceMap = this.node(options)
            .toStringWithSourceMap({
                file: "x"
            });
        return StringWithSourceMap.map;
        // if (typeof StringWithSourceMap === "string") {
        //     return JSON.parse(StringWithSourceMap).map;
        // } else {
        //     return map.toJSON();
        // }
    };

    proto.sourceAndMap = function(options) {
        options = options || {};
        if (options.columns === false) {
            var listMap = this.listMap(options);
            var ret = listMap.toStringWithSourceMap({
                file: "x"
            });
            if (listMap.free) {
                listMap.free();
            }
            return ret;
        }

        var StringWithSourceMap = this.node(options).toStringWithSourceMap({
            file: "x"
        });
        return StringWithSourceMap;

        // if (typeof StringWithSourceMap === "string") {
        //     return JSON.parse(StringWithSourceMap);
        // } else {
        //     return {
        //         source: res.code,
        //         map: res.map.toJSON()
        //     };
        // }
    };
};
