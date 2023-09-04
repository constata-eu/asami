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

const Dashboard = () => {
  const [loading, setLoading] = useSafeSetState(true);
  const [campaigns, setCampaigns] = useSafeSetState(true);
  const [socialNetworks, setSocialNetworks] = useSafeSetState([]);

  useEffect(() => {
    const load = async () => {
      const { asami, signer } = await useContracts();
      const all = await asami.getCampaigns();
      setCampaigns(_.filter(all, (x) => x.advertiser == signer.address ));
      setSocialNetworks(await asami.getSocialNetworks());
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
    <Head1 sx={{my:3}}>Hello Advertiser!</Head1>
    <Typography mt="3em" mb="1em">
      You can invite influencers and content creators to advertise your company or project.
      <br/>
      The Asami infrastructure makes sure the process is transparent and reliable for you and for them.
    </Typography>
    <Button fullWidth href="#/advertiser/campaign_wizard" sx={{flexGrow: 1}} size="large" variant="contained">
      Create a new campaign
    </Button>

    <Card sx={{my:"3em"}}>
      <CardTitle text="Your campaigns" />
      <TableContainer>
        <Table sx={{ minWidth: 650 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell>Start Date</TableCell>
              <TableCell align="right">Network</TableCell>
              <TableCell align="right">Collaborator Count</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            { campaigns.map((c, i) =>
              <TableRow
                key={`campaign-${i}`}
                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
              >
                <TableCell component="th" scope="row"> {dayjs.unix(Number(c.startDate)).toString()}</TableCell>
                <TableCell align="right">{c.nostrTerms ? "Nostr" : socialNetworks[c.classicTerms.socialNetworkId] }</TableCell>
                <TableCell align="right">{c.nostrTerms ? c.nostrTerms.offers.length : c.classicTerms.offers.length }</TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </TableContainer>
    </Card>
  </Container>;
}

export default Dashboard;
