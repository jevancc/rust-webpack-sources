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
        return this.node(options)
            .toStringWithSourceMap({
                file: "x"
            })
            .map.toJSON();
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

        var res = this.node(options).toStringWithSourceMap({
            file: "x"
        });
        return {
            source: res.code,
            map: res.map.toJSON()
        };
    };
};
