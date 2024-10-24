// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {FreeMint} from "../src/FreeMint.sol";

contract FreeMintTest is Test {
    FreeMint public freeMint;

    function setUp() public {
        freeMint = new FreeMint();
    }

    function test_mint() public {
        address alice = makeAddr("Alice");
        vm.prank(alice, alice);
        freeMint.mint();
        assertEq(freeMint.balanceOf(alice), freeMint.MINT_AMOUNT());
    }

    function testFail_doubleMint() public {
        address alice = makeAddr("Alice");
        vm.startPrank(alice, alice);

        freeMint.mint();
        freeMint.mint();
    }
}
