function lineCounts(script) {
  var result = Object.create(null);
  script.split(/\n/).forEach(function(line) {
    var fields = line.split(/,/);
    var role = fields[2];
    result[role] = (result[role] || 0) + 1;
  });
  return result;
}

function lead(script) {
  var counts = lineCounts(script);
  return Object.keys(counts)
               .map(function(name) {
                 return {
                   name: name,
                   lines: counts[name]
                 };
               })
               .reduce(function(acc, role) {
                 return (!acc || acc.lines < role.lines)
                      ? role
                      : acc;
               });
}

function best(corpus) {
  return Object.keys(corpus)
               .map(function(title) {
                 var role = lead(corpus[title]);
                 return {
                   title: title,
                   lead: role
                 };
               })
               .reduce(function(acc, play) {
                 return (!acc || acc.lead.lines < play.lead.lines)
                      ? play
                      : acc;
               });
}

exports.best = best;
exports.lead = lead;
