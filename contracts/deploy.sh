#!/bin/bash

echo "Deploying EnclavaPayments to Duckchain..."

forge script script/DeployEnclavaPayments.s.sol:DeployEnclavaPayments \
  --rpc-url duckchain_mainnet \
  --broadcast 


echo -e "\n\nDeployed EnclavaPayments to Duckchain!"