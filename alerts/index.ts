import {postToInsta} from './instagrambot';

if (prompt("Would you like to post to instagram? (y/n?)") === "y") {
    process.stdout.write("How tall is the wave? (in cm) ");
    for await (const line of console) {
        process.stdout.write("Making an instagram post now with the caption: \n");
        process.stdout.write("\"WARNING A FAKE TSUNAMI OF HEIGHT " + line + "cm HAS BEEN RECORDED\"");
        postToInsta(line)
        break;
    }
} else {
    process.stdout.write("Alright, I'll do nothing then");
}