import { defaultTheme, useTranslate } from "react-admin";
import { alpha, Box, styled } from "@mui/material";

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
].map(
  (font) =>
    `@font-face {
    font-family: 'LeagueSpartan${font[1]}';
    font-weight: ${font[2]};
    src: local('LeagueSpartan'), local('LeagueSpartan-${font[1]}'), url("${font[0]}") format('truetype');
    unicodeRange: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF;
    ascent-override: 90%;
    descent-override: 20%;
    line-gap-override: 0%;
  }`,
);

fontFaces.push(` @font-face {
  font-family: 'Sacramento';
  font-weight: 500;
  src: local("Sacramento"), local("Sacramento-Regular"), url("${Sacramento}") format('truetype');
  unicodeRange: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF;
}`);

/*
Dark surface background:
radial-gradient(49% 81% at 45% 47%, #214255 0%, #0d314600 100%),
radial-gradient(113% 91% at 17% -2%, #4d5962 1%, #0000ff00 99%),
radial-gradient(142% 91% at 83% 7%, #147bb6b3 1%, #19303d 99%),
radial-gradient(142% 91% at -6% 74%, #4B6574 1%, #081b2763 99%),
radial-gradient(142% 91% at 111% 84%, #282426a8 0%, #533f4921 100%)
*/

/*
Light surface background:
radial-gradient(49% 81% at 45% 47%, #f8f4d745 0%, #e2dac34d 100%),
radial-gradient(113% 91% at 17% -2%, #e4ad0c0f 1%, #e2dac300 99%),
radial-gradient(142% 91% at 83% 7%, #e2dac300 1%, #fdeee2a8 99%),
radial-gradient(142% 91% at -6% 74%, #f8f7f2 1%, #f8f4d703 99%),
radial-gradient(142% 91% at 111% 84%, #e2dac303 0%, #fbfbfb 100%)

radial-gradient(49% 81% at 45% 47%, #f8f4d745 0%, #e2dac34d 100%),
radial-gradient(113% 91% at 17% -2%, #e4ad0c0f 1%, #e2dac300 99%),
radial-gradient(142% 91% at 83% 7%, #e2dac300 1%, #fde2f499 99%),
radial-gradient(142% 91% at -6% 74%, #f8f7f2 1%, #f8f4d703 99%),
radial-gradient(142% 91% at 111% 84%, #e2dac303 0%, #fbfbfb 100%)


radial-gradient(49% 81% at 45% 47%, #f8d7d745 0%, #ffeefd00 100%),
radial-gradient(113% 91% at 17% -2%, #e286bc17 1%, #e2c3d700 99%),
radial-gradient(142% 91% at 83% 7%, #e2c3e000 1%, #fde2f49c 99%),
radial-gradient(142% 91% at -6% 74%, #f8f7f2 1%, #f8f4d703 99%),
radial-gradient(142% 91% at 111% 84%, #e2dac303 0%, #fbfbfb 100%)
*/

export const light = "#fffcf6";
export const pink = "#cc1e7d";
export const red = "#e50062";
export const orange = "#d0774a";
export const lightBlue = "#4B6574";
export const lighterBlue = "#7196ab";
export const green = "#3a9780";
export const brightGreen = "#388e3c";
export const dark = "#1d3644";

const backgroundGradientRules = (opacity = 100) => {
  const yellow = alpha("#FFE203", (opacity * 27) / 100.0 / 100.0);
  const one = alpha("#FFADB9", opacity / 100);
  const two = alpha("#F8F4D7", opacity / 100);
  const three = alpha("#FE7296", opacity / 100);
  const four = alpha("#F6EDEE", opacity / 100);

  return {
    backgroundSize: "200% 100%",
    "@media (min-width:600px)": {
      backgroundSize: "100% 100%",
    },
    backgroundPosition: "0px 0px,0px 0px,0px 0px,0px 0px,0px 0px",
    backgroundImage: `
      radial-gradient(49% 81% at 45% 47%, ${yellow} 0%, #073AFF00 100%),
      radial-gradient(113% 91% at 17% -2%, ${one} 1%, #FF000000 99%),
      radial-gradient(142% 91% at 83% 7%, ${two} 1%, #FF000000 99%),
      radial-gradient(142% 91% at -6% 74%, ${three} 1%, #FF000000 99%),
      radial-gradient(142% 91% at 111% 84%, ${four} 0%, ${one} 100%)
      `,
  };
};

const paperBackground = {
  backgroundSize: "100% 100%",
  backgroundPosition: "0px 0px,0px 0px,0px 0px,0px 0px,0px 0px",
  backgroundImage: `
    radial-gradient(49% 81% at 45% 47%, #f8f4d745 0%, #e2dac34d 100%),
    radial-gradient(113% 91% at 17% -2%, #e4ad0c0f 1%, #e2dac300 99%),
    radial-gradient(142% 91% at 83% 7%, #e2dac300 1%, #fde2f499 99%),
    radial-gradient(142% 91% at -6% 74%, #f8f7f2 1%, #f8f4d703 99%),
    radial-gradient(142% 91% at 111% 84%, #e2dac303 0%, #fbfbfb 100%)
  `,
};

const theme = {
  ...defaultTheme,
  typography: {
    fontFamily: '"LeagueSpartanLight", serif',
  },
  palette: {
    mode: "light",
    text: {
      primary: dark,
    },
    background: {
      default: light,
      paper: light,
    },
    inverted: {
      main: light,
      contrastText: dark,
    },
    primary: {
      main: pink,
      contrastText: light,
    },
    secondary: {
      main: lightBlue,
      contrastText: light,
    },
    error: {
      main: red,
    },
    warning: {
      main: orange,
    },
    info: {
      main: lighterBlue,
    },
    success: {
      main: brightGreen,
    },
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          fontFamily: '"LeagueSpartanMedium", serif',
          "&.Mui-disabled": {
            background: "linear-gradient(to right, #f9dbd6, #fbe4d5)",
            color: "#d4b39e",
          },
        },
      },
    },
    MuiFilledInput: {
      styleOverrides: {
        root: {
          background: "linear-gradient(to right, #f9dbd6, #fbe4d5)",
          "&:hover": {
            background: "linear-gradient(to right, #f8cec6, #f9dcca)",
          },
          "&.Mui-focused": {
            background: "linear-gradient(to right, #f9dbd6, #fbe4d5)",
          },
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: paperBackground,
      },
    },
    MuiPaper: {
      styleOverrides: {
        root: ({ theme, ownerState }) => ({
          // Only apply to dialogs
          ...(ownerState.variant === "elevation" &&
            ownerState["data-mui-dialog"] &&
            paperBackground),
        }),
      },
    },
    MuiDialog: {
      styleOverrides: {
        paper: paperBackground,
      },
    },
    MuiAlert: {
      styleOverrides: {
        root: {
          "& .MuiAlert-message": {
            flex: "1 1 auto !important",
          },
        },
      },
    },
    MuiCssBaseline: {
      styleOverrides: {
        "&": `.twitter-tweet { color: 'red'; margin: 0 !important;} ${fontFaces.join("\n")}`,
        body: {
          ...backgroundGradientRules(100),
          backgroundAttachment: "fixed",
        },
      },
    },
    RaListToolbar: {
      styleOverrides: {
        root: {
          backgroundColor: "transparent !important",
          marginBottom: "1em",
          "& .MuiToolbar-root": {
            backgroundColor: "transparent !important",
          },
        },
      },
    },
    RaFilterForm: {
      styleOverrides: {
        root: {
          "& .RaFilterForm-filterFormInput .MuiFormControl-root": {
            marginBottom: 0,
          },
          "& .hide-filter": {
            marginBottom: "4px",
          },
        },
      },
    },
    RaSimpleShowLayout: {
      styleOverrides: {
        root: {
          "& .RaSimpleShowLayout-stack": {
            padding: "1em 0 0.5em 0",
            flexWrap: "wrap",
            gap: "2em",
          },
          "& .RaSimpleShowLayout-row": {
            margin: 0,
          },
          "& .RaLabeled-label": {
            color: lightBlue,
            fontSize: "0.8em !important",
            lineHeight: "0.9em",
            marginBottom: 0,
            fontFamily: '"LeagueSpartanBold"',
            textTransform: "uppercase",
          },
        },
      },
    },
    MuiBackdrop: {
      styleOverrides: {
        root: backgroundGradientRules(40),
      },
    },
  },
};

export default theme;

const baseHeadingStyles = (theme, smallSize, largeSize) => ({
  fontFamily: "'LeagueSpartanBold'",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  margin: 0,
  color: theme.palette.secondary.main,
  fontSize: smallSize,
  [theme.breakpoints.up("md")]: {
    fontSize: largeSize,
  },
});

export const Head1 = styled("h1")(({ theme }) =>
  baseHeadingStyles(theme, "50px", "60px"),
);

export const Head2 = styled("h2")(({ theme }) =>
  baseHeadingStyles(theme, "35px", "45px"),
);
export const Head3 = styled("h3")(({ theme }) =>
  baseHeadingStyles(theme, "25px", "35px"),
);

export const Lead = styled("p")(({ theme }) => {
  return {
    fontFamily: "'LeagueSpartanLight'",
    color: theme.palette.secondary.main,
    lineHeight: 1.2,
    fontSize: "15px",
    margin: 0,
    [theme.breakpoints.up("md")]: {
      fontSize: "20px",
    },
  };
});

export const CardTitle = ({ text, ...props }) => {
  const translate = useTranslate();

  return (
    <Box
      {...props}
      sx={{ p: 2, borderBottom: "2px solid", borderColor: green }}
    >
      <Head2>{translate(text, { _: text })}</Head2>
      {props.children}
    </Box>
  );
};

export const BigText = styled("h2")(({ theme }) => ({
  fontFamily: "'LeagueSpartanBold'",
  fontSize: "40px",
  lineHeight: "1.1em",
  letterSpacing: "-0.05em",
  margin: 0,
}));
