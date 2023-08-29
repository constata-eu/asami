import {
  Admin,
  Resource,
  ListGuesser,
  EditGuesser,
  ShowGuesser,
} from "react-admin";
import { authProvider } from "./authProvider";

export const App = () => <Admin authProvider={authProvider}></Admin>;
