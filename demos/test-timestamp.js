import { exec } from "child_process";

exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P rVcL1OjYHeJApPsA4fT9 -m "T ${Date.now()}"`)
