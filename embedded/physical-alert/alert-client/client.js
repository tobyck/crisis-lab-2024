const ws = require('ws');
const SerialPort = require('serialport');

const socket = new ws.WebSocket("ws://dashboard.alex-berry.net:8080");
const serialport = new SerialPort.SerialPort({ path: '/dev/ttyACM0', baudRate: 9600 });

socket.onopen = () => console.log("Connected to WebSocket");
socket.onerror = err => console.log("WebSocket Error: ", err);

socket.onmessage = message => {
	let { height, pressure } = JSON.parse(message.data);

	// Alerts are formatted as { height: 0.0, timestamp: 0 } whereas normal data is { pressure: 0.0, height: 0.0 }
	// So we use this to distinguish between the two
	let isAlertPacket = pressure == undefined && height != undefined;
	if (!isAlertPacket) return;

	if (serialport.isOpen) {
		serialport.write("T\r");
		console.log("Triggered alert");
	} else {
		console.log("Serial port is not open");
	}
};

socket.onclose = e => console.log("WebSocket closed", e);