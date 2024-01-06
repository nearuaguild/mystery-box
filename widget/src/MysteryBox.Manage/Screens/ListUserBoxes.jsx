console.log('props', props);

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  flex-basis: 70%;

  width: 40%;

  @media (max-width: 1024px) {
    width: 54%;
  }

  @media (max-width: 786px) {
    width: 70%;
  }

  @media (max-width: 512px) {
    width: 90%;
  }
`;

const MenuTitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 28px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  margin: 0;
`;

const MenuSubtitle = styled.p`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: rgba(43, 204, 194, 1);
  margin: 0;
`;

const MenuHeader = styled.div``;
const MenuContent = styled.div`
  width: 80%;

  @media (max-width: 512px) {
    width: 90%;
  }

  display: flex;
  justify-content: space-between;
  flex-direction: column;

  flex-basis: 60%;
`;
const MenuFooter = styled.div`
  display: flex;
  flex-direction: column;
  width: 100%;

  flex-basis: 10%;
`;

const TableContainer = styled.div`
  display: flex;
  flex-direction: column;

  width: 100%;

  background: #27384c;

  border: 0;
  border-radius: 10px;
`;

const TableScrollContainer = styled.div`
  flex-basis: 200px;

  @media (max-width: 786px) {
    flex-basis: 270px;
  }

  @media (max-width: 512px) {
    flex-basis: 300px;
  }

  overflow-x: hidden;
  overflow-x: scroll;
`;

const RarityButton = styled.div`
  width: 14px;
  height: 14px;
  border-radius: 50%;
  border: 0;

  ${(props) => {
    if (props.rarity === 'rare') return `background: #1EA3AF;`;
    if (props.rarity === 'epic') return `background: #B263C3;`;
    if (props.rarity === 'legendary') return `background: #FBC70F;`;
  }}
`;

const HeaderRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;

  background: #334a5f;
  height: 30px;

  border: 0;
  border-radius: 10px;
`;

const TableRow = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 32px;

  flex-grow: 1;
  border-radius: 10px;

  ${(props) =>
    props.index % 2 === 1 ? 'background: rgba(24, 36, 50, 0.30);' : ''}
`;

const HeaderCell = styled.div`
  flex: 1;
  ${(props) => (props.wide ? 'flex-grow: 2;' : '')}

  text-align: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 300;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  display: flex;
  justify-content: space-evenly;
  align-items: center;
`;

const TableCell = styled.div`
  flex: 1;
  ${(props) => (props.wide ? 'flex-grow: 2;' : '')}

  text-align: center;

  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 500;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  display: flex;
  justify-content: center;
  align-items: center;
`;

const TextCell = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 11px;
  font-weight: 400;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  width: 110px;
  height: 21px;

  background: #18243280;
  border: 0;
  border-radius: 50px;
`;

const NumberCell = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  width: 80px;
  height: 21px;

  background: #18243280;
  border: 0;
  border-radius: 50px;
`;

const WrapperPoolButtons = styled.div`
  width: 100%;
  background: #202f3f;

  border-radius: 10px;
  height: 40px;

  display: flex;

  flex-basis: 15%;
`;

const PoolButton = styled.div`
  flex-basis: 50%;

  border-radius: 10px;
  text-align: center;

  color: #fff;
  font-family: Kodchasan;
  font-size: 12px;
  font-weight: 400;

  display: flex;
  justify-content: center;
  align-items: center;

  ${(props) =>
    props.active
      ? 'background: #27394B;'
      : `&:hover {
    cursor: pointer;
  }`}
`;

const accounts = props.accounts || [];

return (
  <>
    <Widget
      src="denbite.testnet/widget/MysteryBox.Manage.Components.Title"
      props={{
        text: 'List User Boxes',
      }}
    />
    <WrapperMenu>
      <MenuHeader>
        <MenuTitle>Contract Name</MenuTitle>
        <MenuSubtitle>{props.contract_id}</MenuSubtitle>
      </MenuHeader>
      <MenuContent>
        <TableContainer>
          <HeaderRow key={0}>
            <HeaderCell key={0} wide>
              Account
            </HeaderCell>
            <HeaderCell key={1}>All</HeaderCell>
            <HeaderCell key={2}>Opened</HeaderCell>
            <HeaderCell key={3}>Closed</HeaderCell>
          </HeaderRow>
          <TableScrollContainer>
            {accounts.map((account, index) => (
              <TableRow index={index + 1}>
                <TableCell key={0} wide>
                  {account.account_id}
                </TableCell>
                <TableCell key={1}>{account.boxes.length}</TableCell>
                <TableCell key={2}>
                  {
                    account.boxes.filter((box) => box.status.kind === 'claimed')
                      .length
                  }
                </TableCell>
                <TableCell key={3}>
                  {
                    account.boxes.filter(
                      (box) => box.status.kind === 'non_claimed'
                    ).length
                  }
                </TableCell>
              </TableRow>
            ))}
          </TableScrollContainer>
        </TableContainer>
      </MenuContent>
      <MenuFooter>{/* Filtering will be here */}</MenuFooter>
    </WrapperMenu>
  </>
);
