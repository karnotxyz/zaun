pragma solidity ^0.8.0;
import "./ERC20.sol";

contract ERC20_1 is ERC20 {
	constructor() {
		_mint(msg.sender, 100000000000); 
	}
}
