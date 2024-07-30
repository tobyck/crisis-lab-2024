// Fake websocket server that sends fake pressure/height data to the frontend, for testing purposes

import { IncomingMessage } from 'http';
import r_ws from 'ws';
import { RingBuffer } from './ring-buffer';
import { question } from 'readline-sync';

type DataPacket = {
    timestamp: number;
    pressure: number;
    height: number;
}

const hertz = 25;
const bufferSize = 20;


let ws = new r_ws.Server({ port: 8443 });

let conns: r_ws[] = [];

let prevData = new RingBuffer<DataPacket>(bufferSize * hertz);

ws.on('connection', (conn: r_ws, req: IncomingMessage) => {
    conn.send(JSON.stringify({
        type: 'init',
        previous_data: prevData.toArray(),
        previous_alerts: incidents
    }));
    console.log('new connection');
    conns.push(conn);
    conn.on('close', () => {
        console.log('disconnected')
        conns.splice(conns.indexOf(conn), 1);
    })
})

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

let currentPressure = randGenerator(1020, 0.05, 1);
let currentWaterLevel = randGenerator(1, 0.05, 1);

let triggerAlert = false;

setInterval(() => {
    let newPacket: DataPacket = {
        timestamp: Date.now(),
        pressure: currentPressure.next().value,
        height: currentWaterLevel.next().value
    }
    prevData.pushpop(newPacket);
    let toDeliver = JSON.stringify(newPacket)
    for (let conn of conns) {
        conn.send(toDeliver);
    }
    if (triggerAlert) triggerAlert = false;
}, 1000 / hertz)

let incidents: Incident[] = [];

while (true) {
    let trigger = question('alert?');
    if (trigger.trim() === 'y') {
        let incident = {
            timestamp: Date.now(),
            height: prevData.get(0).height
        };
        incidents.push(incident);
        for (let conn of conns) {
            conn.send(JSON.stringify(incident))
        }

    }
}

type Incident = {
    timestamp: number;
    height: number;
}