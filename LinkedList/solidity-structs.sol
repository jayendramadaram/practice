// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.18;

interface ILinkedList {

        // as recursive Structs are not possible in solidity i wanna try this approach
    struct Node {
        uint next ;
        uint value ;
    }

    function add(uint _val) external returns (uint);
    function lookup(uint index) external view returns (uint);
    function _delete(uint index) external returns (bool);
    // function size() public view returns (uint256);

}

contract LinkedList is ILinkedList {
    

    Node public linkedList;
    Node[] public nodes;
    uint count;

    constructor () {
        linkedList =  Node(0 , 0);
        nodes.push(linkedList);
    }

    modifier check_len(uint index) {
        require(index < nodes.length , "index exceeded the limit");
        _;
    }

    function add(uint _val) external returns (uint) {
        count++;
        Node memory newNode = Node(count , _val);
        nodes.push(newNode);
        return count;
    }

    function lookup(uint index) external view check_len(index) returns (uint) {
        
        return nodes[index].value;
    }
    function _delete(uint index) external check_len(index) returns (bool){
        
        // if (index == nodes.length - 1) ;
        for(uint i = index ; i < nodes.length - 1 ; i++) {
            Node memory _node = nodes[index + 1];
            _node.next = index;
            nodes[index] = _node;
        }
        nodes.pop();
        count--;
        return true;
    }
    function size() external view returns (uint256) {
        return nodes.length;
    }

}