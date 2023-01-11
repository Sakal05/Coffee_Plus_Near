import { useContract, useConfig } from './utils';

const MyComponent = () => {
  const { contract } = useContract('my-contract');
  const { networkId } = useConfig();

  const handleSubmit = async event => {
    event.preventDefault();
    const receiptId = event.target.elements.receiptId.value;
    const cupNumber = event.target.elements.cupNumber.value;
    await contract.coupon_generator(receiptId, cupNumber);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Receipt ID:
        <input type="text" name="receiptId" />
      </label>
      <br />
      <label>
        Cup Number:
        <input type="number" name="cupNumber" />
      </label>
      <br />
      <button type="submit">Generate Coupon</button>
    </form>
  );
};
