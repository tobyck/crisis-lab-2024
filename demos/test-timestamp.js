import { exec } from "child_process";

exec(`mosquitto_pub -h 170.64.254.27 -t data -u sensor -P $PASSWORD -m "T ${Date.now()}"`)
