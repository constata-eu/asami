import asamiABI from "./abi/Asami.json";
import docABI from "./abi/Doc.json";
import { createContext } from "react";
import { ethers } from "ethers";
import asamiABI from "../abi/Asami.json";
import docABI from "../abi/Doc.json";

const useContracts = async () => {
  // staging
  // const docAddress = "0xcb46c0ddc60d18efeb0e586c17af6ea36452dae0";
  // const asamiAddress = "0x16039AB4E9b0BF3b79F9a221898d152151026034";
  // development
  const docAddress = "0x02DD036B7D0AF40B2b85DE802BfdF4ba2FE4e789";
  const asamiAddress = '0xC63459e67f94f1f7055aa7ae988980D81E5374E4';

  const provider = new ethers.BrowserProvider(window.ethereum);
  const signer = await provider.getSigner();

  const asami = new ethers.Contract(asamiAddress, asamiABI.abi, provider);
  const doc = new ethers.Contract(docAddress, docABI, signer);

  return {asami, asamiAddress, doc, docAddress, signer, provider};
}

export {useContracts};
