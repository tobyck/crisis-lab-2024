import { Events, Client, TextChannel } from "discord.js";

const client = new Client({
    intents: ["Guilds", "GuildMessages", "DirectMessages"],
});
client.login(process.env.DISCORD_TOKEN)

let channel: TextChannel | null = null;

client.once(Events.ClientReady, readyClient => {
    console.log('Discord bot ready');
    channel = readyClient.channels.cache.find(channel =>
        channel instanceof TextChannel && channel.name === 'tsunami-alerts'
    ) as TextChannel;
    postDiscord('Bot online!');
})

export const postDiscord = async (message: string) => {
    if (channel === null) {
        console.error('Channel not found');
        return;
    }
    await channel.send(message);
}
