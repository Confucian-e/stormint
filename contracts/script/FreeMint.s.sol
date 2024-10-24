// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {Script} from "forge-std/Script.sol";
import {FreeMint} from "../src/FreeMint.sol";

contract FreeMintScript is Script {
    FreeMint public freeMint;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        freeMint = new FreeMint();

        vm.stopBroadcast();
    }
}
