import merge from 'lodash/merge';
import { defaultTheme, useTranslate, useTheme } from 'react-admin';
import { Box, Typography, createTheme, styled } from '@mui/material';
import KeyboardDoubleArrowRightIcon from '@mui/icons-material/KeyboardDoubleArrowRight';

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

import _ from 'lodash';

export const light = '#fafafb';
export const green = '#2ec4b6';
export const yellow = '#ff9f1c';
export const dark = '#011627';
export const red = '#e71d36';
// Opacity: 7%.
//

const theme = {
  ...defaultTheme,
  typography: {
    fontFamily: '"LeagueSpartanLight", serif',
  },
  palette: {
    mode: 'dark',
    primary: { main: green },
    secondary: { main: yellow },
    inverted: { main: light },
    error: { main: red },
  },
  components: {
    MuiBox: {
      styleOverrides: {
        "& iframe body": {
          background: "red",
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
  color: light,
  fontSize: "20px",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  [theme.breakpoints.up('md')]: {
    fontSize: "30px",
  },
  margin: 0,
}));

export const CardTitle = ({text, ...props}) => {
  const [theme] = useTheme();
  const translate = useTranslate();

  return(<Box {...props} sx={{ p: 2, borderBottom: "2px solid", borderColor: green }}>
    <Head2>{ translate(text, { _: text }) }</Head2>
    { props.children }
  </Box>)
}

interface BulletPointInterface {
  label: any,
  icon?: any,
  children?: ReactElement,
  noGutter?: boolean,
}

export const BulletPoint = ({label, icon, children, noGutter} : BulletPointInterface) => {
  const translate = useTranslate();
  const opts = { sx: { my: (noGutter ? 0 : 0.5) } };
  const text = _.isString(label) ? translate(label) : label;

  return (<Box { ...opts } display="flex" alignItems="center">
    { icon || <KeyboardDoubleArrowRightIcon/> }
    <Typography variant="body2" sx={{ml:1}}>{text}</Typography>
    <Box component="div" sx={{ flex: 1 }} />
    { children }
  </Box>);
};

declare module '@mui/material/styles' {
  interface PaletteOptions {
    highlight?: {
      main?: string;
      light?: string;
      contrastText?: string;
      otherContrastText?: string;
    },
    inverted?: {
      main?: string;
      light?: string;
      contrastText?: string;
      otherContrastText?: string;
    };
  }
}

declare module '@mui/material/Chip' {
  interface ChipPropsColorOverrides {
    highlight: true;
    inverted: true;
  }
}

declare module '@mui/material/Button' {
  interface ButtonPropsColorOverrides {
    highlight: true;
    inverted: true;
  }
}

declare module '@mui/material/AppBar' {
  interface AppBarPropsColorOverrides {
    highlight: true;
    inverted: true;
  }
}

declare module '@mui/material/IconButton' {
  interface IconButtonPropsColorOverrides {
    highlight: true;
    inverted: true;
  }
}

declare module '@mui/material/LinearProgress' {
  interface LinearProgressPropsColorOverrides {
    highlight: true;
    inverted: true;
  }
}
