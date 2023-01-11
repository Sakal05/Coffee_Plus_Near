import { useNear } from 'react-near-hooks';

export const useContract = contractName => {
  const { contract, near } = useNear();
  if (!contract) {
    throw new Error('contract is not loaded');
  }
  return { contract, near };
};

export const useConfig = () => {
  return require('../config.json');
};
