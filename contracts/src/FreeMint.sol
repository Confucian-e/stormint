// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {ERC20} from "@openzeppelin-contracts-5.1.0/token/ERC20/ERC20.sol";

/**
 * @title FreeMint Token
 * @author Daram Meme Coin
 * @notice This is a free executor token for Daram Meme Coin.
 * @dev This contract is used to executor tokens for free.
 */
contract FreeMint is ERC20 {
    uint256 public constant MAX_SUPPLY = 210000000000 * 1 ether;
    uint256 public constant MINT_AMOUNT = 5000000 * 1 ether;

    mapping(address => bool) private hasMinted;

    constructor() ERC20("Daram", "Daram") {}

    function mint() external {
        require(totalSupply() + MINT_AMOUNT <= MAX_SUPPLY, "Total supply exceeded");
        require(!hasMinted[msg.sender], "Address has already minted");
        require(msg.sender == tx.origin, "Contracts are not allowed to mint");

        hasMinted[msg.sender] = true;
        _mint(msg.sender, MINT_AMOUNT);
    }
}
