"use strict";

var SourceNode = require("source-map").SourceNode;
var SourceMapConsumer = require("source-map").SourceMapConsumer;
var SourceListMap = require("./source-list-map").SourceListMap;
var wasm = require("./build/webpack_sources");

class OriginalSource extends wasm._OriginalSource {
	constructor(value, name) {
		super(0);
		this.ptr = OriginalSource._new_string_string(value, name).ptr;
	}

	source() {
		return this._source();
	}

	size() {
		return this._size();
	}

	node(options) {
		options = options || {};
		return this._node_bool(!(options.columns === false));
	}

	listMap(options) {
		var value = this._source();
		var name = this._name();
		return new SourceListMap(value, name, value);
	}

	updateHash(hash) {
		hash.update(this._source());
	}
}

require("./SourceAndMapMixin")(OriginalSource.prototype);

module.exports = OriginalSource;
