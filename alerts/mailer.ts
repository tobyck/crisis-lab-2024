import { createTransport } from 'nodemailer';
import type { MailOptions } from 'nodemailer/lib/sendmail-transport';
import { readFile } from 'fs';
import { promisify } from 'util';
const readFileAsync = promisify(readFile);

// create reusable transporter object using the default SMTP transport
const transporter = createTransport({
  service: "gmail",
  host: "smtp.gmail.com",
  port: 465,
  secure: true,
  auth: {
    user: process.env.EMAIL,
    pass: process.env.PASSWORD
  },
});

const sendMail = async (mailDetails: MailOptions) => {
    try {
        const info = await transporter.sendMail(mailDetails);
    } catch (error) {
        console.log(error);
    } 
};

export async function sendEmail(height: string) {
    let recipients = await readFileAsync('mail-list.txt', {encoding: 'utf-8'});
    const message = 'WARNING A FAKE TSUNAMI OF HEIGHT ' + height + 'cm HAS BEEN RECORDED';
    const options = {
        from: "Crisis Lab 2024 Tsunami Mail <crisislab2024@gmail.com>", // sender address
        bcc: recipients, // receiver email
        subject: 'FAKE TSUNAMI DETECTED', // Subject line
        text: message,
        html: message
    };
    sendMail(options);
}