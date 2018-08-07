"use strict";

class Source {
    source() {
        throw new Error("Abstract");
    }

    size() {
        return this.source().length;
    }

    map(options) {
        return null;
    }

    sourceAndMap(options) {
        return {
            source: this.source(),
            map: this.map()
        };
    }

    node() {
        throw new Error("Abstract");
    }

    listNode() {
        throw new Error("Abstract");
    }

    updateHash(hash) {
        var source = this.source();
        hash.update(source || "");
    }
}

module.exports = Source;
