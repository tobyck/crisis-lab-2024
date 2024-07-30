/*
 * Author: Theo Keith & Max Robati
 * Date: 29th July 2024
 * Purpose: Modded version of the original version of this in demos/fake-ws
 * Language: TypeScript
 * */

import { IncomingMessage } from 'http';
import r_ws from 'ws';
import { RingBuffer } from './ring-buffer';
import { question } from 'readline-sync';

type DataPacket = {
    timestamp: number;
    pressure?: number;
    height: number;
}

const hertz = 10;
const bufferSize = 10;

let ws = new r_ws.Server({ host: "10.165.228.97", port: 8081 });

console.log(ws)

let conns: r_ws[] = [];

let prevData = new RingBuffer<DataPacket>(bufferSize * hertz);

// send previous data upon client connection
ws.on('connection', (conn: r_ws, req: IncomingMessage) => {
    conn.send(JSON.stringify({
        prevData: prevData.toArray().concat(incidents),
    }));
    console.log('new connection');
    conns.push(conn);
    conn.on('close', () => {
        console.log('disconnected')
        conns.splice(conns.indexOf(conn), 1); // remove client from list of connections
    })
})

// generator that creates random fake data
function* randGenerator(avg: number, variation: number, bound: number): Generator<number> {
    let val = avg;
    while (true) {
        //yield val;
        yield 1000;
        /*
        let dv = Math.random() * variation;
        if (Math.random() > (val - avg + bound) / bound / 2) {
            val += dv;
        } else {
            val -= dv
        }
        */
    }
}

let currentPressure = randGenerator(1020, 0.05, 1);
let currentWaterLevel = randGenerator(1, 0.05, 1);

let triggerAlert = false;

// send data packets to all clients at the specified rate
setInterval(() => {
    let newPacket: DataPacket = {
        timestamp: Date.now(),
        pressure: currentPressure.next().value,
        height: currentWaterLevel.next().value
    }
    prevData.pushpop(newPacket); // pretend pushpop is just called push
    let toDeliver = JSON.stringify({
        data: newPacket
    })
    for (let conn of conns) {
        conn.send(toDeliver);
    }
    if (triggerAlert) triggerAlert = false;
}, 1000 / hertz)

let incidents: Incident[] = [];

// continually ask if we want to alert
while (true) {
    let trigger = question('alert?');
    if (trigger.trim() === 'y') {
		// if yes, send alert packet
        let incident = {
            timestamp: Date.now(),
            height: prevData.get(0).height
        };
        incidents.push(incident);
        for (let conn of conns) {
            conn.send(JSON.stringify({
                data: incident
            }))
        }

    }
}

type Incident = {
    timestamp: number;
    height: number;
}
