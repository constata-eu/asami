import {
  CardActions, CardContent, CardHeader,
  IconButton, Box, Button, Container, Paper, styled,
  Toolbar, Typography, Skeleton, useMediaQuery
} from '@mui/material';
import { makeXUrl,  } from '../lib/auth_provider';
import { ethers, parseUnits, formatEther, toBeHex, zeroPadValue, parseEther } from "ethers";
import {  useTranslate, } from 'react-admin';

import { BareLayout, DeckCard } from './layout';
import { Head1, Head2, CardTitle, BulletPoint, yellow, dark, red, light, green } from '../components/theme';

const CampaignListEmpty = ({loginAs}) => {
  const translate = useTranslate();
  return <DeckCard id="campaign-list-empty">
    <CardContent>
      <Head2>{ translate("out_of_campaigns.title") }</Head2>
      <Typography>{ translate("out_of_campaigns.message") }</Typography>
    </CardContent>
  </DeckCard>;
}
export default CampaignListEmpty;
