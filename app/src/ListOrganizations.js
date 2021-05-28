import React, { useEffect, useState } from 'react';
import { Table, Grid, Button } from 'semantic-ui-react';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import { useSubstrate } from './substrate-lib';

function Main(props) {
  const { api } = useSubstrate();

  // The transaction submission status
  const [organizationsDetails, setOrganizationsDetails] = useState([]);
  const [organizationAddresses, setOrganizationAddresses] = useState([]);

  const { setOrgs } = props;

  useEffect(() => {
    let unsubscribe;

    const interval = setInterval(() => {
      if (unsubscribe) {
        // unsub from previous subs
        unsubscribe();
      }

      api.query.organizations.parameters.keys().then(orgs => {
        const orgAddresses = orgs.map(({ args: [orgId] }) => orgId.toHuman());
        setOrganizationAddresses(orgAddresses);

        api.query.organizations.parameters.multi(orgAddresses, (details) => {
          const orgsMap = orgAddresses.reduce((acc, address, index) => ({
            ...acc, [address]: details[index].toHuman()
          }), {});

          setOrganizationsDetails(orgsMap);
          setOrgs(orgsMap);
        }).then(unsub => {
          unsubscribe = unsub;
        }).catch(console.error);
      });
    }, 2000);

    return () => clearInterval(interval)
  }, [api.query.organizations.parameters]);

  return (
    <Grid.Column>
      <h1>Organizations</h1>
      <Table celled striped size='small'>
        <Table.Body>
          <Table.Row>
            <Table.Cell width={10}>
              <strong>Address</strong>
            </Table.Cell>
            <Table.Cell width={3}>
              <strong>Voting System</strong>
            </Table.Cell>
            <Table.Cell width={3}>
              <strong>Number of Executors</strong>
            </Table.Cell>
          </Table.Row>
          {organizationAddresses.map(orgAddr =>
            <Table.Row key={orgAddr}>
              <Table.Cell width={10}>
                <span style={{ display: 'inline-block', minWidth: '31em' }}>
                  {orgAddr}
                </span>
                <CopyToClipboard text={orgAddr}>
                  <Button
                    basic
                    circular
                    compact
                    size='mini'
                    color='blue'
                    icon='copy outline'
                  />
                </CopyToClipboard>
              </Table.Cell>
              <Table.Cell width={3}>{
                organizationsDetails && organizationsDetails[orgAddr] &&
                organizationsDetails[orgAddr].voting[0]
              }</Table.Cell>
              <Table.Cell width={3}>{
                organizationsDetails && organizationsDetails[orgAddr] &&
                organizationsDetails[orgAddr].executors.length
              }</Table.Cell>
            </Table.Row>
          )}
        </Table.Body>
      </Table>
    </Grid.Column>
  );
}

export default function ListOrganizations(props) {
  const { api } = useSubstrate();
  return api.query.organizations && api.query.organizations.counter
    ? <Main {...props} />
    : null;
}
