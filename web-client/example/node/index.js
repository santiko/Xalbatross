const Nimiq = require("@nimiq/core");

async function main() {
    const config = new Nimiq.ClientConfiguration();
    // config.logLevel('debug');

    const client = await Nimiq.Client.create(config.build());

    setInterval(async () => {
        const consensus = await client.isConsensusEstablished();
        console.log(`Consensus ${consensus ? 'established' : 'not established'}`);
    }, 1000);
}

main();
