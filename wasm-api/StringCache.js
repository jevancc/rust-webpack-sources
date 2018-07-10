let array = [];
let set = new Map();

function init() {
    array.push("webpack/bootstrap");
    set.set("webpack/bootstrap", 0);
}

function add(str) {
    let idx = indexOf(str);
    if (idx === undefined) {
        idx = array.length;
        array.push(str);
        set.set(str, idx);
    }
    return idx;
}
module.exports.add = add;

function at(idx) {
    if (idx < 0) {
        return null;
    } else if (idx < array.length) {
        return array[idx];
    }
    throw new Error("No element indexed by " + idx);
}
module.exports.at = at;

function indexOf(str) {
    if (str === "webpack/bootstrap") {
        return 0;
    }
    return set.get(str);
}
module.exports.indexOf = indexOf;
