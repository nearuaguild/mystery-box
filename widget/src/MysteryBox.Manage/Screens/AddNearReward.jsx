const widget_owner_id = "evasive-dime.testnet";
const top_contract_id = 'coherent-rail.testnet';

const { logInfo, logError } = VM.require(`${widget_owner_id}/widget/Utils.Logger`);

logInfo('AddNearReward props', props);

const isValidPool = (pool) => {
  if (pool.capacity === undefined || pool.capacity === null) return false;

  const capacityNum = Number(pool.capacity);

  if (!Number.isInteger(capacityNum)) return false;

  if (capacityNum < 1) return false;

  if (pool.amount === undefined || pool.amount === null) return false;

  const amountNum = Number(pool.amount);

  if (Number.isNaN(amountNum)) return false;

  if (amountNum < 0.1) return false;

  return true;
};

const createPool = () => ({
  id: Date.now(),
  rarity: 'rare',
  capacity: '1',
  amount: '0.1',
});

State.init({
  pools: [createPool()],
});

const Svg = styled.svg`
  height: 36px;
  cursor: pointer;
`;

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
  font-size: 24px;
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
  flex-basis: 230px;

  @media (max-width: 512px) {
    flex-basis: 320px;
  }

  width: 100%;

  overflow-x: hidden;
  overflow-x: scroll;
`;

const WrapperMenuButton = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
`;

const MenuButton = styled.button`
  border: 0;
  border-radius: 10px;

  width: 45%;
  height: 36px;

  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;

  background: #638caf;
`;

const WrapperAddPoolButton = styled.div`
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
`;

const AddPoolButton = styled.button`
  border: 0;
  border-radius: 10px;

  width: 60%;
  height: 24px;

  font-family: 'Kodchasan', sans-serif;
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0em;
  text-align: center;
  color: #ffffff;
  text-transform: uppercase;

  background: #638caf33;
`;

const WrapperTable = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;

  width: 100%;
  margin-bottom: 16px;
`;

const deletePool = (id) => {
  logInfo('deleting pool', id);

  State.update((previousState) => {
    return {
      ...previousState,
      pools: previousState.pools.filter((pool) => pool.id !== id),
    };
  });
};

const addPool = () => {
  logInfo('add new pool');

  State.update((previousState) => {
    return {
      ...previousState,
      pools: [...previousState.pools, createPool()],
    };
  });
};

const updadePool = (id, field, value) => {
  logInfo(`updating pool.${field}`, id, value);

  State.update((previousState) => {
    const { pools } = previousState;

    const pool = pools.find((el) => el.id === id);

    if (pool === -1) return previousState;

    // unknown field
    if (pool[field] === 'undefined') {
      logError(
        `Can't update pool property ${field} since it doesn't exist`
      );

      return previousState;
    }

    pool[field] = value;

    return {
      ...previousState,
      pools,
    };
  });
};

const pools = state.pools || [];

const everyPoolIsValid = pools.every(isValidPool);

const shouldSubmitButtonBeDisabled = !everyPoolIsValid;

logInfo('shouldSubmitButtonBeDisabled', shouldSubmitButtonBeDisabled);

const submitTransactionToAddPools = () => {
  logInfo('submitTransactionToAddPools', pools);

  if (!everyPoolIsValid) return;

  const txn = pools.map((pool) => {
    const amount = Big(pool.amount).mul(1e24);

    const total = Big(pool.capacity).mul(amount);

    return {
      contractName: top_contract_id,
      methodName: 'add_near_reward',
      args: {
        quest_id: props.quest.quest_id,
        rarity: pool.rarity,
        capacity: pool.capacity,
        amount: amount.toFixed(),
      },
      gas: Big(10).pow(13), // 10 TGas
      deposit: Big(10).pow(21).mul(5).plus(total).toFixed(), // 0.005N + rewards
    };
  });

  Near.call(txn);
};

return (
  <>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.Title`}
      props={{
        text: 'Add Near Reward',
      }}
    />
    <WrapperMenu>
        <Widget
          src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.MenuHeader`}
          props={{
            title: props.quest?.title,
            subtitle: props.quest?.quest_id,
          }}
        />
      <MenuContent>
        {pools.map((pool) => (
          <WrapperTable key={`pool_unique_key_${pool.id}`}>
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.DeleteButton`}
              props={{
                onClick: () => deletePool(pool.id),
              }}
            />
            <Widget
              src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.NearPool`}
              props={{
                pool: {
                  rarity: pool.rarity,
                  capacity: pool.capacity,
                  amount: pool.amount,
                },
                onUpdate: (field, value) => {
                  updadePool(pool.id, field, value);
                },
              }}
            />
          </WrapperTable>
        ))}
        <WrapperAddPoolButton>
          <AddPoolButton onClick={addPool}>Add new pool</AddPoolButton>
        </WrapperAddPoolButton>
      </MenuContent>
    </WrapperMenu>
    <Widget
      src={`${widget_owner_id}/widget/MysteryBox.Manage.Components.SubmitButton`}
      props={{
        text: 'Submit',
        disabled: shouldSubmitButtonBeDisabled,
        onClick: submitTransactionToAddPools,
      }}
    />
  </>
);
