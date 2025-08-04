import { Client } from "../index.js";

const client = await Client.connect();

const deployments = await client.listDeployments();

for (const deployment of deployments) {
    console.log("Deployment: ");
    console.log(deployment.containerId);
    console.log(deployment.creationSource);
}
