import { useEffect } from "react";
import { useSafeSetState, useTranslate } from "react-admin";
import LoadingButton from '@mui/lab/LoadingButton';
import { Alert, Box, Button, Card, CardActions, CardContent, Container, FormControl, FormHelperText, InputLabel, MenuItem, Select, Skeleton, TextField, Typography } from "@mui/material";
import { ethers, parseUnits, formatEther } from "ethers";
import schnorr from "bip-schnorr";
import { Buffer } from "buffer";
import Login from "./views/login";
import { useContracts } from "../../components/contracts_context";
import { Head1, Head2, BulletPoint, CardTitle } from '../../components/theme';
import LoginIcon from '@mui/icons-material/Login';
import _ from 'lodash';
import { LocalizationProvider } from '@mui/x-date-pickers';
import { DateField } from '@mui/x-date-pickers/DateField';
import { AdapterDayjs } from '@mui/x-date-pickers/AdapterDayjs'
import dayjs from 'dayjs';
import { nip19 } from 'nostr-tools'

import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';
import { Campaign } from '../../models';

const Dashboard = () => {
  //const [loading, setLoading] = useSafeSetState(true);
  //const [offers, setOffers] = useSafeSetState(true);

  return <Container maxWidth="md">
    <Head1 sx={{my:3}}>Hello Member!</Head1>
    <Typography my="1em">
      Check out these projects that need your repost.
    </Typography>

    <Typography my="1em">
      Price per X repost.
    </Typography>
    <Typography my="1em">
      Price per Instagram Repost.
    </Typography>
    <Typography my="1em">
      Price per Nostr repost.
    </Typography>
  </Container>;

  /*
  useEffect(() => {
    const load = async () => {
      const { asami, signer } = await useContracts();
      const all = await asami.getCampaigns();
      const socialNetworks = await asami.getSocialNetworks();
      setOffers( all.map((c, i) => new Campaign(c, i, socialNetworks).getOffersForCreator(signer.address) ).flat() );
      setLoading(false);
    }
    load();
  }, []);

  if(loading) {
    return <Container maxWidth="md">
      <Skeleton animation="wave" />
    </Container>;
  }

  return <Container maxWidth="md">
    <Head1 sx={{my:3}}>Hello Collaborator!</Head1>
    <Typography my="1em">
      This is your collaboration inbox, here you can manage the invitations and collect payments.
      <br/>
      Advertisers already know about you, and they will invite you to participate in their campaigns.
      <br/>
      The advertiser may request you submit proof of your publication, at your expense, we'll let you know if that happens.
    </Typography>

    <Card sx={{my:"3em"}}>
      <CardTitle text="Your collaborations" />
      <TableContainer>
        <Table sx={{ minWidth: 650 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell>Post Before</TableCell>
              <TableCell>Network</TableCell>
              <TableCell>Instructions</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            { offers.map((o) =>
              <TableRow
                key={`campaign-${o.terms.campaign.id}-${o.id}`}
                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
              >
                <TableCell> {dayjs.unix(Number(o.terms.campaign.startDate)).toString()}</TableCell>
                <TableCell>{o.terms.socialNetwork}</TableCell>
                <TableCell>{o.terms.instructions}</TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </TableContainer>
    </Card>
  </Container>;
  */
}

export default Dashboard;
