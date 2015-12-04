var fs = require('fs');
var path = require('path');
var _ = require('lodash');
var roles = {
  js: require('./roles'),
  native: require('rust-bindings')()
};
var bench = require('./bench');

function mapAsKeys(array, f) {
  return _.mapValues(_.invert(array), function(val, key) { return f(key); });
}

var ROOT = path.resolve(__dirname, "..");
var PLAYS = path.resolve(ROOT, "plays");

var titles = fs.readdirSync(PLAYS)
               .filter(function(filename) { return /\.csv$/.test(filename); })
               .map(function(filename) { return filename.replace(/\.csv$/, "") });

var stringCorpus = mapAsKeys(titles, function(title) {
  return fs.readFileSync(path.resolve(PLAYS, title + ".csv"), 'utf8');
});

var bufferCorpus = mapAsKeys(titles, function(title) {
  return fs.readFileSync(path.resolve(PLAYS, title + ".csv"));
});

console.log(bench(function() { return roles.js.best(stringCorpus); }));
console.log(bench(function() { return roles.native.best(bufferCorpus); }));
