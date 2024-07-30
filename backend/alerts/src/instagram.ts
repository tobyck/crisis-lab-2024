/*
 * Author: Alex Berry
 * Version: 31/07/2024
 * Purpose: Log onto instagram, and make a post with the tsunami icon picture, and the caption
 */
import { IgApiClient } from 'instagram-private-api';
import { readFile } from 'fs';
import { promisify } from 'util';
const readFileAsync = promisify(readFile);

export const postInstagram = async (message: string) => {
    let { IG_USERNAME, IG_PASSWORD } = process.env; // Read Instagram username and password from .env file
    if (!IG_USERNAME || !IG_PASSWORD) { // If they don't exist error
        console.log('Instagram credentials not found');
        return;
    }
    try {
        const ig = new IgApiClient(); // Create a new instagram API client
        ig.state.generateDevice(IG_USERNAME); // Generate a fake device's metadata to log the instagram connection as coming from
        await ig.account.login(IG_USERNAME, IG_PASSWORD); // Log into the account
        const path = './tsunami-icon.jpg';
        // Publish the photo to instagram with the specified caption
        await ig.publish.photo({
            file: await readFileAsync(path),
            caption: message
        });
    } catch (error) {
        console.log('Failed to post to Instagram', error);
    }
}