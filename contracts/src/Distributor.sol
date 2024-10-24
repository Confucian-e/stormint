// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/**
 * @dev Transaction struct for the transaction payload.
 */
struct Transaction {
    address payable recipient;
    uint256 amount;
}

/**
 * @dev Error that occurs when transferring ether has failed.
 * @param sender The address that attempted to transfer the ether.
 * @param receiver The address that was supposed to receive the ether.
 */
error EtherTransferFail(address sender, address receiver);

/**
 * @title Native Token Distributor
 * @author Confucian-e
 * @notice Helper smart contract for batch sending native tokens.
 */
contract Distributor {
    /**
     * @dev You can cut out 10 opcodes to save gas in the creation-time EVM bytecode
     * if you declare a constructor `payable`.
     *
     * For more in-depth information see here:
     * https://forum.openzeppelin.com/t/a-collection-of-gas-optimisation-tricks/19966/5.
     */
    constructor() payable {}

    /**
     * @dev Distributes ether, denominated in wei, to a predefined batch
     * of recipient addresses.
     * @notice In the event that excessive ether is sent, the residual
     * amount is returned back to the `msg.sender`.
     * @param txns the array of Transaction
     */
    function distributeEther(Transaction[] calldata txns) external payable {
        address sender = address(this);
        /**
         * @dev Caching the length in for loops saves 3 additional gas
         * for a `calldata` array for each iteration except for the first.
         */
        uint256 length = txns.length;

        /**
         * @dev If a variable is not set/initialised, it is assumed to have
         * the default value. The default value for the `uint` types is 0.
         */
        for (uint256 i; i < length; ++i) {
            // solhint-disable-next-line avoid-low-level-calls
            address recipient = txns[i].recipient;
            uint256 amount = txns[i].amount;
            (bool sent,) = recipient.call{value: amount}("");
            require(sent, EtherTransferFail(sender, recipient));
        }

        uint256 balance = sender.balance;
        if (balance != 0) {
            address receiver = msg.sender;
            /**
             * @dev Any wei amount previously forced into this contract (e.g. by
             * using the `SELFDESTRUCT` opcode) will be part of the refund transaction.
             */
            // solhint-disable-next-line avoid-low-level-calls
            (bool refunded,) = payable(receiver).call{value: balance}("");
            require(refunded, EtherTransferFail(sender, receiver));
        }
    }
}
