const fs = require('fs');

function A(x,m,a,s) {
    return 1;
}

function A_2(ranges) {
    mul = 1;
    for (i of ["x","m","a","s"]) {
        mul *= ranges[`${i}max`] - ranges[`${i}min`] + 1;
    }
    distinct_combinations += mul;
}

function R(x,m,a,s) {
    return 0;
}

function R_2(ranges) {
    return;
}

// some names are already reserved
function_names = {
    "in": "start",
    "rs": "rs0815",
}

var distinct_combinations = 0;

fs.readFile( "src/day_19/input.txt", 'utf8', function(err, f) {
    let [rules, parts] = f.split("\n\n");

    for (let rule of rules.split("\n")) {
        let [name, rs] = rule.split("{");
        let function_body = ""
        let function_body_part2 = ""
        for (let r of rs.split(",")) {
            if (r.includes(":")) {
                // mapping
                let [cond, next] = r.split(":");
                if (next in function_names) {
                    next = function_names[next];
                }
                p2if = ""
                p2else = ""
                if (cond.includes("<")) {
                    let [k,v] = cond.split("<");
                    p2if += `saved = r.${k}max;r.${k}max = ${v} - 1`
                    p2else += `r.${k}max = saved;r.${k}min = ${v}`
                } else {
                    let [k,v] = cond.split(">");
                    p2if += `saved = r.${k}min;r.${k}min = ${v} + 1`
                    p2else += `r.${k}min = saved;r.${k}max = ${v}`
                }
                function_body += `if (${cond}) { return ${next}(x,m,a,s) }\n`
                function_body_part2 += `${p2if};${next}_2(r);${p2else};`
            } else if (r.includes("}")) {
                // last
                let next = r.slice(0, -1);
                if (next in function_names) {
                    next = function_names[next];
                }
                function_body += `return ${next}(x,m,a,s)`
                function_body_part2 += `${next}_2(r);`
            } else {
                console.error("Should not happen!");
                process.exit(1);
            }
        }

        if (name in function_names) {
            name = function_names[name];
        }

        eval(`function ${name}(x,m,a,s,ranges) {${function_body}};`);
        eval(`function ${name}_2(ranges) {let saved=0;let r={...ranges};${function_body_part2}};`);
    }

    let res = 0;
    for (let part of parts.split("\n")) {
        if (part.length == 0) {
            continue
        }
        eval(`var obj = ${part.replaceAll("=", ":")}`)
        if (start(obj.x, obj.m, obj.a, obj.s)) {
            res += obj.x + obj.m + obj.a + obj.s;
        }
    }
    console.log(`Part One ${res}`);

    let ranges = {
        xmin: 1, xmax: 4000,
        mmin: 1, mmax: 4000,
        amin: 1, amax: 4000,
        smin: 1, smax: 4000,
    };
    start_2(ranges);
    console.log(`Part Two ${distinct_combinations}`);
})