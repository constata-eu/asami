import { ResourceContextProvider, SimpleShowLayout } from "react-admin";

export const AttributeTable = ({
  children,
  fontSize = "0.8em !important",
  ...props
}) => (
  <ResourceContextProvider value={props.resource}>
    <SimpleShowLayout
      sx={{
        padding: 0,
        alignSelf: "stretch",
        "& .RaSimpleShowLayout-stack": {
          margin: "0 !important",
          gap: "0.2em !important",
          padding: "0 !important",
          flexWrap: "none",
        },
        "& .RaLabeled-label": {
          fontSize: fontSize,
          lineHeight: "0.9em",
          marginBottom: 0,
          color: "text.primary",
          fontFamily: '"LeagueSpartanLight"',
          textTransform: "none",
        },
        "& .RaSimpleShowLayout-row.ra-field": {
          display: "flex",
          flexDirection: "row",
          justifyContent: "space-between",
          alignItems: "center",
          padding: 0,
          margin: 0,
        },
        "& .ra-field > p": {
          marginRight: 1,
          fontWeight: 500,
          display: "flex",
        },
        "& .ra-field > span": {
          margin: 0,
        },
      }}
      {...props}
    >
      {children}
    </SimpleShowLayout>
  </ResourceContextProvider>
);
