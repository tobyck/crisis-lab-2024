import { IncomingMessage } from 'http';
import r_ws from 'ws';
import { RingBuffer } from './ring-buffer';
import { question } from 'readline-sync';

type DataPacket = {
    timeStamp: number;
    pressure: number;
    waterLevel: number;
}

const hertz = 25;
const bufferSize = 10;


let ws = new r_ws.Server({ port: 8081 });

let conns: r_ws[] = [];

let prevData = new RingBuffer<DataPacket>(bufferSize * hertz);

ws.on('connection', (conn: r_ws, req: IncomingMessage) => {
    conn.send(JSON.stringify({
        type: 'init',
        data: prevData.toArray(),
        incidents: incidents
    }));
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
let currentWaterLevel = randGenerator(1, 0.2, 1);

setInterval(() => {
    let newPacket: DataPacket = {
        timeStamp: Date.now(),
        pressure: currentPressure.next().value,
        waterLevel: currentWaterLevel.next().value
    }
    prevData.pushpop(newPacket);
    let toDeliver = JSON.stringify({
        type: "data", 
        data: newPacket
    })
    for (let conn of conns) {
        conn.send(toDeliver);
    }
}, 1000 / hertz)

let incidents: Incident[] = [];

while (true) {
    let trigger = question('alert?');
    if (trigger.trim() === 'y') {
        let incident = {
            timeStamp: Date.now(),
            height: prevData.get(0).waterLevel
        };
        incidents.push(incident);
        for (let conn of conns) {
            conn.send(JSON.stringify({
                type: 'alert',
                data: incident
            }))
        }

    }
}

type Incident = {
    timeStamp: number;
    height: number;
}