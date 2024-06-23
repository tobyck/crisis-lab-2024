import { serve, type ServerWebSocket } from "bun"
import { sendEmail, addEmail, removeEmail } from "./mailer";
import { postInstagram } from "./instagram";
import { postDiscord } from "./discord";

// yes, seriously
const EMAILREGEX = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/

let DEBUG = process.env.DEBUG === "true";

let constantTimeCompare = (a: string, b: string) => {
    if (a.length < b.length) [a, b] = [b, a];
    let result = 0;
    for (let i = 0; i < a.length; i++) {
        result |= a.charCodeAt(i) ^ (b.charCodeAt(i) ?? 0);
    }
    return result === 0;
}

// Adds CORS headers to a response
let cors = (res: Response) => {
    res.headers.set('Access-Control-Allow-Origin', '*');
    res.headers.set('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, OPTIONS');
    return res;
}

const conns: ServerWebSocket<any>[] = [];
// This won't handle http->https redirects, but it's fine since all it is is an API
serve({
    port: 8783,
    async fetch(req, server) {
        const url = new URL(req.url);

        if (url.pathname === "/subscribe") {
            // subscribe email
            let email = url.searchParams.get("email");
            if (DEBUG) console.log('Subscribing', email);
            if (email !== null && EMAILREGEX.test(email as string)) {
                addEmail(email);
                return cors(new Response("Subscribed!"));
            } else {
                return cors(new Response("Invalid email", { status: 400 }));
            }
        }

        if (url.pathname === "/unsubscribe") {
            let uuid = url.searchParams.get("uuid");
            if (DEBUG) console.log('Unsubscribing', uuid);
            if (uuid !== null) {
                removeEmail(uuid);
                return cors(new Response("Unsubscribed!"));
            } else {
                return cors(new Response("Invalid UUID", { status: 400 }));
            }
        }

        if (url.pathname === "/blog") return cors(new Response("Blog!"));

        if (url.pathname === "/alert") {
            if (req.method == "GET") {
                return cors(new Response("Tried to send alert via GET", { status: 400 }));
            }
            let json = await req.json();
            if (json.password === undefined || typeof json.password !== "string") {
                return cors(new Response("No password provided", { status: 400 }));
            }

            if (!constantTimeCompare(json.password, process.env.ALERT_PASSWORD as string)) {
                return cors(new Response("Incorrect password", { status: 401 }));
            }

            if (json.height === undefined) {
                return cors(new Response("No height provided", { status: 400 }));
            }

            if (typeof json.height !== "number") {
                return cors(new Response("Invalid height", { status: 400 }));
            }

            let message = `WARNING A FAKE TSUNAMI OF HEIGHT ${json.height}cm HAS BEEN RECORDED`;
            console.log('Triggering alert', message)

            sendEmail(message);
            postInstagram(message);
            postDiscord(message);
            return cors(new Response("Alert sent!"));
        }

        // The sole purpose of the websocket is for the alerts "online" indicator
        // it pings once a second
        if (url.pathname === "/ws") {
            if (server.upgrade(req)) {
                return new Response(null, { status: 101 });
            }
            return new Response("Upgrade failed", { status: 500 });
        }

        return new Response("Not found", { status: 404 });
    },
    websocket: {
        open(ws) {
            if (DEBUG) console.log('New connection');
            conns.push(ws);
            ws.send('ping');
        },
        message() { }, // fuck off
        close(ws) {
            conns.splice(conns.indexOf(ws));
            if (DEBUG) console.log('Connection closed');
        }
    },
    tls: {
        cert: Bun.file("../../ssl/certificate.crt"),
        key: Bun.file("../../ssl/private.key"),
    }
});

setInterval(() => {
    for (let conn of conns) {
        conn.send('ping')
    }
}, 1000)
