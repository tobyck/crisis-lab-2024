// Library imports
const ws = require('ws');
const SerialPort = require('serialport');

// WebSocket variables
const socket = new ws.WebSocket("ws://dashboard.alex-berry.net:8080");
let triggered = false;

// Serial variables
let serialport = new SerialPort.SerialPort({ path: '/dev/ttyACM0', baudRate: 9600 });

// WebSocket logs
socket.onopen = e => {
 	console.log("Connected to WebSocket")
	
};

socket.onerror = e => {
	console.log("WebSocket Error: ", e)
};

socket.onmessage = e => {
	// Data is sent from relay server in stringified JSON, convert it back to JSON
	let JSONdata = JSON.parse(e.data);

	/* console.log("Recieved message: ", JSONdata); */

	// Tells us if a trigger has been sent
	// If a trigger has been sent, the server will only send wave height, and not pressure.
	triggered = (JSONdata.pressure == undefined && JSONdata.height != undefined);

	if(triggered) {
		console.log("Serial port is open? " + serialport.isOpen);
		console.log("Triggered");
		// Output trigger via Serial to Arduino
		serialport.write("T\r");
	}

};

socket.onclose = e => {
	console.log("WebSocket connection has been closed successfully.", e)
};
