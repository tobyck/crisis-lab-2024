//const WebSocketServer = require('ws');

import { WebSocketServer } from 'ws';

const wss = new WebSocketServer({ host: "192.168.1.39", port: 80 });

console.log("Created new WebSocketServer on port " + wss.options.port);
console.log(wss.options.host);

wss.on('connection', ws => {
	console.log("WebSocket Connection accepted.");
	ws.send("Welcome from "+wss.options.host+":"+wss.options.port+"!");

	ws.on('error', err => {
		console.log(err);
	})

	ws.on('close', () => {
		console.log("Connection Closed.");
	})
	ws.on('message', msg => {
		if(!msg.isBinary) {
			console.log("Recieved: '" + msg + "'");
			ws.send("Echo: '" + msg + "'");
		}
		else if(msg.isBinary) {
			console.log("Recieved Binary message, '" + msg.length + " bytes.");
			ws.send("Echo: '" + msg + "'");
		}
	})
});