// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {Cert} from "../src/Cert.sol";

contract CounterTest is Test {
    Cert public cert;

    function setUp() public {
        cert = new Cert();
    }

    function test_Issue() public {
        vm.expectEmit(true, false, false, false);
        emit Cert.Issued("MBCC", 105, "2025");
        cert.issue(105, "Jasmine", "MBCC", "S", "2023");
    }

    function test_Certificates() public {
        cert.issue(395, "Levy", "MBCC", "A", "2023");
        string memory name;
        string memory course;
        string memory grade;
        string memory date;
        (name, course, grade, date) = cert.Certificates(395);
        Cert.Certificate memory cc = Cert.Certificate(
            "Levy",
            "MBCC",
            "A",
            "2023"
        );
        assertEq(name, cc.name);
        assertEq(course, cc.course);
        assertEq(grade, cc.grade);
        assertEq(date, cc.date);
    }

    function test_Revert() public {
        vm.expectRevert("Access Denied");
        vm.prank(address(0));
        cert.issue(250, "Pepper", "MBCC", "B", "2022");
    }
}
