import { question } from 'readline-sync';
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

let child = require('child_process');

let gen = randGenerator(1020, 0.1, 0.5);

let alertActive = false, alertIndex = 0;

let inc = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4];

setInterval(() => {
    let val = gen.next().value;
    if (alertActive) {
        val += inc[alertIndex];
        alertIndex++;
        if (alertIndex >= inc.length) {
            alertActive = false;
            alertIndex = 0;
        }
    }
    child.exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P $PASSWORD -m ` + val.toFixed(2))
    console.log('sending data', val.toFixed(2));
}, 40)

while (true) {
    let trigger = question('alert?');
    if (trigger.trim() === 'y') {
        alertActive = true;
    } else if (trigger.trim() == 'a') {
        child.exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P $PASSWORD -m "C AIR"`)
    } else if (trigger.trim() == 'w') {
        child.exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P $PASSWORD -m "C WATER"`)
    }
}
