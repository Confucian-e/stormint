// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {Distributor} from "../src/Distributor.sol";

contract DistributorScript is Script {
    Distributor public distributor;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        distributor = new Distributor();

        vm.stopBroadcast();
    }
}
