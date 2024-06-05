import { serve, sha } from "bun"
import { sendEmail, addEmail, removeEmail } from "./mailer";
import { postInstagram } from "./instagram";

// yes, seriously
const EMAILREGEX = /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/

serve({
    port: 8783,
    async fetch(req) {
        const url = new URL(req.url);
        if (url.pathname === "/subscribe") {
            // subscribe email
            let email = url.searchParams.get("email");
            if (email !== null && EMAILREGEX.test(email as string)) {
                addEmail(email);
                return new Response("Subscribed!");
            } else {
                return new Response("Invalid email", { status: 400 });
            }
        }
        if (url.pathname === "/unsubscribe") {
            // unsubscribe email
            let uuid = url.searchParams.get("uuid");
            if (uuid !== null) {
                removeEmail(uuid);
                return new Response("Unsubscribed!");
            } else {
                return new Response("Invalid UUID", { status: 400 });
            }
        }
        if (url.pathname === "/blog") return new Response("Blog!");
        if (url.pathname === "/alert") {
            if (req.method == "GET") {
                return new Response("Tried to send alert via GET", { status: 400 });
            }
            // TODO: password

            let message = `WARNING A FAKE TSUNAMI OF HEIGHT ${url.searchParams.get("height")}cm HAS BEEN RECORDED`;

            sendEmail(message);
            postInstagram(message);
        }
        return new Response("Not found", { status: 404 });
    },
});
