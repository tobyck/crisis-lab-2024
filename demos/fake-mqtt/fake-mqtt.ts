function* randGenerator(avg: number, variation: number, bound: number): Generator<number> {
    let val = avg;
    while (true) {
        yield val;
        let dv = Math.random() * variation;
        if (Math.random() > (val - avg + bound) / bound / 2) {
            val += dv;
        } else {
            val -= dv
        }
    }
}

// @ts-ignore
let child = require('child_process');

let gen = randGenerator(1020, 0.1, 0.5);

setInterval(() => {
    let val = gen.next().value;
    child.exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P $PASSWORD -m ` + val.toFixed(2))
}, 40)
