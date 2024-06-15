import { createTransport } from 'nodemailer';
import type { MailOptions } from 'nodemailer/lib/sendmail-transport';
import { readFile, writeFile } from 'fs';
import { promisify } from 'util';
import { v4 } from 'uuid';
const readFileAsync = promisify(readFile);
const writeFileAsync = promisify(writeFile);

let DEBUG = process.env.DEBUG === "true";

// create reusable transporter object using the default SMTP transport
// this may randomly not work it's fine
const transporter = createTransport({
    service: "gmail",
    host: "smtp.gmail.com",
    port: 465,
    secure: true,
    tls: {
        ciphers: 'SSLv3',
        rejectUnauthorized: false
    },
    requireTLS: true,
    auth: {
        user: process.env.EMAIL,
        pass: process.env.EMAIL_PASSWORD
    },
    logger: DEBUG,
    debug: DEBUG,
});

const sendMail = async (mailDetails: MailOptions) => {
    try {
        const info = await transporter.sendMail(mailDetails);
    } catch (error) {
        console.log('Failed to send email', error);
    }
};

export async function sendEmail(message: string) {
    let recipients = JSON.parse(await readFileAsync('mail-list.json', { encoding: 'utf-8' }));
    for (let [uid, email] of Object.entries(recipients)) {
        //console.log(email, uid);
        const options = {
            from: "Crisis Lab 2024 Tsunami Mail <crisislab2024@gmail.com>",
            to: email as string,
            subject: 'FAKE TSUNAMI DETECTED',
            text: message,
            html: message + `<br><a href="https://dashboard.alex-berry.net:8783/unsubscribe?uuid=${uid}">Unsubscribe</a>`
        };
        sendMail(options);
    }
}

export async function addEmail(email: string) {
    let list = JSON.parse(await readFileAsync('mail-list.json', { encoding: 'utf-8' }));
    let uid = v4();
    list[uid] = email;
    await writeFileAsync('mail-list.json', JSON.stringify(list));
}

export async function removeEmail(uuid: string) {
    let list = JSON.parse(await readFileAsync('mail-list.json', { encoding: 'utf-8' }));
    delete list[uuid];
    await writeFileAsync('mail-list.json', JSON.stringify(list));
}