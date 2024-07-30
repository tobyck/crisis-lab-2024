/*
 * Author: Alex Berry & Theo Keith
 * Version: 31/07/2024
 * Purpose: Create a discord bot client, and ready it for sending messages
 */
import { Events, Client, TextChannel } from "discord.js";

const client = new Client({
    intents: ["Guilds", "GuildMessages", "DirectMessages"], // Discordjs labels servers as guilds
});
client.login(process.env.DISCORD_TOKEN)

let channel: TextChannel | null = null;

// Setup process for when the bot connection is initialised
client.once(Events.ClientReady, readyClient => {
    console.log('Discord bot ready');
    channel = readyClient.channels.cache.find(channel =>
        channel instanceof TextChannel && channel.name === 'tsunami-alerts' // Set the channel to post in to be any thats called tsunami-alerts
    ) as TextChannel;
    postDiscord('Bot online!');
})

// Error if the bot can't find any channels
export const postDiscord = async (message: string) => {
    if (channel === null) {
        console.error('Channel not found');
        return;
    }
    await channel.send(message);
}
