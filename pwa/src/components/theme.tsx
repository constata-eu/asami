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

const darkPink = '#fc6b70';
const regularPink = '#fc798f';
const darkBlue = '#13445d';

export default createTheme(
  merge({}, defaultTheme, {
  typography: {
    fontFamily: '"LeagueSpartanLight", serif',
    allVariants: {
      color: darkBlue,
    },
  },
  palette: {
    primary: {
      main: regularPink,
      light: '#fff',
      contrastText: '#fff',
      otherContrastText: '#000',
    },
    secondary: {
      main: '#1f4668',
    },
    inverted: {
      main: '#fafafa',
      light: '#13445d',
      contrastText: '#13445d',
      otherContrastText: '#000',
    },
    highlight: {
      main: darkBlue,
      light: '#fff',
      contrastText: '#fff',
      otherContrastText: '#000',
    },
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: fontFaces.join("\n")
    },
    MuiCard: {
      styleOverrides: {
        root: {
          boxShadow: `4px 3px 0px 3px ${regularPink}`,
          borderRadius: "0.9375rem !important",
          borderWidth: "2px",
          borderStyle: "solid",
          borderColor: darkPink,
        },
      }
    },
    MuiCardActions: {
      styleOverrides: {
        spacing: {
          margin: "0 1em 2em 1em ",
          padding: 0,
        },
      }
    },
    MuiChip: {
      styleOverrides: {
        root: {
          borderRadius: "4px 0 4px 0",
        },
      }
    },
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: "9999px",
          textTransform: "none",
          fontWeight: "400",
        },
        contained: {
          boxShadow: "none",
        },
        outlined: {
          border: "2px solid",
          '&:hover': {
            borderWidth: "2px",
          },
        },
      }
    },
    RaListToolbar: {
      styleOverrides: {
        root: {
          marginBottom: "1em",
          marginTop: "1em",
        },
      }
    },
    RaFilterForm: {
      styleOverrides: {
        root: {
          marginLeft: "1em",
        },
      }
    },
    RaList: {
      styleOverrides: {
        root: {
          "overflow": "auto",
          "& .RaList-content": {
            borderRadius: "0 !important",
            borderWidth: "0",
            boxShadow: "none !important",
          }
        },
      }
    },
    RaSimpleShowLayout: {
      styleOverrides: {
        root: {
          "& .RaSimpleShowLayout-stack": {
            flexDirection: "row",
            flexWrap: "wrap",
          },
          "& .RaSimpleShowLayout-row": {
            borderRadius: "4px",
            background: "#f0f0f0",
            padding: "0.5em",
            margin: "0.5em 0.5em 0 0",
          }
        },
      }
    },
    RaShow: {
      styleOverrides: {
        root: {
          "& pre": {
            whiteSpace: "break-spaces",
          },
          "& .RaShow-card": {
            borderRadius: "0 !important",
            borderWidth: "0",
            boxShadow: "none !important",
          }
        },
      }
    },
    MuiTableBody: {
      styleOverrides: {
        root: {
          "& .column-createdAt": {
            minWidth: 130,
          },
          "& .params": {
            display: "flex",
            flexWrap: "wrap",
          },
          "& .column-params": {
            minWidth: "300px",
            maxWidth: "400px",
          },
          "& .column-rowNumber": {
            maxWidth: "100px",
          },
          "& .column-statistics": {
            minWidth: "140px",
          },
          "& .column-copyLink": {
            display: "block",
            minWidth: "100px",
          }
        }
      }
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          background: "#fff",
          boxShadow: "none"
        },
      }
    }
  },
}));

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
  color: darkBlue,
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

  return(<Box {...props} sx={{ p: 2, borderBottom: "2px solid", borderColor: darkPink }}>
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
