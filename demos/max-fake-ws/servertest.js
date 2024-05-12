import { WebSocketServer } from 'ws';

const ws = new WebSocketServer({ host: "127.0.0.1", port: 8080 });

console.log("Created new WebSocketServer on port " + ws.options.port);
console.log(ws.options.host);

ws.on('connection', ws => {
	console.log("WebSocket Connection accepted.");
	ws.send("Welcome from "+ws.options.host+":"+ws.options.port+"!");

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
