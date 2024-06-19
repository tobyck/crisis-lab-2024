const { Events } = require('discord.js');

export const postDiscord = async(client, message) => {
    client.once(Events.ClientReady, readyClient => {
        client.channels.cache.forEach(channel => {
            if (channel.isTextBased()) {
                if (channel.name === 'tsunami-alerts')
                channel.send(message).then('sent').catch(console.error)
            }
        })
    })

}