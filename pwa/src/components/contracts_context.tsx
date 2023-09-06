import asamiABI from "./abi/Asami.json";
import docABI from "./abi/Doc.json";
import { createContext } from "react";
import { ethers } from "ethers";
import asamiABI from "../abi/Asami.json";
import docABI from "../abi/Doc.json";

const useContracts = async () => {
  // staging
  const docAddress = "0xcb46c0ddc60d18efeb0e586c17af6ea36452dae0";
  const asamiAddress = "0x0E8033f453775AE150c5c6643b1691d2B07B3bAa";
  // development
  // const asamiAddress = "0x7f96b3050d42b1F51294d029bAA959D1E5d2a4cb";
  // const docAddress = "0xd31C3320e2ACD74BE43c26645D5e1C0FAfE09B96";


  const provider = new ethers.BrowserProvider(window.ethereum);
  const signer = await provider.getSigner();

  const asami = new ethers.Contract(asamiAddress, asamiABI.abi, provider);
  const doc = new ethers.Contract(docAddress, docABI, signer);

  return {asami, asamiAddress, doc, docAddress, signer, provider};
}

export {useContracts};
