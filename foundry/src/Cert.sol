// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract Cert {
    address admin;
    event Issued(string indexed course, uint256 id, string grade);

    constructor() {
        admin = msg.sender;
    }

    modifier onlyAdmin() {
        _onlyAdmin();
        _;
    }
         
    function _onlyAdmin() view internal {
        require(msg.sender == admin, "Access Denied");
    }

    struct Certificate {
        string name;
        string course;
        string grade;
        string date;
    }

    mapping(uint256 => Certificate) public certificates;

    function issue(
        uint256 _id,
        string memory _name,
        string memory _course,
        string memory _grade,
        string memory _date
    ) public onlyAdmin {
        certificates[_id] = Certificate({ name: _name, course: _course, grade: _grade, date: _date });
        emit Issued(_course, _id, _grade);
    }
}
