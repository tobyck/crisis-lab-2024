import { IncomingMessage } from 'http';
import r_ws from 'ws';
import { RingBuffer } from './ring-buffer';

type Packet = {
    timeStamp: number;
    pressure: number;
    waveHeight: number;
}

const hertz = 25;
const bufferSize = 10;


let ws = new r_ws.Server({ port: 8081 });

let conns: r_ws[] = [];

let prevData = new RingBuffer<Packet>(bufferSize * hertz);

ws.on('connection', (conn: r_ws, req: IncomingMessage) => {
    conn.send(JSON.stringify(prevData.toArray()));
    console.log('new connection');
    conns.push(conn);
    conn.on('close', () => {
        console.log('disconnected')
        conns.splice(conns.indexOf(conn), 1);
    })
})

function* randGenerator (avg: number, variation: number, bound: number): Generator<number> {
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

let currentPressure = randGenerator(1020, 0.2, 1);
let currentWaveHeight = randGenerator(1, 0.2, 1);

setInterval(() => {
    let newPacket: Packet = {
        timeStamp: Date.now(),
        pressure: currentPressure.next().value,
        waveHeight: currentWaveHeight.next().value
    }
    prevData.pushpop(newPacket);
    for (let conn of conns) {
        conn.send(JSON.stringify(newPacket));
    }
}, 1000 / hertz)