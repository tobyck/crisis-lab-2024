const ws = require('ws');
const fs = require('fs');
const SerialPort = require('serialport');
const { question } = require('readline-sync');

// WebSocket variables
// const socket = new ws.WebSocket("ws://dashboard.alex-berry.net:8080");
let triggered = false;
let start = 0;

// Serial variables
let serialport = new SerialPort.SerialPort({ path: '/dev/ttyACM0', baudRate: 9600 });

/* serialport.on("open", () => {
	console.log("Successfully opened Serial Port.");
	let q;
	while (true) {
		q = question("Tsunami? ").trim();
		if (q == 'y') {
			console.log("Serial port is open? " + serialport.isOpen);
			console.log("Triggered");
			serialport.write("T\r");
		}
	}
}); */

while (true) {
	let q = question("Tsunami? ").trim();
	if (q == 'y') {
		console.log("Serial port is open? " + serialport.isOpen);
		console.log("Triggered");
		serialport.write("T\r");
	}

}

// WebSocket things
// socket.onopen = e => {
// 	console.log("Connected to WebSocket")
	
// };

// socket.onerror = e => {
// 	console.log("WebSocket Error: ", e)
// };



// socket.onmessage = e => {
// 	let JSONdata = JSON.parse(e.data);
// 	/* console.log("Recieved message: ", JSONdata); */

// 	// Tells us if a trigger has been sent
// 	triggered = (JSONdata.pressure == undefined && JSONdata.height != undefined);
// 	if(triggered) {
// 		console.log("Serial port is open? " + serialport.isOpen);
// 		console.log("Triggered");
// 		// Output via Serial to Arduino
// 		serialport.write("T\r");
// 	}

// };

// socket.onclose = e => {
// 	console.log("WebSocket connection has been closed successfully.", e)
// };
