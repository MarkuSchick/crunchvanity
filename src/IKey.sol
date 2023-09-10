// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

/**
 * @dev Interface of the ERC1271 standard signature validation method for
 * contracts as defined in https://eips.ethereum.org/EIPS/eip-1271[ERC-1271].
 *
 * _Available since v4.1._
 */

 type Password is bytes32;
interface IKey {

    function SolveThePuzzleOfCoastWithImpressionInNightAndSquallOnCayAndEndToVictory(address owner, Password password)
        external
        view
        returns (bytes4);
}
