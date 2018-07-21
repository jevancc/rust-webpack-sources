let pool = [];

function add(source) {
    pool.push(source);
}
module.exports.add = add;

function clear() {
    for (let i = pool.length - 1; i >= 0; i--) {
        if (pool[i].ptr && pool[i].free) {
            pool[i].free();
        }
    }
    pool.splice(0, pool.length);
}
module.exports.clear = clear;
