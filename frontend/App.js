import "regenerator-runtime/runtime";
import React from "react";
import React, { useState, useEffect } from "react";
import { Button, Modal, Form, FloatingLabel } from "react-bootstrap";
import { Card, Button, Col, Badge, Stack } from "react-bootstrap";
import "bootstrap";
//import "bootstrap-icons/font/bootstrap-icons.css";
import "bootstrap/dist/css/bootstrap.min.css";
import "./assets/global.css";

import { EducationalText, SignInPrompt, SignOutButton } from "./ui-components";

export default function App({ isSignedIn, contractId, wallet }) {
  //const [uiPleaseWait, setUiPleaseWait] = useState(false);

  // const [show, setShow] = useState(true);
  // const handleClose = () => setShow(false);
  // const handleShow = () => setShow(true);

  const [receiptId, setReceiptId] = useState("");
  const [cupNum, setCupNum] = useState("");

  const isFormFilled = () => receiptId && cupNum;
  // Get blockchian state once on component load
  // useEffect(() => {
  //   getCoupons();
  // }, []);

  /// If user not signed-in with wallet - show prompt
  if (!isSignedIn) {
    // Sign-in flow will reload the page later
    return <SignInPrompt onClick={() => wallet.signIn()} />;
  }

  function getCoupons(info) {
    // use the wallet to query the coupon generated
    //const { creator, coupon_id, cup_number, discount} = info;
    wallet.viewMethod({ method: "get_coupons", contractId });
    return (
<>
{/* <Col key={1}>
      <Card className=" h-100">
        <Card.Header>
          <Stack direction="horizontal" gap={2}>
            <span className="font-monospace text-secondary">{owner}</span>
            <Badge bg="secondary" className="ms-auto">
              {creator} Creator
            </Badge>
          </Stack>
        </Card.Header>
        <Card.Body className="d-flex  flex-column text-center">
          <Card.Title>Coupon {coupon_id}</Card.Title>
          <Card.Text className="flex-grow-1 ">Amount of Coffee: {cup_number}</Card.Text>
          <Card.Text className="text-secondary">
            <span>Discount: {discount}%</span>
          </Card.Text>
        </Card.Body>
      </Card>
    </Col> */}
</>
    );
  }
  function generateCoupon() {
    // use the wallet to send the greeting to the contract
    wallet
      .callMethod({
        method: "coupon_generator",
        args: { r_id: receiptId, cup_num: parseInt(cupNum) },
        contractId,
      })
      .then(async () => {
        return generateCoupon();
      })
      .then(getCoupons)
      .finally(() => {
        getCoupons();
      });
  }

  

  return (
    <>
      <h2>Welcome to Coffee Incentive Porgram</h2>
      <Button
        accountId={wallet.accountId}
        onClick={() => wallet.signOut()}
        className="mt-5"
      >
        {" "}
        Sign Out {wallet.accountId}
      </Button>

      <Form>
        <Modal.Body centered>
          <h3 className="mt-5">Receipt Number</h3>
          <Form.Control
            className="mb-5 "
            type="Number"
            onChange={(e) => {
              setReceiptId(e.target.value);
            }}
            placeholder="Enter Receipt Number"
          />
          <h3 className="mt-5">Amount of Cup</h3>
          <Form.Control
            type="Number"
            className="mb-5"
            placeholder="Amount of Cup"
            onChange={(e) => {
              setCupNum(e.target.value);
            }}
          />
        </Modal.Body>
      </Form>
      <Modal.Footer>
        <Button
          variant="dark"
          disabled={!isFormFilled()}
          onClick={() => {
            generateCoupon();
            getCoupons();
          }}
        >
          Generate Coupon
        </Button>
      </Modal.Footer>
    </>
  );
}
