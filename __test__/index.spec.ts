import test from 'ava'

import { Client } from '../index'

test('smoke test, list deployments returns nothing', async (t) => {
  // Create client
  let client = await Client.connect();

  // List deployments
  let deployments = await client.listDeployments();

  // Verify no deployments are returned
  t.is(deployments.length, 0);
})
