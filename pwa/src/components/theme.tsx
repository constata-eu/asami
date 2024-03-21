import { defaultTheme, useTranslate } from 'react-admin';
import { Box, styled } from '@mui/material';

import LeagueSpartanBlack from "../assets/League_Spartan/static/LeagueSpartan-Black.ttf";
import LeagueSpartanExtraBold from "../assets/League_Spartan/static/LeagueSpartan-ExtraBold.ttf";
import LeagueSpartanBold from "../assets/League_Spartan/static/LeagueSpartan-Bold.ttf";
import LeagueSpartanSemiBold from "../assets/League_Spartan/static/LeagueSpartan-SemiBold.ttf";
import LeagueSpartanMedium from "../assets/League_Spartan/static/LeagueSpartan-Medium.ttf";
import LeagueSpartanRegular from "../assets/League_Spartan/static/LeagueSpartan-Regular.ttf";
import LeagueSpartanLight from "../assets/League_Spartan/static/LeagueSpartan-Light.ttf";
import LeagueSpartanExtraLight from "../assets/League_Spartan/static/LeagueSpartan-ExtraLight.ttf";
import LeagueSpartanThin from "../assets/League_Spartan/static/LeagueSpartan-Thin.ttf";
import Sacramento from "../assets/Sacramento/Sacramento-Regular.ttf";

let fontFaces = [
  [LeagueSpartanBlack, "Black", 900],
  [LeagueSpartanExtraBold, "ExtraBold", 800],
  [LeagueSpartanBold, "Bold", 700],
  [LeagueSpartanSemiBold, "SemiBold", 600],
  [LeagueSpartanMedium, "Medium", 500],
  [LeagueSpartanRegular, "Regular", 400],
  [LeagueSpartanLight, "Light", 300],
  [LeagueSpartanExtraLight, "ExtraLight", 200],
  [LeagueSpartanThin, "Thin", 100],
].map((font) => 
  `@font-face {
    font-family: 'LeagueSpartan${font[1]}';
    font-weight: ${font[2]};
    src: local('LeagueSpartan'), local('LeagueSpartan-${font[1]}'), url("${font[0]}") format('truetype');
    unicodeRange: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF;
  }`
);

fontFaces.push(` @font-face {
  font-family: 'Sacramento';
  font-weight: 500;
  src: local("Sacramento"), local("Sacramento-Regular"), url("${Sacramento}") format('truetype');
  unicodeRange: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF;
}`);

export const light = '#fafafb';
export const green = '#2ec4b6';
export const yellow = '#ff9f1c';
export const dark = '#011627';
export const red = '#e71d36';

const theme = {
  ...defaultTheme,
  typography: {
    fontFamily: '"LeagueSpartanLight", serif',
  },
  palette: {
    mode: 'dark',
    primary: { main: green },
    secondary: { main: yellow },
    inverted: { main: light, contrastText: dark },
    error: { main: red },
  },
  components: {
    MuiAlert: {
      styleOverrides: {
        root: {
          "& .MuiAlert-message": {
            flex: "1 1 auto !important"
          }
        }
      }
    },
    MuiCssBaseline: {
      styleOverrides: `.twitter-tweet iframe { border-radius: 13px; } ${fontFaces.join("\n")}`
    },
  }
};

export default theme;

export const Head1 = styled("h1")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "40px",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  [theme.breakpoints.up('md')]: {
    fontSize: "60px",
  },
  margin: "0",
}));

export const Head2 = styled("h2")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "20px",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  [theme.breakpoints.up('md')]: {
    fontSize: "30px",
  },
  margin: 0,
}));

export const CardTitle = ({text, ...props}) => {
  const translate = useTranslate();

  return(<Box {...props} sx={{ p: 2, borderBottom: "2px solid", borderColor: green }}>
    <Head2>{ translate(text, { _: text }) }</Head2>
    { props.children }
  </Box>)
}
