const widget_owner_id = "evasive-dime.testnet";

const { logInfo, logError } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);

logInfo('props', props);

State.init({
  activeKind: 'near',
});

const WrapperMenu = styled.div`
  background: rgba(24, 36, 50, 1);
  border: 0;
  border-radius: 30px;

  display: flex;
  flex-direction: column;
  justify-content: space-around;
  align-items: center;

  flex-basis: 80%;

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

  flex-grow: 1;

  width: 100%;

  background: #27384c;

  border: 0;
  border-radius: 10px;
`;

const TableScrollContainer = styled.div`
  flex-basis: 160px;

  @media (max-width: 786px) {
    flex-basis: 180px;
  }

  @media (max-width: 512px) {
    flex-basis: 200px;
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

  height: 100%;

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

const TextCell = styled.div`
  overflow: hidden;
  text-overflow: ellipsis;

  width: 80px;
`;

const WrapperPoolButtons = styled.div`
  width: 100%;
  background: #202f3f;

  border-radius: 10px;
  height: 32px;

  display: flex;

  margin-bottom: 8px;
`;

const PoolButton = styled.button`
  flex-basis: 50%;

  height: 100%;

  border: 0;
  border-radius: 10px;
  text-align: center;

  background: none;

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

const updateActiveKind = (kind) => {
  State.update({
    activeKind: kind,
  });
};

const getTitleForReward = (reward) => {
  switch (reward.kind) {
    case 'near': {
      const amount = Big(reward.amount).div(1e24).toFixed(2);

      return `${amount} NEAR`;
    }
    case 'non_fungible_token': {
      const metadata = Near.view(reward.contract_id, 'nft_metadata');

      return metadata?.name;
    }
    default:
      return '';
  }
};

const getAvailableForReward = (reward) => {
  switch (reward.kind) {
    case 'near': {
      return reward.available;
    }
    case 'non_fungible_token': {
      return (reward.token_ids || []).length;
    }
    default:
      return '';
  }
};

const getTotalForReward = (reward) => {
  switch (reward.kind) {
    case 'near': {
      return reward.total;
    }
    case 'non_fungible_token': {
      return reward.total;
    }
    default:
      return '';
  }
};

const rewards = (props.rewards || []).filter(
  (reward) => reward.kind === state.activeKind
);

return (
  <>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.Title`}
      props={{
        text: 'List Rewards',
      }}
    />
    <WrapperMenu>
      <Widget
        src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuHeader`}
        props={{
          title: props.contract?.title,
          subtitle: props.contract?.contract_id,
        }}
      />
      <MenuContent>
        <WrapperPoolButtons>
          <PoolButton
            active={state.activeKind === 'near'}
            disabled={state.activeKind === 'near'}
            onClick={() => updateActiveKind('near')}
          >
            NEAR Pool
          </PoolButton>
          <PoolButton
            active={state.activeKind === 'non_fungible_token'}
            disabled={state.activeKind === 'non_fungible_token'}
            onClick={() => updateActiveKind('non_fungible_token')}
          >
            NFT Pool
          </PoolButton>
        </WrapperPoolButtons>
        <TableContainer>
          <HeaderRow key={0}>
            <HeaderCell key={0} wide>
              Reward
            </HeaderCell>
            <HeaderCell key={1}>Total</HeaderCell>
            <HeaderCell key={2}>Available</HeaderCell>
          </HeaderRow>
          <TableScrollContainer>
            {rewards.map((reward, index) => (
              <TableRow index={index + 1}>
                <TableCell key={0} wide>
                  <div
                    style={{
                      display: 'flex',
                      justifyContent: 'center',
                      flexBasis: '14px',
                    }}
                  >
                    <RarityButton rarity={reward.rarity} />
                  </div>
                  <TextCell>{getTitleForReward(reward)}</TextCell>
                </TableCell>
                <TableCell key={1}>{getTotalForReward(reward)}</TableCell>
                <TableCell key={2}>{getAvailableForReward(reward)}</TableCell>
              </TableRow>
            ))}
          </TableScrollContainer>
        </TableContainer>
      </MenuContent>
      <MenuFooter>{/* Filtering will be here */}</MenuFooter>
    </WrapperMenu>
  </>
);
