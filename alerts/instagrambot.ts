require("dotenv").config();
const { IgApiClient } = require('instagram-private-api');
const { get } = require('request-promise');
import { readFile } from 'fs';
import { promisify } from 'util';
const readFileAsync = promisify(readFile);


export function postToInsta(height: string) {
    let postToInsta = async (height: string) => {
        const ig = new IgApiClient();
        ig.state.generateDevice(process.env.IG_USERNAME);
        await ig.account.login(process.env.IG_USERNAME, process.env.IG_PASSWORD);
        const path = './tsunami-icon.jpg';
        await ig.publish.photo({
            file: await readFileAsync(path),
            caption: 'WARNING A FAKE TSUNAMI OF HEIGHT '+height +'cm HAS BEEN RECORDED',
        
        });
    }
    
}
