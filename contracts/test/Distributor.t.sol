// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {Distributor, Transaction} from "../src/Distributor.sol";

contract DistributorTest is Test {
    Distributor public distributor;

    function setUp() public {
        distributor = new Distributor();
    }

    function test_distributeEther() public {
        address sender = makeAddr("Sender");

        address alice = makeAddr("Alice");
        address bob = makeAddr("Bob");

        Transaction[] memory txns = new Transaction[](2);

        uint256 amount_alice = 1 ether;
        txns[0] = Transaction(payable(alice), amount_alice);

        uint256 amount_bob = 2 ether;
        txns[1] = Transaction(payable(bob), amount_bob);

        uint256 totalAmount;
        for (uint256 i; i < txns.length; i++) {
            totalAmount += txns[i].amount;
        }
        deal(sender, totalAmount);
        distributor.distributeEther{value: totalAmount}(txns);

        assertEq(alice.balance, amount_alice);
        assertEq(bob.balance, amount_bob);
    }
}
