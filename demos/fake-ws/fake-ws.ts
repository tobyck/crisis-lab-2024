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

let currentPressure = 1020, currentWaveHeight = 1;

setInterval(() => {
    currentPressure += Math.random() * 0.1 - 0.05;
    currentWaveHeight += Math.random() * 0.1 - 0.05;

    let newPacket: Packet = {
        timeStamp: Date.now(),
        pressure: Math.random() * 2 + 1019,
        waveHeight: currentWaveHeight
    }
    prevData.pushpop(newPacket);
    for (let conn of conns) {
        conn.send(JSON.stringify(newPacket));
    }
}, 1000 / hertz)