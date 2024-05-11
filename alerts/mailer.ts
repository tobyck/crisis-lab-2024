import { createTransport } from 'nodemailer';
import type { MailOptions } from 'nodemailer/lib/sendmail-transport';
import { readFile } from 'fs';
import { promisify } from 'util';
const readFileAsync = promisify(readFile);

// create reusable transporter object using the default SMTP transport
const transporter = createTransport({
  service: "gmail",
  host: "smtp.gmail.com",
  port: 587,
  secure: false,
  auth: {
    user: "crisislab2024@gmail.com",
    pass: "incorrect horse capacitor nail",
  },
});

const sendMail = async (mailDetails: MailOptions) => {
    try {
        const info = await transporter.sendMail(mailDetails);
    } catch (error) {
        console.log(error);
    } 
};

export async function sendEmail() {
    let recipients = await readFileAsync('./mail.txt', {encoding: 'utf-8'});
    
}