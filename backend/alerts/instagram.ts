import { IgApiClient } from 'instagram-private-api';
import { readFile } from 'fs';
import { promisify } from 'util';
const readFileAsync = promisify(readFile);

export const postInstagram = async (message: string) => {
    let { IG_USERNAME, IG_PASSWORD } = process.env;
    if (IG_USERNAME === undefined || IG_PASSWORD === undefined) {
        console.log('Instagram credentials not found');
        return;
    }
    const ig = new IgApiClient();
    ig.state.generateDevice(IG_USERNAME);
    await ig.account.login(IG_USERNAME, IG_PASSWORD);
    const path = './tsunami-icon.jpg';
    await ig.publish.photo({
        file: await readFileAsync(path),
        caption: message
    });
}