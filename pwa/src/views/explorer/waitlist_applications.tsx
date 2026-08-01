import {
  NumberInput,
  Datagrid,
  Edit,
  EditButton,
  BooleanInput,
  FormDataConsumer,
  List,
  SelectInput,
  SimpleForm,
  TextField,
  TextInput,
  DateField,
  useRecordContext,
  useTranslate,
} from "react-admin";
import { Button, Stack, Typography } from "@mui/material";
import { Link as RouterLink } from "react-router-dom";
import { Head2 } from "../../components/theme";

const statusChoices = [
  { id: "PENDING", name: "Pending" },
  { id: "REVIEWED", name: "Reviewed" },
  { id: "ACCEPTED", name: "Accepted" },
  { id: "REJECTED", name: "Rejected" },
];

export const WaitlistApplicationList = () => (
  <List
    filters={[
      <TextInput source="nameIlike" alwaysOn />,
      <TextInput source="emailIlike" alwaysOn />,
      <SelectInput source="statusEq" choices={statusChoices} alwaysOn />,
    ]}
    sort={{ field: "createdAt", order: "DESC" }}
  >
    <Datagrid bulkActionButtons={false}>
      <TextField source="name" />
      <TextField source="email" />
      <TextField source="status" />
      <DateField source="reviewedAt" showTime />
      <DateField source="createdAt" showTime />
      <EditButton />
    </Datagrid>
  </List>
);

const InviteApplicantButton = () => {
  const record = useRecordContext<any>();
  const comment = encodeURIComponent(record?.note || "");
  const name = encodeURIComponent(record?.name || "");

  return (
    <Button
      component={RouterLink}
      to={`/community/invite?name=${name}&comment=${comment}`}
      variant="outlined"
    >
      Invite this applicant
    </Button>
  );
};

export const WaitlistApplicationEdit = () => {
  const t = useTranslate();

  return (
    <Edit>
      <SimpleForm>
        <Head2>{t("waitlist.admin_review_title")}</Head2>
        <ApplicantSummary />
        <InviteApplicantButton />
        <SelectInput source="status" choices={statusChoices} />
        <TextInput source="reviewNotes" multiline minRows={4} fullWidth />
        <FormDataConsumer>{({ formData }) => (formData?.status === "ACCEPTED" ? <AcceptanceFields /> : null)}</FormDataConsumer>
      </SimpleForm>
    </Edit>
  );
};

const ApplicantSummary = () => {
  const record = useRecordContext<any>();
  const t = useTranslate();

  if (!record) {
    return null;
  }

  return (
    <Stack gap="0.25em" mb="1em">
      <Typography variant="body2">
        <strong>{t("waitlist.fields.name")}:</strong> {record.name}
      </Typography>
      <Typography variant="body2">
        <strong>{t("waitlist.fields.email")}:</strong> {record.email}
      </Typography>
      <Typography variant="body2">
        <strong>{t("waitlist.fields.link1")}:</strong> {record.link1}
      </Typography>
      <Typography variant="body2">
        <strong>{t("waitlist.fields.link2")}:</strong> {record.link2}
      </Typography>
      {record.link3 && (
        <Typography variant="body2">
          <strong>{t("waitlist.fields.link3")}:</strong> {record.link3}
        </Typography>
      )}
      <Typography variant="body2">
        <strong>{t("waitlist.fields.note")}:</strong> {record.note}
      </Typography>
      {record.inviteUrl && (
        <Typography variant="body2" sx={{ wordBreak: "break-all" }}>
          <strong>Invite URL:</strong> {record.inviteUrl}
        </Typography>
      )}
    </Stack>
  );
};

const AcceptanceFields = () => {
  const record = useRecordContext<any>();
  const t = useTranslate();

  if (!record) {
    return null;
  }

  return (
    <Stack gap="0.75em">
      <Head2>{t("community.invite_page_title")}</Head2>
      <TextInput
        source="accountName"
        label={t("community.invite_fields.name")}
        defaultValue={record.name}
        fullWidth
      />
      <Stack direction="row" gap="1em" flexWrap="wrap">
        <NumberInput source="alignment" label={t("community.invite_fields.alignment")} defaultValue={5} />
        <NumberInput source="trust" label={t("community.invite_fields.trust")} defaultValue={5} />
        <NumberInput source="contribution" label={t("community.invite_fields.contribution")} defaultValue={5} />
        <NumberInput source="advocacy" label={t("community.invite_fields.advocacy")} defaultValue={5} />
        <NumberInput source="judgment" label={t("community.invite_fields.judgment")} defaultValue={5} />
      </Stack>
      <TextInput
        source="rootstockJoinedAt"
        type="date"
        label={t("community.invite_fields.rootstock_joined_at")}
        fullWidth
      />
      <BooleanInput source="priority" label={t("explorer.accounts.priority_override")} />
      <TextInput
        source="comment"
        label={t("community.invite_fields.comment")}
        defaultValue={record.note}
        multiline
        minRows={3}
        fullWidth
      />
    </Stack>
  );
};
