const widget_owner_id = 'evasive-dime.testnet';

const TableContainer = styled.div`
  display: flex;
  flex-direction: column;

  flex-basis: 80%;

  height: 70px;

  background: #27384c;

  border: 0;
  border-radius: 10px;
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

  flex-grow: 1;
`;

const TableCell = styled.div`
  flex: 1;
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

const NumberCell = styled.input`
  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #2bccc2;
  width: 80px;

  background: #18243280;
  border: 0;
  border-radius: 50px;

  ::-webkit-outer-spin-button,
  ::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  -moz-appearance: textfield;
`;

const pool = props.pool;
const onUpdate = props.onUpdate;

const updateRarity = (rarity) => () => onUpdate('rarity', rarity);
const updateCapacity = (e) => onUpdate('capacity', e.target.value);
const updateAmount = (e) => onUpdate('amount', e.target.value);

return (
  <TableContainer>
    <HeaderRow key={0}>
      <TableCell key={0}>Box Rarity</TableCell>
      <TableCell key={1}>Rewards</TableCell>
      <TableCell key={2}>NEAR Amount</TableCell>
    </HeaderRow>
    <TableRow key={1}>
      <TableCell key={0}>
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
          props={{
            rarity: 'rare',
            active: pool.rarity === 'rare',
            onClick: updateRarity('rare'),
            tooltip: 'Rare',
          }}
        />
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
          props={{
            rarity: 'epic',
            active: pool.rarity === 'epic',
            onClick: updateRarity('epic'),
            tooltip: 'Epic',
          }}
        />
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.RarityButton`}
          props={{
            rarity: 'legendary',
            active: pool.rarity === 'legendary',
            onClick: updateRarity('legendary'),
            tooltip: 'Legendary',
          }}
        />
      </TableCell>
      <TableCell key={1}>
        <NumberCell
          type="number"
          min="1"
          step="1"
          value={pool.capacity}
          onChange={updateCapacity}
        />
      </TableCell>
      <TableCell key={2}>
        <NumberCell
          type="number"
          min="0"
          lang="en"
          step="0.01"
          value={pool.amount}
          onChange={updateAmount}
        />
      </TableCell>
    </TableRow>
  </TableContainer>
);
