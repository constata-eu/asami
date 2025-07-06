import { createContext, useContext } from "react";
import { useSearchParams } from "react-router-dom";

const EmbeddedContext = createContext(true);

export const EmbeddedProvider = ({ children }) => {
  const isEmbedded = localStorage.getItem("embedded");

  return (
    <EmbeddedContext.Provider value={isEmbedded}>
      {children}
    </EmbeddedContext.Provider>
  );
};

export const useEmbedded = () => useContext(EmbeddedContext);
